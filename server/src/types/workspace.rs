use lunar::entities;
use seaography::async_graphql;
use seaography::CustomInputType;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(CustomInputType, Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[seaography(input_type_name = "SyncWorkspaceInput")]
pub struct SyncWorkspaceInput {
    pub identifier: Uuid,
    pub name: String,
    pub description: String,
    pub created_at: String,
    pub updated_at: String,
    pub is_default: bool,
    pub is_hidden: bool,
    pub is_secured: bool,
    pub password_hash: Option<String>,
}

impl TryFrom<SyncWorkspaceInput> for entities::workspaces::Model {
    type Error = async_graphql::Error;

    fn try_from(val: SyncWorkspaceInput) -> Result<Self, Self::Error> {
        Ok(entities::workspaces::Model {
            identifier: val.identifier,
            name: val.name,
            description: val.description,
            user_identifier: None,
            created_at: val
                .created_at
                .parse()
                .map_err(|e| async_graphql::Error::new(format!("invalid created_at: {e}")))?,
            updated_at: val
                .updated_at
                .parse()
                .map_err(|e| async_graphql::Error::new(format!("invalid updated_at: {e}")))?,
            is_default: val.is_default,
            is_hidden: val.is_hidden,
            is_secured: val.is_secured,
            password_hash: val.password_hash,
        })
    }
}
