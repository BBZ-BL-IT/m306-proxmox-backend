use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;

use crate::clients::nodes::NodesClient;
use crate::clients::proxmox::ProxmoxClient;
use crate::state::AppState;

pub async fn list_users(State(state): State<AppState>) -> impl IntoResponse {
    tracing::debug!("Fetching user list from Proxmox");
    match ProxmoxClient::list_users(&state).await {
        Ok(data) => {
            let users: Vec<String> = data["data"]
                .as_array()
                .unwrap_or(&vec![])
                .iter()
                .filter_map(|entry| {
                    entry["userid"]
                        .as_str()
                        .map(|uid| uid.split('@').next().unwrap_or(uid).to_string())
                })
                .collect();

            tracing::debug!("Found {} users", users.len());
            (StatusCode::OK, Json(serde_json::json!(users))).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to fetch users: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to fetch users: {}", e)
                })),
            )
                .into_response()
        }
    }
}

pub async fn list_groups(State(state): State<AppState>) -> impl IntoResponse {
    tracing::debug!("Fetching group list from Proxmox");
    match ProxmoxClient::list_groups(&state).await {
        Ok(data) => {
            let groups: Vec<serde_json::Value> = data["data"]
                .as_array()
                .unwrap_or(&vec![])
                .iter()
                .filter_map(|entry| {
                    entry["groupid"]
                        .as_str()
                        .map(|name| serde_json::json!({ "group_name": name }))
                })
                .collect();

            tracing::debug!("Found {} groups", groups.len());
            (StatusCode::OK, Json(serde_json::json!(groups))).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to fetch groups: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to fetch groups: {}", e)
                })),
            )
                .into_response()
        }
    }
}

pub async fn list_vms(State(state): State<AppState>) -> impl IntoResponse {
    tracing::debug!("Fetching VM list across all nodes");
    let nodes_result = NodesClient::get_nodes(&state).await;
    let node_names: Vec<String> = match nodes_result {
        Ok(data) => data["data"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|entry| entry["node"].as_str().map(|s| s.to_string()))
            .collect(),
        Err(e) => {
            tracing::error!("Failed to fetch nodes for VM list: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to fetch nodes: {}", e)
                })),
            )
                .into_response();
        }
    };

    let mut vms: Vec<serde_json::Value> = Vec::new();

    for node in &node_names {
        match ProxmoxClient::list_vms(&state, node).await {
            Ok(data) => {
                if let Some(entries) = data["data"].as_array() {
                    for entry in entries {
                        let vmid = entry["vmid"].as_u64().unwrap_or(0) as u32;
                        let name = entry["name"].as_str().unwrap_or("").to_string();
                        vms.push(serde_json::json!({
                            "vmid": vmid,
                            "name": name,
                            "node": node,
                        }));
                    }
                }
            }
            Err(e) => {
                tracing::error!("Failed to fetch VMs from node {}: {}", node, e);
            }
        }
    }

    tracing::debug!("Found {} VMs across {} nodes", vms.len(), node_names.len());
    (StatusCode::OK, Json(serde_json::json!(vms))).into_response()
}

pub async fn list_storage(State(state): State<AppState>) -> impl IntoResponse {
    tracing::debug!("Fetching storage list from Proxmox");
    match ProxmoxClient::list_storage(&state).await {
        Ok(data) => {
            let storages: Vec<serde_json::Value> = data["data"]
                .as_array()
                .unwrap_or(&vec![])
                .iter()
                .map(|entry| {
                    serde_json::json!({
                        "storage": entry["storage"].as_str().unwrap_or(""),
                        "type": entry["type"].as_str().unwrap_or(""),
                        "content": entry["content"].as_str().unwrap_or(""),
                        "active": entry["active"].as_u64().unwrap_or(0),
                        "shared": entry["shared"].as_u64().unwrap_or(0),
                        "total": entry["total"].as_u64().unwrap_or(0),
                        "used": entry["used"].as_u64().unwrap_or(0),
                        "avail": entry["avail"].as_u64().unwrap_or(0),
                    })
                })
                .collect();

            tracing::debug!("Found {} storage pools", storages.len());
            (StatusCode::OK, Json(serde_json::json!(storages))).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to fetch storage: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to fetch storage: {}", e)
                })),
            )
                .into_response()
        }
    }
}

pub async fn list_roles(State(state): State<AppState>) -> impl IntoResponse {
    tracing::debug!("Fetching role list from Proxmox");
    match ProxmoxClient::list_roles(&state).await {
        Ok(data) => {
            let roles: Vec<serde_json::Value> = data["data"]
                .as_array()
                .unwrap_or(&vec![])
                .iter()
                .filter_map(|entry| {
                    entry["roleid"]
                        .as_str()
                        .map(|name| serde_json::json!({ "role": name }))
                })
                .collect();

            tracing::debug!("Found {} roles", roles.len());
            (StatusCode::OK, Json(serde_json::json!(roles))).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to fetch roles: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to fetch roles: {}", e)
                })),
            )
                .into_response()
        }
    }
}

pub async fn get_infrastructure(
    State(state): State<AppState>,
    Path(vm_id): Path<u32>,
) -> impl IntoResponse {
    tracing::debug!(vm_id, "Fetching infrastructure for VM");
    let nodes_result = NodesClient::get_nodes(&state).await;
    let node_names: Vec<String> = match nodes_result {
        Ok(data) => data["data"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|entry| entry["node"].as_str().map(|s| s.to_string()))
            .collect(),
        Err(e) => {
            tracing::error!("Failed to fetch nodes: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to fetch nodes: {}", e)
                })),
            )
                .into_response();
        }
    };

    // Find the VM on one of the nodes
    let mut vm_node: Option<String> = None;
    for node in &node_names {
        if let Ok(data) = ProxmoxClient::list_vms(&state, node).await {
            if let Some(entries) = data["data"].as_array() {
                if entries
                    .iter()
                    .any(|e| e["vmid"].as_u64() == Some(vm_id as u64))
                {
                    vm_node = Some(node.clone());
                    break;
                }
            }
        }
    }

    let node = match vm_node {
        Some(n) => {
            tracing::debug!(vm_id, node = %n, "Found VM on node");
            n
        }
        None => {
            tracing::debug!(vm_id, "VM not found on any node");
            return (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "error": format!("VM {} not found", vm_id)
                })),
            )
                .into_response();
        }
    };

    // Fetch firewall options
    let firewall_enabled = match ProxmoxClient::get_vm_firewall(&state, &node, vm_id).await {
        Ok(data) => data["data"]["enable"].as_u64().unwrap_or(0) == 1,
        Err(_) => false,
    };

    // Count network interfaces from VM config to determine network profile
    let network_profile = match ProxmoxClient::get_vm_config(&state, &node, vm_id).await {
        Ok(data) => {
            let config = &data["data"];
            let mut nic_count = 0u32;
            for i in 0..10 {
                let key = format!("net{}", i);
                if config[&key].is_string() {
                    nic_count += 1;
                }
            }
            if nic_count > 0 { Some(nic_count) } else { None }
        }
        Err(_) => None,
    };

    (
        StatusCode::OK,
        Json(serde_json::json!({
            "firewall_setup": {
                "firewall_enabled": firewall_enabled,
                "firewall_vm_id": vm_id,
                "firewall_network_profile": network_profile,
                "apply_firewall_interfaces_config": firewall_enabled,
            }
        })),
    )
        .into_response()
}
