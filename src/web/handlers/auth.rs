use axum::http::StatusCode;
use axum::response::IntoResponse;

pub async fn verify() -> impl IntoResponse {
    tracing::debug!("Auth verification successful");
    StatusCode::OK
}
