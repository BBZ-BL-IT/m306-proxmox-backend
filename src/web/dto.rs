use serde::{Deserialize, Serialize};

// ══════════════════════════════════════
// POST /api/config/create
// ══════════════════════════════════════

#[derive(Debug, Deserialize)]
pub struct CreateEnvironmentRequest {
    pub modul_configuration: ModulConfiguration,
    pub global_infrastructure_setup: GlobalInfrastructureSetup,
    pub group_details: Vec<GroupDetail>,
}

#[derive(Debug, Deserialize)]
pub struct ModulConfiguration {
    pub modulnumber: String,
    pub class: String,
    pub global_usernames: Vec<String>,
    pub group_amount: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GlobalInfrastructureSetup {
    pub node: String,
    pub firewall_setup: FirewallSetup,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FirewallSetup {
    pub firewall_enabled: bool,
    pub firewall_vm_id: Option<u32>,
    pub firewall_network_profile: Option<u32>,
    pub apply_firewall_interfaces_config: bool,
}

#[derive(Debug, Deserialize)]
pub struct GroupDetail {
    pub group_name: String,
    pub userlist: Vec<String>,
}

// ══════════════════════════════════════
// GET /api/environment/list
// ══════════════════════════════════════

#[derive(Debug, Deserialize)]
pub struct EnvironmentListQuery {
    pub module: Option<String>,
    pub class: Option<String>,
    pub group_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Environment {
    pub group_id: String,
    pub group_name: String,
    pub module: String,
    pub class: String,
    pub node: String,
    pub resource_pool: String,
    pub simple_zone: String,
    pub vnets: Vec<String>,
    pub vms: Vec<u32>,
    pub members: Vec<String>,
}

// ══════════════════════════════════════
// DELETE /api/environment/delete
// ══════════════════════════════════════

#[derive(Debug, Deserialize)]
pub struct DeleteEnvironmentRequest {
    pub group_ids: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct DeleteEnvironmentResponse {
    pub deleted: Vec<DeletedEnvironment>,
    pub errors: Vec<DeleteError>,
}

#[derive(Debug, Serialize)]
pub struct DeletedEnvironment {
    pub group_id: String,
    pub status: String,
    pub deleted_resources: DeletedResources,
}

#[derive(Debug, Serialize)]
pub struct DeletedResources {
    pub vms: Vec<u32>,
    pub vnets: Vec<String>,
    pub simple_zone: String,
    pub resource_pool: String,
    pub user_group: String,
}

#[derive(Debug, Serialize)]
pub struct DeleteError {
    pub group_id: String,
    pub message: String,
}

// ══════════════════════════════════════
// GET/PUT /api/settings
// ══════════════════════════════════════

#[derive(Debug, Deserialize, Serialize)]
pub struct Settings {
    pub role: String,
    pub user_group_templates: String,
    pub prefix_user_group: String,
    pub prefix_resourcepool: String,
    pub prefix_simple_zone: String,
    pub prefix_vnets: String,
    pub postfix_vnet_dmz: String,
    pub postfix_vnet_lan: String,
    pub prefix_firewall: String,
    pub vm_storage: String,
    pub template_storage: String,
    pub wan_interface: String,
}

// ══════════════════════════════════════
// Dropdown-Daten
// ══════════════════════════════════════

#[derive(Debug, Serialize)]
pub struct Group {
    pub group_name: String,
}

#[derive(Debug, Serialize)]
pub struct Node {
    pub node: String,
}

#[derive(Debug, Serialize)]
pub struct Vm {
    pub vmid: u32,
    pub name: String,
    pub node: String,
}

#[derive(Debug, Serialize)]
pub struct Storage {
    pub storage: String,
    #[serde(rename = "type")]
    pub storage_type: String,
    pub content: String,
    pub active: u32,
    pub shared: u32,
    pub total: u64,
    pub used: u64,
    pub avail: u64,
}

#[derive(Debug, Serialize)]
pub struct Role {
    pub role: String,
}

// ══════════════════════════════════════
// GET /api/infrastructure/{vm_id}
// ══════════════════════════════════════

#[derive(Debug, Serialize)]
pub struct InfrastructureResponse {
    pub firewall_setup: FirewallSetup,
}
