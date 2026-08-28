use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// A workspace the authenticated account belongs to, as recorded by its
/// workspace membership row.
#[derive(Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "workspace_member.ts")]
pub struct AccountWorkspaceResponse {
    pub identifier: String,
    pub name: String,
    pub description: String,
    pub role: String,
}
