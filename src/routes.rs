use axum::{
    Router, middleware,
    routing::{get, post},
};

use crate::state::AppState;
use crate::web::handlers;
use crate::web::mw;

pub fn build_routes(state: AppState) -> Router {
    let protected = Router::new()
        .route("/auth/verify", get(handlers::auth::verify))
        .route(
            "/umgebung/create",
            post(handlers::umgebung::create_umgebung),
        )
        .layer(middleware::from_fn_with_state(
            state.clone(),
            mw::auth_basic::basic_auth,
        ));

    let public = Router::new().route("/health", get(handlers::health::check_health));

    Router::new()
        .merge(public)
        .merge(protected)
        .with_state(state)
}
