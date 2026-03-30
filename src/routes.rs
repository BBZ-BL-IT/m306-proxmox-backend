use axum::http::HeaderValue;
use axum::{
    Router, middleware,
    routing::{delete, get, post},
};
use tower_http::cors::{AllowOrigin, CorsLayer};

use crate::state::AppState;
use crate::web::handlers;
use crate::web::mw;

pub fn build_routes(state: AppState, cors_origin: Option<String>) -> Router {
    let protected = Router::new()
        .route("/api/auth/verify", get(handlers::auth::verify))
        .route(
            "/api/config/create",
            post(handlers::environment::create_environment),
        )
        .route(
            "/api/environment/list",
            get(handlers::environment::list_environments),
        )
        .route(
            "/api/environment/delete",
            delete(handlers::environment::delete_environment),
        )
        .route("/api/node/list", get(handlers::nodes::list_nodes))
        .route("/api/settings", get(handlers::settings::get_settings).put(handlers::settings::update_settings))
        //.layer(middleware::from_fn_with_state(
        //    state.clone(),
        //   mw::auth_basic::basic_auth,
        // ))
    ;

    let public = Router::new().route("/health", get(handlers::health::check_health));

    let mut app = Router::new().merge(public).merge(protected);

    match cors_origin {
        Some(origin) => {
            let origin: HeaderValue = origin.parse().expect("Invalid CORS origin value");
            let cors = CorsLayer::new()
                .allow_origin(AllowOrigin::exact(origin))
                .allow_methods(tower_http::cors::Any)
                .allow_headers(tower_http::cors::Any);
            app = app.layer(cors);
        }
        None => {
            tracing::warn!("APP_CORS_ORIGIN is not set — CORS is disabled");
        }
    };

    app
        .with_state(state)
}
