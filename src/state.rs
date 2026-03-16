///
/// Here we will define the global appstate that will be shared
///
#[derive(Clone)]
pub struct AppState {
    pub proxmox_url: String,
    pub username_admin: String,
    pub password_admin: String,
}
