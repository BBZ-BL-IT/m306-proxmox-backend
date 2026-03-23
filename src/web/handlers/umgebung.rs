use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

use crate::clients::ProxmoxAPI;
use crate::state::AppState;
use crate::web::dto::CreateEnvironmentRequest;

pub async fn create_umgebung(
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

        // Clone firewall VM if enabled
        if body.global_infrastructure_setup.firewall_setup.firewall_enabled {
            if let Some(fw_vm_id) = body.global_infrastructure_setup.firewall_setup.firewall_vm_id {
                let new_vm_id = 900 + (i as u32);
                let fw_name = format!(
                    "fw_{}_{}_grp{}",
                    modulnumber.to_lowercase(),
                    class.to_lowercase().replace('-', ""),
                    i + 1
                );
                if let Err(e) = ProxmoxAPI::create_umgebung(
                    &state, node, fw_vm_id, new_vm_id, &fw_name,
                )
                .await
                {
                    tracing::error!("Failed to clone firewall VM for group {}: {}", group_name, e);
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

    (
        StatusCode::CREATED,
        Json(serde_json::json!({ "created": created_groups })),
    )
        .into_response()
}
