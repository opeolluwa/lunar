use tauri::State;

use crate::{
    errors::AppError,
    state::{app::AppState, sync::SyncReport},
};

/// Synchronizes every contract table through the authenticated orchard
/// endpoint. The token is the JWT the frontend already holds for the logged-in
/// user; the tenant and device scope are derived server-side.
#[tauri::command]
pub async fn sync_all(
    state: State<'_, AppState>,
    token: String,
) -> Result<SyncReport, AppError> {
    state
        .sync_manager
        .sync_all(&token)
        .await
        .map_err(|err| AppError::Sync(err.to_string()))
}

/// Returns the persisted device identifier for this install.
#[tauri::command]
pub fn current_device_id(state: State<'_, AppState>) -> String {
    state.sync_manager.device_id().to_string()
}