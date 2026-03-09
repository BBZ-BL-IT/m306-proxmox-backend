use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(non_camel_case_types)]
pub enum ListGroupsResponse {
    /// List of group names
    Status200_ListOfGroupNames(Vec<String>),
}

/// Group management endpoints.
#[async_trait]
pub trait Groups<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    /// ListGroups - GET /api/group/list
    async fn list_groups(&self) -> Result<ListGroupsResponse, E>;
}
