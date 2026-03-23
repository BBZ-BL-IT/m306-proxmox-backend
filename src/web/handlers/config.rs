use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

use crate::clients::ProxmoxAPI;
use crate::state::AppState;
use crate::web::dto::CreateEnvironmentRequest;

/// POST /api/config/create
pub async fn create_environment(
    State(state): State<AppState>,
    Json(body): Json<CreateEnvironmentRequest>,
) -> impl IntoResponse {
    let node = &body.global_infrastructure_setup.node;
    let modulnumber = &body.modul_configuration.modulnumber;
    let class = &body.modul_configuration.class;

    let mut created_groups = Vec::new();

    for (i, group) in body.group_details.iter().enumerate() {
        let group_id = format!("{:03}", i + 1);
        let group_name = &group.group_name;

        // 1. Create user group
        if let Err(e) = ProxmoxAPI::create_group(&state, group_name).await {
            tracing::error!("Failed to create group {}: {}", group_name, e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to create group {}: {}", group_name, e)
                })),
            )
                .into_response();
        }

        // 2. Add members to group
        let members = group.userlist.join(",");
        if let Err(e) = ProxmoxAPI::set_group_members(&state, group_name, &members).await {
            tracing::error!("Failed to set members for group {}: {}", group_name, e);
        }

        // 3. Create resource pool
        let pool_name = format!("rp_{}_{}_grp{}", modulnumber.to_lowercase(), class.to_lowercase().replace('-', ""), i + 1);
        if let Err(e) = ProxmoxAPI::create_pool(&state, &pool_name).await {
            tracing::error!("Failed to create pool {}: {}", pool_name, e);
        }

        // 4. Create simple zone
        let zone_name = format!("sz{}", group_id);
        if let Err(e) = ProxmoxAPI::create_zone(&state, &zone_name).await {
            tracing::error!("Failed to create zone {}: {}", zone_name, e);
        }

        // 5. Create VNets (DMZ + LAN)
        let vnet_dmz = format!("vn{}DMZ", group_id);
        let vnet_lan = format!("vn{}LAN", group_id);
        if let Err(e) = ProxmoxAPI::create_vnet(&state, &vnet_dmz, &zone_name).await {
            tracing::error!("Failed to create vnet {}: {}", vnet_dmz, e);
        }
        if let Err(e) = ProxmoxAPI::create_vnet(&state, &vnet_lan, &zone_name).await {
            tracing::error!("Failed to create vnet {}: {}", vnet_lan, e);
        }

        // 6. Set ACL on resource pool
        if let Err(e) = ProxmoxAPI::set_acl(&state, &format!("/pool/{}", pool_name), "PVEVMUser", group_name).await {
            tracing::error!("Failed to set ACL for pool {}: {}", pool_name, e);
        }

        // 7. Clone firewall VM if enabled
        if body.global_infrastructure_setup.firewall_setup.firewall_enabled {
            if let Some(fw_vm_id) = body.global_infrastructure_setup.firewall_setup.firewall_vm_id {
                let new_vm_id = 900 + (i as u32);
                let fw_name = format!("fw_{}_{}_grp{}", modulnumber.to_lowercase(), class.to_lowercase().replace('-', ""), i + 1);
                if let Err(e) = ProxmoxAPI::clone_vm(&state, node, fw_vm_id, new_vm_id, &fw_name).await {
                    tracing::error!("Failed to clone firewall VM: {}", e);
                }
            }
        }

        created_groups.push(serde_json::json!({
            "group_id": group_id,
            "group_name": group_name,
            "resource_pool": pool_name,
            "simple_zone": zone_name,
            "vnets": [vnet_dmz, vnet_lan],
        }));
    }

    (
        StatusCode::CREATED,
        Json(serde_json::json!({ "created": created_groups })),
    )
        .into_response()
}
