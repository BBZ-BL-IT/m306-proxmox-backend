use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;

use crate::config::Settings;
use crate::state::AppState;

pub async fn get_settings(State(state): State<AppState>) -> impl IntoResponse {
    let settings = state.settings.read().unwrap().clone();
    (StatusCode::OK, Json(settings))
}

pub async fn update_settings(
    State(state): State<AppState>,
    Json(new_settings): Json<Settings>,
) -> impl IntoResponse {
    let path = std::path::Path::new("config/app_settings.json");

    if let Some(parent) = path.parent() {
        if let Err(e) = std::fs::create_dir_all(parent) {
            tracing::error!("Failed to create config directory: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    }

    let json = match serde_json::to_string_pretty(&new_settings) {
        Ok(json) => json,
        Err(e) => {
            tracing::error!("Failed to serialize settings: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    };

    if let Err(e) = std::fs::write(path, json) {
        tracing::error!("Failed to write settings file: {}", e);
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    *state.settings.write().unwrap() = new_settings;

    StatusCode::OK
}
