use serde::{Deserialize, Serialize};

/// A workspace the authenticated account belongs to, as recorded by its
/// workspace membership row.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountWorkspaceResponse {
    pub identifier: String,
    pub name: String,
    pub description: String,
    pub role: String,
}
