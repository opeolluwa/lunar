use lunar::{
    adapters::meta::RequestMeta,
    entities::tts_config,
    repositories::tts_config::{TtsConfigRepository, TtsConfigRepositoryExt},
};
use tauri::State;
use uuid::Uuid;

use crate::{
    adapters::tts_config::{CreateTtsConfig, UpdateTtsConfig},
    errors::AppError,
    state::app::AppState,
};

#[tauri::command]
pub async fn get_tts_config(
    state: State<'_, AppState>,
    user_identifier: Option<Uuid>,
    meta: Option<RequestMeta>,
) -> Result<Option<tts_config::Model>, AppError> {
    state
        .tts_config_repository
        .get_by_user_identifier(&user_identifier)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn set_tts_config(
    state: State<'_, AppState>,
    config: CreateTtsConfig,
    meta: Option<RequestMeta>,
) -> Result<tts_config::Model, AppError> {
    let saved = state
        .tts_config_repository
        .upsert(&config.into())
        .await
        .map_err(AppError::from)?;
    Ok(saved)
}

#[tauri::command]
pub async fn update_tts_config(
    state: State<'_, AppState>,
    user_identifier: Option<Uuid>,
    config: UpdateTtsConfig,
    meta: Option<RequestMeta>,
) -> Result<tts_config::Model, AppError> {
    let updated = state
        .tts_config_repository
        .update(&user_identifier, &config.into())
        .await
        .map_err(AppError::from)?;
    Ok(updated)
}
