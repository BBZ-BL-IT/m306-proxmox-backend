use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(non_camel_case_types)]
pub enum ListVmsResponse {
    /// List of VM names
    Status200_ListOfVmNames(Vec<String>),
}

/// Virtual machine management endpoints.
#[async_trait]
pub trait Vms<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    /// ListVms - GET /api/vm/list
    async fn list_vms(&self) -> Result<ListVmsResponse, E>;
}
