use std::sync::{Arc, Mutex, RwLock};

use rusqlite::Connection;

use crate::config::Settings;

#[derive(Clone)]
pub struct AppState {
    pub proxmox_url: String,
    pub proxmox_token_id: String,
    pub proxmox_token_secret: String,
    pub username_admin: String,
    pub password_admin: String,
    pub http_client: reqwest::Client,
    pub settings: Arc<RwLock<Settings>>,
    pub db: Arc<Mutex<Connection>>,
}
