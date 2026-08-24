use std::sync::Arc;

use lunar::{
    entities,
    repositories::workspace::{WorkspaceRepository, WorkspaceRepositoryExt},
    sync_engine::EntitySyncResult,
};
use seaography::{
    async_graphql::{self, Context},
    CustomFields,
};
use serde::{Deserialize, Serialize};

use crate::{
    errors::app_error::AppError,
    services::workspace_member_service::WorkspaceMemberService,
    types::workspace::SyncWorkspaceInput,
    utils::context::{extract_claims, extract_db_conn},
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncWorkspace;

#[CustomFields]
impl SyncWorkspace {
    async fn sync_workspace(
        ctx: &Context<'_>,
        input: Vec<SyncWorkspaceInput>,
    ) -> async_graphql::Result<Vec<EntitySyncResult>> {
        let claims = extract_claims(ctx)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        let db = extract_db_conn(ctx)?;
        let members = WorkspaceMemberService::init(&Arc::new(db.clone()));

        // Reject foreign workspaces before touching anything.
        let identifiers: Vec<uuid::Uuid> = input.iter().map(|item| item.identifier).collect();
        for identifier in &identifiers {
            members
                .assert_can_modify(*identifier, &claims.email)
                .await
                .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        }

        let repo = WorkspaceRepository::new(Arc::new(db.clone()));

        let models: Vec<entities::workspaces::Model> = input
            .into_iter()
            .map(|item| item.try_into())
            .collect::<Result<_, _>>()?;

        let res = repo
            .upsert_many(models)
            .await
            .map_err(|e| AppError::InternalError(e.to_string()))?;

        // Claim/stamp ownership once the rows exist (FK requires them).
        for identifier in identifiers {
            members
                .ensure_owner(identifier, &claims)
                .await
                .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        }

        Ok(res)
    }
}
