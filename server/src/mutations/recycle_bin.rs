use std::sync::Arc;

use lunar::{
    entities,
    repositories::recycle_bin::{RecycleBinRepository, RecycleBinRepositoryExt},
    sync_engine::EntitySyncResult,
};
use seaography::{
    async_graphql::{self, Context},
    CustomFields,
};
use serde::{Deserialize, Serialize};

use crate::{
    errors::app_error::AppError,
    types::recycle_bin::SyncRecycleBinInput,
    utils::context::{ensure_workspace_access, extract_db_conn},
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncRecycleBinItem;

#[CustomFields]
impl SyncRecycleBinItem {
    async fn sync_recycle_bin_item(
        ctx: &Context<'_>,
        input: Vec<SyncRecycleBinInput>,
    ) -> async_graphql::Result<Vec<EntitySyncResult>> {
        ensure_workspace_access(
            ctx,
            input.iter().filter_map(|item| item.workspace_identifier).collect(),
        )
        .await?;

        let db = extract_db_conn(ctx)?;
        let repo = RecycleBinRepository::new(Arc::new(db.clone()));

        let models: Vec<entities::recycle_bin::Model> = input
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
