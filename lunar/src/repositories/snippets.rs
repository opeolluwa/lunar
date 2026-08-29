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
use crate::entities::sea_orm_active_enums::ItemType;
use crate::{
    adapters::{
        meta::RequestMeta,
        recycle_bin::CreateRecycleBinEntry,
        snippets::{CreateSnippet, UpdateSnippet},
    },
    entities::{recycle_bin, snippets, sync_queue},
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
pub struct SnippetRepository {
    conn: Arc<DatabaseConnection>,
    workspace_repository: WorkspaceRepository,
}

#[async_trait]
pub trait SnippetRepositoryExt {
    fn new(conn: Arc<DatabaseConnection>) -> Self;

    async fn create(
        &self,
        payload: &CreateSnippet,
        meta: &Option<RequestMeta>,
    ) -> Result<snippets::Model, LunarError>;

    async fn find_by_id(
        &self,
        identifier: &Uuid,
        meta: &Option<RequestMeta>,
    ) -> Result<Option<snippets::Model>, LunarError>;

    async fn find_all(
        &self,
        meta: &Option<RequestMeta>,
    ) -> Result<Vec<snippets::Model>, LunarError>;

    async fn delete(
        &self,
        identifier: &Uuid,
        meta: &Option<RequestMeta>,
    ) -> Result<recycle_bin::Model, LunarError>;

    async fn recently_added(
        &self,
        meta: &Option<RequestMeta>,
    ) -> Result<Vec<snippets::Model>, LunarError>;

    async fn update(
        &self,
        identifier: &Uuid,
        payload: &UpdateSnippet,
        meta: &Option<RequestMeta>,
    ) -> Result<snippets::Model, LunarError>;

    async fn extract_unsynced(&self) -> Result<Vec<snippets::Model>, LunarError>;

    async fn clear_synced(&self, identifiers: Vec<String>) -> Result<(), LunarError>;
    async fn upsert_many(
        &self,
        models: Vec<snippets::Model>,
    ) -> Result<Vec<EntitySyncResult>, LunarError>;
}

#[async_trait]
impl SnippetRepositoryExt for SnippetRepository {
    fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self {
            conn: conn.clone(),
            workspace_repository: WorkspaceRepository::new(conn),
        }
    }

    async fn create(
        &self,
        payload: &CreateSnippet,
        meta: &Option<RequestMeta>,
    ) -> Result<snippets::Model, LunarError> {
        let mut active_model: snippets::ActiveModel = payload.to_owned().into();

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
    ) -> Result<Option<snippets::Model>, LunarError> {
        let meta = extract_req_meta(meta)?;

        snippets::Entity::find()
            .filter(snippets::Column::Identifier.eq(*identifier))
            .filter(snippets::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))
    }

    async fn find_all(
        &self,
        meta: &Option<RequestMeta>,
    ) -> Result<Vec<snippets::Model>, LunarError> {
        let meta = extract_req_meta(meta)?;

        snippets::Entity::find()
            .filter(snippets::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .all(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))
    }

    async fn delete(
        &self,
        identifier: &Uuid,
        meta: &Option<RequestMeta>,
    ) -> Result<recycle_bin::Model, LunarError> {
        let meta = extract_req_meta(meta)?;

        let model = snippets::Entity::find()
            .filter(snippets::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .filter(snippets::Column::Identifier.eq(*identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))?
            .ok_or_else(|| LunarError::DbOperationError("snippet not found".to_string()))?;

        let payload = serde_json::to_string(&model)
            .map_err(|err| LunarError::DbOperationError(err.to_string()))?;

        let bin = RecycleBinRepository::new(self.conn.clone())
            .store(
                &CreateRecycleBinEntry {
                    item_id: model.identifier,
                    item_type: ItemType::Snippet,
                    workspace_identifier: model.workspace_identifier,
                    payload,
                },
                &Some(meta.clone()),
            )
            .await?;

        snippets::Entity::delete_many()
            .filter(snippets::Column::Identifier.eq(*identifier))
            .filter(snippets::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .exec(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))?;
        Ok(bin)
    }

    async fn recently_added(
        &self,
        meta: &Option<RequestMeta>,
    ) -> Result<Vec<snippets::Model>, LunarError> {
        let meta = extract_req_meta(meta)?;

        snippets::Entity::find()
            .limit(10)
            .filter(snippets::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .order_by_asc(snippets::Column::CreatedAt)
            .all(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))
    }

    async fn update(
        &self,
        identifier: &Uuid,
        payload: &UpdateSnippet,
        meta: &Option<RequestMeta>,
    ) -> Result<snippets::Model, LunarError> {
        let meta = extract_req_meta(meta)?;

        let model = snippets::Entity::find()
            .filter(snippets::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .filter(snippets::Column::Identifier.eq(*identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))?
            .ok_or_else(|| LunarError::DbOperationError("snippet not found".to_string()))?;

        let mut active_model = model.into_active_model();

        if let Some(title) = &payload.title {
            active_model.title = Set(Some(title.clone()));
        }
        if let Some(language) = &payload.language {
            active_model.language = Set(Some(language.clone()));
        }
        if let Some(code) = &payload.code {
            active_model.code = Set(code.clone());
        }
        if let Some(description) = &payload.description {
            active_model.description = Set(Some(description.clone()));
        }
        if let Some(is_pinned) = payload.is_pinned {
            active_model.is_pinned = Set(is_pinned);
        }
        active_model.updated_at = Set(Utc::now().fixed_offset());

        active_model
            .update(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))
    }

    async fn extract_unsynced(&self) -> Result<Vec<snippets::Model>, LunarError> {
        let queue_entries = sync_queue::Entity::find()
            .filter(sync_queue::Column::TableName.eq("snippets"))
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

        snippets::Entity::find()
            .filter(snippets::Column::Identifier.is_in(identifiers))
            .all(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))
    }

    async fn clear_synced(&self, identifiers: Vec<String>) -> Result<(), LunarError> {
        sync_queue::Entity::delete_many()
            .filter(sync_queue::Column::TableName.eq("snippets"))
            .filter(sync_queue::Column::RecordIdentifier.is_in(identifiers))
            .exec(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))?;
        Ok(())
    }
    async fn upsert_many(
        &self,
        models: Vec<snippets::Model>,
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
                            let exists = snippets::Entity::find()
                                .filter(snippets::Column::Identifier.eq(model.identifier))
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

impl TransferRecord for SnippetRepository {
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
            return Err(LunarError::SnippetNotFound(record_identifier.to_string()));
        }

        let Some(record) = snippets::Entity::find()
            .filter(snippets::Column::Identifier.eq(*record_identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))?
        else {
            return Err(LunarError::SnippetNotFound(record_identifier.to_string()));
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

impl RecordExistInWorkspace for SnippetRepository {
    async fn record_exists_in_workspace(
        &self,
        record_identifier: &Uuid,
        workspace_identifier: &Uuid,
    ) -> Result<bool, LunarError> {
        let record = snippets::Entity::find()
            .filter(snippets::Column::Identifier.eq(*record_identifier))
            .filter(snippets::Column::WorkspaceIdentifier.eq(*workspace_identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))?;

        Ok(record.is_some())
    }
}
#[async_trait::async_trait]

impl DuplicateRecord for SnippetRepository {
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

        let Some(record) = snippets::Entity::find()
            .filter(snippets::Column::Identifier.eq(*record_identifier))
            .filter(snippets::Column::WorkspaceIdentifier.eq(*previous_workspace_identifier))
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
impl SnippetRepository {
    #[wasm_bindgen(constructor)]
    pub fn new_wasm() -> Self {
        Self::new(mock_connection())
    }

    #[wasm_bindgen(js_name = "create")]
    pub async fn create_js(&self, payload: JsValue, meta: JsValue) -> Result<JsValue, JsValue> {
        let payload: CreateSnippet = serde_wasm_bindgen::from_value(payload).map_err(js_err)?;
        let meta: Option<RequestMeta> = serde_wasm_bindgen::from_value(meta).map_err(js_err)?;
        let model = <Self as SnippetRepositoryExt>::create(self, &payload, &meta).await?;
        to_js(&model)
    }

    #[wasm_bindgen(js_name = "find_by_id")]
    pub async fn find_by_id_js(&self, identifier: &str, meta: JsValue) -> Result<JsValue, JsValue> {
        let id = Uuid::parse_str(identifier).map_err(js_err)?;
        let meta: Option<RequestMeta> = serde_wasm_bindgen::from_value(meta).map_err(js_err)?;
        let model = <Self as SnippetRepositoryExt>::find_by_id(self, &id, &meta).await?;
        to_js(&model)
    }

    #[wasm_bindgen(js_name = "find_all")]
    pub async fn find_all_js(&self, meta: JsValue) -> Result<JsValue, JsValue> {
        let meta: Option<RequestMeta> = serde_wasm_bindgen::from_value(meta).map_err(js_err)?;
        let models = <Self as SnippetRepositoryExt>::find_all(self, &meta).await?;
        to_js(&models)
    }

    #[wasm_bindgen(js_name = "delete")]
    pub async fn delete_js(&self, identifier: &str, meta: JsValue) -> Result<JsValue, JsValue> {
        let id = Uuid::parse_str(identifier).map_err(js_err)?;
        let meta: Option<RequestMeta> = serde_wasm_bindgen::from_value(meta).map_err(js_err)?;
        <Self as SnippetRepositoryExt>::delete(self, &id, &meta).await?;
        Ok(JsValue::UNDEFINED)
    }

    #[wasm_bindgen(js_name = "recently_added")]
    pub async fn recently_added_js(&self, meta: JsValue) -> Result<JsValue, JsValue> {
        let meta: Option<RequestMeta> = serde_wasm_bindgen::from_value(meta).map_err(js_err)?;
        let models = <Self as SnippetRepositoryExt>::recently_added(self, &meta).await?;
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
        let payload: UpdateSnippet = serde_wasm_bindgen::from_value(payload).map_err(js_err)?;
        let meta: Option<RequestMeta> = serde_wasm_bindgen::from_value(meta).map_err(js_err)?;
        let model = <Self as SnippetRepositoryExt>::update(self, &id, &payload, &meta).await?;
        to_js(&model)
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
