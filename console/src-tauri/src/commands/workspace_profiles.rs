use lunar::{
    adapters::meta::RequestMeta,
    entities::workspace_profiles,
    repositories::workspace_manager::{DuplicateRecord, TransferRecord},
    repositories::workspace_profiles::WorkspaceProfileRepositoryExt,
};
use tauri::State;
use uuid::Uuid;

use crate::{
    adapters::workspace_profile::{CreateWorkspaceProfile, UpdateWorkspaceProfile},
    errors::AppError,
    state::app::AppState,
    state::mirror,
};

#[tauri::command]
pub async fn get_workspace_profile(
    state: State<'_, AppState>,
    meta: Option<RequestMeta>,
) -> Result<Option<workspace_profiles::Model>, AppError> {
    state
        .workspace_profile_repository
        .get(&meta)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn create_workspace_profile(
    state: State<'_, AppState>,
    profile: CreateWorkspaceProfile,
    meta: Option<RequestMeta>,
) -> Result<workspace_profiles::Model, AppError> {
    let created = state
        .workspace_profile_repository
        .create(&profile.into(), &meta)
        .await?;
    mirror::mirror_workspace_profile(&state.sync_manager, &created).await;
    Ok(created)
}

#[tauri::command]
pub async fn update_workspace_profile(
    state: State<'_, AppState>,
    identifier: Uuid,
    profile: UpdateWorkspaceProfile,
    meta: Option<RequestMeta>,
) -> Result<workspace_profiles::Model, AppError> {
    let updated = state
        .workspace_profile_repository
        .update(&identifier, &profile.into(), &meta)
        .await?;
    mirror::mirror_workspace_profile(&state.sync_manager, &updated).await;
    Ok(updated)
}

#[tauri::command]
pub async fn duplicate_workspace_profile(
    state: State<'_, AppState>,
    record_identifier: Uuid,
    previous_workspace_identifier: Uuid,
    target_workspace_identifier: Uuid,
    _meta: Option<RequestMeta>,
) -> Result<(), AppError> {
    state
        .workspace_profile_repository
        .duplicate_record(
            &record_identifier,
            &previous_workspace_identifier,
            &target_workspace_identifier,
        )
        .await
        .map_err(AppError::from)?;
    let meta = RequestMeta {
        workspace_identifier: target_workspace_identifier,
    };
    if let Some(model) = state
        .workspace_profile_repository
        .get(&Some(meta))
        .await
        .map_err(AppError::from)?
    {
        mirror::mirror_workspace_profile(&state.sync_manager, &model).await;
    }
    Ok(())
}

#[tauri::command]
pub async fn transfer_workspace_profile(
    state: State<'_, AppState>,
    record_identifier: Uuid,
    previous_workspace_identifier: Uuid,
    target_workspace_identifier: Uuid,
    _meta: Option<RequestMeta>,
) -> Result<(), AppError> {
    state
        .workspace_profile_repository
        .transfer_record(
            &record_identifier,
            &previous_workspace_identifier,
            &target_workspace_identifier,
        )
        .await
        .map_err(AppError::from)?;
    mirror::transfer(
        &state.sync_manager,
        mirror::TABLE_WORKSPACE_PROFILES,
        &record_identifier,
        &target_workspace_identifier,
    )
    .await;
    Ok(())
}

#[tauri::command]
pub async fn get_unsynced_workspace_profiles(
    state: State<'_, AppState>,
) -> Result<Vec<workspace_profiles::Model>, AppError> {
    state
        .workspace_profile_repository
        .extract_unsynced()
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn clear_synced_workspace_profiles(
    state: State<'_, AppState>,
    identifiers: Vec<String>,
) -> Result<(), AppError> {
    state
        .workspace_profile_repository
        .clear_synced(identifiers)
        .await
        .map_err(Into::into)
}
