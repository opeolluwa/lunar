use tauri::Manager;
use tauri::State;

use crate::{
    errors::AppError,
    state::{app::AppState, sync::SyncReport},
};

/// Synchronizes every contract table through the authenticated orchard
/// endpoint. The token is the JWT the frontend already holds for the logged-in
/// user; the tenant and device scope are derived server-side.
///
/// When invoked without a token (guest mode), a development token is taken from
/// the `LUNAR_DEV_TOKEN` environment variable so the offline store can be
/// exercised without a cloud account.
#[tauri::command]
pub async fn sync_all(
    state: State<'_, AppState>,
    token: Option<String>,
) -> Result<SyncReport, AppError> {
    let token = token
        .filter(|token| !token.is_empty())
        .or_else(|| std::env::var("LUNAR_DEV_TOKEN").ok())
        .ok_or_else(|| AppError::Sync("no sync token available".to_string()))?;

    state
        .sync_manager
        .sync_all(&token, &state.conn)
        .await
        .map_err(|err| AppError::Sync(err.to_string()))
}

/// Returns the OS-derived device identifier for this install.
#[tauri::command]
pub fn current_device_id(state: State<'_, AppState>) -> String {
    state.sync_manager.device_id()
}

/// Re-records the device identifier in the offline-first store. Used to migrate
/// a database that belongs to a previous device identity (e.g. recovered from a
/// backup) without hard-blocking startup, and to refresh the in-memory id used
/// for subsequent syncs. Returns the newly recorded id.
#[tauri::command]
pub async fn update_device_id(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    device_id: String,
) -> Result<String, AppError> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|err| AppError::Sync(err.to_string()))?;
    let db_path = match std::env::var("ALMONDS_DB_PATH") {
        Ok(path) => std::path::PathBuf::from(path),
        Err(_) => app_data_dir.join("almonds.db"),
    };
    loomabase::client::SqliteClient::set_device_id(&db_path, &device_id)
        .await
        .map_err(|err| AppError::Sync(err.to_string()))?;
    state.sync_manager.set_device_id(device_id.clone());
    Ok(device_id)
}