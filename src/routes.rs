use axum::{
    Router, middleware,
    routing::{delete, get, post, put},
};

use crate::state::AppState;
use crate::web::handlers;
use crate::web::mw;

pub fn build_routes(state: AppState) -> Router {
    // Protected API routes (require basic auth)
    let api = Router::new()
        // Umgebung erstellen (Lastenheft 2.2.2)
        .route("/config/create", post(handlers::config::create_environment))
        // Umgebung löschen (Lastenheft 2.2.3)
        .route("/environment/list", get(handlers::environment::list_environments))
        .route("/environment/delete", delete(handlers::environment::delete_environments))
        // Einstellungen (Lastenheft 2.2.1)
        .route("/settings", get(handlers::settings::get_settings))
        .route("/settings", put(handlers::settings::put_settings))
        // Dropdown-Daten
        .route("/user/list", get(handlers::dropdown::list_users))
        .route("/group/list", get(handlers::dropdown::list_groups))
        .route("/node/list", get(handlers::dropdown::list_nodes))
        .route("/vm/list", get(handlers::dropdown::list_vms))
        .route("/config/storage", get(handlers::dropdown::list_storage))
        .route("/role/list", get(handlers::dropdown::list_roles))
        .route("/infrastructure/{vm_id}", get(handlers::dropdown::get_infrastructure))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            mw::auth_basic::basic_auth,
        ));

    // Public routes
    let public = Router::new()
        .route("/health", get(handlers::health::check_health))
        .route("/auth/verify", get(handlers::auth::verify));

    Router::new()
        .merge(public)
        .nest("/api", api)
        .with_state(state)
}
