#![allow(unused_qualifications)]

use serde::{Deserialize, Serialize};

/// Health check response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

/// Generic success message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageResponse {
    pub message: String,
}

/// Generic error message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

// ---------------------------------------------------------------------------
// Config – creation form
// ---------------------------------------------------------------------------

/// Top-level creation request payload.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateConfigRequest {
    pub modul_configuration: ModulConfiguration,
    pub global_infrastructure_setup: GlobalInfrastructureSetup,
    pub group_details: Vec<GroupDetail>,
}

/// Module configuration section of the creation form.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModulConfiguration {
    pub modulnumber: String,
    pub class: String,
    pub global_usernames: Vec<String>,
    pub group_amount: i32,
}

/// Global infrastructure setup section of the creation form.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GlobalInfrastructureSetup {
    pub node: String,
    pub firewall_setup: FirewallSetup,
}

/// Firewall configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FirewallSetup {
    pub firewall_enabled: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_vm_id: Option<i64>,

    pub apply_firewall_interfaces_config: bool,
}

/// A single group with its name and member list.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupDetail {
    pub group_name: String,
    pub userlist: Vec<String>,
}

// ---------------------------------------------------------------------------
// Config – update form
// ---------------------------------------------------------------------------

/// Update request payload for an existing resource.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateConfigRequest {
    pub resource_type: String,
    pub resource_id: i64,
    pub firewall_networkconfig: String,
}

// ---------------------------------------------------------------------------
// Storage
// ---------------------------------------------------------------------------

/// A storage entry with usage information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StorageEntry {
    pub storage: String,

    #[serde(rename = "type")]
    pub r_type: String,

    pub content: String,
    pub active: bool,
    pub shared: bool,
    pub total: i64,
    pub used: i64,
    pub avail: i64,
}

// ---------------------------------------------------------------------------
// Infrastructure
// ---------------------------------------------------------------------------

/// Firewall / infrastructure information for a VM.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InfrastructureInfo {
    pub vm_id: i64,
    pub firewall_enabled: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interfaces: Option<Vec<NetworkInterface>>,
}

/// A network interface attached to a VM.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkInterface {
    pub name: String,
    pub bridge: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<i32>,
}
