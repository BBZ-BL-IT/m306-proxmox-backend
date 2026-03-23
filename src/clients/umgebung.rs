use crate::state::AppState;

pub struct UmgebungClient;

impl UmgebungClient {
    /// Creates a new VM (Umgebung) in Proxmox by cloning a template via the Proxmox API.
    pub async fn create_umgebung(
        state: &AppState,
        node: &str,
        template_vm_id: u32,
        new_vm_id: u32,
        name: &str,
    ) -> Result<serde_json::Value, reqwest::Error> {
        let url = format!(
            "{}/api2/json/nodes/{}/qemu/{}/clone",
            state.proxmox_url, node, template_vm_id
        );

        let auth_header = format!(
            "PVEAPIToken={}={}",
            state.proxmox_token_id, state.proxmox_token_secret
        );

        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()?;

        let response = client
            .post(&url)
            .header("Authorization", &auth_header)
            .form(&[
                ("newid", new_vm_id.to_string()),
                ("name", name.to_string()),
                ("full", "1".to_string()),
            ])
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        Ok(response)
    }
}
