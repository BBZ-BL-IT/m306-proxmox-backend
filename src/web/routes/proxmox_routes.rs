use axum::{routing::{get, Router};
use crate::app_state::AppState;

pub fn proxmox_routes() -> Router<AppState> {
        Router::new()
            .route("/config/create", get())
            .route("/config/update", get())
            .route("/config/storage", get())
            .route("/user/list", get())
            .route("/group/list", get())
            .route("/infrastructure/{vm_id}", get())
            .route("/node/list", get())
            .route("/vm/list", get())
}
