use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "users.ts")]
pub struct UserProfile {
    pub identifier: Uuid,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub profile_picture: Option<String>,
    pub username: Option<String>,
    pub country_identifier: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, Validate, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "users.ts")]
pub struct PartialUserProfile {
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub username: Option<String>,
}
