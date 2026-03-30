use crate::state::AppState;

pub struct EnvironmentClient;

impl EnvironmentClient {
    fn auth_header(state: &AppState) -> String {
        format!(
            "PVEAPIToken={}={}",
            state.proxmox_token_id, state.proxmox_token_secret
        )
    }

    pub async fn create_environment(
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

        state
            .http_client
            .post(&url)
            .header("Authorization", Self::auth_header(state))
            .form(&[
                ("newid", new_vm_id.to_string()),
                ("name", name.to_string()),
                ("full", "1".to_string()),
            ])
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

    pub async fn get_group(
        state: &AppState,
        group_id: &str,
    ) -> Result<serde_json::Value, reqwest::Error> {
        let url = format!("{}/api2/json/access/groups/{}", state.proxmox_url, group_id);

        state
            .http_client
            .get(&url)
            .header("Authorization", Self::auth_header(state))
            .send()
            .await?
            .json()
            .await
    }

    pub async fn get_pool(
        state: &AppState,
        pool_id: &str,
    ) -> Result<serde_json::Value, reqwest::Error> {
        let url = format!("{}/api2/json/pools/{}", state.proxmox_url, pool_id);

        state
            .http_client
            .get(&url)
            .header("Authorization", Self::auth_header(state))
            .send()
            .await?
            .json()
            .await
    }

    pub async fn stop_vm(
        state: &AppState,
        node: &str,
        vm_id: u32,
    ) -> Result<serde_json::Value, reqwest::Error> {
        let url = format!(
            "{}/api2/json/nodes/{}/qemu/{}/status/stop",
            state.proxmox_url, node, vm_id
        );

        state
            .http_client
            .post(&url)
            .header("Authorization", Self::auth_header(state))
            .send()
            .await?
            .json()
            .await
    }

    pub async fn delete_vm(
        state: &AppState,
        node: &str,
        vm_id: u32,
    ) -> Result<serde_json::Value, reqwest::Error> {
        let url = format!(
            "{}/api2/json/nodes/{}/qemu/{}",
            state.proxmox_url, node, vm_id
        );

        state
            .http_client
            .delete(&url)
            .header("Authorization", Self::auth_header(state))
            .send()
            .await?
            .json()
            .await
    }

    pub async fn delete_vnet(
        state: &AppState,
        vnet: &str,
    ) -> Result<serde_json::Value, reqwest::Error> {
        let url = format!("{}/api2/json/cluster/sdn/vnets/{}", state.proxmox_url, vnet);

        state
            .http_client
            .delete(&url)
            .header("Authorization", Self::auth_header(state))
            .send()
            .await?
            .json()
            .await
    }

    pub async fn delete_zone(
        state: &AppState,
        zone: &str,
    ) -> Result<serde_json::Value, reqwest::Error> {
        let url = format!("{}/api2/json/cluster/sdn/zones/{}", state.proxmox_url, zone);

        state
            .http_client
            .delete(&url)
            .header("Authorization", Self::auth_header(state))
            .send()
            .await?
            .json()
            .await
    }

    pub async fn delete_pool(
        state: &AppState,
        pool_id: &str,
    ) -> Result<serde_json::Value, reqwest::Error> {
        let url = format!("{}/api2/json/pools/{}", state.proxmox_url, pool_id);

        state
            .http_client
            .delete(&url)
            .header("Authorization", Self::auth_header(state))
            .send()
            .await?
            .json()
            .await
    }

    pub async fn delete_group(
        state: &AppState,
        group_id: &str,
    ) -> Result<serde_json::Value, reqwest::Error> {
        let url = format!("{}/api2/json/access/groups/{}", state.proxmox_url, group_id);

        state
            .http_client
            .delete(&url)
            .header("Authorization", Self::auth_header(state))
            .send()
            .await?
            .json()
            .await
    }
}
