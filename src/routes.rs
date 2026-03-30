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
    let mut protected = Router::new()
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
        .route("/api/user/list", get(handlers::dropdown::list_users))
        .route("/api/group/list", get(handlers::dropdown::list_groups))
        .route("/api/vm/list", get(handlers::dropdown::list_vms))
        .route("/api/config/storage", get(handlers::dropdown::list_storage))
        .route("/api/role/list", get(handlers::dropdown::list_roles))
        .route(
            "/api/infrastructure/{vm_id}",
            get(handlers::dropdown::get_infrastructure),
        )
        .route(
            "/api/settings",
            get(handlers::settings::get_settings).put(handlers::settings::update_settings),
        );

    if state.username_admin.is_some() && state.password_admin.is_some() {
        tracing::info!("Basic auth is enabled");
        protected = protected.layer(middleware::from_fn_with_state(
            state.clone(),
            mw::auth_basic::basic_auth,
        ));
    } else {
        tracing::warn!("APP_USERNAME_ADMIN is not set -> basic auth is disabled");
    }

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
            tracing::warn!("APP_CORS_ORIGIN is not set -> CORS is disabled");
        }
    };

    app.with_state(state)
}
