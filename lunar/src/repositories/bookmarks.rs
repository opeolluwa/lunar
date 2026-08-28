use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait,
    IntoActiveModel, QueryFilter, QueryOrder, QuerySelect,
};
use uuid::Uuid;
use wasm_bindgen::prelude::*;

use crate::types::EntitySyncResult;
use crate::entities::sea_orm_active_enums::{ItemType, Tag};
use crate::{
    adapters::{
        bookmarks::{CreateBookmark, UpdateBookmark},
        meta::RequestMeta,
        recycle_bin::CreateRecycleBinEntry,
    },
    entities::{bookmark, recycle_bin, sync_queue},
    error::LunarError,
    repositories::{
        prelude::WorkspaceRepositoryExt,
        recycle_bin::{RecycleBinRepository, RecycleBinRepositoryExt},
        workspace::WorkspaceRepository,
        workspace_manager::{DuplicateRecord, RecordExistInWorkspace, TransferRecord},
    },
    utils::{extract_req_meta, js_err, mock_connection, to_js},
};

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct BookmarkRepository {
    conn: Arc<DatabaseConnection>,
    workspace_repository: WorkspaceRepository,
}

#[async_trait]
pub trait BookmarkRepositoryExt {
    fn new(conn: Arc<DatabaseConnection>) -> Self;

    async fn create(
        &self,
        payload: &CreateBookmark,
        meta: &Option<RequestMeta>,
    ) -> Result<bookmark::Model, LunarError>;

    async fn find_by_id(
        &self,
        identifier: &Uuid,
        meta: &Option<RequestMeta>,
    ) -> Result<Option<bookmark::Model>, LunarError>;

    async fn find_all(
        &self,
        meta: &Option<RequestMeta>,
    ) -> Result<Vec<bookmark::Model>, LunarError>;

    async fn find_by_tag(
        &self,
        tag: &Tag,
        meta: &Option<RequestMeta>,
    ) -> Result<Vec<bookmark::Model>, LunarError>;

    async fn recently_added(
        &self,
        meta: &Option<RequestMeta>,
    ) -> Result<Vec<bookmark::Model>, LunarError>;

    async fn update(
        &self,
        identifier: &Uuid,
        payload: &UpdateBookmark,
        meta: &Option<RequestMeta>,
    ) -> Result<bookmark::Model, LunarError>;

    async fn delete(
        &self,
        identifier: &Uuid,
        meta: &Option<RequestMeta>,
    ) -> Result<recycle_bin::Model, LunarError>;

    async fn exists(&self, identifier: &Uuid) -> Result<bool, LunarError>;

    async fn extract_unsynced(&self) -> Result<Vec<bookmark::Model>, LunarError>;

    async fn clear_synced(&self, identifiers: Vec<String>) -> Result<(), LunarError>;
    async fn upsert_many(
        &self,
        models: Vec<bookmark::Model>,
    ) -> Result<Vec<EntitySyncResult>, LunarError>;
}

#[async_trait]
impl BookmarkRepositoryExt for BookmarkRepository {
    fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self {
            conn: conn.clone(),
            workspace_repository: WorkspaceRepository::new(conn),
        }
    }

    async fn create(
        &self,
        payload: &CreateBookmark,
        meta: &Option<RequestMeta>,
    ) -> Result<bookmark::Model, LunarError> {
        let mut active_model: bookmark::ActiveModel = payload.to_owned().into();

        if let Some(meta) = meta {
            active_model.workspace_identifier = Set(Some(meta.workspace_identifier));
        } else {
            return Err(LunarError::DbOperationError(
                "workspace identifier is required".into(),
            ));
        };

        active_model
            .insert(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))
    }

    async fn find_by_id(
        &self,
        identifier: &Uuid,
        meta: &Option<RequestMeta>,
    ) -> Result<Option<bookmark::Model>, LunarError> {
        let meta = extract_req_meta(meta)?;

        bookmark::Entity::find()
            .filter(bookmark::Column::Identifier.eq(*identifier))
            .filter(bookmark::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))
    }

    async fn find_all(
        &self,
        meta: &Option<RequestMeta>,
    ) -> Result<Vec<bookmark::Model>, LunarError> {
        let meta = extract_req_meta(meta)?;

        bookmark::Entity::find()
            .filter(bookmark::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .all(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))
    }

    async fn find_by_tag(
        &self,
        tag: &Tag,
        meta: &Option<RequestMeta>,
    ) -> Result<Vec<bookmark::Model>, LunarError> {
        let meta = extract_req_meta(meta)?;

        bookmark::Entity::find()
            .filter(bookmark::Column::Tag.eq(tag.to_owned()))
            .filter(bookmark::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .all(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))
    }

    async fn recently_added(
        &self,
        meta: &Option<RequestMeta>,
    ) -> Result<Vec<bookmark::Model>, LunarError> {
        let meta = extract_req_meta(meta)?;

        bookmark::Entity::find()
            .filter(bookmark::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .limit(10)
            .order_by_desc(bookmark::Column::CreatedAt)
            .all(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))
    }

    async fn update(
        &self,
        identifier: &Uuid,
        payload: &UpdateBookmark,
        meta: &Option<RequestMeta>,
    ) -> Result<bookmark::Model, LunarError> {
        let meta = extract_req_meta(meta)?;

        let model = bookmark::Entity::find()
            .filter(bookmark::Column::Identifier.eq(*identifier))
            .filter(bookmark::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))?
            .ok_or_else(|| LunarError::DbOperationError("bookmark not found".to_string()))?;

        let mut active_model = model.into_active_model();

        if let Some(title) = &payload.title {
            active_model.title = Set(title.clone());
        }
        if let Some(url) = &payload.url {
            active_model.url = Set(url.clone());
        }

        if let Some(tag) = &payload.tag {
            active_model.tag = Set(tag.to_owned());
        }
        active_model.updated_at = Set(Utc::now().fixed_offset());

        active_model
            .update(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))
    }

    async fn delete(
        &self,
        identifier: &Uuid,
        meta: &Option<RequestMeta>,
    ) -> Result<recycle_bin::Model, LunarError> {
        let meta = extract_req_meta(meta)?;

        let model = bookmark::Entity::find()
            .filter(bookmark::Column::Identifier.eq(*identifier))
            .filter(bookmark::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))?
            .ok_or_else(|| LunarError::DbOperationError("bookmark not found".to_string()))?;

        let payload = serde_json::to_string(&model)
            .map_err(|err| LunarError::DbOperationError(err.to_string()))?;

        let bin = RecycleBinRepository::new(self.conn.clone())
            .store(
                &CreateRecycleBinEntry {
                    item_id: model.identifier,
                    item_type: ItemType::Bookmark,
                    workspace_identifier: model.workspace_identifier,
                    payload,
                },
                &Some(meta.clone()),
            )
            .await?;

        bookmark::Entity::delete_many()
            .filter(bookmark::Column::Identifier.eq(*identifier))
            .filter(bookmark::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .exec(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))?;
        Ok(bin)
    }

    async fn exists(&self, identifier: &Uuid) -> Result<bool, LunarError> {
        let result = bookmark::Entity::find()
            .filter(bookmark::Column::Identifier.eq(*identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))
            .ok();

        Ok(result.is_some())
    }

    async fn extract_unsynced(&self) -> Result<Vec<bookmark::Model>, LunarError> {
        let queue_entries = sync_queue::Entity::find()
            // .filter(sync_queue::Column::TableName.eq("bookmarks"))
            .limit(25)
            .all(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))?;

        let identifiers = queue_entries
            .iter()
            .map(|entry| {
                Uuid::parse_str(&entry.record_identifier)
                    .map_err(|err| LunarError::DbOperationError(err.to_string()))
            })
            .collect::<Result<Vec<Uuid>, LunarError>>()?;

        if identifiers.is_empty() {
            return Ok(Vec::new());
        }

        bookmark::Entity::find()
            .filter(bookmark::Column::Identifier.is_in(identifiers))
            .all(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))
    }

    async fn clear_synced(&self, identifiers: Vec<String>) -> Result<(), LunarError> {
        sync_queue::Entity::delete_many()
            .filter(sync_queue::Column::TableName.eq("bookmark"))
            .filter(sync_queue::Column::RecordIdentifier.is_in(identifiers))
            .exec(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))?;
        Ok(())
    }
    async fn upsert_many(
        &self,
        models: Vec<bookmark::Model>,
    ) -> Result<Vec<EntitySyncResult>, LunarError> {
        let mut sync_results: Vec<EntitySyncResult> = Vec::new();
        for chunk in models.chunks(20) {
            let futures: Vec<_> = chunk
                .iter()
                .map(|model| {
                    let conn = self.conn.clone();
                    let model = model.clone();
                    async move {
                        let identifier = model.identifier.to_string();
                        let op_result: Result<(), LunarError> = async {
                            let exists = bookmark::Entity::find()
                                .filter(bookmark::Column::Identifier.eq(model.identifier))
                                .one(conn.as_ref())
                                .await
                                .map_err(|err| LunarError::DbOperationError(err.to_string()))?
                                .is_some();

                            let active_model = model.into_active_model();

                            if exists {
                                active_model.update(conn.as_ref()).await.map_err(|err| {
                                    LunarError::DbOperationError(err.to_string())
                                })?;
                            } else {
                                active_model.insert(conn.as_ref()).await.map_err(|err| {
                                    LunarError::DbOperationError(err.to_string())
                                })?;
                            }
                            Ok(())
                        }
                        .await;
                        EntitySyncResult {
                            identifier,
                            success: op_result.is_ok(),
                            error_message: op_result.err().map(|e| e.to_string()),
                        }
                    }
                })
                .collect();

            let chunk_results = futures::future::join_all(futures).await;
            sync_results.extend(chunk_results);
        }
        Ok(sync_results)
    }
}

#[async_trait::async_trait]
impl TransferRecord for BookmarkRepository {
    async fn transfer_record(
        &self,
        record_identifier: &Uuid,
        previous_workspace_identifier: &Uuid,
        target_workspace_identifier: &Uuid,
    ) -> Result<(), LunarError> {
        let (prev_exists_res, target_exists_res) = tokio::join!(
            self.workspace_repository
                .exists(previous_workspace_identifier),
            self.workspace_repository
                .exists(target_workspace_identifier),
        );

        let prev_exists = prev_exists_res?;
        let target_exists = target_exists_res?;

        if !prev_exists {
            return Err(LunarError::WorkspaceNotFound(
                previous_workspace_identifier.to_string(),
            ));
        }

        if !target_exists {
            return Err(LunarError::WorkspaceNotFound(
                target_workspace_identifier.to_string(),
            ));
        }

        if !self
            .record_exists_in_workspace(record_identifier, previous_workspace_identifier)
            .await?
        {
            return Err(LunarError::BookmarkNotFound(record_identifier.to_string()));
        }

        let Some(record) = bookmark::Entity::find()
            .filter(bookmark::Column::Identifier.eq(*record_identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))?
        else {
            return Err(LunarError::BookmarkNotFound(record_identifier.to_string()));
        };

        let mut active_model = record.into_active_model();

        active_model.updated_at = Set(Utc::now().fixed_offset());
        active_model.workspace_identifier = Set(Some(*target_workspace_identifier));

        active_model
            .update(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))?;

        Ok(())
    }
}
#[async_trait::async_trait]
impl RecordExistInWorkspace for BookmarkRepository {
    async fn record_exists_in_workspace(
        &self,
        record_identifier: &Uuid,
        workspace_identifier: &Uuid,
    ) -> Result<bool, LunarError> {
        let record = bookmark::Entity::find()
            .filter(bookmark::Column::Identifier.eq(*record_identifier))
            .filter(bookmark::Column::WorkspaceIdentifier.eq(*workspace_identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))?;

        Ok(record.is_some())
    }
}

#[async_trait::async_trait]
impl DuplicateRecord for BookmarkRepository {
    async fn duplicate_record(
        &self,
        record_identifier: &Uuid,
        previous_workspace_identifier: &Uuid,
        target_workspace_identifier: &Uuid,
    ) -> Result<Uuid, LunarError> {
        let (prev_exists_res, target_exists_res) = tokio::join!(
            self.workspace_repository
                .exists(previous_workspace_identifier),
            self.workspace_repository
                .exists(target_workspace_identifier),
        );

        let prev_exists = prev_exists_res?;
        let target_exists = target_exists_res?;

        if !prev_exists {
            return Err(LunarError::WorkspaceNotFound(
                previous_workspace_identifier.to_string(),
            ));
        }

        if !target_exists {
            return Err(LunarError::WorkspaceNotFound(
                target_workspace_identifier.to_string(),
            ));
        }

        let Some(record) = bookmark::Entity::find()
            .filter(bookmark::Column::Identifier.eq(*record_identifier))
            .filter(bookmark::Column::WorkspaceIdentifier.eq(*previous_workspace_identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))?
        else {
            return Err(LunarError::BookmarkNotFound(record_identifier.to_string()));
        };

        let mut new_record = record.into_active_model();

        let new_identifier = Uuid::new_v4();
        new_record.identifier = Set(new_identifier);
        new_record.workspace_identifier = Set(Some(*target_workspace_identifier));
        new_record.created_at = Set(Utc::now().fixed_offset());
        new_record.updated_at = Set(Utc::now().fixed_offset());

        new_record
            .insert(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))?;

        Ok(new_identifier)
    }
}

#[wasm_bindgen]
impl BookmarkRepository {
    #[wasm_bindgen(constructor)]
    pub fn new_wasm() -> Self {
        Self::new(mock_connection())
    }

    #[wasm_bindgen(js_name = "create")]
    pub async fn create_js(&self, payload: JsValue, meta: JsValue) -> Result<JsValue, JsValue> {
        let payload: CreateBookmark = serde_wasm_bindgen::from_value(payload).map_err(js_err)?;
        let meta: Option<RequestMeta> = serde_wasm_bindgen::from_value(meta).map_err(js_err)?;
        let model = <Self as BookmarkRepositoryExt>::create(self, &payload, &meta).await?;
        to_js(&model)
    }

    #[wasm_bindgen(js_name = "find_by_id")]
    pub async fn find_by_id_js(&self, identifier: &str, meta: JsValue) -> Result<JsValue, JsValue> {
        let id = Uuid::parse_str(identifier).map_err(js_err)?;
        let meta: Option<RequestMeta> = serde_wasm_bindgen::from_value(meta).map_err(js_err)?;
        let model = <Self as BookmarkRepositoryExt>::find_by_id(self, &id, &meta).await?;
        to_js(&model)
    }

    #[wasm_bindgen(js_name = "find_all")]
    pub async fn find_all_js(&self, meta: JsValue) -> Result<JsValue, JsValue> {
        let meta: Option<RequestMeta> = serde_wasm_bindgen::from_value(meta).map_err(js_err)?;
        let models = <Self as BookmarkRepositoryExt>::find_all(self, &meta).await?;
        to_js(&models)
    }

    #[wasm_bindgen(js_name = "find_by_tag")]
    pub async fn find_by_tag_js(
        &self,
        tag: JsValue,
        meta: JsValue,
    ) -> Result<JsValue, JsValue> {
        let tag: Tag = serde_wasm_bindgen::from_value(tag).map_err(js_err)?;
        let meta: Option<RequestMeta> = serde_wasm_bindgen::from_value(meta).map_err(js_err)?;
        let models = <Self as BookmarkRepositoryExt>::find_by_tag(self, &tag, &meta).await?;
        to_js(&models)
    }

    #[wasm_bindgen(js_name = "recently_added")]
    pub async fn recently_added_js(&self, meta: JsValue) -> Result<JsValue, JsValue> {
        let meta: Option<RequestMeta> = serde_wasm_bindgen::from_value(meta).map_err(js_err)?;
        let models = <Self as BookmarkRepositoryExt>::recently_added(self, &meta).await?;
        to_js(&models)
    }

    #[wasm_bindgen(js_name = "update")]
    pub async fn update_js(
        &self,
        identifier: &str,
        payload: JsValue,
        meta: JsValue,
    ) -> Result<JsValue, JsValue> {
        let id = Uuid::parse_str(identifier).map_err(js_err)?;
        let payload: UpdateBookmark = serde_wasm_bindgen::from_value(payload).map_err(js_err)?;
        let meta: Option<RequestMeta> = serde_wasm_bindgen::from_value(meta).map_err(js_err)?;
        let model = <Self as BookmarkRepositoryExt>::update(self, &id, &payload, &meta).await?;
        to_js(&model)
    }

    #[wasm_bindgen(js_name = "delete")]
    pub async fn delete_js(&self, identifier: &str, meta: JsValue) -> Result<JsValue, JsValue> {
        let id = Uuid::parse_str(identifier).map_err(js_err)?;
        let meta: Option<RequestMeta> = serde_wasm_bindgen::from_value(meta).map_err(js_err)?;
        <Self as BookmarkRepositoryExt>::delete(self, &id, &meta).await?;
        Ok(JsValue::UNDEFINED)
    }

    #[wasm_bindgen(js_name = "exists")]
    pub async fn exists_js(&self, identifier: &str) -> Result<bool, JsValue> {
        let id = Uuid::parse_str(identifier).map_err(js_err)?;
        <Self as BookmarkRepositoryExt>::exists(self, &id).await.map_err(JsValue::from)
    }

    #[wasm_bindgen(js_name = "transfer_record")]
    pub async fn transfer_record_js(
        &self,
        record_identifier: &str,
        previous_workspace_identifier: &str,
        target_workspace_identifier: &str,
    ) -> Result<JsValue, JsValue> {
        let record_identifier = Uuid::parse_str(record_identifier).map_err(js_err)?;
        let previous_workspace_identifier =
            Uuid::parse_str(previous_workspace_identifier).map_err(js_err)?;
        let target_workspace_identifier =
            Uuid::parse_str(target_workspace_identifier).map_err(js_err)?;
        <Self as TransferRecord>::transfer_record(
            self,
            &record_identifier,
            &previous_workspace_identifier,
            &target_workspace_identifier,
        )
        .await?;
        Ok(JsValue::UNDEFINED)
    }

    #[wasm_bindgen(js_name = "duplicate_record")]
    pub async fn duplicate_record_js(
        &self,
        record_identifier: &str,
        previous_workspace_identifier: &str,
        target_workspace_identifier: &str,
    ) -> Result<JsValue, JsValue> {
        let record_identifier = Uuid::parse_str(record_identifier).map_err(js_err)?;
        let previous_workspace_identifier =
            Uuid::parse_str(previous_workspace_identifier).map_err(js_err)?;
        let target_workspace_identifier =
            Uuid::parse_str(target_workspace_identifier).map_err(js_err)?;
        <Self as DuplicateRecord>::duplicate_record(
            self,
            &record_identifier,
            &previous_workspace_identifier,
            &target_workspace_identifier,
        )
        .await?;
        Ok(JsValue::UNDEFINED)
    }

    #[wasm_bindgen(js_name = "record_exists_in_workspace")]
    pub async fn record_exists_in_workspace_js(
        &self,
        record_identifier: &str,
        workspace_identifier: &str,
    ) -> Result<bool, JsValue> {
        let record_identifier = Uuid::parse_str(record_identifier).map_err(js_err)?;
        let workspace_identifier = Uuid::parse_str(workspace_identifier).map_err(js_err)?;
        <Self as RecordExistInWorkspace>::record_exists_in_workspace(
            self,
            &record_identifier,
            &workspace_identifier,
        )
        .await
        .map_err(JsValue::from)
    }
}
