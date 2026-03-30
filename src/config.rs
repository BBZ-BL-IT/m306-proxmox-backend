use config::{Config, ConfigError, Environment};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub struct AppConfig {
    pub server_port: u16,
    pub proxmox_url: String,
    pub proxmox_token_id: String,
    pub proxmox_token_secret: String,
    pub ssl_verify: bool,
    pub ssl_cert_path: Option<String>,
    pub username_admin: String,
    pub password_admin: String,
    pub cors_origin: Option<String>,
}

impl AppConfig {
    pub fn load() -> Result<Self, ConfigError> {
        let settings = Config::builder()
            .set_default("server_port", 3000)?
            .set_default("ssl_verify", true)?
            .add_source(Environment::with_prefix("APP").prefix_separator("_"))
            .build()?;

        settings.try_deserialize()
    }
}
