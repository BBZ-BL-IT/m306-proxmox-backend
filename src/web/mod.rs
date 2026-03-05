use axum::{Router, routing::get};

use crate::state::State;

mod handlers;

// TODO Add middleware
pub fn build_router(state: State) -> Router {
    Router::new()
        .route("/health", get(handlers::health::check_health))
        .route("/api/nodes", get(handlers::proxmox::get_nodes))
        .route("/api/cluster/status", get(handlers::proxmox::get_cluster_status))
        .route("/api/cluster/resources", get(handlers::proxmox::get_resources))
        .with_state(state)
}
