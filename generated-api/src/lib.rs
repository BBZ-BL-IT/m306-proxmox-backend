#![allow(
    missing_docs,
    unused_variables,
    unused_imports,
)]

pub const BASE_PATH: &str = "";
pub const API_VERSION: &str = "0.1.0";

#[cfg(feature = "server")]
pub mod server;

pub mod models;
pub mod apis;
