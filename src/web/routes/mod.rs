pub mod proxmox_routes;

use axum::{middleware, Router};
use crate::app_state::AppState;
use crate::infrastructure::web::routes::proxmox_routes::proxmox_routes;

pub fn create_router() -> Router<AppState> {
    Router::new()
		.nest("/api", proxmox_routes())
}
