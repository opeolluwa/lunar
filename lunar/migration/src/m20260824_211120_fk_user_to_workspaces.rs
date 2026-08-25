use sea_orm_migration::{prelude::*,sea_orm::DatabaseBackend};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260824_211120_fk_user_to_workspaces"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        let db_backend = manager.get_database_backend();
        if db_backend == DatabaseBackend::Postgres{
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_user_workspace_identifier")
                    .from("workspaces", "user_identifier")
                    .to("users", "identifier")
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db_backend = manager.get_database_backend();
        if db_backend == DatabaseBackend::Postgres{
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_user_workspace_identifier")
                    .to_owned(),
            )
            .await?;
        }
        Ok(())
    }
}
