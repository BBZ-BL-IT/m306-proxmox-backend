use axum::{Router, routing::get};

use crate::state::State;

mod handlers;

// TODO Add middleware
pub fn build_router(state: State) -> Router {
    Router::new().route("/health", get(handlers::health::check_health))
}
