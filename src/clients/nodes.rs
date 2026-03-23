use crate::state::AppState;

pub struct NodesClient;

impl NodesClient {
    /// Fetches the list of nodes from Proxmox via GET /api2/json/nodes.
    pub async fn get_nodes(state: &AppState) -> Result<serde_json::Value, reqwest::Error> {
        let url = format!("{}/api2/json/nodes", state.proxmox_url);

        let auth_header = format!(
            "PVEAPIToken={}={}",
            state.proxmox_token_id, state.proxmox_token_secret
        );

        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()?;

        client
            .get(&url)
            .header("Authorization", &auth_header)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await
    }
}
