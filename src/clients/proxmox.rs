use crate::state::AppState;

pub struct ProxmoxClient;

impl ProxmoxClient {
    fn auth_header(state: &AppState) -> String {
        format!(
            "PVEAPIToken={}={}",
            state.proxmox_token_id, state.proxmox_token_secret
        )
    }

    pub async fn list_users(state: &AppState) -> Result<serde_json::Value, reqwest::Error> {
        let url = format!("{}/api2/json/access/users", state.proxmox_url);

        state
            .http_client
            .get(&url)
            .header("Authorization", Self::auth_header(state))
            .send()
            .await?
            .json()
            .await
    }

    pub async fn list_groups(state: &AppState) -> Result<serde_json::Value, reqwest::Error> {
        let url = format!("{}/api2/json/access/groups", state.proxmox_url);

        state
            .http_client
            .get(&url)
            .header("Authorization", Self::auth_header(state))
            .send()
            .await?
            .json()
            .await
    }

    pub async fn list_vms(
        state: &AppState,
        node: &str,
    ) -> Result<serde_json::Value, reqwest::Error> {
        let url = format!("{}/api2/json/nodes/{}/qemu", state.proxmox_url, node);

        state
            .http_client
            .get(&url)
            .header("Authorization", Self::auth_header(state))
            .send()
            .await?
            .json()
            .await
    }

    pub async fn list_storage(state: &AppState) -> Result<serde_json::Value, reqwest::Error> {
        let url = format!("{}/api2/json/storage", state.proxmox_url);

        state
            .http_client
            .get(&url)
            .header("Authorization", Self::auth_header(state))
            .send()
            .await?
            .json()
            .await
    }

    pub async fn list_roles(state: &AppState) -> Result<serde_json::Value, reqwest::Error> {
        let url = format!("{}/api2/json/access/roles", state.proxmox_url);

        state
            .http_client
            .get(&url)
            .header("Authorization", Self::auth_header(state))
            .send()
            .await?
            .json()
            .await
    }

    pub async fn get_vm_firewall(
        state: &AppState,
        node: &str,
        vm_id: u32,
    ) -> Result<serde_json::Value, reqwest::Error> {
        let url = format!(
            "{}/api2/json/nodes/{}/qemu/{}/firewall/options",
            state.proxmox_url, node, vm_id
        );

        state
            .http_client
            .get(&url)
            .header("Authorization", Self::auth_header(state))
            .send()
            .await?
            .json()
            .await
    }

    pub async fn get_vm_config(
        state: &AppState,
        node: &str,
        vm_id: u32,
    ) -> Result<serde_json::Value, reqwest::Error> {
        let url = format!(
            "{}/api2/json/nodes/{}/qemu/{}/config",
            state.proxmox_url, node, vm_id
        );

        state
            .http_client
            .get(&url)
            .header("Authorization", Self::auth_header(state))
            .send()
            .await?
            .json()
            .await
    }
}
