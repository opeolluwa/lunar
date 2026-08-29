use lunar::{
    adapters::meta::RequestMeta,
    entities::notes,
    repositories::notes::NotesRepositoryExt,
    repositories::workspace_manager::{DuplicateRecord, TransferRecord},
};
use tauri::State;
use uuid::Uuid;

use crate::{
    adapters::notes::{CreateNote, UpdateNote},
    errors::AppError,
    state::app::AppState,
    state::mirror,
};

#[tauri::command]
pub async fn create_note(
    state: State<'_, AppState>,
    note: CreateNote,
    meta: Option<RequestMeta>,
) -> Result<notes::Model, AppError> {
    let created = state.notes_repository.create(&note.into(), &meta).await?;
    mirror::mirror_notes(&state.sync_manager, &created).await;
    Ok(created)
}

#[tauri::command]
pub async fn get_note(
    state: State<'_, AppState>,
    identifier: Uuid,
    meta: Option<RequestMeta>,
) -> Result<Option<notes::Model>, AppError> {
    state
        .notes_repository
        .find_by_id(&identifier, &meta)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn get_all_notes(
    state: State<'_, AppState>,
    meta: Option<RequestMeta>,
) -> Result<Vec<notes::Model>, AppError> {
    state
        .notes_repository
        .find_all(&meta)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn delete_note(
    state: State<'_, AppState>,
    identifier: Uuid,
    meta: Option<RequestMeta>,
) -> Result<(), AppError> {
    let bin = state.notes_repository.delete(&identifier, &meta).await?;
    mirror::mirror_recycle_bin(&state.sync_manager, &bin).await;
    mirror::tombstone(&state.sync_manager, mirror::TABLE_NOTES, &identifier).await;
    Ok(())
}

#[tauri::command]
pub async fn update_note(
    state: State<'_, AppState>,
    identifier: Uuid,
    note: UpdateNote,
    meta: Option<RequestMeta>,
) -> Result<notes::Model, AppError> {
    let updated = state
        .notes_repository
        .update(&identifier, &note.into(), &meta)
        .await?;
    mirror::mirror_notes(&state.sync_manager, &updated).await;
    Ok(updated)
}

#[tauri::command]
pub async fn get_recently_added_notes(
    state: State<'_, AppState>,
    meta: Option<RequestMeta>,
) -> Result<Vec<notes::Model>, AppError> {
    state
        .notes_repository
        .recently_added(&meta)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn duplicate_note(
    state: State<'_, AppState>,
    record_identifier: Uuid,
    previous_workspace_identifier: Uuid,
    target_workspace_identifier: Uuid,
    _meta: Option<RequestMeta>,
) -> Result<(), AppError> {
    let new_identifier = state
        .notes_repository
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
        .notes_repository
        .find_by_id(&new_identifier, &Some(meta))
        .await
        .map_err(AppError::from)?
    {
        mirror::mirror_notes(&state.sync_manager, &model).await;
    }
    Ok(())
}

#[tauri::command]
pub async fn transfer_note(
    state: State<'_, AppState>,
    record_identifier: Uuid,
    previous_workspace_identifier: Uuid,
    target_workspace_identifier: Uuid,
    _meta: Option<RequestMeta>,
) -> Result<(), AppError> {
    state
        .notes_repository
        .transfer_record(
            &record_identifier,
            &previous_workspace_identifier,
            &target_workspace_identifier,
        )
        .await
        .map_err(AppError::from)?;
    mirror::transfer(
        &state.sync_manager,
        mirror::TABLE_NOTES,
        &record_identifier,
        &target_workspace_identifier,
    )
    .await;
    Ok(())
}

#[tauri::command]
pub async fn export_notes_as_pdf(
    state: State<'_, AppState>,
    record_identifier: Uuid,
    _previous_workspace_identifier: Uuid,
    meta: Option<RequestMeta>,
) -> Result<(), AppError> {
    let note = state
        .notes_repository
        .find_by_id(&record_identifier, &meta)
        .await
        .map_err(|err| AppError::Io(err.to_string()))?;

    let note = note.unwrap();
    let note_content = note.content;
    let note_title = note.title;

    lunar::markdown2pdf::parse_markdown_to_pdf(&note_content, &note_title)
        .map_err(|err| AppError::Io(err.to_string()))?;

    Ok(())
}

#[tauri::command]
pub async fn get_unsynced_notes(state: State<'_, AppState>) -> Result<Vec<notes::Model>, AppError> {
    state
        .notes_repository
        .extract_unsynced()
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn clear_synced_notes(
    state: State<'_, AppState>,
    identifiers: Vec<String>,
) -> Result<(), AppError> {
    state
        .notes_repository
        .clear_synced(identifiers)
        .await
        .map_err(Into::into)
}
