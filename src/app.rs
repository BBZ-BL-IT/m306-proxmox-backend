use std::net::SocketAddr;

use tower_http::cors::{Any, CorsLayer};

use crate::{clients::ProxmoxClient, config::AppConfig, state};

pub async fn run(config: AppConfig) -> anyhow::Result<()> {
    let state = state::State {
        proxmox: ProxmoxClient::new(
            config.proxmox_url,
            config.proxmox_token_id,
            config.proxmox_token_secret,
        ),
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = loomox_api::server::new(state).layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.server_port));
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tracing::info!("Listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
