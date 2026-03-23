///
/// Data Transfer Objects in rust called "structs", will be defined here
///
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateUmgebungRequest {
    pub node: String,
    pub template_vm_id: u32,
    pub new_vm_id: u32,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct CreateUmgebungResponse {
    pub success: bool,
    pub data: serde_json::Value,
}
