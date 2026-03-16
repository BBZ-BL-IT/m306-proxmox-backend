use axum::{
    extract::Request,
    http::{StatusCode, header::AUTHORIZATION},
    middleware::Next,
    response::Response,
};
use base64::{Engine as _, prelude::BASE64_STANDARD};

use crate::state::AppState;

pub async fn basic_auth(req: Request, next: Next, state: AppState) -> Result<Response, StatusCode> {
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

    if username == state.username_admin && pass == state.password_admin {
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

