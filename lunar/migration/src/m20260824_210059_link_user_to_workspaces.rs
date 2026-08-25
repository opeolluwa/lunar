use sea_orm_migration::{prelude::*, sea_orm::DatabaseBackend};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260824_210059_link_user_to_workspaces"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        let db_backend = manager.get_database_backend();
        if db_backend == DatabaseBackend::Postgres{
        manager
            .alter_table(
                Table::alter()
                    .table("workspaces")
                    .add_column(ColumnDef::new("user_identifier").uuid())
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
            .alter_table(
                Table::alter()
                    .table("workspaces")
                    .drop_column("user_identifier")
                    .to_owned(),
            )
            .await?;
        }

        Ok(())
    }
}
