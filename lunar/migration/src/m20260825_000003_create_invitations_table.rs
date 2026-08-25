use sea_orm_migration::{prelude::*, schema::*, sea_orm::DatabaseBackend};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Invitation::Table)
                    .if_not_exists()
                    .col(pk_uuid(Invitation::Identifier))
                    .col(uuid(Invitation::WorkspaceIdentifier).not_null())
                    .col(string(Invitation::Email))
                    .col(string_null(Invitation::FirstName))
                    .col(string_null(Invitation::LastName))
                    .col(ColumnDef::new(Invitation::Token).string().not_null().unique_key())
                    .col(ColumnDef::new(Invitation::Status).string().not_null().default("pending"))
                    .col(timestamp_with_time_zone(Invitation::ExpiresAt))
                    .col(timestamp_with_time_zone(Invitation::CreatedAt))
                    .to_owned(),
            )
            .await?;

        let db_backend = manager.get_database_backend();
        if db_backend == DatabaseBackend::Postgres {
            manager
                .create_foreign_key(
                    ForeignKey::create()
                        .name("fk_invitations_workspace_identifier")
                        .from(Invitation::Table, Invitation::WorkspaceIdentifier)
                        .to("workspaces", "identifier")
                        .on_delete(ForeignKeyAction::Cascade)
                        .to_owned(),
                )
                .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Invitation::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Invitation {
    Table,
    Identifier,
    WorkspaceIdentifier,
    Email,
    FirstName,
    LastName,
    Token,
    Status,
    ExpiresAt,
    CreatedAt,
}
