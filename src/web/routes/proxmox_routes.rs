use axum::{routing::{get, post}, Router};
use http::header::{CACHE_CONTROL, PRAGMA};
use http::HeaderValue;
use tower_http::set_header::SetResponseHeaderLayer;
use crate::app_state::AppState;
use crate::infrastructure::web::handlers::auth_handler{login_handler, logout_handler, me_handler, oauth_callback_handler, refresh_handler};

pub fn auth_routes() -> Router<AppState> {
        Router::new()
            .route("/config/create",       get())
            .route("/config/update",       get())
            .route("/config/storage",       get())
            .route("/user/list",       get())
            .route("/group/list",       get())
            .route("/infrastructure/{vm_id}",       get())
            .route("/node/list",       get())
            .route("/vm/list",       get())
}
