use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(non_camel_case_types)]
pub enum ListUsersResponse {
    /// List of usernames
    Status200_ListOfUsernames(Vec<String>),
}

/// User management endpoints.
#[async_trait]
pub trait Users<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    /// ListUsers - GET /api/user/list
    async fn list_users(&self) -> Result<ListUsersResponse, E>;
}
