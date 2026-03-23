use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

use crate::clients::ProxmoxAPI;
use crate::state::AppState;
use crate::web::dto::{CreateUmgebungRequest, CreateUmgebungResponse};

pub async fn create_umgebung(
    State(state): State<AppState>,
    Json(body): Json<CreateUmgebungRequest>,
) -> impl IntoResponse {
    match ProxmoxAPI::create_umgebung(
        &state,
        &body.node,
        body.template_vm_id,
        body.new_vm_id,
        &body.name,
    )
    .await
    {
        Ok(data) => (
            StatusCode::OK,
            Json(CreateUmgebungResponse {
                success: true,
                data,
            }),
        )
            .into_response(),
        Err(e) => {
            tracing::error!("Failed to create Umgebung: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "success": false,
                    "error": format!("Failed to create Umgebung: {}", e)
                })),
            )
                .into_response()
        }
    }
}
