use axum::Json;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;

use crate::clients::environment::EnvironmentClient;
use crate::state::AppState;
use crate::web::dto::{CreateEnvironmentRequest, DeleteEnvironmentRequest, ListEnvironmentParams};

struct ParsedEnvironment {
    group_id: String,
    group_name: String,
    module: String,
    class: String,
    suffix: String,
}

fn parse_group_name(group_name: &str) -> Option<ParsedEnvironment> {
    let stripped = group_name.strip_prefix("ugr_")?;
    let grp_pos = stripped.rfind("_grp")?;
    let grp_num: u32 = stripped[grp_pos + 4..].parse().ok()?;

    let prefix_part = &stripped[..grp_pos];
    let underscore_pos = prefix_part.find('_')?;
    let module_raw = &prefix_part[..underscore_pos];
    let class_raw = &prefix_part[underscore_pos + 1..];

    Some(ParsedEnvironment {
        group_id: format!("{:03}", grp_num),
        group_name: group_name.to_string(),
        module: module_raw.to_uppercase(),
        class: class_raw.to_string(),
        suffix: stripped.to_string(),
    })
}

pub async fn list_environments(
    State(state): State<AppState>,
    Query(params): Query<ListEnvironmentParams>,
) -> impl IntoResponse {
    tracing::debug!(
        module = ?params.module,
        class = ?params.class,
        group_id = ?params.group_id,
        "Listing environments"
    );
    let groups_response = match EnvironmentClient::list_groups(&state).await {
        Ok(data) => data,
        Err(e) => {
            tracing::error!("Failed to list groups: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": format!("Failed to list groups: {}", e) })),
            )
                .into_response();
        }
    };

    let groups = groups_response["data"]
        .as_array()
        .cloned()
        .unwrap_or_default();
    let mut environments = Vec::new();

    for group in &groups {
        let group_name = match group["groupid"].as_str() {
            Some(name) => name,
            None => continue,
        };

        let parsed = match parse_group_name(group_name) {
            Some(p) => p,
            None => continue,
        };

        if let Some(ref module_filter) = params.module {
            if !parsed.module.eq_ignore_ascii_case(module_filter) {
                continue;
            }
        }
        if let Some(ref class_filter) = params.class {
            if parsed.class != class_filter.to_lowercase().replace('-', "") {
                continue;
            }
        }
        if let Some(ref gid_filter) = params.group_id {
            if parsed.group_id != *gid_filter {
                continue;
            }
        }

        let resource_pool = format!("rp_{}", parsed.suffix);
        let simple_zone = format!("sz{}", parsed.group_id);
        let vnets = vec![
            format!("vn{}DMZ", parsed.group_id),
            format!("vn{}LAN", parsed.group_id),
        ];

        let members: Vec<String> = match EnvironmentClient::get_group(&state, group_name).await {
            Ok(data) => data["data"]["members"]
                .as_array()
                .cloned()
                .unwrap_or_default()
                .iter()
                .filter_map(|m| m.as_str().map(|s| s.to_string()))
                .collect(),
            Err(_) => vec![],
        };

        let mut vms: Vec<u32> = Vec::new();
        let mut node = String::new();
        if let Ok(pool_data) = EnvironmentClient::get_pool(&state, &resource_pool).await {
            if let Some(pool_members) = pool_data["data"]["members"].as_array() {
                for member in pool_members {
                    if member["type"].as_str() == Some("qemu") {
                        if let Some(vmid) = member["vmid"].as_u64() {
                            vms.push(vmid as u32);
                        }
                        if node.is_empty() {
                            if let Some(n) = member["node"].as_str() {
                                node = n.to_string();
                            }
                        }
                    }
                }
            }
        }

        environments.push(serde_json::json!({
            "group_id": parsed.group_id,
            "group_name": parsed.group_name,
            "module": parsed.module,
            "class": parsed.class,
            "node": node,
            "resource_pool": resource_pool,
            "simple_zone": simple_zone,
            "vnets": vnets,
            "vms": vms,
            "members": members,
        }));
    }

    tracing::debug!("Returning {} environments", environments.len());
    (StatusCode::OK, Json(serde_json::json!(environments))).into_response()
}

pub async fn create_environment(
    State(state): State<AppState>,
    Json(body): Json<CreateEnvironmentRequest>,
) -> impl IntoResponse {
    let node = &body.global_infrastructure_setup.node;
    let modulnumber = &body.modul_configuration.modulnumber;
    let class = &body.modul_configuration.class;
    tracing::debug!(
        node,
        modulnumber,
        class,
        groups = body.group_details.len(),
        firewall = body
            .global_infrastructure_setup
            .firewall_setup
            .firewall_enabled,
        "Creating environment"
    );

    let mut created_groups = Vec::new();

    for (i, group) in body.group_details.iter().enumerate() {
        let group_id = format!("{:03}", i + 1);
        let group_name = &group.group_name;

        if body
            .global_infrastructure_setup
            .firewall_setup
            .firewall_enabled
        {
            if let Some(fw_vm_id) = body
                .global_infrastructure_setup
                .firewall_setup
                .firewall_vm_id
            {
                let new_vm_id = 900 + (i as u32);
                let fw_name = format!(
                    "fw_{}_{}_grp{}",
                    modulnumber.to_lowercase(),
                    class.to_lowercase().replace('-', ""),
                    i + 1
                );
                if let Err(e) = EnvironmentClient::create_environment(
                    &state, node, fw_vm_id, new_vm_id, &fw_name,
                )
                .await
                {
                    tracing::error!(
                        "Failed to clone firewall VM for group {}: {}",
                        group_name,
                        e
                    );
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({
                            "error": format!("Failed to clone firewall VM: {}", e)
                        })),
                    )
                        .into_response();
                }
            }
        }

        created_groups.push(serde_json::json!({
            "group_id": group_id,
            "group_name": group_name,
            "members": group.userlist,
        }));
    }

    tracing::debug!("Created {} groups", created_groups.len());
    (
        StatusCode::CREATED,
        Json(serde_json::json!({ "created": created_groups })),
    )
        .into_response()
}

pub async fn delete_environment(
    State(state): State<AppState>,
    Json(body): Json<DeleteEnvironmentRequest>,
) -> impl IntoResponse {
    tracing::debug!(group_ids = ?body.group_ids, "Deleting environments");
    let groups_response = match EnvironmentClient::list_groups(&state).await {
        Ok(data) => data,
        Err(e) => {
            tracing::error!("Failed to list groups: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": format!("Failed to list groups: {}", e) })),
            )
                .into_response();
        }
    };

    let groups = groups_response["data"]
        .as_array()
        .cloned()
        .unwrap_or_default();

    let mut deleted: Vec<serde_json::Value> = Vec::new();
    let mut errors: Vec<serde_json::Value> = Vec::new();

    for target_group_id in &body.group_ids {
        let matching_env = groups.iter().find_map(|g| {
            let name = g["groupid"].as_str()?;
            let parsed = parse_group_name(name)?;
            if parsed.group_id == *target_group_id {
                Some(parsed)
            } else {
                None
            }
        });

        let parsed = match matching_env {
            Some(p) => p,
            None => {
                errors.push(serde_json::json!({
                    "group_id": target_group_id,
                    "message": format!("No environment found for group_id {}", target_group_id)
                }));
                continue;
            }
        };

        let resource_pool = format!("rp_{}", parsed.suffix);
        let simple_zone = format!("sz{}", parsed.group_id);
        let vnet_dmz = format!("vn{}DMZ", parsed.group_id);
        let vnet_lan = format!("vn{}LAN", parsed.group_id);

        let mut vm_ids: Vec<u32> = Vec::new();
        let mut node = String::new();
        if let Ok(pool_data) = EnvironmentClient::get_pool(&state, &resource_pool).await {
            if let Some(members) = pool_data["data"]["members"].as_array() {
                for member in members {
                    if member["type"].as_str() == Some("qemu") {
                        if let Some(vmid) = member["vmid"].as_u64() {
                            vm_ids.push(vmid as u32);
                        }
                        if node.is_empty() {
                            if let Some(n) = member["node"].as_str() {
                                node = n.to_string();
                            }
                        }
                    }
                }
            }
        }

        let mut group_errors = Vec::new();

        tracing::debug!(
            group_id = %target_group_id,
            group_name = %parsed.group_name,
            vms = ?vm_ids,
            "Deleting environment resources"
        );

        for vm_id in &vm_ids {
            if !node.is_empty() {
                let _ = EnvironmentClient::stop_vm(&state, &node, *vm_id).await;
                if let Err(e) = EnvironmentClient::delete_vm(&state, &node, *vm_id).await {
                    group_errors.push(format!("Failed to delete VM {}: {}", vm_id, e));
                }
            }
        }

        for vnet in [&vnet_dmz, &vnet_lan] {
            if let Err(e) = EnvironmentClient::delete_vnet(&state, vnet).await {
                group_errors.push(format!("Failed to delete VNet {}: {}", vnet, e));
            }
        }

        if let Err(e) = EnvironmentClient::delete_zone(&state, &simple_zone).await {
            group_errors.push(format!("Failed to delete zone {}: {}", simple_zone, e));
        }

        if let Err(e) = EnvironmentClient::delete_pool(&state, &resource_pool).await {
            group_errors.push(format!("Failed to delete pool {}: {}", resource_pool, e));
        }

        if let Err(e) = EnvironmentClient::delete_group(&state, &parsed.group_name).await {
            group_errors.push(format!(
                "Failed to delete group {}: {}",
                parsed.group_name, e
            ));
        }

        if group_errors.is_empty() {
            deleted.push(serde_json::json!({
                "group_id": parsed.group_id,
                "status": "success",
                "deleted_resources": {
                    "vms": vm_ids,
                    "vnets": [vnet_dmz, vnet_lan],
                    "simple_zone": simple_zone,
                    "resource_pool": resource_pool,
                    "user_group": parsed.group_name,
                }
            }));
        } else {
            for msg in group_errors {
                errors.push(serde_json::json!({
                    "group_id": target_group_id,
                    "message": msg,
                }));
            }
        }
    }

    tracing::debug!(
        deleted = deleted.len(),
        errors = errors.len(),
        "Environment deletion complete"
    );
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "deleted": deleted,
            "errors": errors,
        })),
    )
        .into_response()
}
