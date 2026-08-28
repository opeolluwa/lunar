use std::collections::BTreeMap;
use std::path::Path;
use std::sync::Arc;

use loomabase::client::SqliteClient;
use loomabase::crdt::{RowChange, SyncPayload};
use loomabase::{Result as LoomabaseResult, SyncError};
use serde::Serialize;

use crate::state::mirror;

/// HTTP header carrying the persisted device identifier.
pub const DEVICE_ID_HEADER: &str = "x-device-id";

/// Mirrors [`loomabase::client::MAX_SYNC_PAGES_PER_CALL`], which is private.
pub const MAX_SYNC_PAGES_PER_CALL: u32 = 100;

/// Holds one `SqliteClient` per contract table, all sharing the app database
/// file and one stable device identifier. This is the offline-first source of
/// truth; every client stays in the same SQLite database, so the CRDT stores
/// are transactional together with the legacy lunar tables.
#[derive(Clone)]
pub struct SyncManager {
    clients: BTreeMap<String, SqliteClient>,
    device_id: Arc<std::sync::RwLock<String>>,
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
    pub applied_rows: usize,
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
            device_id: Arc::new(std::sync::RwLock::new(device_id)),
            server_url,
        })
    }

    #[must_use]
    pub fn device_id(&self) -> String {
        self.device_id
            .read()
            .expect("sync device_id lock poisoned")
            .clone()
    }

    /// Updates the in-memory device identifier after it has been re-recorded
    /// in the store (see the `update_device_id` command).
    pub fn set_device_id(&self, device_id: String) {
        *self
            .device_id
            .write()
            .expect("sync device_id lock poisoned") = device_id;
    }

    /// Resolves the store client for one contract table (e.g. `"todos"`).
    pub fn client(&self, table: &str) -> Option<&SqliteClient> {
        self.clients.get(table)
    }

    /// Runs the offline-first sync loop for every contract table against the
    /// orchard server. All stores synchronize through the same authenticated
    /// transport; each table is an independent CRDT domain and converges under
    /// last-writer-wins at the cell level.
    ///
    /// Every page returned across all tables is accumulated and then written
    /// back into the sea-orm tables (see [`mirror::apply_all`]) so changes made
    /// on other devices are reflected in this app's read path.
    pub async fn sync_all(&self, token: &str, conn: &lunar::sea_orm::DatabaseConnection) -> Result<SyncReport, SyncError> {
        let mut by_table: BTreeMap<String, Vec<RowChange>> = BTreeMap::new();
        let mut report = SyncReport::default();
        let device_id = self.device_id();

        for (table_name, client) in &self.clients {
            let outbound = client.local_delta().await?;
            let sent_rows = outbound.changes.len();

            let mut received_rows = 0usize;
            let mut has_more = false;
            let mut received: Vec<RowChange> = Vec::new();

            for _ in 0..MAX_SYNC_PAGES_PER_CALL {
                let outbound = client.local_delta().await?;
                let response = transport(&self.server_url, token, &device_id, outbound.clone()).await?;
                client.complete_sync(outbound, response.clone()).await?;

                received_rows += response.changes.len();
                received.extend(response.changes);
                has_more = response.has_more;
                if !has_more {
                    break;
                }
            }
            if has_more {
                return Err(SyncError::SyncPageBudgetExhausted);
            }

            let applied_rows = received.len();
            if !received.is_empty() && is_mirror_table(table_name) {
                by_table.insert(table_name.clone(), std::mem::take(&mut received));
            }

            report.tables.push(SyncTableReport {
                table: table_name.clone(),
                sent_rows,
                received_rows,
                has_more,
                caught_up: !has_more,
                applied_rows,
            });
        }

        mirror::apply_all(conn, &by_table).await;

        Ok(report)
    }
}

fn is_mirror_table(table: &str) -> bool {
    matches!(
        table,
        mirror::TABLE_TODOS
            | mirror::TABLE_NOTES
            | mirror::TABLE_BOOKMARKS
            | mirror::TABLE_REMINDERS
            | mirror::TABLE_SNIPPETS
            | mirror::TABLE_WORKSPACES
            | mirror::TABLE_WORKSPACE_PROFILES
            | mirror::TABLE_RECYCLE_BIN
    )
}

/// POSTs one synchronization page through the authenticated transport.
async fn transport(
    server_url: &str,
    token: &str,
    device_id: &str,
    payload: SyncPayload,
) -> Result<SyncPayload, SyncError> {
    let http = reqwest::Client::new();
    let result = http
        .post(server_url)
        .bearer_auth(token)
        .header(DEVICE_ID_HEADER, device_id)
        .json(&payload)
        .send()
        .await
        .map_err(transport_error)?;
    if !result.status().is_success() {
        let status = result.status();
        let body = result.text().await.unwrap_or_else(|_| String::new());
        return Err(SyncError::InvalidPayload(format!(
            "sync failed ({status}): {body}"
        )));
    }
    result.json::<SyncPayload>().await.map_err(transport_error)
}

fn transport_error(error: reqwest::Error) -> SyncError {
    SyncError::BlockingTask(error.to_string())
}

/// Derives a stable per-device identifier from OS-level information reported
/// by the `tauri-plugin-os` plugin. Combining the hostname with the platform
/// and architecture reduces the chance two devices collide on the same name.
pub fn os_device_id() -> String {
    let hostname = tauri_plugin_os::hostname();
    let platform = tauri_plugin_os::platform();
    let arch = tauri_plugin_os::arch();
    format!("{hostname}@{platform}:{arch}")
}