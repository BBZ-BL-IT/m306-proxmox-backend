use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(non_camel_case_types)]
pub enum GetInfrastructureResponse {
    /// Infrastructure information
    Status200_InfrastructureInformation(models::InfrastructureInfo),
    /// Invalid VM ID or VM not found
    Status400_InvalidVmId(models::ErrorResponse),
}

/// Infrastructure and firewall endpoints.
#[async_trait]
pub trait Infrastructure<E: std::fmt::Debug + Send + Sync + 'static = ()>:
    super::ErrorHandler<E>
{
    /// GetInfrastructure - GET /api/infrastructure/{vm_id}
    async fn get_infrastructure(&self, vm_id: i64) -> Result<GetInfrastructureResponse, E>;
}
