use axum::{Router, routing::get};

mod handlers;

// TODO Add middleware
pub fn build_router() -> Router {
    Router::new().route("/health", get(handlers::health::check_health))
}
