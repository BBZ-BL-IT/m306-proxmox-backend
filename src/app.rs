use std::net::SocketAddr;

use crate::{config::AppConfig, state, web};

pub async fn run(config: AppConfig) -> anyhow::Result<()> {
    let state = state::AppState {
        proxmox_url: config.proxmox_url,
        username_admin: config.username_admin,
        password_admin: config.password_admin,
    };

    let app = web::build_router(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.server_port));
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tracing::info!("Listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
