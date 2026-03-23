use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

use crate::clients::ProxmoxAPI;
use crate::state::AppState;
use crate::web::dto::{FirewallSetup, Group, InfrastructureResponse, Node, Role, Storage, Vm};

/// GET /api/user/list
pub async fn list_users(State(state): State<AppState>) -> impl IntoResponse {
    match ProxmoxAPI::list_users(&state).await {
        Ok(data) => {
            let users: Vec<String> = data
                .get("data")
                .and_then(|d| d.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|u| u.get("userid").and_then(|id| id.as_str()))
                        .map(|s| s.to_string())
                        .collect()
                })
                .unwrap_or_default();
            (StatusCode::OK, Json(users)).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to list users: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": format!("{}", e) })),
            )
                .into_response()
        }
    }
}

/// GET /api/group/list
pub async fn list_groups(State(state): State<AppState>) -> impl IntoResponse {
    match ProxmoxAPI::list_groups(&state).await {
        Ok(data) => {
            let groups: Vec<Group> = data
                .get("data")
                .and_then(|d| d.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|g| {
                            g.get("groupid")
                                .and_then(|id| id.as_str())
                                .map(|name| Group {
                                    group_name: name.to_string(),
                                })
                        })
                        .collect()
                })
                .unwrap_or_default();
            (StatusCode::OK, Json(groups)).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to list groups: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": format!("{}", e) })),
            )
                .into_response()
        }
    }
}

/// GET /api/node/list
pub async fn list_nodes(State(state): State<AppState>) -> impl IntoResponse {
    match ProxmoxAPI::list_nodes(&state).await {
        Ok(data) => {
            let nodes: Vec<Node> = data
                .get("data")
                .and_then(|d| d.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|n| {
                            n.get("node")
                                .and_then(|id| id.as_str())
                                .map(|name| Node {
                                    node: name.to_string(),
                                })
                        })
                        .collect()
                })
                .unwrap_or_default();
            (StatusCode::OK, Json(nodes)).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to list nodes: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": format!("{}", e) })),
            )
                .into_response()
        }
    }
}

/// GET /api/vm/list
pub async fn list_vms(State(state): State<AppState>) -> impl IntoResponse {
    match ProxmoxAPI::list_vms(&state).await {
        Ok(data) => {
            let vms: Vec<Vm> = data
                .get("data")
                .and_then(|d| d.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| {
                            let vmid = v.get("vmid").and_then(|id| id.as_u64())? as u32;
                            let name = v
                                .get("name")
                                .and_then(|n| n.as_str())
                                .unwrap_or_default()
                                .to_string();
                            let node = v
                                .get("node")
                                .and_then(|n| n.as_str())
                                .unwrap_or_default()
                                .to_string();
                            Some(Vm { vmid, name, node })
                        })
                        .collect()
                })
                .unwrap_or_default();
            (StatusCode::OK, Json(vms)).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to list VMs: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": format!("{}", e) })),
            )
                .into_response()
        }
    }
}

/// GET /api/config/storage
pub async fn list_storage(State(state): State<AppState>) -> impl IntoResponse {
    match ProxmoxAPI::list_storage(&state).await {
        Ok(data) => {
            let storages: Vec<Storage> = data
                .get("data")
                .and_then(|d| d.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|s| {
                            let storage = s
                                .get("storage")
                                .and_then(|v| v.as_str())?
                                .to_string();
                            let storage_type = s
                                .get("type")
                                .and_then(|v| v.as_str())
                                .unwrap_or_default()
                                .to_string();
                            let content = s
                                .get("content")
                                .and_then(|v| v.as_str())
                                .unwrap_or_default()
                                .to_string();
                            let active = s.get("active").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
                            let shared = s.get("shared").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
                            let total = s.get("total").and_then(|v| v.as_u64()).unwrap_or(0);
                            let used = s.get("used").and_then(|v| v.as_u64()).unwrap_or(0);
                            let avail = s.get("avail").and_then(|v| v.as_u64()).unwrap_or(0);
                            Some(Storage {
                                storage,
                                storage_type,
                                content,
                                active,
                                shared,
                                total,
                                used,
                                avail,
                            })
                        })
                        .collect()
                })
                .unwrap_or_default();
            (StatusCode::OK, Json(storages)).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to list storage: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": format!("{}", e) })),
            )
                .into_response()
        }
    }
}

/// GET /api/role/list
pub async fn list_roles(State(state): State<AppState>) -> impl IntoResponse {
    match ProxmoxAPI::list_roles(&state).await {
        Ok(data) => {
            let roles: Vec<Role> = data
                .get("data")
                .and_then(|d| d.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|r| {
                            r.get("roleid")
                                .and_then(|id| id.as_str())
                                .map(|name| Role {
                                    role: name.to_string(),
                                })
                        })
                        .collect()
                })
                .unwrap_or_default();
            (StatusCode::OK, Json(roles)).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to list roles: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": format!("{}", e) })),
            )
                .into_response()
        }
    }
}

/// GET /api/infrastructure/{vm_id}
pub async fn get_infrastructure(
    State(state): State<AppState>,
    Path(vm_id): Path<u32>,
) -> impl IntoResponse {
    // First find which node the VM is on
    let vms_result = ProxmoxAPI::list_vms(&state).await;
    let node = match vms_result {
        Ok(data) => data
            .get("data")
            .and_then(|d| d.as_array())
            .and_then(|arr| {
                arr.iter().find_map(|v| {
                    let id = v.get("vmid").and_then(|id| id.as_u64())? as u32;
                    if id == vm_id {
                        v.get("node").and_then(|n| n.as_str()).map(|s| s.to_string())
                    } else {
                        None
                    }
                })
            }),
        Err(e) => {
            tracing::error!("Failed to find VM {}: {}", vm_id, e);
            None
        }
    };

    let Some(node) = node else {
        return (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": format!("VM {} not found", vm_id) })),
        )
            .into_response();
    };

    match ProxmoxAPI::get_vm_config(&state, &node, vm_id).await {
        Ok(data) => {
            let config = data.get("data").cloned().unwrap_or_default();
            // Extract firewall-related config from VM
            let net_count = (0..10)
                .filter(|i| config.get(format!("net{}", i)).is_some())
                .count() as u32;

            let response = InfrastructureResponse {
                firewall_setup: FirewallSetup {
                    firewall_enabled: true,
                    firewall_vm_id: Some(vm_id),
                    firewall_network_profile: Some(net_count),
                    apply_firewall_interfaces_config: false,
                },
            };
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to get VM config for {}: {}", vm_id, e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": format!("{}", e) })),
            )
                .into_response()
        }
    }
}
