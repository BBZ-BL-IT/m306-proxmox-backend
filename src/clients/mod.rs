use reqwest::Client;
use serde::Deserialize;
use loomox_api::models;

/// Alias for results from provisioning operations.
pub type ProvisionResult<T = ()> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug, Clone)]
pub struct ProxmoxClient {
    http: Client,
    base_url: String,
    token_id: String,
    token_secret: String,
}

/// Proxmox API responses wrap data in a `{ "data": ... }` envelope.
#[derive(Debug, Deserialize)]
struct ApiResponse<T> {
    data: T,
}

/// Raw user entry as returned by Proxmox `/access/users`.
#[derive(Debug, Deserialize)]
struct RawUser {
    userid: String,
}

/// Raw group entry as returned by Proxmox `/access/groups`.
#[derive(Debug, Deserialize)]
struct RawGroup {
    groupid: String,
}

/// Raw node entry as returned by Proxmox `/nodes`.
#[derive(Debug, Deserialize)]
struct RawNode {
    node: String,
}

/// Raw VM entry as returned by Proxmox `/nodes/{node}/qemu`.
#[derive(Debug, Deserialize)]
struct RawVm {
    name: Option<String>,
    vmid: i64,
}

/// Raw storage entry as returned by Proxmox (uses `type` as field name).
#[derive(Debug, Deserialize)]
struct RawStorageEntry {
    storage: String,
    r#type: String,
    #[serde(default)]
    content: String,
    #[serde(default)]
    active: u8,
    #[serde(default)]
    shared: u8,
    #[serde(default)]
    total: i64,
    #[serde(default)]
    used: i64,
    #[serde(default)]
    avail: i64,
}

impl From<RawStorageEntry> for models::StorageEntry {
    fn from(r: RawStorageEntry) -> Self {
        Self {
            storage: r.storage,
            r_type: r.r#type,
            content: r.content,
            active: r.active != 0,
            shared: r.shared != 0,
            total: r.total,
            used: r.used,
            avail: r.avail,
        }
    }
}

impl ProxmoxClient {
    pub fn new(url: String, token_id: String, token_secret: String) -> Self {
        Self {
            http: Client::new(),
            base_url: url.trim_end_matches('/').to_string(),
            token_id,
            token_secret,
        }
    }

    fn auth_header(&self) -> String {
        format!("PVEAPIToken={}={}", self.token_id, self.token_secret)
    }

    // -----------------------------------------------------------------------
    // Read-only endpoints (GET)
    // -----------------------------------------------------------------------

    /// List all usernames from Proxmox (flat string list).
    pub async fn list_users(&self) -> Result<Vec<String>, reqwest::Error> {
        let url = format!("{}/api2/json/access/users", self.base_url);
        let resp: ApiResponse<Vec<RawUser>> = self
            .http
            .get(&url)
            .header("Authorization", self.auth_header())
            .send()
            .await?
            .json()
            .await?;
        Ok(resp.data.into_iter().map(|u| u.userid).collect())
    }

    /// List all group names from Proxmox (flat string list).
    pub async fn list_groups(&self) -> Result<Vec<String>, reqwest::Error> {
        let url = format!("{}/api2/json/access/groups", self.base_url);
        let resp: ApiResponse<Vec<RawGroup>> = self
            .http
            .get(&url)
            .header("Authorization", self.auth_header())
            .send()
            .await?
            .json()
            .await?;
        Ok(resp.data.into_iter().map(|g| g.groupid).collect())
    }

    /// List all node names from Proxmox (flat string list).
    pub async fn list_nodes(&self) -> Result<Vec<String>, reqwest::Error> {
        let url = format!("{}/api2/json/nodes", self.base_url);
        let resp: ApiResponse<Vec<RawNode>> = self
            .http
            .get(&url)
            .header("Authorization", self.auth_header())
            .send()
            .await?
            .json()
            .await?;
        Ok(resp.data.into_iter().map(|n| n.node).collect())
    }

    /// List all VM names from Proxmox (flat string list).
    ///
    /// Iterates over all nodes and collects VMs from each.
    pub async fn list_vms(&self) -> Result<Vec<String>, reqwest::Error> {
        let nodes = self.list_nodes().await?;
        let mut vms = Vec::new();

        for node in &nodes {
            let url = format!("{}/api2/json/nodes/{}/qemu", self.base_url, node);
            let resp: ApiResponse<Vec<RawVm>> = self
                .http
                .get(&url)
                .header("Authorization", self.auth_header())
                .send()
                .await?
                .json()
                .await?;

            for vm in resp.data {
                vms.push(vm.name.unwrap_or_else(|| format!("vm-{}", vm.vmid)));
            }
        }

        Ok(vms)
    }

    /// List storage entries from Proxmox.
    pub async fn list_storage(&self) -> Result<Vec<models::StorageEntry>, reqwest::Error> {
        let url = format!("{}/api2/json/storage", self.base_url);
        let resp: ApiResponse<Vec<RawStorageEntry>> = self
            .http
            .get(&url)
            .header("Authorization", self.auth_header())
            .send()
            .await?
            .json()
            .await?;
        Ok(resp.data.into_iter().map(Into::into).collect())
    }

    /// Get firewall / infrastructure information for a specific VM.
    pub async fn get_infrastructure(&self, vm_id: i64) -> ProvisionResult<models::InfrastructureInfo> {
        // First, find which node this VM lives on.
        let node = self.find_vm_node(vm_id).await?;

        // Get firewall status.
        let fw_url = format!(
            "{}/api2/json/nodes/{}/qemu/{}/firewall/options",
            self.base_url, node, vm_id
        );
        let fw_resp: ApiResponse<serde_json::Value> = self
            .http
            .get(&fw_url)
            .header("Authorization", self.auth_header())
            .send()
            .await?
            .json()
            .await?;
        let firewall_enabled = fw_resp
            .data
            .get("enable")
            .and_then(|v| v.as_u64())
            .map(|v| v != 0)
            .unwrap_or(false);

        // Get VM config to extract network interfaces.
        let config_url = format!(
            "{}/api2/json/nodes/{}/qemu/{}/config",
            self.base_url, node, vm_id
        );
        let config_resp: ApiResponse<serde_json::Value> = self
            .http
            .get(&config_url)
            .header("Authorization", self.auth_header())
            .send()
            .await?
            .json()
            .await?;

        let network_interfaces = Self::parse_network_interfaces(&config_resp.data);

        Ok(models::InfrastructureInfo {
            vm_id,
            firewall_enabled,
            network_interfaces: if network_interfaces.is_empty() {
                None
            } else {
                Some(network_interfaces)
            },
        })
    }

    // -----------------------------------------------------------------------
    // Provisioning — create_config orchestration
    // -----------------------------------------------------------------------

    /// Orchestrate the full creation flow:
    ///
    /// 1. Create a resource pool (modulnumber-class)
    /// 2. Create groups and add members
    /// 3. Create an SDN simple zone
    /// 4. Create VNets (LAN + DMZ) in the zone
    /// 5. Set ACLs on the pool and zone for each group
    /// 6. Add global users to all groups
    /// 7. If firewall enabled: clone the firewall VM template
    /// 8. If apply_firewall_interfaces_config: update VM network config
    pub async fn create_config(&self, req: &models::CreateConfigRequest) -> ProvisionResult {
        let module = &req.modul_configuration;
        let infra = &req.global_infrastructure_setup;
        let fw = &infra.firewall_setup;

        // Derive names.
        let pool_id = format!("{}-{}", module.modulnumber, module.class);
        let zone_id = pool_id.replace('-', "").to_lowercase();

        // 1. Create resource pool.
        tracing::info!(pool = %pool_id, "Creating resource pool");
        self.create_pool(&pool_id, &format!("Pool for {} {}", module.modulnumber, module.class))
            .await?;

        // 2. Create groups and assign members.
        for group in &req.group_details {
            tracing::info!(group = %group.group_name, "Creating group");
            self.create_group(&group.group_name).await?;

            // Add group-specific users.
            for user in &group.userlist {
                self.add_user_to_group(user, &group.group_name).await?;
            }

            // Add global users to every group.
            for user in &module.global_usernames {
                self.add_user_to_group(user, &group.group_name).await?;
            }
        }

        // 3. Create SDN simple zone.
        tracing::info!(zone = %zone_id, "Creating SDN zone");
        self.create_sdn_zone(&zone_id).await?;

        // 4. Create VNets (LAN + DMZ) inside the zone.
        let vnet_lan = format!("{}-lan", zone_id);
        let vnet_dmz = format!("{}-dmz", zone_id);

        tracing::info!(vnet = %vnet_lan, "Creating LAN VNet");
        self.create_vnet(&vnet_lan, &zone_id, None, Some("LAN")).await?;

        tracing::info!(vnet = %vnet_dmz, "Creating DMZ VNet");
        self.create_vnet(&vnet_dmz, &zone_id, None, Some("DMZ")).await?;

        // 5. Set ACLs — pool + zone permissions for each group.
        let pool_path = format!("/pool/{}", pool_id);
        let zone_path = format!("/sdn/zones/{}", zone_id);

        for group in &req.group_details {
            tracing::info!(group = %group.group_name, "Setting ACLs");
            self.set_acl(&pool_path, &group.group_name, "PVEVMUser").await?;
            self.set_acl(&zone_path, &group.group_name, "PVESDNUser").await?;
        }

        // 6. Apply SDN changes.
        tracing::info!("Applying SDN configuration");
        self.apply_sdn().await?;

        // 7. Clone firewall VM if enabled.
        if fw.firewall_enabled {
            if let Some(template_id) = fw.firewall_vm_id {
                tracing::info!(
                    template = template_id,
                    node = %infra.node,
                    "Cloning firewall VM"
                );
                let new_vmid = self
                    .clone_vm(&infra.node, template_id, &pool_id)
                    .await?;

                // 8. Apply firewall interface config if requested.
                if fw.apply_firewall_interfaces_config {
                    tracing::info!(vmid = new_vmid, "Applying firewall interface config");
                    self.update_vm_network(
                        &infra.node,
                        new_vmid,
                        &vnet_lan,
                        &vnet_dmz,
                    )
                    .await?;
                }
            }
        }

        tracing::info!(pool = %pool_id, "Configuration created successfully");
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Provisioning — update_config
    // -----------------------------------------------------------------------

    /// Update a resource's firewall network configuration.
    pub async fn update_config(&self, req: &models::UpdateConfigRequest) -> ProvisionResult {
        // Find which node the VM lives on.
        let node = self.find_vm_node(req.resource_id).await?;

        // Update the VM's network config (net0 bridge).
        let url = format!(
            "{}/api2/json/nodes/{}/qemu/{}/config",
            self.base_url, node, req.resource_id
        );
        let resp = self
            .http
            .put(&url)
            .header("Authorization", self.auth_header())
            .form(&[("net0", format!("bridge={}", req.firewall_networkconfig))])
            .send()
            .await?;

        Self::check_response(resp).await?;

        tracing::info!(
            resource_type = %req.resource_type,
            resource_id = req.resource_id,
            networkconfig = %req.firewall_networkconfig,
            "Configuration updated"
        );
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Low-level Proxmox API write operations
    // -----------------------------------------------------------------------

    /// Create a resource pool.
    async fn create_pool(&self, poolid: &str, comment: &str) -> ProvisionResult {
        let url = format!("{}/api2/json/pools", self.base_url);
        let resp = self
            .http
            .post(&url)
            .header("Authorization", self.auth_header())
            .form(&[("poolid", poolid), ("comment", comment)])
            .send()
            .await?;
        Self::check_response(resp).await
    }

    /// Create an access group.
    async fn create_group(&self, groupid: &str) -> ProvisionResult {
        let url = format!("{}/api2/json/access/groups", self.base_url);
        let resp = self
            .http
            .post(&url)
            .header("Authorization", self.auth_header())
            .form(&[("groupid", groupid)])
            .send()
            .await?;
        Self::check_response(resp).await
    }

    /// Add a user to a group by updating the group's members.
    async fn add_user_to_group(&self, userid: &str, groupid: &str) -> ProvisionResult {
        let url = format!("{}/api2/json/access/groups/{}", self.base_url, groupid);
        let resp = self
            .http
            .put(&url)
            .header("Authorization", self.auth_header())
            .form(&[("members", userid)])
            .send()
            .await?;
        Self::check_response(resp).await
    }

    /// Create an SDN simple zone.
    async fn create_sdn_zone(&self, zone: &str) -> ProvisionResult {
        let url = format!("{}/api2/json/cluster/sdn/zones", self.base_url);
        let resp = self
            .http
            .post(&url)
            .header("Authorization", self.auth_header())
            .form(&[("zone", zone), ("type", "simple")])
            .send()
            .await?;
        Self::check_response(resp).await
    }

    /// Create a VNet inside an SDN zone.
    async fn create_vnet(
        &self,
        vnet: &str,
        zone: &str,
        tag: Option<i32>,
        alias: Option<&str>,
    ) -> ProvisionResult {
        let url = format!("{}/api2/json/cluster/sdn/vnets", self.base_url);
        let mut params: Vec<(&str, String)> = vec![
            ("vnet", vnet.to_string()),
            ("zone", zone.to_string()),
        ];
        if let Some(t) = tag {
            params.push(("tag", t.to_string()));
        }
        if let Some(a) = alias {
            params.push(("alias", a.to_string()));
        }

        let resp = self
            .http
            .post(&url)
            .header("Authorization", self.auth_header())
            .form(&params)
            .send()
            .await?;
        Self::check_response(resp).await
    }

    /// Set an ACL entry (grant a role to a group on a path).
    async fn set_acl(&self, path: &str, groups: &str, roles: &str) -> ProvisionResult {
        let url = format!("{}/api2/json/access/acl", self.base_url);
        let resp = self
            .http
            .put(&url)
            .header("Authorization", self.auth_header())
            .form(&[("path", path), ("groups", groups), ("roles", roles)])
            .send()
            .await?;
        Self::check_response(resp).await
    }

    /// Apply pending SDN changes.
    async fn apply_sdn(&self) -> ProvisionResult {
        let url = format!("{}/api2/json/cluster/sdn", self.base_url);
        let resp = self
            .http
            .put(&url)
            .header("Authorization", self.auth_header())
            .send()
            .await?;
        Self::check_response(resp).await
    }

    /// Clone a VM template. Returns the new VM ID assigned by Proxmox.
    async fn clone_vm(
        &self,
        node: &str,
        template_vmid: i64,
        pool: &str,
    ) -> ProvisionResult<i64> {
        // Get the next available VM ID.
        let next_id = self.next_vmid().await?;

        let url = format!(
            "{}/api2/json/nodes/{}/qemu/{}/clone",
            self.base_url, node, template_vmid
        );
        let resp = self
            .http
            .post(&url)
            .header("Authorization", self.auth_header())
            .form(&[
                ("newid", next_id.to_string()),
                ("pool", pool.to_string()),
                ("full", "1".to_string()),
            ])
            .send()
            .await?;
        Self::check_response(resp).await?;
        Ok(next_id)
    }

    /// Update a VM's network interfaces (net0 = LAN, net1 = DMZ).
    async fn update_vm_network(
        &self,
        node: &str,
        vmid: i64,
        vnet_lan: &str,
        vnet_dmz: &str,
    ) -> ProvisionResult {
        let url = format!(
            "{}/api2/json/nodes/{}/qemu/{}/config",
            self.base_url, node, vmid
        );
        let resp = self
            .http
            .put(&url)
            .header("Authorization", self.auth_header())
            .form(&[
                ("net0", format!("virtio,bridge={},firewall=1", vnet_lan)),
                ("net1", format!("virtio,bridge={},firewall=1", vnet_dmz)),
            ])
            .send()
            .await?;
        Self::check_response(resp).await
    }

    /// Get the next available VM ID from the cluster.
    async fn next_vmid(&self) -> ProvisionResult<i64> {
        let url = format!("{}/api2/json/cluster/nextid", self.base_url);
        let resp: ApiResponse<String> = self
            .http
            .get(&url)
            .header("Authorization", self.auth_header())
            .send()
            .await?
            .json()
            .await?;
        let id: i64 = resp.data.parse().map_err(|e| {
            format!("Failed to parse next VMID '{}': {}", resp.data, e)
        })?;
        Ok(id)
    }

    // -----------------------------------------------------------------------
    // Internal helpers
    // -----------------------------------------------------------------------

    /// Find which node a VM lives on by querying cluster resources.
    async fn find_vm_node(&self, vm_id: i64) -> ProvisionResult<String> {
        let url = format!("{}/api2/json/cluster/resources?type=vm", self.base_url);
        let resp: ApiResponse<Vec<serde_json::Value>> = self
            .http
            .get(&url)
            .header("Authorization", self.auth_header())
            .send()
            .await?
            .json()
            .await?;

        for resource in &resp.data {
            if resource.get("vmid").and_then(|v| v.as_i64()) == Some(vm_id) {
                if let Some(node) = resource.get("node").and_then(|v| v.as_str()) {
                    return Ok(node.to_string());
                }
            }
        }

        Err(format!("VM {} not found in cluster resources", vm_id).into())
    }

    /// Parse network interfaces (net0, net1, ...) from VM config data.
    fn parse_network_interfaces(config: &serde_json::Value) -> Vec<models::NetworkInterface> {
        let mut interfaces = Vec::new();

        for i in 0..32 {
            let key = format!("net{}", i);
            if let Some(val) = config.get(&key).and_then(|v| v.as_str()) {
                let mut bridge = String::new();
                let mut firewall = None;
                let mut tag = None;

                for part in val.split(',') {
                    let part = part.trim();
                    if let Some(b) = part.strip_prefix("bridge=") {
                        bridge = b.to_string();
                    } else if let Some(f) = part.strip_prefix("firewall=") {
                        firewall = Some(f != "0");
                    } else if let Some(t) = part.strip_prefix("tag=") {
                        tag = t.parse().ok();
                    }
                }

                interfaces.push(models::NetworkInterface {
                    name: key,
                    bridge,
                    firewall,
                    tag,
                });
            }
        }

        interfaces
    }

    /// Check a Proxmox API response for errors.
    async fn check_response(resp: reqwest::Response) -> ProvisionResult {
        if resp.status().is_success() {
            return Ok(());
        }

        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        Err(format!("Proxmox API error ({}): {}", status, body).into())
    }
}
