use std::sync::Arc;

use axum::extract::State;
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use loomabase::crdt::SyncPayload;
use loomabase::server::{initialize_server_schema_with, merge_crdt_states};
use loomabase::SyncError;
use sqlx_postgres::{PgPool, PgPoolOptions};

use crate::adapters::jwt::Claims;
use crate::errors::app_error::AppError;
use crate::states::AppState;

/// HTTP header carrying the persisted device identifier of the syncing client.
/// The server treats it as an untrusted channel identifier; the tenant is always
/// established from the authenticated JWT claims.
pub const DEVICE_ID_HEADER: &str = "x-device-id";

/// Creates the Postgres pool used for CRDT sync merges.
pub async fn build_pool(database_url: &str, max_connections: u32) -> Result<PgPool, AppError> {
    PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(database_url)
        .await
        .map_err(|err| AppError::InternalError(format!("failed to connect to sync pool: {err}")))
}

/// Initializes the schema for every table in the shared contract. Idempotent
/// and safe to call on every boot.
pub async fn initialize_schema(pool: &PgPool) -> Result<(), AppError> {
    let contract = lunar::loomabase::contract::contract().map_err(|err| {
        AppError::InternalError(format!("failed to build shared sync contract: {err}"))
    })?;
    for table in contract.tables() {
        initialize_server_schema_with(pool, table)
            .await
            .map_err(|err| {
                AppError::InternalError(format!(
                    "failed to initialize sync schema for {}: {err}",
                    table.name()
                ))
            })?;
    }
    Ok(())
}

/// Merges a client's CRDT changes and returns the incremental change feed, all
/// inside a single tenant-scoped Postgres transaction.
async fn sync_once(
    pool: &PgPool,
    payload: SyncPayload,
    device_id: &str,
    tenant_id: &str,
) -> Result<SyncPayload, AppError> {
    let table = lunar::loomabase::contract::table_by_fingerprint(payload.schema_fingerprint)
        .ok_or_else(|| {
            AppError::OperationFailed(format!(
                "schema fingerprint {:#018x} is not part of the shared contract",
                payload.schema_fingerprint
            ))
        })?;

    let mut tx = pool.begin().await.map_err(|err| {
        AppError::InternalError(format!("failed to start sync transaction: {err}"))
    })?;

    let response = merge_crdt_states(&mut tx, payload, device_id, tenant_id, &table)
        .await
        .map_err(map_sync_error)?;

    tx.commit().await.map_err(|err| {
        AppError::InternalError(format!("failed to commit sync transaction: {err}"))
    })?;

    Ok(response)
}

fn map_sync_error(error: SyncError) -> AppError {
    match error {
        SyncError::InvalidPayload(_) | SyncError::SchemaMismatch { .. } | SyncError::ClockOverflow => {
            AppError::OperationFailed(error.to_string())
        }
        SyncError::Sqlite(_) => AppError::OperationFailed(error.to_string()),
        _ => AppError::InternalError(error.to_string()),
    }
}

fn device_id_from(headers: &HeaderMap) -> Result<&str, AppError> {
    headers
        .get(DEVICE_ID_HEADER)
        .and_then(|value| value.to_str().ok())
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            AppError::OperationFailed("missing or empty `x-device-id` header".to_string())
        })
}

pub async fn sync_handler(
    State(state): State<Arc<AppState>>,
    claims: Claims,
    headers: HeaderMap,
    Json(payload): Json<SyncPayload>,
) -> Result<impl IntoResponse, AppError> {
    let device_id = device_id_from(&headers)?;
    let tenant_id = claims.user_identifier.to_string();

    let response = sync_once(&state.sync_pool, payload, device_id, &tenant_id).await?;
    Ok(Json(response))
}

pub fn sync_routes(state: Arc<AppState>) -> Router {
    Router::new().route("/", post(sync_handler)).with_state(state)
}