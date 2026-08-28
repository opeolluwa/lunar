//! Offline-first round-trip test: two `SqliteClient` devices synchronize through
//! the real PostgreSQL-backed orchard merge (`loomabase::server`), the same
//! code path exposed at `POST /sync`. Mirrors
//! `loomabase/examples/offline_roundtrip.rs` but with tenant-scoped Postgres.

use std::sync::Arc;

use loomabase::client::{SqliteClient, Todo};
use loomabase::crdt::SyncPayload;
use loomabase::schema::TableDef;
use loomabase::server::{initialize_server_schema_with, merge_crdt_states};
use sqlx_postgres::{PgPool, PgPoolOptions};
use uuid::Uuid;
use orchard_migration::MigratorTrait;

fn todos_table() -> TableDef {
    lunar::loomabase::contract::contract()
        .expect("shared contract must build")
        .tables()
        .iter()
        .find(|table| table.name() == "todos")
        .expect("contract must contain the todos table")
        .clone()
}

fn database_url() -> String {
    std::env::var("LOOMABASE_TEST_DATABASE_URL")
        .or_else(|_| std::env::var("DATABASE_URL"))
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/lunar".to_string())
}

async fn remote_merge(
    pool: &PgPool,
    payload: SyncPayload,
    device_id: &str,
    tenant_id: &str,
    table: &TableDef,
) -> loomabase::Result<SyncPayload> {
    let mut tx = pool.begin().await?;
    let response = merge_crdt_states(&mut tx, payload, device_id, tenant_id, table).await?;
    tx.commit().await?;
    Ok(response)
}

async fn sync(
    client: &SqliteClient,
    pool: PgPool,
    device_id: &str,
    tenant_id: &str,
    table: &TableDef,
) -> loomabase::Result<()> {
    let table = table.clone();
    client
        .sync_until_caught_up(move |payload| {
            let pool = pool.clone();
            let table = table.clone();
            let device_id = device_id.to_owned();
            let tenant_id = tenant_id.to_owned();
            async move { remote_merge(&pool, payload, &device_id, &tenant_id, &table).await }
        })
        .await?;
    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn two_devices_converge_over_postgres() -> loomabase::Result<()> {
    let pool = PgPoolOptions::new()
        .max_connections(4)
        .connect(&database_url())
        .await
        .expect("connect to test database");

    let table = todos_table();
    initialize_server_schema_with(&pool, &table).await.expect("init schema");

    let device_a = SqliteClient::open_with(":memory:", "device-a", table.clone()).await?;
    let device_b = SqliteClient::open_with(":memory:", "device-b", table.clone()).await?;

    device_a
        .create_todo("todo-1".into(), "Initial title".into(), false)
        .await?;
    sync(&device_a, pool.clone(), "device-a", "tenant-a", &table).await?;
    sync(&device_b, pool.clone(), "device-b", "tenant-a", &table).await?;

    // Both devices edit the same row offline, on different cells.
    device_a
        .update_title("todo-1".into(), "Edited offline on A".into())
        .await?;
    device_b
        .update_completed("todo-1".into(), true)
        .await?;

    sync(&device_a, pool.clone(), "device-a", "tenant-a", &table).await?;
    sync(&device_b, pool.clone(), "device-b", "tenant-a", &table).await?;
    sync(&device_a, pool.clone(), "device-a", "tenant-a", &table).await?;

    let expected = Todo {
        id: "todo-1".into(),
        title: "Edited offline on A".into(),
        completed: true,
    };
    assert_eq!(
        device_a.get_todo("todo-1".into()).await?,
        Some(expected.clone())
    );
    assert_eq!(device_b.get_todo("todo-1".into()).await?, Some(expected));

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn tenants_are_isolated() -> loomabase::Result<()> {
    let pool = PgPoolOptions::new()
        .max_connections(4)
        .connect(&database_url())
        .await
        .expect("connect to test database");

    let table = todos_table();
    initialize_server_schema_with(&pool, &table).await.expect("init schema");

    let tenant_a = SqliteClient::open_with(":memory:", "device-ta", table.clone()).await?;
    let tenant_b = SqliteClient::open_with(":memory:", "device-tb", table.clone()).await?;

    tenant_a
        .create_todo("secret-1".into(), "only for tenant a".into(), false)
        .await?;
    sync(&tenant_a, pool.clone(), "device-ta", "tenant-a", &table).await?;

    // A different tenant must never observe tenant-a's data.
    sync(&tenant_b, pool.clone(), "device-tb", "tenant-b", &table).await?;
    assert_eq!(tenant_b.get_todo("secret-1".into()).await?, None);

    // The owning tenant sees it.
    assert_eq!(
        tenant_a
            .get_todo("secret-1".into())
            .await?
            .map(|todo| todo.title),
        Some("only for tenant a".to_string())
    );

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn http_sync_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
    set_var_if_missing("JWT_SIGNING_KEY", "test-signing-key-for-smoke-test");
    set_var_if_missing("PORT", "8000");
    set_var_if_missing("MAX_DB_CONNECTIONS", "4");
    set_var_if_missing("BODY_LIMIT_MB", "25");
    set_var_if_missing("ENVIRONMENT", "development");
    set_var_if_missing("DATABASE_URL", &database_url());

    let pool = PgPoolOptions::new()
        .max_connections(4)
        .connect(&database_url())
        .await
        .expect("connect to test database");

    let table = todos_table();
    initialize_server_schema_with(&pool, &table).await.expect("init schema");

    let db_conn = Arc::new(sea_orm::Database::connect(&database_url()).await?);
    orchard_migration::Migrator::up(db_conn.as_ref(), None)
        .await
        .expect("run server migrations for auth tables");
    let router = orchard_lib::routes::router::load_routes(&db_conn, pool);

    let token = orchard_lib::adapters::jwt::Claims::builder()
        .email("smoke@example.com")
        .user_identifier(&Uuid::new_v4())
        .build_and_sign()
        .expect("build & sign smoke token");

    use axum::{body::Body, http::Request, http::StatusCode};
    use loomabase::crdt::{CrdtValue, RowChange};
    use tower::ServiceExt;

    let payload = loomabase::crdt::SyncPayload {
        protocol_version: 4,
        schema_fingerprint: table.fingerprint(),
        source_device_id: "smoke-device".into(),
        source_lamport: 1,
        changes: vec![RowChange {
            todo_id: "smoke-1".into(),
            columns: [
                (
                    "title".into(),
                    loomabase::crdt::CrdtColumn {
                        value: CrdtValue::Text("smoke title".into()),
                        metadata: loomabase::crdt::ColumnMetadata {
                            lamport_clock: 1,
                            device_id: "smoke-device".into(),
                        },
                    },
                ),
                (
                    "completed".into(),
                    loomabase::crdt::CrdtColumn {
                        value: CrdtValue::Boolean(false),
                        metadata: loomabase::crdt::ColumnMetadata {
                            lamport_clock: 1,
                            device_id: "smoke-device".into(),
                        },
                    },
                ),
            ]
            .into_iter()
            .collect(),
        }],
        cursor: 0,
        has_more: false,
        cursor_reset: false,
        cursor_token: None,
        server_epoch: None,
        rejections: vec![],
    };

    let request = Request::builder()
        .method("POST")
        .uri("/sync")
        .header(axum::http::header::CONTENT_TYPE, "application/json")
        .header(axum::http::header::AUTHORIZATION, format!("Bearer {token}"))
        .header("x-device-id", "smoke-device")
        .body(Body::from(serde_json::to_vec(&payload)?))
        .unwrap();

    let response = router.clone().oneshot(request).await?;
    assert_eq!(response.status(), StatusCode::OK);
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await?;
    let merged: loomabase::crdt::SyncPayload = serde_json::from_slice(&bytes)?;
    assert!(merged.source_device_id == "loomabase-server");

    // Pull the rows back on a second sync (empty changes, cursor 0 => full feed).
    let request = Request::builder()
        .method("POST")
        .uri("/sync")
        .header(axum::http::header::CONTENT_TYPE, "application/json")
        .header(axum::http::header::AUTHORIZATION, format!("Bearer {token}"))
        .header("x-device-id", "smoke-device")
        .body(Body::from(serde_json::to_vec(&loomabase::crdt::SyncPayload {
            protocol_version: 4,
            schema_fingerprint: table.fingerprint(),
            source_device_id: "smoke-device".into(),
            source_lamport: 2,
            changes: vec![],
            cursor: 0,
            has_more: false,
            cursor_reset: false,
            cursor_token: None,
            server_epoch: None,
            rejections: vec![],
        })?))
        .unwrap();

    let response = router.oneshot(request).await?;
    assert_eq!(response.status(), StatusCode::OK);
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await?;
    let feed: loomabase::crdt::SyncPayload = serde_json::from_slice(&bytes)?;
    let row = feed
        .changes
        .iter()
        .find(|row| row.todo_id == "smoke-1")
        .expect("smoke-1 present in change feed");
    assert_eq!(
        row.columns.get("title").map(|column| column.value.clone()),
        Some(CrdtValue::Text("smoke title".into()))
    );

    Ok(())
}

fn set_var_if_missing(key: &str, value: &str) {
    if std::env::var(key).is_err() {
        std::env::set_var(key, value);
    }
}