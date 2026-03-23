#[derive(Clone)]
pub struct AppState {
    pub proxmox_url: String,
    pub proxmox_token_id: String,
    pub proxmox_token_secret: String,
    pub username_admin: String,
    pub password_admin: String,
}
