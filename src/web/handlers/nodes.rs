use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

use crate::clients::nodes::NodesClient;
use crate::state::AppState;

pub async fn list_nodes(State(state): State<AppState>) -> impl IntoResponse {
    match NodesClient::get_nodes(&state).await {
        Ok(data) => (StatusCode::OK, Json(data)).into_response(),
        Err(e) => {
            tracing::error!("Failed to fetch nodes: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to fetch nodes: {}", e)
                })),
            )
                .into_response()
        }
    }
}
