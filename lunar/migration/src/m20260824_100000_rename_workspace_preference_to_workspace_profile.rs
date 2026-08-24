use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .rename_table(
                Table::rename()
                    .table("workspace_preferences", "workspace_profiles")
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table("workspace_profiles")
                    .add_column_if_not_exists(string_null("profile_picture"))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table("workspace_profiles")
                    .drop_column("profile_picture")
                    .to_owned(),
            )
            .await?;

        manager
            .rename_table(
                Table::rename()
                    .table("workspace_profiles", "workspace_preferences")
                    .to_owned(),
            )
            .await
    }
}
