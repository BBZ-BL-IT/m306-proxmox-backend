use serde::{Deserialize, Serialize};

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

#[derive(Debug, Deserialize)]
pub struct GlobalInfrastructureSetup {
    pub node: String,
    pub firewall_setup: FirewallSetup,
}

#[derive(Debug, Deserialize)]
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
