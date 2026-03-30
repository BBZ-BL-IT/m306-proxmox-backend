use std::net::SocketAddr;

use crate::{config::AppConfig, routes, state};

pub async fn run(config: AppConfig) -> anyhow::Result<()> {
    let http_client = build_http_client(&config)?;

    let state = state::AppState {
        proxmox_url: config.proxmox_url,
        proxmox_token_id: config.proxmox_token_id,
        proxmox_token_secret: config.proxmox_token_secret,
        username_admin: config.username_admin,
        password_admin: config.password_admin,
        http_client,
    };

    let app = routes::build_routes(state, config.cors_origin);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.server_port));
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tracing::info!("Listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}

fn build_http_client(config: &AppConfig) -> anyhow::Result<reqwest::Client> {
    let mut builder = reqwest::Client::builder();

    if !config.ssl_verify {
        tracing::warn!("SSL certificate verification is disabled");
        builder = builder.danger_accept_invalid_certs(true);
    } else if let Some(ref cert_path) = config.ssl_cert_path {
        let cert_pem = std::fs::read(cert_path)?;
        let cert = reqwest::tls::Certificate::from_pem(&cert_pem)?;
        builder = builder.add_root_certificate(cert);
        tracing::info!("Loaded SSL certificate from {}", cert_path);
    }

    Ok(builder.build()?)
}
