use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(non_camel_case_types)]
pub enum CheckHealthResponse {
    /// Service is healthy
    Status200_ServiceIsHealthy(models::HealthResponse),
}

/// Health check endpoints.
#[async_trait]
pub trait Health<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    /// CheckHealth - GET /health
    async fn check_health(&self) -> Result<CheckHealthResponse, E>;
}
