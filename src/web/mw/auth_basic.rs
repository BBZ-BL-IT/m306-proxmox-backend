use axum::{
    extract::{Request, State},
    http::{StatusCode, header::AUTHORIZATION},
    middleware::Next,
    response::Response,
};
use base64::{Engine as _, prelude::BASE64_STANDARD};

use crate::state::AppState;

pub async fn basic_auth(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|v| v.to_str().ok());

    let Some(auth_header) = auth_header else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let Some(encoded) = auth_header.strip_prefix("Basic ") else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let decoded = BASE64_STANDARD
        .decode(encoded)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    let credentials = String::from_utf8(decoded).map_err(|_| StatusCode::UNAUTHORIZED)?;
    let Some((username, pass)) = credentials.split_once(':') else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let expected_user = state.username_admin.as_deref().unwrap_or_default();
    let expected_pass = state.password_admin.as_deref().unwrap_or_default();

    if username == expected_user && pass == expected_pass {
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

