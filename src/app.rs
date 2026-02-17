use std::net::SocketAddr;

use crate::{config::AppConfig, state, web};

pub async fn run(config: AppConfig) -> anyhow::Result<()> {
    let state = state::State {
        proxmox_url: config.proxmox_url,
    };

    let app = web::build_router(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.server_port));
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tracing::info!("Listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
