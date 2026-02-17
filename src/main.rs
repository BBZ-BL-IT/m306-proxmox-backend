mod app;
mod clients;
mod config;
mod errors;
mod routes;
mod state;
mod web;

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();

    setup_tracing();

    let cfg = config::AppConfig::load().expect("Failed to load config");

    if let Err(e) = app::run(cfg).await {
        tracing::error!("Application error: {}", e);
        std::process::exit(1);
    }
}

fn setup_tracing() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
}
