//! Development helper: acts as a second device against the real orchard sync
//! endpoint so inbound mirroring can be verified without running a second Tauri
//! instance (the app enforces a single instance per install).
//!
//! Opens an in-memory `SqliteClient` for the shared `todos` table, creates (or
//! updates the title of) one row, and synchronizes it through `POST
//! /sync` exactly like the console transport. The row then converges into the
//! app on its next `sync_all`.
//!
//! Usage (run from `console/src-tauri`):
//!   LUNAR_DEV_TOKEN=<jwt> cargo run --example dev_device_b -- --id <uuid> --title "from device B"
//!
//! Env: `LOOMABASE_SYNC_URL` (defaults to http://localhost:8000/sync).

use loomabase::client::SqliteClient;
use loomabase::crdt::SyncPayload;
use loomabase::{Result as LoomabaseResult, SyncError};

const DEVICE_ID: &str = "dev-device-b";

#[tokio::main]
async fn main() -> LoomabaseResult<()> {
    let args: Vec<String> = std::env::args().collect();
    let id = arg(&args, "--id").unwrap_or_else(|| format!("dev-b-{}", uuid::Uuid::new_v4()));
    let title = arg(&args, "--title").unwrap_or_else(|| "from device B".to_string());
    let token = std::env::var("LUNAR_DEV_TOKEN")
        .ok()
        .or_else(|| arg(&args, "--token"))
        .expect("LUNAR_DEV_TOKEN is required (or pass --token)");
    let server_url =
        std::env::var("LOOMABASE_SYNC_URL").unwrap_or_else(|_| "http://localhost:8000/sync".to_string());

    let table = lunar::loomabase::contract::contract()?
        .tables()
        .iter()
        .find(|table| table.name() == "todos")
        .expect("contract must contain the todos table")
        .clone();

    let client = SqliteClient::open_with(":memory:", DEVICE_ID, table).await?;
    let existing = client.get_todo(id.clone()).await?;
    match existing {
        Some(_) => client.update_title(id.clone(), title.clone()).await?,
        None => client.create_todo(id.clone(), title.clone(), false).await?,
    }

    let outbound = client.local_delta().await?;
    println!("device B outbound: {} change(s)", outbound.changes.len());

    let mut received = 0usize;
    loop {
        let outbound = client.local_delta().await?;
        let response = transport(&server_url, &token, &outbound).await?;
        received += response.changes.len();
        client.complete_sync(outbound, response.clone()).await?;
        if !response.has_more {
            break;
        }
    }

    println!(
        "device B synchronized ({received} cell feed rows); see the app's todo list for \"{title}\" ({id})"
    );
    Ok(())
}

fn arg(args: &[String], flag: &str) -> Option<String> {
    args.iter()
        .position(|arg| arg == flag)
        .and_then(|index| args.get(index + 1))
        .cloned()
}

async fn transport(
    server_url: &str,
    token: &str,
    payload: &SyncPayload,
) -> Result<SyncPayload, SyncError> {
    let http = reqwest::Client::new();
    let response = http
        .post(server_url)
        .bearer_auth(token)
        .header("x-device-id", DEVICE_ID)
        .json(payload)
        .send()
        .await
        .map_err(|err| SyncError::BlockingTask(err.to_string()))?;
    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_else(|_| String::new());
        return Err(SyncError::InvalidPayload(format!(
            "sync failed ({status}): {body}"
        )));
    }
    response
        .json::<SyncPayload>()
        .await
        .map_err(|err| SyncError::BlockingTask(err.to_string()))
}