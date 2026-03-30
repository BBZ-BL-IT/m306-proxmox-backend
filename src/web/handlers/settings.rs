use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;

use crate::config::Settings;
use crate::db;
use crate::state::AppState;

pub async fn get_settings(State(state): State<AppState>) -> impl IntoResponse {
    tracing::debug!("Fetching current settings");
    let settings = state.settings.read().unwrap().clone();
    (StatusCode::OK, Json(settings))
}

pub async fn update_settings(
    State(state): State<AppState>,
    Json(new_settings): Json<Settings>,
) -> impl IntoResponse {
    tracing::debug!("Updating settings");
    let conn = state.db.lock().unwrap();
    if let Err(e) = db::save_settings(&conn, &new_settings) {
        tracing::error!("Failed to save settings to database: {}", e);
        return StatusCode::INTERNAL_SERVER_ERROR;
    }
    drop(conn);

    *state.settings.write().unwrap() = new_settings;
    tracing::debug!("Settings updated and persisted to database");

    StatusCode::OK
}
