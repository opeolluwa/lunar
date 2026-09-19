use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait,
    IntoActiveModel, QueryFilter,
};
use sea_orm::ActiveValue::NotSet;
use uuid::Uuid;
use wasm_bindgen::prelude::*;

use crate::{
    adapters::tts_config::{CreateTtsConfig, UpdateTtsConfig},
    entities::{self, tts_config},
    error::LunarError,
    utils::{js_err, mock_connection, to_js},
};

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct TtsConfigRepository {
    conn: Arc<DatabaseConnection>,
}

#[async_trait]
pub trait TtsConfigRepositoryExt {
    fn new(conn: Arc<DatabaseConnection>) -> Self;

    async fn get_by_user_identifier(
        &self,
        user_identifier: &Option<Uuid>,
    ) -> Result<Option<tts_config::Model>, LunarError>;

    async fn create(
        &self,
        payload: &CreateTtsConfig,
    ) -> Result<tts_config::Model, LunarError>;

    async fn update(
        &self,
        user_identifier: &Option<Uuid>,
        payload: &UpdateTtsConfig,
    ) -> Result<tts_config::Model, LunarError>;

    async fn upsert(
        &self,
        payload: &CreateTtsConfig,
    ) -> Result<tts_config::Model, LunarError>;
}

#[async_trait]
impl TtsConfigRepositoryExt for TtsConfigRepository {
    fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }

    async fn get_by_user_identifier(
        &self,
        user_identifier: &Option<Uuid>,
    ) -> Result<Option<tts_config::Model>, LunarError> {
        let mut filter = tts_config::Entity::find();
        filter = match user_identifier {
            Some(id) => filter.filter(tts_config::Column::UserIdentifier.eq(*id)),
            None => filter.filter(tts_config::Column::UserIdentifier.is_null()),
        };
        filter
            .one(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))
    }

    async fn create(&self, payload: &CreateTtsConfig) -> Result<tts_config::Model, LunarError> {
        let active_model: tts_config::ActiveModel = payload.to_owned().into();
        active_model
            .insert(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))
    }

    async fn update(
        &self,
        user_identifier: &Option<Uuid>,
        payload: &UpdateTtsConfig,
    ) -> Result<tts_config::Model, LunarError> {
        let model = self
            .get_by_user_identifier(user_identifier)
            .await?
            .ok_or_else(|| LunarError::DbOperationError("tts config not found".to_string()))?;

        let mut active_model = model.into_active_model();
        if payload.user_identifier != *user_identifier {
            active_model.user_identifier = match &payload.user_identifier {
                Some(id) => Set(Some(*id)),
                None => Set(None),
            };
        }
        active_model.language = Set(payload.language.clone());
        active_model.voice_actor = Set(payload.voice_actor.clone());
        active_model.rate = Set(payload.rate);
        active_model.pitch = Set(payload.pitch);
        active_model.volume = Set(payload.volume);
        active_model.updated_at = Set(Utc::now().fixed_offset());

        active_model
            .update(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))
    }

    async fn upsert(&self, payload: &CreateTtsConfig) -> Result<tts_config::Model, LunarError> {
        if self
            .get_by_user_identifier(&payload.user_identifier)
            .await?
            .is_some()
        {
            let update: UpdateTtsConfig = payload.into();
            self.update(&payload.user_identifier, &update).await
        } else {
            self.create(payload).await
        }
    }
}

impl From<&CreateTtsConfig> for UpdateTtsConfig {
    fn from(payload: &CreateTtsConfig) -> Self {
        Self {
            user_identifier: payload.user_identifier,
            language: payload.language.clone(),
            voice_actor: payload.voice_actor.clone(),
            rate: payload.rate,
            pitch: payload.pitch,
            volume: payload.volume,
        }
    }
}

#[wasm_bindgen]
impl TtsConfigRepository {
    #[wasm_bindgen(constructor)]
    pub fn new_wasm() -> Self {
        Self::new(mock_connection())
    }

    #[wasm_bindgen(js_name = "getByUserIdentifier")]
    pub async fn get_by_user_identifier_js(
        &self,
        user_identifier: Option<String>,
    ) -> Result<JsValue, JsValue> {
        let id: Option<Uuid> = user_identifier
            .map(|value| Uuid::parse_str(&value).map(Some).unwrap_or(None))
            .filter(|value| value.is_some())
            .flatten();
        let model = self
            .get_by_user_identifier_mock(&id)
            .await
            .map_err(js_err)?;
        to_js(&model)
    }

    async fn get_by_user_identifier_mock(
        &self,
        user_identifier: &Option<Uuid>,
    ) -> Result<Option<tts_config::Model>, LunarError> {
        <Self as TtsConfigRepositoryExt>::get_by_user_identifier(self, user_identifier).await
    }

    #[wasm_bindgen(js_name = "upsert")]
    pub async fn upsert_js(&self, payload: JsValue) -> Result<JsValue, JsValue> {
        let payload: CreateTtsConfig = serde_wasm_bindgen::from_value(payload).map_err(js_err)?;
        let model = <Self as TtsConfigRepositoryExt>::upsert(self, &payload).await?;
        to_js(&model)
    }
}
