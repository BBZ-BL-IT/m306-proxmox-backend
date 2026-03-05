use axum::{
    extract::State,
    Json,
};
use crate::state::State as AppState;

pub async fn get_nodes(State(state): State<AppState>) -> Json<Vec<crate::clients::Node>> {
    match state.proxmox.get_nodes().await {
        Ok(nodes) => Json(nodes),
        Err(e) => {
            tracing::error!("Failed to get nodes: {}", e);
            Json(vec![])
        }
    }
}

pub async fn get_cluster_status(State(state): State<AppState>) -> Json<Vec<crate::clients::ClusterStatus>> {
    match state.proxmox.get_cluster_status().await {
        Ok(status) => Json(status),
        Err(e) => {
            tracing::error!("Failed to get cluster status: {}", e);
            Json(vec![])
        }
    }
}

pub async fn get_resources(State(state): State<AppState>) -> Json<Vec<crate::clients::ClusterResource>> {
    match state.proxmox.get_cluster_resources().await {
        Ok(resources) => Json(resources),
        Err(e) => {
            tracing::error!("Failed to get cluster resources: {}", e);
            Json(vec![])
        }
    }
}
