use async_trait::async_trait;

use loomox_api::apis::config::{Config, CreateConfigResponse, UpdateConfigResponse};
use loomox_api::apis::groups::{Groups, ListGroupsResponse};
use loomox_api::apis::health::{CheckHealthResponse, Health};
use loomox_api::apis::infrastructure::{GetInfrastructureResponse, Infrastructure};
use loomox_api::apis::nodes::{ListNodesResponse, Nodes};
use loomox_api::apis::storage::{ListStorageResponse, Storage};
use loomox_api::apis::users::{ListUsersResponse, Users};
use loomox_api::apis::vms::{ListVmsResponse, Vms};
use loomox_api::apis::ErrorHandler;
use loomox_api::models;

use crate::clients::ProxmoxClient;

/// Shared application state that implements the generated API traits.
#[derive(Clone)]
pub struct State {
    pub proxmox: ProxmoxClient,
}

impl AsRef<State> for State {
    fn as_ref(&self) -> &State {
        self
    }
}

#[async_trait]
impl ErrorHandler for State {}

// ---------------------------------------------------------------------------
// Health
// ---------------------------------------------------------------------------

#[async_trait]
impl Health for State {
    async fn check_health(&self) -> Result<CheckHealthResponse, ()> {
        let response = models::HealthResponse {
            status: "ok".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        };
        Ok(CheckHealthResponse::Status200_ServiceIsHealthy(response))
    }
}

// ---------------------------------------------------------------------------
// Config
// ---------------------------------------------------------------------------

#[async_trait]
impl Config for State {
    async fn create_config(
        &self,
        body: models::CreateConfigRequest,
    ) -> Result<CreateConfigResponse, ()> {
        match self.proxmox.create_config(&body).await {
            Ok(()) => Ok(CreateConfigResponse::Status201_ConfigurationCreated(
                models::MessageResponse {
                    message: "Configuration created successfully".to_string(),
                },
            )),
            Err(e) => {
                tracing::error!("Failed to create config: {}", e);
                Ok(CreateConfigResponse::Status400_InvalidRequest(
                    models::ErrorResponse {
                        error: format!("Failed to create configuration: {}", e),
                    },
                ))
            }
        }
    }

    async fn update_config(
        &self,
        body: models::UpdateConfigRequest,
    ) -> Result<UpdateConfigResponse, ()> {
        match self.proxmox.update_config(&body).await {
            Ok(()) => Ok(
                UpdateConfigResponse::Status202_ConfigurationUpdateAccepted(
                    models::MessageResponse {
                        message: "Configuration update accepted".to_string(),
                    },
                ),
            ),
            Err(e) => {
                tracing::error!("Failed to update config: {}", e);
                Ok(UpdateConfigResponse::Status400_InvalidRequest(
                    models::ErrorResponse {
                        error: format!("Failed to update configuration: {}", e),
                    },
                ))
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Users
// ---------------------------------------------------------------------------

#[async_trait]
impl Users for State {
    async fn list_users(&self) -> Result<ListUsersResponse, ()> {
        match self.proxmox.list_users().await {
            Ok(users) => Ok(ListUsersResponse::Status200_ListOfUsernames(users)),
            Err(e) => {
                tracing::error!("Failed to list users: {}", e);
                Ok(ListUsersResponse::Status200_ListOfUsernames(vec![]))
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Groups
// ---------------------------------------------------------------------------

#[async_trait]
impl Groups for State {
    async fn list_groups(&self) -> Result<ListGroupsResponse, ()> {
        match self.proxmox.list_groups().await {
            Ok(groups) => Ok(ListGroupsResponse::Status200_ListOfGroupNames(groups)),
            Err(e) => {
                tracing::error!("Failed to list groups: {}", e);
                Ok(ListGroupsResponse::Status200_ListOfGroupNames(vec![]))
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Infrastructure
// ---------------------------------------------------------------------------

#[async_trait]
impl Infrastructure for State {
    async fn get_infrastructure(&self, vm_id: i64) -> Result<GetInfrastructureResponse, ()> {
        match self.proxmox.get_infrastructure(vm_id).await {
            Ok(info) => Ok(
                GetInfrastructureResponse::Status200_InfrastructureInformation(info),
            ),
            Err(e) => {
                tracing::error!("Failed to get infrastructure for VM {}: {}", vm_id, e);
                Ok(GetInfrastructureResponse::Status400_InvalidVmId(
                    models::ErrorResponse {
                        error: format!("Failed to get infrastructure for VM {}: {}", vm_id, e),
                    },
                ))
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Nodes
// ---------------------------------------------------------------------------

#[async_trait]
impl Nodes for State {
    async fn list_nodes(&self) -> Result<ListNodesResponse, ()> {
        match self.proxmox.list_nodes().await {
            Ok(nodes) => Ok(ListNodesResponse::Status200_ListOfNodeNames(nodes)),
            Err(e) => {
                tracing::error!("Failed to list nodes: {}", e);
                Ok(ListNodesResponse::Status200_ListOfNodeNames(vec![]))
            }
        }
    }
}

// ---------------------------------------------------------------------------
// VMs
// ---------------------------------------------------------------------------

#[async_trait]
impl Vms for State {
    async fn list_vms(&self) -> Result<ListVmsResponse, ()> {
        match self.proxmox.list_vms().await {
            Ok(vms) => Ok(ListVmsResponse::Status200_ListOfVmNames(vms)),
            Err(e) => {
                tracing::error!("Failed to list VMs: {}", e);
                Ok(ListVmsResponse::Status200_ListOfVmNames(vec![]))
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Storage
// ---------------------------------------------------------------------------

#[async_trait]
impl Storage for State {
    async fn list_storage(&self) -> Result<ListStorageResponse, ()> {
        match self.proxmox.list_storage().await {
            Ok(storage) => Ok(ListStorageResponse::Status200_ListOfStorageEntries(storage)),
            Err(e) => {
                tracing::error!("Failed to list storage: {}", e);
                Ok(ListStorageResponse::Status200_ListOfStorageEntries(vec![]))
            }
        }
    }
}
