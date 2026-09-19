use chrono::Utc;
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entities::{self, tts_config::ActiveModel};

#[derive(Debug, Clone, Serialize, Deserialize, ts_rs::TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "tts_config.ts")]
pub struct CreateTtsConfig {
    pub user_identifier: Option<Uuid>,
    pub language: String,
    pub voice_actor: String,
    pub rate: f64,
    pub pitch: f64,
    pub volume: f64,
}

impl Into<entities::tts_config::ActiveModel> for CreateTtsConfig {
    fn into(self) -> entities::tts_config::ActiveModel {
        ActiveModel {
            identifier: Set(Uuid::new_v4()),
            user_identifier: Set(self.user_identifier),
            language: Set(self.language),
            voice_actor: Set(self.voice_actor),
            rate: Set(self.rate),
            pitch: Set(self.pitch),
            volume: Set(self.volume),
            created_at: Set(Utc::now().fixed_offset()),
            updated_at: Set(Utc::now().fixed_offset()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ts_rs::TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "tts_config.ts")]
pub struct UpdateTtsConfig {
    pub user_identifier: Option<Uuid>,
    pub language: String,
    pub voice_actor: String,
    pub rate: f64,
    pub pitch: f64,
    pub volume: f64,
}

impl Into<entities::tts_config::ActiveModel> for UpdateTtsConfig {
    fn into(self) -> entities::tts_config::ActiveModel {
        ActiveModel {
            identifier: Set(Uuid::new_v4()),
            user_identifier: Set(None),
            language: Set(self.language),
            voice_actor: Set(self.voice_actor),
            rate: Set(self.rate),
            pitch: Set(self.pitch),
            volume: Set(self.volume),
            created_at: Set(Utc::now().fixed_offset()),
            updated_at: Set(Utc::now().fixed_offset()),
        }
    }
}
