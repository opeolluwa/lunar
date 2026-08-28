use std::collections::BTreeMap;
use std::path::Path;

use loomabase::client::SqliteClient;
use loomabase::crdt::{CrdtValue, SyncPayload};
use loomabase::{Result as LoomabaseResult, SyncError};
use serde::Serialize;

use crate::errors::AppError;

/// HTTP header carrying the persisted device identifier.
pub const DEVICE_ID_HEADER: &str = "x-device-id";

/// Holds one `SqliteClient` per contract table, all sharing the app database
/// file and one stable device identifier. This is the offline-first source of
/// truth; every client stays in the same SQLite database, so the CRDT stores
/// are transactional together with the legacy lunar tables.
#[derive(Clone)]
pub struct SyncManager {
    clients: BTreeMap<String, SqliteClient>,
    device_id: String,
    server_url: String,
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncReport {
    pub tables: Vec<SyncTableReport>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncTableReport {
    pub table: String,
    pub sent_rows: usize,
    pub received_rows: usize,
    pub has_more: bool,
    pub caught_up: bool,
}

impl SyncManager {
    /// Opens a client for every table in the shared contract against the app
    /// database file. The first `open_with` on a fresh database also migrates
    /// the existing `todos` table up to the contract schema.
    pub async fn open_store(db_path: &Path, device_id: String) -> LoomabaseResult<Self> {
        let contract = lunar::loomabase::contract::contract()?;
        let mut clients = BTreeMap::new();
        for table in contract.tables() {
            let name = table.name().to_owned();
            let client = SqliteClient::open_with(db_path, device_id.clone(), table.clone()).await?;
            clients.insert(name, client);
        }

        let server_url = std::env::var("LOOMABASE_SYNC_URL")
            .unwrap_or_else(|_| "http://localhost:8000/sync".to_string());

        Ok(Self {
            clients,
            device_id,
            server_url,
        })
    }

    #[must_use]
    pub fn device_id(&self) -> &str {
        &self.device_id
    }

    /// Resolves the store client for one contract table (e.g. `"todos"`).
    pub fn client(&self, table: &str) -> Option<&SqliteClient> {
        self.clients.get(table)
    }

    /// Runs the offline-first sync loop for every contract table against the
    /// orchard server. All stores synchronize through the same authenticated
    /// transport; each table is an independent CRDT domain and converges under
    /// last-writer-wins at the cell level.
    pub async fn sync_all(&self, token: &str) -> Result<SyncReport, SyncError> {
        let device_id = self.device_id.clone();
        let token = token.to_owned();
        let mut report = SyncReport::default();

        for (table_name, client) in &self.clients {
            let outbound = client.local_delta().await?;
            let sent_rows = outbound.changes.len();

            let response = client
                .sync_until_caught_up(|payload| {
                    let url = self.server_url.clone();
                    let device_id = device_id.clone();
                    let token = token.clone();
                    async move {
                        let http = reqwest::Client::new();
                        let result = http
                            .post(&url)
                            .bearer_auth(&token)
                            .header(DEVICE_ID_HEADER, &device_id)
                            .json(&payload)
                            .send()
                            .await
                            .map_err(transport_error)?;
                        if !result.status().is_success() {
                            let status = result.status();
                            let body = result
                                .text()
                                .await
                                .unwrap_or_else(|_| String::new());
                            return Err(SyncError::InvalidPayload(format!(
                                "sync failed ({status}): {body}"
                            )));
                        }
                        result.json::<SyncPayload>().await.map_err(transport_error)
                    }
                })
                .await?;

            report.tables.push(SyncTableReport {
                table: table_name.clone(),
                sent_rows,
                received_rows: response.changes.len(),
                has_more: response.has_more,
                caught_up: !response.has_more,
            });
        }

        Ok(report)
    }
}

/// Seeds the `todos` store from the legacy lunar table once, so pre-existing
/// rows participate in the first sync. Safe only while the store is still empty
/// (i.e. before any sync has completed for this device).
pub async fn backfill_todos(
    sync_manager: Option<&SyncManager>,
    conn: &lunar::sea_orm::DatabaseConnection,
) -> Result<usize, AppError> {
    use lunar::entities::todo;
    use lunar::sea_orm::EntityTrait;

    let Some(store) = sync_manager.and_then(|manager| manager.client("todos")) else {
        return Ok(0);
    };

    let models = todo::Entity::find()
        .all(conn)
        .await
        .map_err(|err| AppError::Kernel(lunar::error::LunarError::DbOperationError(err.to_string())))?;

    let mut seeded = 0;
    for model in &models {
        let mut values = BTreeMap::new();
        values.insert("title".to_string(), CrdtValue::Text(model.title.clone()));
        values.insert("completed".to_string(), CrdtValue::Boolean(model.done));
        values.insert(
            "priority".to_string(),
            CrdtValue::Text(priority_to_text(&model.priority).to_string()),
        );
        if let Some(description) = &model.description {
            values.insert("description".to_string(), CrdtValue::Text(description.clone()));
        }
        if let Some(due_date) = &model.due_date {
            values.insert(
                "due_date".to_string(),
                CrdtValue::Text(due_date.format("%Y-%m-%d").to_string()),
            );
        }
        if let Some(due_time) = &model.due_time {
            values.insert(
                "due_time".to_string(),
                CrdtValue::Text(due_time.format("%H:%M:%S").to_string()),
            );
        }
        values.insert(
            "created_at".to_string(),
            CrdtValue::Text(model.created_at.to_rfc3339()),
        );
        values.insert(
            "updated_at".to_string(),
            CrdtValue::Text(model.updated_at.to_rfc3339()),
        );
        if let Some(workspace_identifier) = &model.workspace_identifier {
            values.insert(
                "workspace_identifier".to_string(),
                CrdtValue::Text(workspace_identifier.to_string()),
            );
        }

        store
            .insert(model.identifier.to_string(), values)
            .await
            .map_err(|err| AppError::Kernel(lunar::error::LunarError::DbOperationError(err.to_string())))?;
        seeded += 1;
    }
    Ok(seeded)
}

fn priority_to_text(priority: &lunar::entities::sea_orm_active_enums::Priority) -> &'static str {
    use lunar::entities::sea_orm_active_enums::Priority;
    match priority {
        Priority::High => "high",
        Priority::Medium => "medium",
        Priority::Low => "low",
    }
}

fn transport_error(error: reqwest::Error) -> SyncError {
    SyncError::BlockingTask(error.to_string())
}

/// Loads or creates the stable per-install device identifier persisted in the
/// app data directory.
pub fn load_device_id(app_data_dir: &Path) -> Result<String, AppError> {
    let path = app_data_dir.join("device_id");
    match std::fs::read_to_string(&path) {
        Ok(existing) => Ok(existing.trim().to_string()),
        Err(_) => {
            let device_id = uuid::Uuid::new_v4().to_string();
            std::fs::write(&path, &device_id).map_err(AppError::io)?;
            Ok(device_id)
        }
    }
}