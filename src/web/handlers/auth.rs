use axum::http::StatusCode;
use axum::response::IntoResponse;

pub async fn verify() -> impl IntoResponse {
    StatusCode::OK
}
