use std::sync::Arc;

use lunar::{
    entities,
    repositories::bookmarks::{BookmarkRepository, BookmarkRepositoryExt},
    sync_engine::EntitySyncResult,
};
use seaography::{
    async_graphql::{self, Context},
    CustomFields,
};
use serde::{Deserialize, Serialize};

use crate::{
    errors::app_error::AppError,
    types::bookmark::SyncBookmarkInput,
    utils::context::{ensure_workspace_access, extract_db_conn},
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncBookmark;

#[CustomFields]
impl SyncBookmark {
    async fn sync_bookmark(
        ctx: &Context<'_>,
        input: Vec<SyncBookmarkInput>,
    ) -> async_graphql::Result<Vec<EntitySyncResult>> {
        ensure_workspace_access(
            ctx,
            input.iter().filter_map(|item| item.workspace_identifier).collect(),
        )
        .await?;

        let db = extract_db_conn(ctx)?;
        let repo = BookmarkRepository::new(Arc::new(db.clone()));

        let models: Vec<entities::bookmark::Model> = input
            .into_iter()
            .map(|item| item.try_into())
            .collect::<Result<_, _>>()?;

        let res = repo
            .upsert_many(models)
            .await
            .map_err(|e| AppError::InternalError(e.to_string()))?;

        Ok(res)
    }
}
