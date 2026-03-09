use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(non_camel_case_types)]
pub enum CreateConfigResponse {
    /// Configuration created successfully
    Status201_ConfigurationCreated(models::MessageResponse),
    /// Invalid request
    Status400_InvalidRequest(models::ErrorResponse),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(non_camel_case_types)]
pub enum UpdateConfigResponse {
    /// Configuration update accepted
    Status202_ConfigurationUpdateAccepted(models::MessageResponse),
    /// Invalid request
    Status400_InvalidRequest(models::ErrorResponse),
}

/// Configuration and provisioning endpoints.
#[async_trait]
pub trait Config<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    /// CreateConfig - POST /api/config/create
    async fn create_config(
        &self,
        body: models::CreateConfigRequest,
    ) -> Result<CreateConfigResponse, E>;

    /// UpdateConfig - PUT /api/config/update
    async fn update_config(
        &self,
        body: models::UpdateConfigRequest,
    ) -> Result<UpdateConfigResponse, E>;
}
