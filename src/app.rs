use std::net::SocketAddr;

use crate::{config::AppConfig, routes, state};

pub async fn run(config: AppConfig) -> anyhow::Result<()> {
    let state = state::AppState {
        proxmox_url: config.proxmox_url,
        proxmox_token_id: config.proxmox_token_id,
        proxmox_token_secret: config.proxmox_token_secret,
        username_admin: config.username_admin,
        password_admin: config.password_admin,
    };

    let app = routes::build_routes(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.server_port));
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tracing::info!("Listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
