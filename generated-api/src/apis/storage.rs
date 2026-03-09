use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(non_camel_case_types)]
pub enum ListStorageResponse {
    /// List of storage entries
    Status200_ListOfStorageEntries(Vec<models::StorageEntry>),
}

/// Storage management endpoints.
#[async_trait]
pub trait Storage<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    /// ListStorage - GET /api/config/storage
    async fn list_storage(&self) -> Result<ListStorageResponse, E>;
}
