use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

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

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Invitation::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Invitation::Identifier)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Invitation::WorkspaceIdentifier)
                            .uuid()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Invitation::Email).string().not_null())
                    .col(ColumnDef::new(Invitation::FirstName).string().null())
                    .col(ColumnDef::new(Invitation::LastName).string().null())
                    .col(
                        ColumnDef::new(Invitation::Token)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(Invitation::Status)
                            .string()
                            .not_null()
                            .default("pending"),
                    )
                    .col(ColumnDef::new(Invitation::ExpiresAt).timestamp().not_null())
                    .col(ColumnDef::new(Invitation::CreatedAt).timestamp().not_null())
                    // .foreign_key(
                    //     ForeignKey::create()
                    //         .from(Invitation::Table, Invitation::WorkspaceIdentifier)
                    //         .to("workspaces", "identifier")
                    //         .on_delete(ForeignKeyAction::Cascade),
                    // )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Invitation::Table).to_owned())
            .await
    }
}
