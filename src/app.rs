use std::net::SocketAddr;
use std::sync::{Arc, Mutex, RwLock};

use crate::{config::AppConfig, db, routes, state};

pub async fn run(config: AppConfig) -> anyhow::Result<()> {
    let http_client = build_http_client(&config)?;

    std::fs::create_dir_all("data")?;
    let conn = rusqlite::Connection::open("data/settings.db")?;
    db::init_db(&conn)?;

    let settings = db::load_settings(&conn)?
        .unwrap_or_else(|| config.settings());
    tracing::info!("Settings loaded");

    let state = state::AppState {
        proxmox_url: config.proxmox_url,
        proxmox_token_id: config.proxmox_token_id,
        proxmox_token_secret: config.proxmox_token_secret,
        username_admin: config.username_admin,
        password_admin: config.password_admin,
        http_client,
        settings: Arc::new(RwLock::new(settings)),
        db: Arc::new(Mutex::new(conn)),
    };

    let app = routes::build_routes(state);

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
