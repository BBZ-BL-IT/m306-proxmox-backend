use crate::state::AppState;

pub struct ProxmoxAPI;

impl ProxmoxAPI {
    fn client() -> Result<reqwest::Client, reqwest::Error> {
        reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
    }

    fn auth_header(state: &AppState) -> String {
        format!(
            "PVEAPIToken={}={}",
            state.proxmox_token_id, state.proxmox_token_secret
        )
    }

    /// POST /api2/json/nodes/{node}/qemu/{template_vm_id}/clone
    pub async fn clone_vm(
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

        let client = Self::client()?;
        let response = client
            .post(&url)
            .header("Authorization", Self::auth_header(state))
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

    /// GET /api2/json/access/users
    pub async fn list_users(state: &AppState) -> Result<serde_json::Value, reqwest::Error> {
        let url = format!("{}/api2/json/access/users", state.proxmox_url);
        let client = Self::client()?;
        client
            .get(&url)
            .header("Authorization", Self::auth_header(state))
            .send()
            .await?
            .json()
            .await
    }

    /// GET /api2/json/access/groups
    pub async fn list_groups(state: &AppState) -> Result<serde_json::Value, reqwest::Error> {
        let url = format!("{}/api2/json/access/groups", state.proxmox_url);
        let client = Self::client()?;
        client
            .get(&url)
            .header("Authorization", Self::auth_header(state))
            .send()
            .await?
            .json()
            .await
    }

    /// GET /api2/json/nodes
    pub async fn list_nodes(state: &AppState) -> Result<serde_json::Value, reqwest::Error> {
        let url = format!("{}/api2/json/nodes", state.proxmox_url);
        let client = Self::client()?;
        client
            .get(&url)
            .header("Authorization", Self::auth_header(state))
            .send()
            .await?
            .json()
            .await
    }

    /// GET /api2/json/cluster/resources?type=vm
    pub async fn list_vms(state: &AppState) -> Result<serde_json::Value, reqwest::Error> {
        let url = format!("{}/api2/json/cluster/resources?type=vm", state.proxmox_url);
        let client = Self::client()?;
        client
            .get(&url)
            .header("Authorization", Self::auth_header(state))
            .send()
            .await?
            .json()
            .await
    }

    /// GET /api2/json/storage
    pub async fn list_storage(state: &AppState) -> Result<serde_json::Value, reqwest::Error> {
        let url = format!("{}/api2/json/storage", state.proxmox_url);
        let client = Self::client()?;
        client
            .get(&url)
            .header("Authorization", Self::auth_header(state))
            .send()
            .await?
            .json()
            .await
    }

    /// GET /api2/json/access/roles
    pub async fn list_roles(state: &AppState) -> Result<serde_json::Value, reqwest::Error> {
        let url = format!("{}/api2/json/access/roles", state.proxmox_url);
        let client = Self::client()?;
        client
            .get(&url)
            .header("Authorization", Self::auth_header(state))
            .send()
            .await?
            .json()
            .await
    }

    /// GET /api2/json/nodes/{node}/qemu/{vm_id}/config — get VM config for firewall info
    pub async fn get_vm_config(
        state: &AppState,
        node: &str,
        vm_id: u32,
    ) -> Result<serde_json::Value, reqwest::Error> {
        let url = format!(
            "{}/api2/json/nodes/{}/qemu/{}/config",
            state.proxmox_url, node, vm_id
        );
        let client = Self::client()?;
        client
            .get(&url)
            .header("Authorization", Self::auth_header(state))
            .send()
            .await?
            .json()
            .await
    }

    /// POST /api2/json/access/groups — create user group
    pub async fn create_group(
        state: &AppState,
        groupid: &str,
    ) -> Result<serde_json::Value, reqwest::Error> {
        let url = format!("{}/api2/json/access/groups", state.proxmox_url);
        let client = Self::client()?;
        client
            .post(&url)
            .header("Authorization", Self::auth_header(state))
            .form(&[("groupid", groupid)])
            .send()
            .await?
            .json()
            .await
    }

    /// POST /api2/json/pools — create resource pool
    pub async fn create_pool(
        state: &AppState,
        poolid: &str,
    ) -> Result<serde_json::Value, reqwest::Error> {
        let url = format!("{}/api2/json/pools", state.proxmox_url);
        let client = Self::client()?;
        client
            .post(&url)
            .header("Authorization", Self::auth_header(state))
            .form(&[("poolid", poolid)])
            .send()
            .await?
            .json()
            .await
    }

    /// PUT /api2/json/access/acl — set ACL permissions
    pub async fn set_acl(
        state: &AppState,
        path: &str,
        roles: &str,
        groups: &str,
    ) -> Result<serde_json::Value, reqwest::Error> {
        let url = format!("{}/api2/json/access/acl", state.proxmox_url);
        let client = Self::client()?;
        client
            .put(&url)
            .header("Authorization", Self::auth_header(state))
            .form(&[
                ("path", path),
                ("roles", roles),
                ("groups", groups),
                ("propagate", "1"),
            ])
            .send()
            .await?
            .json()
            .await
    }

    /// PUT /api2/json/access/groups/{groupid} — add members to group
    pub async fn set_group_members(
        state: &AppState,
        groupid: &str,
        members: &str,
    ) -> Result<serde_json::Value, reqwest::Error> {
        let url = format!("{}/api2/json/access/groups/{}", state.proxmox_url, groupid);
        let client = Self::client()?;
        client
            .put(&url)
            .header("Authorization", Self::auth_header(state))
            .form(&[("members", members)])
            .send()
            .await?
            .json()
            .await
    }

    /// DELETE /api2/json/access/groups/{groupid}
    pub async fn delete_group(
        state: &AppState,
        groupid: &str,
    ) -> Result<serde_json::Value, reqwest::Error> {
        let url = format!("{}/api2/json/access/groups/{}", state.proxmox_url, groupid);
        let client = Self::client()?;
        client
            .delete(&url)
            .header("Authorization", Self::auth_header(state))
            .send()
            .await?
            .json()
            .await
    }

    /// DELETE /api2/json/pools/{poolid}
    pub async fn delete_pool(
        state: &AppState,
        poolid: &str,
    ) -> Result<serde_json::Value, reqwest::Error> {
        let url = format!("{}/api2/json/pools/{}", state.proxmox_url, poolid);
        let client = Self::client()?;
        client
            .delete(&url)
            .header("Authorization", Self::auth_header(state))
            .send()
            .await?
            .json()
            .await
    }

    /// POST /api2/json/nodes/{node}/qemu/{vmid}/status/stop
    pub async fn stop_vm(
        state: &AppState,
        node: &str,
        vmid: u32,
    ) -> Result<serde_json::Value, reqwest::Error> {
        let url = format!(
            "{}/api2/json/nodes/{}/qemu/{}/status/stop",
            state.proxmox_url, node, vmid
        );
        let client = Self::client()?;
        client
            .post(&url)
            .header("Authorization", Self::auth_header(state))
            .send()
            .await?
            .json()
            .await
    }

    /// DELETE /api2/json/nodes/{node}/qemu/{vmid}
    pub async fn delete_vm(
        state: &AppState,
        node: &str,
        vmid: u32,
    ) -> Result<serde_json::Value, reqwest::Error> {
        let url = format!(
            "{}/api2/json/nodes/{}/qemu/{}",
            state.proxmox_url, node, vmid
        );
        let client = Self::client()?;
        client
            .delete(&url)
            .header("Authorization", Self::auth_header(state))
            .send()
            .await?
            .json()
            .await
    }

    /// POST /api2/json/cluster/sdn/zones — create simple zone
    pub async fn create_zone(
        state: &AppState,
        zone: &str,
    ) -> Result<serde_json::Value, reqwest::Error> {
        let url = format!("{}/api2/json/cluster/sdn/zones", state.proxmox_url);
        let client = Self::client()?;
        client
            .post(&url)
            .header("Authorization", Self::auth_header(state))
            .form(&[("zone", zone), ("type", "simple")])
            .send()
            .await?
            .json()
            .await
    }

    /// DELETE /api2/json/cluster/sdn/zones/{zone}
    pub async fn delete_zone(
        state: &AppState,
        zone: &str,
    ) -> Result<serde_json::Value, reqwest::Error> {
        let url = format!("{}/api2/json/cluster/sdn/zones/{}", state.proxmox_url, zone);
        let client = Self::client()?;
        client
            .delete(&url)
            .header("Authorization", Self::auth_header(state))
            .send()
            .await?
            .json()
            .await
    }

    /// POST /api2/json/cluster/sdn/vnets — create VNet
    pub async fn create_vnet(
        state: &AppState,
        vnet: &str,
        zone: &str,
    ) -> Result<serde_json::Value, reqwest::Error> {
        let url = format!("{}/api2/json/cluster/sdn/vnets", state.proxmox_url);
        let client = Self::client()?;
        client
            .post(&url)
            .header("Authorization", Self::auth_header(state))
            .form(&[("vnet", vnet), ("zone", zone)])
            .send()
            .await?
            .json()
            .await
    }

    /// DELETE /api2/json/cluster/sdn/vnets/{vnet}
    pub async fn delete_vnet(
        state: &AppState,
        vnet: &str,
    ) -> Result<serde_json::Value, reqwest::Error> {
        let url = format!("{}/api2/json/cluster/sdn/vnets/{}", state.proxmox_url, vnet);
        let client = Self::client()?;
        client
            .delete(&url)
            .header("Authorization", Self::auth_header(state))
            .send()
            .await?
            .json()
            .await
    }
}
