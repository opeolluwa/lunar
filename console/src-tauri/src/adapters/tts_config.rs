use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTtsConfig {
    pub user_identifier: Option<Uuid>,
    pub language: String,
    pub voice_actor: String,
    pub rate: f64,
    pub pitch: f64,
    pub volume: f64,
}

impl From<CreateTtsConfig> for lunar::adapters::tts_config::CreateTtsConfig {
    fn from(p: CreateTtsConfig) -> Self {
        Self {
            user_identifier: p.user_identifier,
            language: p.language,
            voice_actor: p.voice_actor,
            rate: p.rate,
            pitch: p.pitch,
            volume: p.volume,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTtsConfig {
    pub user_identifier: Option<Uuid>,
    pub language: String,
    pub voice_actor: String,
    pub rate: f64,
    pub pitch: f64,
    pub volume: f64,
}

impl From<UpdateTtsConfig> for lunar::adapters::tts_config::UpdateTtsConfig {
    fn from(p: UpdateTtsConfig) -> Self {
        Self {
            user_identifier: p.user_identifier,
            language: p.language,
            voice_actor: p.voice_actor,
            rate: p.rate,
            pitch: p.pitch,
            volume: p.volume,
        }
    }
}
