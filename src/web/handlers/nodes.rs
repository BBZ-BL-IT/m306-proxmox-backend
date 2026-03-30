use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;

use crate::clients::nodes::NodesClient;
use crate::state::AppState;

pub async fn list_nodes(State(state): State<AppState>) -> impl IntoResponse {
    tracing::debug!("Fetching node list from Proxmox");
    match NodesClient::get_nodes(&state).await {
        Ok(data) => {
            let nodes: Vec<serde_json::Value> = data["data"]
                .as_array()
                .unwrap_or(&vec![])
                .iter()
                .filter_map(|entry| {
                    entry["node"]
                        .as_str()
                        .map(|name| serde_json::json!({ "node": name }))
                })
                .collect();

            tracing::debug!("Found {} nodes", nodes.len());
            (StatusCode::OK, Json(serde_json::json!(nodes))).into_response()
        }
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
