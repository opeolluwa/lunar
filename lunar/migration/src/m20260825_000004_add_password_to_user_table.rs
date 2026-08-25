use sea_orm_migration::{prelude::*, sea_orm::DbBackend};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let backend = manager.get_database_backend();

        if backend == DbBackend::Sqlite {
            db.execute_unprepared(
                r#"
                ALTER TABLE users ADD COLUMN password TEXT NOT NULL DEFAULT '';
                "#,
            )
            .await?;
        } else if backend == DbBackend::MySql {
            if !manager.has_column("users", "password").await? {
                db.execute_unprepared(
                    "ALTER TABLE users ADD COLUMN password VARCHAR(255) NOT NULL DEFAULT ''",
                )
                .await?;
            }
        } else {
            db.execute_unprepared(
                r#"
                ALTER TABLE users ADD COLUMN IF NOT EXISTS password VARCHAR NOT NULL DEFAULT '';
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
            db.execute_unprepared(
                r#"
                CREATE TABLE users_backup (
                    identifier BLOB PRIMARY KEY NOT NULL,
                    first_name TEXT,
                    last_name TEXT,
                    email VARCHAR NOT NULL UNIQUE,
                    is_active BOOLEAN NOT NULL DEFAULT FALSE,
                    profile_picture TEXT,
                    username TEXT,
                    created_at TEXT NOT NULL,
                    updated_at TEXT
                );
                INSERT INTO users_backup SELECT identifier, first_name, last_name, email, is_active, profile_picture, username, created_at, updated_at FROM users;
                DROP TABLE users;
                ALTER TABLE users_backup RENAME TO users;
                "#,
            )
            .await?;
        } else {
            db.execute_unprepared(
                r#"
                ALTER TABLE users DROP COLUMN password;
                "#,
            )
            .await?;
        }

        Ok(())
    }
}
