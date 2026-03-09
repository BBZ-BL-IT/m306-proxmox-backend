use axum::{body::Body, extract::*, response::Response, routing::*};
use http::{HeaderValue, StatusCode, header::CONTENT_TYPE};
use tracing::error;

use crate::apis;

/// Setup API Server.
pub fn new<I, A, E>(api_impl: I) -> Router
where
    I: AsRef<A> + Clone + Send + Sync + 'static,
    A: apis::health::Health<E>
        + apis::config::Config<E>
        + apis::users::Users<E>
        + apis::groups::Groups<E>
        + apis::infrastructure::Infrastructure<E>
        + apis::nodes::Nodes<E>
        + apis::vms::Vms<E>
        + apis::storage::Storage<E>
        + Send
        + Sync
        + 'static,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    Router::new()
        .route("/health", get(check_health::<I, A, E>))
        .route("/api/config/create", post(create_config::<I, A, E>))
        .route("/api/config/update", put(update_config::<I, A, E>))
        .route("/api/config/storage", get(list_storage::<I, A, E>))
        .route("/api/user/list", get(list_users::<I, A, E>))
        .route("/api/group/list", get(list_groups::<I, A, E>))
        .route(
            "/api/infrastructure/{vm_id}",
            get(get_infrastructure::<I, A, E>),
        )
        .route("/api/node/list", get(list_nodes::<I, A, E>))
        .route("/api/vm/list", get(list_vms::<I, A, E>))
        .with_state(api_impl)
}

// ---------------------------------------------------------------------------
// Health
// ---------------------------------------------------------------------------

/// CheckHealth - GET /health
#[tracing::instrument(skip_all)]
async fn check_health<I, A, E>(State(api_impl): State<I>) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::health::Health<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    let result = api_impl.as_ref().check_health().await;

    match result {
        Ok(rsp) => match rsp {
            apis::health::CheckHealthResponse::Status200_ServiceIsHealthy(body) => {
                json_response(StatusCode::OK, &body)
            }
        },
        Err(why) => api_impl.as_ref().handle_error(why).await,
    }
}

// ---------------------------------------------------------------------------
// Config
// ---------------------------------------------------------------------------

/// CreateConfig - POST /api/config/create
#[tracing::instrument(skip_all)]
async fn create_config<I, A, E>(
    State(api_impl): State<I>,
    Json(body): Json<crate::models::CreateConfigRequest>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::config::Config<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    let result = api_impl.as_ref().create_config(body).await;

    match result {
        Ok(rsp) => match rsp {
            apis::config::CreateConfigResponse::Status201_ConfigurationCreated(body) => {
                json_response(StatusCode::CREATED, &body)
            }
            apis::config::CreateConfigResponse::Status400_InvalidRequest(body) => {
                json_response(StatusCode::BAD_REQUEST, &body)
            }
        },
        Err(why) => api_impl.as_ref().handle_error(why).await,
    }
}

/// UpdateConfig - PUT /api/config/update
#[tracing::instrument(skip_all)]
async fn update_config<I, A, E>(
    State(api_impl): State<I>,
    Json(body): Json<crate::models::UpdateConfigRequest>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::config::Config<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    let result = api_impl.as_ref().update_config(body).await;

    match result {
        Ok(rsp) => match rsp {
            apis::config::UpdateConfigResponse::Status202_ConfigurationUpdateAccepted(body) => {
                json_response(StatusCode::ACCEPTED, &body)
            }
            apis::config::UpdateConfigResponse::Status400_InvalidRequest(body) => {
                json_response(StatusCode::BAD_REQUEST, &body)
            }
        },
        Err(why) => api_impl.as_ref().handle_error(why).await,
    }
}

// ---------------------------------------------------------------------------
// Storage
// ---------------------------------------------------------------------------

/// ListStorage - GET /api/config/storage
#[tracing::instrument(skip_all)]
async fn list_storage<I, A, E>(State(api_impl): State<I>) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::storage::Storage<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    let result = api_impl.as_ref().list_storage().await;

    match result {
        Ok(rsp) => match rsp {
            apis::storage::ListStorageResponse::Status200_ListOfStorageEntries(body) => {
                json_response(StatusCode::OK, &body)
            }
        },
        Err(why) => api_impl.as_ref().handle_error(why).await,
    }
}

// ---------------------------------------------------------------------------
// Users
// ---------------------------------------------------------------------------

/// ListUsers - GET /api/user/list
#[tracing::instrument(skip_all)]
async fn list_users<I, A, E>(State(api_impl): State<I>) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::users::Users<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    let result = api_impl.as_ref().list_users().await;

    match result {
        Ok(rsp) => match rsp {
            apis::users::ListUsersResponse::Status200_ListOfUsernames(body) => {
                json_response(StatusCode::OK, &body)
            }
        },
        Err(why) => api_impl.as_ref().handle_error(why).await,
    }
}

// ---------------------------------------------------------------------------
// Groups
// ---------------------------------------------------------------------------

/// ListGroups - GET /api/group/list
#[tracing::instrument(skip_all)]
async fn list_groups<I, A, E>(State(api_impl): State<I>) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::groups::Groups<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    let result = api_impl.as_ref().list_groups().await;

    match result {
        Ok(rsp) => match rsp {
            apis::groups::ListGroupsResponse::Status200_ListOfGroupNames(body) => {
                json_response(StatusCode::OK, &body)
            }
        },
        Err(why) => api_impl.as_ref().handle_error(why).await,
    }
}

// ---------------------------------------------------------------------------
// Infrastructure
// ---------------------------------------------------------------------------

/// GetInfrastructure - GET /api/infrastructure/{vm_id}
#[tracing::instrument(skip_all)]
async fn get_infrastructure<I, A, E>(
    State(api_impl): State<I>,
    Path(vm_id): Path<i64>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::infrastructure::Infrastructure<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    let result = api_impl.as_ref().get_infrastructure(vm_id).await;

    match result {
        Ok(rsp) => match rsp {
            apis::infrastructure::GetInfrastructureResponse::Status200_InfrastructureInformation(
                body,
            ) => json_response(StatusCode::OK, &body),
            apis::infrastructure::GetInfrastructureResponse::Status400_InvalidVmId(body) => {
                json_response(StatusCode::BAD_REQUEST, &body)
            }
        },
        Err(why) => api_impl.as_ref().handle_error(why).await,
    }
}

// ---------------------------------------------------------------------------
// Nodes
// ---------------------------------------------------------------------------

/// ListNodes - GET /api/node/list
#[tracing::instrument(skip_all)]
async fn list_nodes<I, A, E>(State(api_impl): State<I>) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::nodes::Nodes<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    let result = api_impl.as_ref().list_nodes().await;

    match result {
        Ok(rsp) => match rsp {
            apis::nodes::ListNodesResponse::Status200_ListOfNodeNames(body) => {
                json_response(StatusCode::OK, &body)
            }
        },
        Err(why) => api_impl.as_ref().handle_error(why).await,
    }
}

// ---------------------------------------------------------------------------
// VMs
// ---------------------------------------------------------------------------

/// ListVms - GET /api/vm/list
#[tracing::instrument(skip_all)]
async fn list_vms<I, A, E>(State(api_impl): State<I>) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::vms::Vms<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    let result = api_impl.as_ref().list_vms().await;

    match result {
        Ok(rsp) => match rsp {
            apis::vms::ListVmsResponse::Status200_ListOfVmNames(body) => {
                json_response(StatusCode::OK, &body)
            }
        },
        Err(why) => api_impl.as_ref().handle_error(why).await,
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Serialize a value to JSON and wrap it in an HTTP response.
fn json_response<T: serde::Serialize>(
    status: StatusCode,
    body: &T,
) -> Result<Response, StatusCode> {
    let body_content = serde_json::to_vec(body).map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Response::builder()
        .status(status)
        .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
        .body(Body::from(body_content))
        .map_err(|e| {
            error!(error = ?e);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}
