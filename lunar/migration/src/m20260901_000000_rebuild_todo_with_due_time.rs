use sea_orm_migration::{prelude::*, sea_orm::DbBackend};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260901_000000_rebuild_todo_with_due_time"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let backend = manager.get_database_backend();

        // The SQLite branch of m20260224_221334 (_add_workspace_id_to_todo_entities)
        // rebuilt the legacy `todo` table through a `todo_new` copy/drop/rename and
        // accidentally dropped the `due_time` column (and failed to copy
        // `due_date`/`priority`/`workspace_identifier`). That migration is already
        // recorded in seaql_migrations on existing installs, so its column is never
        // re-added and queries against `todo` fail with "no such column: todo.due_time".
        //
        // This migration surgically rebuilds the table with the full column set and
        // copies every existing value across, converging both drifted and fresh
        // databases onto the same shape.
        if backend == DbBackend::Sqlite {
            // The source `todo` table may have drifted and be missing columns
            // such as `due_time` (see the comment above). Detect which columns
            // actually exist so the copy below does not fail with
            // "no such column", and NULL the missing ones out. Every column in
            // the destination `todo_new` is either copied or defaulted to NULL,
            // so the rebuilt table converges onto the correct shape.
            let existing: Vec<String> = db
                .query_all_raw(sea_orm::Statement::from_string(
                    sea_orm::DatabaseBackend::Sqlite,
                    "PRAGMA table_info(todo)",
                ))
                .await
                .map(|rows| {
                    rows.iter()
                        .filter_map(|row| row.try_get_by_index::<String>(1).ok())
                        .collect()
                })?;

            let has_workspace_identifier = existing
                .iter()
                .any(|c| c.eq_ignore_ascii_case("workspace_identifier"));
            let has_due_time = existing.iter().any(|c| c.eq_ignore_ascii_case("due_time"));

            let workspace_src = if has_workspace_identifier {
                "workspace_identifier".to_string()
            } else {
                "NULL AS workspace_identifier".to_string()
            };
            let due_time_src = if has_due_time {
                "due_time".to_string()
            } else {
                "NULL AS due_time".to_string()
            };

            let copy_columns =
                "identifier, title, description, due_date, priority, done, created_at, updated_at";
            let sql = format!(
                r#"
                PRAGMA foreign_keys = OFF;

                CREATE TABLE IF NOT EXISTS todo_new (
                    identifier uuid_text NOT NULL PRIMARY KEY,
                    title varchar NOT NULL,
                    description text NULL,
                    due_date date_text NULL,
                    priority enum_text NOT NULL DEFAULT 'medium',
                    done boolean NOT NULL DEFAULT FALSE,
                    created_at timestamp_with_timezone_text NOT NULL,
                    updated_at timestamp_with_timezone_text NOT NULL,
                    due_time time_text NULL,
                    workspace_identifier uuid_text NULL
                );

                INSERT INTO todo_new ({copy_columns}, due_time, workspace_identifier)
                SELECT {copy_columns}, {due_time_src}, {workspace_src} FROM todo;

                DROP TABLE todo;

                ALTER TABLE todo_new RENAME TO todo;

                PRAGMA foreign_keys = ON;

                DROP TRIGGER IF EXISTS todo_sync_insert;
                DROP TRIGGER IF EXISTS todo_sync_update;
                DROP TRIGGER IF EXISTS todo_sync_delete;

                CREATE TRIGGER IF NOT EXISTS todo_sync_insert
                AFTER INSERT ON todo
                BEGIN
                INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
                VALUES (randomblob(16), 'todo', lower(hex(NEW.identifier)), 'INSERT', datetime('now'));
                END;

                CREATE TRIGGER IF NOT EXISTS todo_sync_update
                AFTER UPDATE ON todo
                BEGIN
                INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
                VALUES (randomblob(16), 'todo', lower(hex(NEW.identifier)), 'UPDATE', datetime('now'));
                END;

                CREATE TRIGGER IF NOT EXISTS todo_sync_delete
                AFTER DELETE ON todo
                BEGIN
                INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
                VALUES (randomblob(16), 'todo', lower(hex(OLD.identifier)), 'DELETE', datetime('now'));
                END;
                "#,
            );

            db.execute_unprepared(&sql).await?;
        } else if backend == DbBackend::MySql {
            if !manager.has_column("todo", "due_time").await? {
                db.execute_unprepared("ALTER TABLE todo ADD COLUMN due_time TIME NULL")
                    .await?;
            }
        } else {
            db.execute_unprepared(
                r#"
                ALTER TABLE "todo" ADD COLUMN IF NOT EXISTS "due_time" time;
                "#,
            )
            .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let backend = manager.get_database_backend();

        if backend == DbBackend::Sqlite {
            if !manager.has_column("todo", "due_time").await? {
                return Ok(());
            }

            db.execute_unprepared(
                r#"
                PRAGMA foreign_keys = OFF;

                CREATE TABLE todo_backup (
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

                INSERT INTO todo_backup (identifier, title, description, due_date, priority, done, created_at, updated_at, workspace_identifier)
                SELECT identifier, title, description, due_date, priority, done, created_at, updated_at, workspace_identifier FROM todo;

                DROP TABLE todo;

                ALTER TABLE todo_backup RENAME TO todo;

                PRAGMA foreign_keys = ON;

                DROP TRIGGER IF EXISTS todo_sync_insert;
                DROP TRIGGER IF EXISTS todo_sync_update;
                DROP TRIGGER IF EXISTS todo_sync_delete;

                CREATE TRIGGER IF NOT EXISTS todo_sync_insert
                AFTER INSERT ON todo
                BEGIN
                INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
                VALUES (randomblob(16), 'todo', lower(hex(NEW.identifier)), 'INSERT', datetime('now'));
                END;

                CREATE TRIGGER IF NOT EXISTS todo_sync_update
                AFTER UPDATE ON todo
                BEGIN
                INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
                VALUES (randomblob(16), 'todo', lower(hex(NEW.identifier)), 'UPDATE', datetime('now'));
                END;

                CREATE TRIGGER IF NOT EXISTS todo_sync_delete
                AFTER DELETE ON todo
                BEGIN
                INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
                VALUES (randomblob(16), 'todo', lower(hex(OLD.identifier)), 'DELETE', datetime('now'));
                END;
                "#,
            )
            .await?;
        } else {
            db.execute_unprepared(
                r#"
                ALTER TABLE "todo" DROP COLUMN IF EXISTS "due_time";
                "#,
            )
            .await?;
        }

        Ok(())
    }
}
