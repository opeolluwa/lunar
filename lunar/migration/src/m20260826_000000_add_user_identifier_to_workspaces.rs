use sea_orm_migration::{prelude::*, sea_orm::DbBackend};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260826_000000_add_user_identifier_to_workspaces"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let backend = manager.get_database_backend();

        if backend == DbBackend::Sqlite {
            // if manager.has_column("workspaces", "user_identifier").await? {
            //     return Ok(());
            // }

            db.execute_unprepared(
                r#"
                PRAGMA foreign_keys = OFF;

                CREATE TABLE workspaces_new (
                    identifier BLOB PRIMARY KEY NOT NULL,
                    name TEXT NOT NULL,
                    description TEXT NOT NULL,
                    is_default BOOLEAN NOT NULL DEFAULT FALSE,
                    is_hidden BOOLEAN NOT NULL DEFAULT FALSE,
                    is_secured BOOLEAN NOT NULL DEFAULT FALSE,
                    password_hash TEXT,
                    created_at TEXT NOT NULL,
                    updated_at TEXT NOT NULL,
                    user_identifier BLOB
                );

                INSERT INTO workspaces_new (identifier, name, description, is_default, is_hidden, is_secured, password_hash, created_at, updated_at, user_identifier)
                SELECT identifier, name, description, is_default, is_hidden, is_secured, password_hash, created_at, updated_at, NULL FROM workspaces;

                DROP TABLE workspaces;

                ALTER TABLE workspaces_new RENAME TO workspaces;

                PRAGMA foreign_keys = ON;

                DROP TRIGGER IF EXISTS workspaces_sync_insert;
                DROP TRIGGER IF EXISTS workspaces_sync_update;
                DROP TRIGGER IF EXISTS workspaces_sync_delete;

                CREATE TRIGGER IF NOT EXISTS workspaces_sync_insert
                AFTER INSERT ON workspaces
                BEGIN
                INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
                VALUES (randomblob(16), 'workspaces', lower(hex(NEW.identifier)), 'INSERT', datetime('now'));
                END;

                CREATE TRIGGER IF NOT EXISTS workspaces_sync_update
                AFTER UPDATE ON workspaces
                BEGIN
                INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
                VALUES (randomblob(16), 'workspaces', lower(hex(NEW.identifier)), 'UPDATE', datetime('now'));
                END;

                CREATE TRIGGER IF NOT EXISTS workspaces_sync_delete
                AFTER DELETE ON workspaces
                BEGIN
                INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
                VALUES (randomblob(16), 'workspaces', lower(hex(OLD.identifier)), 'DELETE', datetime('now'));
                END;
                "#,
            )
            .await?;
        } else if backend == DbBackend::MySql {
            if !manager.has_column("workspaces", "user_identifier").await? {
                db.execute_unprepared(
                    "ALTER TABLE workspaces ADD COLUMN user_identifier VARCHAR(36) NULL",
                )
                .await?;
            }
        } else {
            db.execute_unprepared(
                r#"
                ALTER TABLE "workspaces" ADD COLUMN IF NOT EXISTS "user_identifier" uuid;
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
            if !manager.has_column("workspaces", "user_identifier").await? {
                return Ok(());
            }

            db.execute_unprepared(
                r#"
                PRAGMA foreign_keys = OFF;

                CREATE TABLE workspaces_backup (
                    identifier BLOB PRIMARY KEY NOT NULL,
                    name TEXT NOT NULL,
                    description TEXT NOT NULL,
                    is_default BOOLEAN NOT NULL DEFAULT FALSE,
                    is_hidden BOOLEAN NOT NULL DEFAULT FALSE,
                    is_secured BOOLEAN NOT NULL DEFAULT FALSE,
                    password_hash TEXT,
                    created_at TEXT NOT NULL,
                    updated_at TEXT NOT NULL
                );

                INSERT INTO workspaces_backup SELECT identifier, name, description, is_default, is_hidden, is_secured, password_hash, created_at, updated_at FROM workspaces;

                DROP TABLE workspaces;

                ALTER TABLE workspaces_backup RENAME TO workspaces;

                PRAGMA foreign_keys = ON;

                DROP TRIGGER IF EXISTS workspaces_sync_insert;
                DROP TRIGGER IF EXISTS workspaces_sync_update;
                DROP TRIGGER IF EXISTS workspaces_sync_delete;

                CREATE TRIGGER IF NOT EXISTS workspaces_sync_insert
                AFTER INSERT ON workspaces
                BEGIN
                INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
                VALUES (randomblob(16), 'workspaces', lower(hex(NEW.identifier)), 'INSERT', datetime('now'));
                END;

                CREATE TRIGGER IF NOT EXISTS workspaces_sync_update
                AFTER UPDATE ON workspaces
                BEGIN
                INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
                VALUES (randomblob(16), 'workspaces', lower(hex(NEW.identifier)), 'UPDATE', datetime('now'));
                END;

                CREATE TRIGGER IF NOT EXISTS workspaces_sync_delete
                AFTER DELETE ON workspaces
                BEGIN
                INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
                VALUES (randomblob(16), 'workspaces', lower(hex(OLD.identifier)), 'DELETE', datetime('now'));
                END;
                "#,
            )
            .await?;
        } else {
            db.execute_unprepared(
                r#"
                ALTER TABLE "workspaces" DROP COLUMN IF EXISTS "user_identifier";
                "#,
            )
            .await?;
        }

        Ok(())
    }
}
