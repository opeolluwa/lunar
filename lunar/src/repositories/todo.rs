use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use sea_orm::prelude::Date;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait,
    IntoActiveModel, QueryFilter, QuerySelect,
};
use uuid::Uuid;
use wasm_bindgen::prelude::*;

use crate::types::EntitySyncResult;
use crate::adapters::meta::RequestMeta;
use crate::entities::sea_orm_active_enums::{ItemType, Priority};
use crate::utils::{extract_req_meta, js_err, mock_connection, to_js};
use crate::{
    adapters::{
        recycle_bin::CreateRecycleBinEntry,
        todo::{CreateTodo, UpdateTodo},
    },
    entities::{recycle_bin, todo, sync_queue},
    error::LunarError,
    repositories::{
        prelude::WorkspaceRepositoryExt,
        recycle_bin::{RecycleBinRepository, RecycleBinRepositoryExt},
        workspace::WorkspaceRepository,
        workspace_manager::{DuplicateRecord, RecordExistInWorkspace, TransferRecord},
    },
};

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct TodoRepository {
    conn: Arc<DatabaseConnection>,
    workspace_repository: WorkspaceRepository,
}

#[async_trait]
pub trait TodoRepositoryExt {
    fn new(conn: Arc<DatabaseConnection>) -> Self;

    async fn create_todo(
        &self,
        payload: &CreateTodo,
        meta: &Option<RequestMeta>,
    ) -> Result<todo::Model, LunarError>;

    async fn find_by_id(
        &self,
        identifier: &Uuid,
        meta: &Option<RequestMeta>,
    ) -> Result<Option<todo::Model>, LunarError>;

    async fn find_all(&self, meta: &Option<RequestMeta>) -> Result<Vec<todo::Model>, LunarError>;

    async fn update(
        &self,
        identifier: &Uuid,
        payload: &UpdateTodo,
        meta: &Option<RequestMeta>,
    ) -> Result<todo::Model, LunarError>;

    async fn delete(
        &self,
        identifier: &Uuid,
        meta: &Option<RequestMeta>,
    ) -> Result<recycle_bin::Model, LunarError>;

    async fn change_priority(
        &self,
        identifier: &Uuid,
        priority: &Priority,
        meta: &Option<RequestMeta>,
    ) -> Result<todo::Model, LunarError>;

    async fn update_due_date(
        &self,
        identifier: &Uuid,
        due_date: Option<Date>,
        meta: &Option<RequestMeta>,
    ) -> Result<todo::Model, LunarError>;

    async fn mark_done(
        &self,
        identifier: &Uuid,
        done: bool,
        meta: &Option<RequestMeta>,
    ) -> Result<todo::Model, LunarError>;

    async fn extract_unsynced(&self) -> Result<Vec<todo::Model>, LunarError>;

    async fn clear_synced(&self, identifiers: Vec<String>) -> Result<(), LunarError>;
    async fn upsert_many(
        &self,
        models: Vec<todo::Model>,
    ) -> Result<Vec<EntitySyncResult>, LunarError>;
}

#[async_trait]
impl TodoRepositoryExt for TodoRepository {
    fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self {
            conn: conn.clone(),
            workspace_repository: WorkspaceRepository::new(conn.clone()),
        }
    }

    async fn create_todo(
        &self,
        payload: &CreateTodo,
        meta: &Option<RequestMeta>,
    ) -> Result<todo::Model, LunarError> {
        let mut active_model: todo::ActiveModel = payload.to_owned().into();

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
    ) -> Result<Option<todo::Model>, LunarError> {
        let meta = extract_req_meta(meta)?;

        todo::Entity::find()
            .filter(todo::Column::Identifier.eq(*identifier))
            .filter(todo::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))
    }

    async fn find_all(&self, meta: &Option<RequestMeta>) -> Result<Vec<todo::Model>, LunarError> {
        let meta = extract_req_meta(meta)?;

        todo::Entity::find()
            .filter(todo::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .all(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))
    }

    async fn update(
        &self,
        identifier: &Uuid,
        payload: &UpdateTodo,
        meta: &Option<RequestMeta>,
    ) -> Result<todo::Model, LunarError> {
        let meta = extract_req_meta(meta)?;

        let model = todo::Entity::find()
            .filter(todo::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .filter(todo::Column::Identifier.eq(*identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))?
            .ok_or_else(|| LunarError::DbOperationError("todo not found".to_string()))?;

        let mut active_model = model.into_active_model();

        if let Some(title) = &payload.title {
            active_model.title = Set(title.clone());
        }
        if let Some(description) = &payload.description {
            active_model.description = Set(Some(description.clone()));
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

        let model = todo::Entity::find()
            .filter(todo::Column::Identifier.eq(*identifier))
            .filter(todo::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))?
            .ok_or_else(|| LunarError::DbOperationError("todo not found".to_string()))?;

        let payload = serde_json::to_string(&model)
            .map_err(|err| LunarError::DbOperationError(err.to_string()))?;

        let bin = RecycleBinRepository::new(self.conn.clone())
            .store(
                &CreateRecycleBinEntry {
                    item_id: model.identifier,
                    item_type: ItemType::Todo,
                    workspace_identifier: model.workspace_identifier,
                    payload,
                },
                &Some(meta.clone()),
            )
            .await?;

        todo::Entity::delete_many()
            .filter(todo::Column::Identifier.eq(*identifier))
            .exec(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))?;
        Ok(bin)
    }

    async fn change_priority(
        &self,
        identifier: &Uuid,
        priority: &Priority,
        meta: &Option<RequestMeta>,
    ) -> Result<todo::Model, LunarError> {
        let meta = extract_req_meta(meta)?;

        let model = todo::Entity::find()
            .filter(todo::Column::Identifier.eq(*identifier))
            .filter(todo::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))?
            .ok_or_else(|| LunarError::DbOperationError("todo not found".to_string()))?;

        let mut active_model = model.into_active_model();
        active_model.priority = Set(priority.to_owned());
        active_model.updated_at = Set(Utc::now().fixed_offset());

        active_model
            .update(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))
    }

    async fn update_due_date(
        &self,
        identifier: &Uuid,
        due_date: Option<Date>,
        meta: &Option<RequestMeta>,
    ) -> Result<todo::Model, LunarError> {
        let meta = extract_req_meta(meta)?;

        let model = todo::Entity::find()
            .filter(todo::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .filter(todo::Column::Identifier.eq(*identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))?
            .ok_or_else(|| LunarError::DbOperationError("todo not found".to_string()))?;

        let mut active_model = model.into_active_model();
        active_model.due_date = Set(due_date);
        active_model.updated_at = Set(Utc::now().fixed_offset());

        active_model
            .update(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))
    }

    async fn mark_done(
        &self,
        identifier: &Uuid,
        done: bool,
        meta: &Option<RequestMeta>,
    ) -> Result<todo::Model, LunarError> {
        let meta = extract_req_meta(meta)?;

        let model = todo::Entity::find()
            .filter(todo::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .filter(todo::Column::Identifier.eq(*identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))?
            .ok_or_else(|| LunarError::DbOperationError("todo not found".to_string()))?;

        let mut active_model = model.into_active_model();
        active_model.done = Set(done);
        active_model.updated_at = Set(Utc::now().fixed_offset());

        active_model
            .update(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))
    }

    async fn extract_unsynced(&self) -> Result<Vec<todo::Model>, LunarError> {
        let queue_entries = sync_queue::Entity::find()
            .filter(sync_queue::Column::TableName.eq("todo"))
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

        todo::Entity::find()
            .filter(todo::Column::Identifier.is_in(identifiers))
            .all(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))
    }

    async fn clear_synced(&self, identifiers: Vec<String>) -> Result<(), LunarError> {
        sync_queue::Entity::delete_many()
            .filter(sync_queue::Column::TableName.eq("todo"))
            .filter(sync_queue::Column::RecordIdentifier.is_in(identifiers))
            .exec(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))?;
        Ok(())
    }
    async fn upsert_many(
        &self,
        models: Vec<todo::Model>,
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
                            let exists = todo::Entity::find()
                                .filter(todo::Column::Identifier.eq(model.identifier))
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

impl TransferRecord for TodoRepository {
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
            return Err(LunarError::TodoNotFound(record_identifier.to_string()));
        }

        let Some(record) = todo::Entity::find()
            .filter(todo::Column::Identifier.eq(*record_identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))?
        else {
            return Err(LunarError::TodoNotFound(record_identifier.to_string()));
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

impl RecordExistInWorkspace for TodoRepository {
    async fn record_exists_in_workspace(
        &self,
        record_identifier: &Uuid,
        workspace_identifier: &Uuid,
    ) -> Result<bool, LunarError> {
        let record = todo::Entity::find()
            .filter(todo::Column::Identifier.eq(*record_identifier))
            .filter(todo::Column::WorkspaceIdentifier.eq(*workspace_identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))?;

        Ok(record.is_some())
    }
}
#[async_trait::async_trait]
impl DuplicateRecord for TodoRepository {
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

        let Some(record) = todo::Entity::find()
            .filter(todo::Column::Identifier.eq(*record_identifier))
            .filter(todo::Column::WorkspaceIdentifier.eq(*previous_workspace_identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))?
        else {
            return Err(LunarError::TodoNotFound(record_identifier.to_string()));
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
impl TodoRepository {
    #[wasm_bindgen(constructor)]
    pub fn new_wasm() -> Self {
        Self::new(mock_connection())
    }

    #[wasm_bindgen(js_name = "create_todo")]
    pub async fn create_todo_js(&self, payload: JsValue, meta: JsValue) -> Result<JsValue, JsValue> {
        let payload: CreateTodo = serde_wasm_bindgen::from_value(payload).map_err(js_err)?;
        let meta: Option<RequestMeta> = serde_wasm_bindgen::from_value(meta).map_err(js_err)?;
        let model = <Self as TodoRepositoryExt>::create_todo(self, &payload, &meta).await?;
        to_js(&model)
    }

    #[wasm_bindgen(js_name = "find_by_id")]
    pub async fn find_by_id_js(&self, identifier: &str, meta: JsValue) -> Result<JsValue, JsValue> {
        let id = Uuid::parse_str(identifier).map_err(js_err)?;
        let meta: Option<RequestMeta> = serde_wasm_bindgen::from_value(meta).map_err(js_err)?;
        let model = <Self as TodoRepositoryExt>::find_by_id(self, &id, &meta).await?;
        to_js(&model)
    }

    #[wasm_bindgen(js_name = "find_all")]
    pub async fn find_all_js(&self, meta: JsValue) -> Result<JsValue, JsValue> {
        let meta: Option<RequestMeta> = serde_wasm_bindgen::from_value(meta).map_err(js_err)?;
        let models = <Self as TodoRepositoryExt>::find_all(self, &meta).await?;
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
        let payload: UpdateTodo = serde_wasm_bindgen::from_value(payload).map_err(js_err)?;
        let meta: Option<RequestMeta> = serde_wasm_bindgen::from_value(meta).map_err(js_err)?;
        let model = <Self as TodoRepositoryExt>::update(self, &id, &payload, &meta).await?;
        to_js(&model)
    }

    #[wasm_bindgen(js_name = "delete")]
    pub async fn delete_js(&self, identifier: &str, meta: JsValue) -> Result<JsValue, JsValue> {
        let id = Uuid::parse_str(identifier).map_err(js_err)?;
        let meta: Option<RequestMeta> = serde_wasm_bindgen::from_value(meta).map_err(js_err)?;
        <Self as TodoRepositoryExt>::delete(self, &id, &meta).await?;
        Ok(JsValue::UNDEFINED)
    }

    #[wasm_bindgen(js_name = "change_priority")]
    pub async fn change_priority_js(
        &self,
        identifier: &str,
        priority: JsValue,
        meta: JsValue,
    ) -> Result<JsValue, JsValue> {
        let id = Uuid::parse_str(identifier).map_err(js_err)?;
        let priority: Priority = serde_wasm_bindgen::from_value(priority).map_err(js_err)?;
        let meta: Option<RequestMeta> = serde_wasm_bindgen::from_value(meta).map_err(js_err)?;
        let model = <Self as TodoRepositoryExt>::change_priority(self, &id, &priority, &meta)
            .await?;
        to_js(&model)
    }

    #[wasm_bindgen(js_name = "update_due_date")]
    pub async fn update_due_date_js(
        &self,
        identifier: &str,
        due_date: JsValue,
        meta: JsValue,
    ) -> Result<JsValue, JsValue> {
        let id = Uuid::parse_str(identifier).map_err(js_err)?;
        let due_date: Option<Date> = serde_wasm_bindgen::from_value(due_date).map_err(js_err)?;
        let meta: Option<RequestMeta> = serde_wasm_bindgen::from_value(meta).map_err(js_err)?;
        let model =
            <Self as TodoRepositoryExt>::update_due_date(self, &id, due_date, &meta).await?;
        to_js(&model)
    }

    #[wasm_bindgen(js_name = "mark_done")]
    pub async fn mark_done_js(
        &self,
        identifier: &str,
        done: bool,
        meta: JsValue,
    ) -> Result<JsValue, JsValue> {
        let id = Uuid::parse_str(identifier).map_err(js_err)?;
        let meta: Option<RequestMeta> = serde_wasm_bindgen::from_value(meta).map_err(js_err)?;
        let model = <Self as TodoRepositoryExt>::mark_done(self, &id, done, &meta).await?;
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
