use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use std::path::Path;

use crate::state::AppState;
use crate::web::dto::Settings;

const SETTINGS_FILE: &str = "config/app_settings.json";

fn default_settings() -> Settings {
    Settings {
        role: "uro_bbzblit_PVELabUsers".to_string(),
        user_group_templates: "ugr_bbzblit_Lernende".to_string(),
        prefix_user_group: "ug".to_string(),
        prefix_resourcepool: "rp".to_string(),
        prefix_simple_zone: "sz".to_string(),
        prefix_vnets: "vn".to_string(),
        postfix_vnet_dmz: "DMZ".to_string(),
        postfix_vnet_lan: "LAN".to_string(),
        prefix_firewall: "fw".to_string(),
        vm_storage: "pvecephpool01".to_string(),
        template_storage: "templates".to_string(),
        wan_interface: "vmbr1".to_string(),
    }
}

/// GET /api/settings
pub async fn get_settings(State(_state): State<AppState>) -> impl IntoResponse {
    let settings = if Path::new(SETTINGS_FILE).exists() {
        match std::fs::read_to_string(SETTINGS_FILE) {
            Ok(content) => match serde_json::from_str::<Settings>(&content) {
                Ok(s) => s,
                Err(e) => {
                    tracing::warn!("Failed to parse settings file: {}", e);
                    default_settings()
                }
            },
            Err(e) => {
                tracing::warn!("Failed to read settings file: {}", e);
                default_settings()
            }
        }
    } else {
        default_settings()
    };

    (StatusCode::OK, Json(settings))
}

/// PUT /api/settings
pub async fn put_settings(
    State(_state): State<AppState>,
    Json(settings): Json<Settings>,
) -> impl IntoResponse {
    // Ensure config directory exists
    if let Err(e) = std::fs::create_dir_all("config") {
        tracing::error!("Failed to create config directory: {}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": format!("{}", e) })),
        )
            .into_response();
    }

    match serde_json::to_string_pretty(&settings) {
        Ok(json) => match std::fs::write(SETTINGS_FILE, json) {
            Ok(_) => (StatusCode::OK, Json(serde_json::json!({ "status": "saved" }))).into_response(),
            Err(e) => {
                tracing::error!("Failed to write settings: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({ "error": format!("{}", e) })),
                )
                    .into_response()
            }
        },
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": format!("{}", e) })),
        )
            .into_response(),
    }
}
