use config::{Config, ConfigError, Environment};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server_port: u16,
    pub proxmox_url: String,
}

impl AppConfig {
    pub fn load() -> Result<Self, ConfigError> {
        let _ = dotenvy::dotenv();

        let settings = Config::builder()
            .set_default("server_port", 3000)?
            .add_source(Environment::with_prefix("APP").separator("_"))
            .build()?;

        settings.try_deserialize()
    }
}
