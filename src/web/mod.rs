use axum::{Router, routing::get};

use crate::state::AppState;

mod handlers;
mod mw;

// TODO Add middleware
pub fn build_router(state: AppState) -> Router {
    Router::new().route("/health", get(handlers::health::check_health))
}
