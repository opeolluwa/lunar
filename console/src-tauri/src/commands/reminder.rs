use lunar::{
    adapters::meta::RequestMeta,
    entities::reminder,
    repositories::reminder::ReminderRepositoryExt,
    repositories::workspace_manager::{DuplicateRecord, TransferRecord},
};
use tauri::State;
use uuid::Uuid;

use crate::{
    adapters::reminder::{CreateReminder, UpdateReminder},
    errors::AppError,
    state::app::AppState,
    state::mirror,
};

#[tauri::command]
pub async fn create_reminder(
    state: State<'_, AppState>,
    reminder: CreateReminder,
    meta: Option<RequestMeta>,
) -> Result<reminder::Model, AppError> {
    let created = state
        .reminder_repository
        .create(&reminder.into(), &meta)
        .await?;
    mirror::mirror_reminder(&state.sync_manager, &created).await;
    Ok(created)
}

#[tauri::command]
pub async fn get_reminder(
    state: State<'_, AppState>,
    identifier: Uuid,
    meta: Option<RequestMeta>,
) -> Result<Option<reminder::Model>, AppError> {
    state
        .reminder_repository
        .find_by_id(&identifier, &meta)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn get_all_reminders(
    state: State<'_, AppState>,
    meta: Option<RequestMeta>,
) -> Result<Vec<reminder::Model>, AppError> {
    state
        .reminder_repository
        .find_all(&meta)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn update_reminder(
    state: State<'_, AppState>,
    identifier: Uuid,
    reminder: UpdateReminder,
    meta: Option<RequestMeta>,
) -> Result<reminder::Model, AppError> {
    let updated = state
        .reminder_repository
        .update(&identifier, &reminder.into(), &meta)
        .await?;
    mirror::mirror_reminder(&state.sync_manager, &updated).await;
    Ok(updated)
}

#[tauri::command]
pub async fn delete_reminder(
    state: State<'_, AppState>,
    identifier: Uuid,
    meta: Option<RequestMeta>,
) -> Result<(), AppError> {
    let bin = state.reminder_repository.delete(&identifier, &meta).await?;
    mirror::mirror_recycle_bin(&state.sync_manager, &bin).await;
    mirror::tombstone(&state.sync_manager, mirror::TABLE_REMINDERS, &identifier).await;
    Ok(())
}

#[tauri::command]
pub async fn duplicate_reminder(
    state: State<'_, AppState>,
    record_identifier: Uuid,
    previous_workspace_identifier: Uuid,
    target_workspace_identifier: Uuid,
    _meta: Option<RequestMeta>,
) -> Result<(), AppError> {
    let new_identifier = state
        .reminder_repository
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
        .reminder_repository
        .find_by_id(&new_identifier, &Some(meta))
        .await
        .map_err(AppError::from)?
    {
        mirror::mirror_reminder(&state.sync_manager, &model).await;
    }
    Ok(())
}

#[tauri::command]
pub async fn transfer_reminder(
    state: State<'_, AppState>,
    record_identifier: Uuid,
    previous_workspace_identifier: Uuid,
    target_workspace_identifier: Uuid,
    _meta: Option<RequestMeta>,
) -> Result<(), AppError> {
    state
        .reminder_repository
        .transfer_record(
            &record_identifier,
            &previous_workspace_identifier,
            &target_workspace_identifier,
        )
        .await
        .map_err(AppError::from)?;
    mirror::transfer(
        &state.sync_manager,
        mirror::TABLE_REMINDERS,
        &record_identifier,
        &target_workspace_identifier,
    )
    .await;
    Ok(())
}

#[tauri::command]
pub async fn get_unsynced_reminders(
    state: State<'_, AppState>,
) -> Result<Vec<reminder::Model>, AppError> {
    state
        .reminder_repository
        .extract_unsynced()
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn clear_synced_reminders(
    state: State<'_, AppState>,
    identifiers: Vec<String>,
) -> Result<(), AppError> {
    state
        .reminder_repository
        .clear_synced(identifiers)
        .await
        .map_err(Into::into)
}
