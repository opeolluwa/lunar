use lunar::{
    adapters::meta::RequestMeta,
    entities::snippets,
    repositories::snippets::SnippetRepositoryExt,
    repositories::workspace_manager::{DuplicateRecord, TransferRecord},
};
use tauri::State;
use uuid::Uuid;

use crate::{
    adapters::snippets::{CreateSnippet, UpdateSnippet},
    errors::AppError,
    state::app::AppState,
    state::mirror,
};

#[tauri::command]
pub async fn create_snippet(
    state: State<'_, AppState>,
    snippet: CreateSnippet,
    meta: Option<RequestMeta>,
) -> Result<snippets::Model, AppError> {
    let created = state
        .snippet_repository
        .create(&snippet.into(), &meta)
        .await?;
    mirror::mirror_snippet(&state.sync_manager, &created).await;
    Ok(created)
}

#[tauri::command]
pub async fn get_snippet(
    state: State<'_, AppState>,
    identifier: Uuid,
    meta: Option<RequestMeta>,
) -> Result<Option<snippets::Model>, AppError> {
    state
        .snippet_repository
        .find_by_id(&identifier, &meta)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn get_all_snippets(
    state: State<'_, AppState>,
    meta: Option<RequestMeta>,
) -> Result<Vec<snippets::Model>, AppError> {
    state
        .snippet_repository
        .find_all(&meta)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn delete_snippet(
    state: State<'_, AppState>,
    identifier: Uuid,
    meta: Option<RequestMeta>,
) -> Result<(), AppError> {
    let bin = state.snippet_repository.delete(&identifier, &meta).await?;
    mirror::mirror_recycle_bin(&state.sync_manager, &bin).await;
    mirror::tombstone(&state.sync_manager, mirror::TABLE_SNIPPETS, &identifier).await;
    Ok(())
}

#[tauri::command]
pub async fn update_snippet(
    state: State<'_, AppState>,
    identifier: Uuid,
    snippet: UpdateSnippet,
    meta: Option<RequestMeta>,
) -> Result<snippets::Model, AppError> {
    let updated = state
        .snippet_repository
        .update(&identifier, &snippet.into(), &meta)
        .await?;
    mirror::mirror_snippet(&state.sync_manager, &updated).await;
    Ok(updated)
}

#[tauri::command]
pub async fn get_recently_added_snippet(
    state: State<'_, AppState>,
    meta: Option<RequestMeta>,
) -> Result<Vec<snippets::Model>, AppError> {
    let snippets = state.snippet_repository.recently_added(&meta).await?;
    Ok(snippets)
}

#[tauri::command]
pub async fn duplicate_snippet(
    state: State<'_, AppState>,
    record_identifier: Uuid,
    previous_workspace_identifier: Uuid,
    target_workspace_identifier: Uuid,
    _meta: Option<RequestMeta>,
) -> Result<(), AppError> {
    let new_identifier = state
        .snippet_repository
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
        .snippet_repository
        .find_by_id(&new_identifier, &Some(meta))
        .await
        .map_err(AppError::from)?
    {
        mirror::mirror_snippet(&state.sync_manager, &model).await;
    }
    Ok(())
}

#[tauri::command]
pub async fn transfer_snippet(
    state: State<'_, AppState>,
    record_identifier: Uuid,
    previous_workspace_identifier: Uuid,
    target_workspace_identifier: Uuid,
    _meta: Option<RequestMeta>,
) -> Result<(), AppError> {
    state
        .snippet_repository
        .transfer_record(
            &record_identifier,
            &previous_workspace_identifier,
            &target_workspace_identifier,
        )
        .await
        .map_err(AppError::from)?;
    mirror::transfer(
        &state.sync_manager,
        mirror::TABLE_SNIPPETS,
        &record_identifier,
        &target_workspace_identifier,
    )
    .await;
    Ok(())
}

#[tauri::command]
pub async fn get_unsynced_snippets(
    state: State<'_, AppState>,
) -> Result<Vec<snippets::Model>, AppError> {
    state
        .snippet_repository
        .extract_unsynced()
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn clear_synced_snippets(
    state: State<'_, AppState>,
    identifiers: Vec<String>,
) -> Result<(), AppError> {
    state
        .snippet_repository
        .clear_synced(identifiers)
        .await
        .map_err(Into::into)
}
