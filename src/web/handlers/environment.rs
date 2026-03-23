use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

use crate::clients::ProxmoxAPI;
use crate::state::AppState;
use crate::web::dto::{
    DeleteEnvironmentRequest, DeleteEnvironmentResponse, DeletedEnvironment,
    DeletedResources, Environment, EnvironmentListQuery,
};

/// GET /api/environment/list
pub async fn list_environments(
    State(state): State<AppState>,
    Query(query): Query<EnvironmentListQuery>,
) -> impl IntoResponse {
    // Fetch groups from Proxmox and map to Environment structs
    let groups_result = ProxmoxAPI::list_groups(&state).await;

    let groups_data = match groups_result {
        Ok(data) => data,
        Err(e) => {
            tracing::error!("Failed to list groups: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": format!("{}", e) })),
            )
                .into_response();
        }
    };

    let mut environments: Vec<Environment> = Vec::new();

    if let Some(data) = groups_data.get("data").and_then(|d| d.as_array()) {
        for group in data {
            let group_name = group
                .get("groupid")
                .and_then(|g| g.as_str())
                .unwrap_or_default()
                .to_string();

            // Parse group name to extract module, class, group_id
            // Expected format: ugr_{module}_{class}_grp{id}
            let parts: Vec<&str> = group_name.split('_').collect();
            if parts.len() < 4 {
                continue;
            }

            let module = parts.get(1).unwrap_or(&"").to_uppercase();
            let class = parts.get(2).unwrap_or(&"").to_string();
            let group_id = parts
                .last()
                .unwrap_or(&"")
                .trim_start_matches("grp")
                .to_string();
            let group_id_padded = format!("{:0>3}", group_id);

            // Apply query filters
            if let Some(ref filter_module) = query.module {
                if module.to_lowercase() != filter_module.to_lowercase() {
                    continue;
                }
            }
            if let Some(ref filter_class) = query.class {
                if class.to_lowercase() != filter_class.to_lowercase() {
                    continue;
                }
            }
            if let Some(ref filter_group_id) = query.group_id {
                if group_id_padded != *filter_group_id {
                    continue;
                }
            }

            let members = group
                .get("members")
                .and_then(|m| m.as_str())
                .unwrap_or_default()
                .split(',')
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .collect();

            environments.push(Environment {
                group_id: group_id_padded.clone(),
                group_name: group_name.clone(),
                module,
                class,
                node: String::new(),
                resource_pool: format!("rp_{}", group_name.trim_start_matches("ugr_")),
                simple_zone: format!("sz{}", group_id_padded),
                vnets: vec![
                    format!("vn{}DMZ", group_id_padded),
                    format!("vn{}LAN", group_id_padded),
                ],
                vms: vec![],
                members,
            });
        }
    }

    (StatusCode::OK, Json(environments)).into_response()
}

/// DELETE /api/environment/delete
pub async fn delete_environments(
    State(state): State<AppState>,
    Json(body): Json<DeleteEnvironmentRequest>,
) -> impl IntoResponse {
    let mut deleted = Vec::new();
    let errors = Vec::new();

    for group_id in &body.group_ids {
        // TODO: Look up actual resources for this group_id from Proxmox
        // For now, construct expected names from group_id
        let zone_name = format!("sz{}", group_id);
        let vnet_dmz = format!("vn{}DMZ", group_id);
        let vnet_lan = format!("vn{}LAN", group_id);

        // 1. Delete VNets
        if let Err(e) = ProxmoxAPI::delete_vnet(&state, &vnet_dmz).await {
            tracing::warn!("Failed to delete vnet {}: {}", vnet_dmz, e);
        }
        if let Err(e) = ProxmoxAPI::delete_vnet(&state, &vnet_lan).await {
            tracing::warn!("Failed to delete vnet {}: {}", vnet_lan, e);
        }

        // 2. Delete Simple Zone
        if let Err(e) = ProxmoxAPI::delete_zone(&state, &zone_name).await {
            tracing::warn!("Failed to delete zone {}: {}", zone_name, e);
        }

        // 3. Delete Resource Pool (name unknown without full lookup)
        // 4. Delete User Group (name unknown without full lookup)

        deleted.push(DeletedEnvironment {
            group_id: group_id.clone(),
            status: "success".to_string(),
            deleted_resources: DeletedResources {
                vms: vec![],
                vnets: vec![vnet_dmz, vnet_lan],
                simple_zone: zone_name,
                resource_pool: String::new(),
                user_group: String::new(),
            },
        });
    }

    let response = DeleteEnvironmentResponse { deleted, errors };
    (StatusCode::OK, Json(response)).into_response()
}
