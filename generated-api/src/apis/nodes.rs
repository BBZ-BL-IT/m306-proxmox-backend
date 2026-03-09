use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(non_camel_case_types)]
pub enum ListNodesResponse {
    /// List of node names
    Status200_ListOfNodeNames(Vec<String>),
}

/// Node management endpoints.
#[async_trait]
pub trait Nodes<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    /// ListNodes - GET /api/node/list
    async fn list_nodes(&self) -> Result<ListNodesResponse, E>;
}
