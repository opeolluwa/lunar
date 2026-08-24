use std::sync::Arc;

use async_trait::async_trait;
use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection,
    EntityTrait, IntoActiveModel, QueryFilter, QueryOrder, QuerySelect,
};
use uuid::Uuid;
use wasm_bindgen::prelude::*;

use crate::types::EntitySyncResult;
use crate::entities::sea_orm_active_enums::ItemType;
use crate::{
    adapters::{meta::RequestMeta, recycle_bin::CreateRecycleBinEntry},
    entities::{bookmark, notes, recycle_bin, reminder, snippets, sync_queue, todo},
    error::LunarError,
    utils::{extract_req_meta, js_err, mock_connection, to_js},
};

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct RecycleBinRepository {
    conn: Arc<DatabaseConnection>,
}

#[async_trait]
pub trait RecycleBinRepositoryExt {
    fn new(conn: Arc<DatabaseConnection>) -> Self;

    async fn store(
        &self,
        payload: &CreateRecycleBinEntry,
        meta: &Option<RequestMeta>,
    ) -> Result<recycle_bin::Model, LunarError>;

    async fn find_all(
        &self,
        meta: &Option<RequestMeta>,
    ) -> Result<Vec<recycle_bin::Model>, LunarError>;

    async fn find_by_id(
        &self,
        identifier: &Uuid,
        meta: &Option<RequestMeta>,
    ) -> Result<Option<recycle_bin::Model>, LunarError>;

    async fn find_by_item_type(
        &self,
        item_type: &ItemType,
        meta: &Option<RequestMeta>,
    ) -> Result<Vec<recycle_bin::Model>, LunarError>;

    async fn purge(&self, identifier: &Uuid, meta: &Option<RequestMeta>)
    -> Result<(), LunarError>;

    async fn purge_all(&self, meta: &Option<RequestMeta>) -> Result<(), LunarError>;

    async fn restore(
        &self,
        identifier: &Uuid,
        meta: &Option<RequestMeta>,
    ) -> Result<(), LunarError>;

    async fn extract_unsynced(&self) -> Result<Vec<recycle_bin::Model>, LunarError>;

    async fn clear_synced(&self, identifiers: Vec<String>) -> Result<(), LunarError>;
    async fn upsert_many(
        &self,
        models: Vec<recycle_bin::Model>,
    ) -> Result<Vec<EntitySyncResult>, LunarError>;
}

#[async_trait]
impl RecycleBinRepositoryExt for RecycleBinRepository {
    fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }

    async fn store(
        &self,
        payload: &CreateRecycleBinEntry,
        meta: &Option<RequestMeta>,
    ) -> Result<recycle_bin::Model, LunarError> {
        let mut active_model: recycle_bin::ActiveModel = payload.to_owned().into();

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

    async fn find_all(
        &self,
        meta: &Option<RequestMeta>,
    ) -> Result<Vec<recycle_bin::Model>, LunarError> {
        let meta = extract_req_meta(meta)?;

        recycle_bin::Entity::find()
            .filter(recycle_bin::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .order_by_desc(recycle_bin::Column::DeletedAt)
            .all(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))
    }

    async fn find_by_id(
        &self,
        identifier: &Uuid,
        meta: &Option<RequestMeta>,
    ) -> Result<Option<recycle_bin::Model>, LunarError> {
        let meta = extract_req_meta(meta)?;

        recycle_bin::Entity::find()
            .filter(recycle_bin::Column::Identifier.eq(*identifier))
            .filter(recycle_bin::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))
    }

    async fn find_by_item_type(
        &self,
        item_type: &ItemType,
        meta: &Option<RequestMeta>,
    ) -> Result<Vec<recycle_bin::Model>, LunarError> {
        let meta = extract_req_meta(meta)?;

        recycle_bin::Entity::find()
            .filter(recycle_bin::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .filter(recycle_bin::Column::ItemType.eq(item_type.to_owned()))
            .order_by_desc(recycle_bin::Column::DeletedAt)
            .all(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))
    }

    async fn purge(
        &self,
        identifier: &Uuid,
        meta: &Option<RequestMeta>,
    ) -> Result<(), LunarError> {
        let meta = extract_req_meta(meta)?;

        recycle_bin::Entity::delete_many()
            .filter(recycle_bin::Column::Identifier.eq(*identifier))
            .filter(recycle_bin::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .exec(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))?;
        Ok(())
    }

    async fn purge_all(&self, meta: &Option<RequestMeta>) -> Result<(), LunarError> {
        let meta = extract_req_meta(meta)?;

        recycle_bin::Entity::delete_many()
            .filter(recycle_bin::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .exec(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))?;
        Ok(())
    }

    async fn restore(
        &self,
        identifier: &Uuid,
        meta: &Option<RequestMeta>,
    ) -> Result<(), LunarError> {
        let meta = extract_req_meta(meta)?;

        let entry = recycle_bin::Entity::find()
            .filter(recycle_bin::Column::Identifier.eq(*identifier))
            .filter(recycle_bin::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .one(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))?
            .ok_or_else(|| {
                LunarError::DbOperationError("recycle bin entry not found".to_string())
            })?;

        match entry.item_type {
            ItemType::Note => {
                Self::restore_payload::<notes::Model, notes::ActiveModel>(
                    &entry.payload,
                    self.conn.as_ref(),
                )
                .await?
            }
            ItemType::Todo => {
                Self::restore_payload::<todo::Model, todo::ActiveModel>(
                    &entry.payload,
                    self.conn.as_ref(),
                )
                .await?
            }
            ItemType::Bookmark => {
                Self::restore_payload::<bookmark::Model, bookmark::ActiveModel>(
                    &entry.payload,
                    self.conn.as_ref(),
                )
                .await?
            }
            ItemType::Snippet => {
                Self::restore_payload::<snippets::Model, snippets::ActiveModel>(
                    &entry.payload,
                    self.conn.as_ref(),
                )
                .await?
            }
            ItemType::Reminder => {
                Self::restore_payload::<reminder::Model, reminder::ActiveModel>(
                    &entry.payload,
                    self.conn.as_ref(),
                )
                .await?
            }
        }

        recycle_bin::Entity::delete_many()
            .filter(recycle_bin::Column::Identifier.eq(*identifier))
            .filter(recycle_bin::Column::WorkspaceIdentifier.eq(meta.workspace_identifier))
            .exec(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))?;
        Ok(())
    }

    async fn extract_unsynced(&self) -> Result<Vec<recycle_bin::Model>, LunarError> {
        let queue_entries = sync_queue::Entity::find()
            .filter(sync_queue::Column::TableName.eq("recycle_bin"))
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

        recycle_bin::Entity::find()
            .filter(recycle_bin::Column::Identifier.is_in(identifiers))
            .all(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))
    }

    async fn clear_synced(&self, identifiers: Vec<String>) -> Result<(), LunarError> {
        sync_queue::Entity::delete_many()
            .filter(sync_queue::Column::TableName.eq("recycle_bin"))
            .filter(sync_queue::Column::RecordIdentifier.is_in(identifiers))
            .exec(self.conn.as_ref())
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))?;
        Ok(())
    }
    async fn upsert_many(
        &self,
        models: Vec<recycle_bin::Model>,
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
                            let exists = recycle_bin::Entity::find()
                                .filter(recycle_bin::Column::Identifier.eq(model.identifier))
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

impl RecycleBinRepository {
    async fn restore_payload<M, A>(
        payload: &str,
        conn: &DatabaseConnection,
    ) -> Result<(), LunarError>
    where
        M: IntoActiveModel<A> + serde::de::DeserializeOwned,
        A: ActiveModelBehavior + Send,
        <A::Entity as EntityTrait>::Model: IntoActiveModel<A>,
    {
        let model: M = serde_json::from_str(payload)
            .map_err(|err| LunarError::DbOperationError(err.to_string()))?;

        model
            .into_active_model()
            .insert(conn)
            .await
            .map_err(|err| LunarError::DbOperationError(err.to_string()))?;
        Ok(())
    }
}

#[wasm_bindgen]
impl RecycleBinRepository {
    #[wasm_bindgen(constructor)]
    pub fn new_wasm() -> Self {
        Self::new(mock_connection())
    }

    #[wasm_bindgen(js_name = "store")]
    pub async fn store_js(&self, payload: JsValue, meta: JsValue) -> Result<JsValue, JsValue> {
        let payload: CreateRecycleBinEntry =
            serde_wasm_bindgen::from_value(payload).map_err(js_err)?;
        let meta: Option<RequestMeta> = serde_wasm_bindgen::from_value(meta).map_err(js_err)?;
        let model = <Self as RecycleBinRepositoryExt>::store(self, &payload, &meta).await?;
        to_js(&model)
    }

    #[wasm_bindgen(js_name = "find_all")]
    pub async fn find_all_js(&self, meta: JsValue) -> Result<JsValue, JsValue> {
        let meta: Option<RequestMeta> = serde_wasm_bindgen::from_value(meta).map_err(js_err)?;
        let models = <Self as RecycleBinRepositoryExt>::find_all(self, &meta).await?;
        to_js(&models)
    }

    #[wasm_bindgen(js_name = "find_by_id")]
    pub async fn find_by_id_js(&self, identifier: &str, meta: JsValue) -> Result<JsValue, JsValue> {
        let id = Uuid::parse_str(identifier).map_err(js_err)?;
        let meta: Option<RequestMeta> = serde_wasm_bindgen::from_value(meta).map_err(js_err)?;
        let model = <Self as RecycleBinRepositoryExt>::find_by_id(self, &id, &meta).await?;
        to_js(&model)
    }

    #[wasm_bindgen(js_name = "find_by_item_type")]
    pub async fn find_by_item_type_js(
        &self,
        item_type: JsValue,
        meta: JsValue,
    ) -> Result<JsValue, JsValue> {
        let item_type: ItemType = serde_wasm_bindgen::from_value(item_type).map_err(js_err)?;
        let meta: Option<RequestMeta> = serde_wasm_bindgen::from_value(meta).map_err(js_err)?;
        let models = <Self as RecycleBinRepositoryExt>::find_by_item_type(self, &item_type, &meta)
            .await?;
        to_js(&models)
    }

    #[wasm_bindgen(js_name = "purge")]
    pub async fn purge_js(&self, identifier: &str, meta: JsValue) -> Result<JsValue, JsValue> {
        let id = Uuid::parse_str(identifier).map_err(js_err)?;
        let meta: Option<RequestMeta> = serde_wasm_bindgen::from_value(meta).map_err(js_err)?;
        <Self as RecycleBinRepositoryExt>::purge(self, &id, &meta).await?;
        Ok(JsValue::UNDEFINED)
    }

    #[wasm_bindgen(js_name = "purge_all")]
    pub async fn purge_all_js(&self, meta: JsValue) -> Result<JsValue, JsValue> {
        let meta: Option<RequestMeta> = serde_wasm_bindgen::from_value(meta).map_err(js_err)?;
        <Self as RecycleBinRepositoryExt>::purge_all(self, &meta).await?;
        Ok(JsValue::UNDEFINED)
    }

    #[wasm_bindgen(js_name = "restore")]
    pub async fn restore_js(&self, identifier: &str, meta: JsValue) -> Result<JsValue, JsValue> {
        let id = Uuid::parse_str(identifier).map_err(js_err)?;
        let meta: Option<RequestMeta> = serde_wasm_bindgen::from_value(meta).map_err(js_err)?;
        <Self as RecycleBinRepositoryExt>::restore(self, &id, &meta).await?;
        Ok(JsValue::UNDEFINED)
    }
}
