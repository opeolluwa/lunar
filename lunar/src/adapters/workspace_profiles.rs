use chrono::Utc;
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entities::{self, workspace_profiles::ActiveModel};

#[derive(Debug, Clone, Serialize, Deserialize, ts_rs::TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "workspace_profiles.ts")]
pub struct CreateWorkspaceProfile {
    pub first_name: String,
    pub last_name: String,
    pub profile_picture: Option<String>,
}

impl Into<entities::workspace_profiles::ActiveModel> for CreateWorkspaceProfile {
    fn into(self) -> entities::workspace_profiles::ActiveModel {
        ActiveModel {
            identifier: Set(Uuid::new_v4()),
            first_name: Set(self.first_name),
            last_name: Set(self.last_name),
            profile_picture: Set(self.profile_picture),
            workspace_identifier: Set(None),
            created_at: Set(Utc::now().fixed_offset()),
            updated_at: Set(Utc::now().fixed_offset()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ts_rs::TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "workspace_profiles.ts")]
pub struct UpdateWorkspaceProfile {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub profile_picture: Option<String>,
}
