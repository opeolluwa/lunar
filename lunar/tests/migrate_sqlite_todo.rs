use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectionTrait, DbBackend, Statement};

async fn columns(db: &sea_orm::DatabaseConnection) -> Vec<String> {
    let rows = db
        .query_all_raw(Statement::from_string(
            DbBackend::Sqlite,
            "PRAGMA table_info(todo)",
        ))
        .await
        .expect("pragma");
    rows.iter()
        .filter_map(|r| r.try_get_by_index::<String>(1).ok())
        .collect()
}

async fn init_pre_migration_schema(db: &sea_orm::DatabaseConnection) {
    db.execute_unprepared(
        r#"
        CREATE TABLE seaql_migrations (version varchar NOT NULL PRIMARY KEY, applied_at bigint NOT NULL DEFAULT (strftime('%s','now')));

        CREATE TABLE todo (
            identifier uuid_text NOT NULL PRIMARY KEY,
            title varchar NOT NULL,
            description text NULL,
            due_date date_text NULL,
            priority enum_text NOT NULL DEFAULT 'medium',
            done boolean NOT NULL DEFAULT FALSE,
            created_at timestamp_with_timezone_text NOT NULL,
            updated_at timestamp_with_timezone_text NOT NULL,
            workspace_identifier uuid_text NULL
        );

        INSERT INTO todo (identifier, title, description, due_date, priority, done, created_at, updated_at, workspace_identifier)
        VALUES ('00000000-0000-0000-0000-000000000001', 'Drift Task', 'kept', NULL, 'high', 0, '2026-01-01T00:00:00Z', '2026-01-01T00:00:00Z', NULL);
        "#,
    )
    .await
    .expect("seed");
}

#[tokio::test]
async fn drifted_todo_without_due_time_keeps_rows_and_gains_due_time() {
    let path = format!(
        "/var/folders/84/fm4bc0c57lnbb2pyn3lpz2y00000gn/T/opencode/todo_drift_{}.db",
        std::process::id()
    );
    std::fs::remove_file(&path).ok();
    let db = sea_orm::Database::connect(&format!("sqlite://{path}?mode=rwc"))
        .await
        .expect("connect");

    init_pre_migration_schema(&db).await;

    Migrator::up(&db, None).await.expect("run migrations");

    let cols = columns(&db).await;
    assert!(cols.iter().any(|c| c == "due_time"), "missing due_time: {cols:?}");
    assert!(cols.iter().any(|c| c == "workspace_identifier"));

    let row = db
        .query_one_raw(Statement::from_string(
            DbBackend::Sqlite,
            "SELECT title, due_time IS NULL AS due_null FROM todo WHERE identifier = '00000000-0000-0000-0000-000000000001'",
        ))
        .await
        .expect("query")
        .expect("row");
    let title: String = row.try_get_by_index(0).expect("title");
    let due_null: bool = row.try_get_by_index(1).expect("due_null");
    assert_eq!(title, "Drift Task", "row data must be preserved");
    assert!(due_null, "due_time must be NULL for migrated rows");

    std::fs::remove_file(&path).ok();
}

#[tokio::test]
async fn fresh_install_full_chain_keeps_due_time() {
    let path = format!(
        "/var/folders/84/fm4bc0c57lnbb2pyn3lpz2y00000gn/T/opencode/todo_fresh_{}.db",
        std::process::id()
    );
    std::fs::remove_file(&path).ok();
    let db = sea_orm::Database::connect(&format!("sqlite://{path}?mode=rwc"))
        .await
        .expect("connect");

    Migrator::up(&db, None).await.expect("run full chain");

    let cols = columns(&db).await;
    assert!(cols.iter().any(|c| c == "due_time"), "missing due_time: {cols:?}");
    assert!(cols.iter().any(|c| c == "workspace_identifier"));

    let applied = db
        .query_all_raw(Statement::from_string(
            DbBackend::Sqlite,
            "SELECT count(*) FROM seaql_migrations WHERE version = 'm20260901_000000_rebuild_todo_with_due_time'",
        ))
        .await
        .expect("seaql");
    let count: i64 = applied[0].try_get_by_index(0).expect("count");
    assert_eq!(count, 1, "rebuild migration must be recorded exactly once");

    std::fs::remove_file(&path).ok();
}
