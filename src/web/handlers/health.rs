use axum::{Json, response::IntoResponse};
use reqwest::StatusCode;
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthResponse {
    status: &'static str,
    version: &'static str,
}

pub async fn check_health() -> impl IntoResponse {
    let response = HealthResponse {
        status: "ok",
        version: env!("CARGO_PKG_VERSION"),
    };

    (StatusCode::OK, Json(response))
}

