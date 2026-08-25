use sea_orm_migration::{prelude::*, schema::*, sea_orm::DatabaseBackend};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(OneTimePassword::Table)
                    .if_not_exists()
                    .col(pk_uuid(OneTimePassword::Identifier))
                    .col(uuid(OneTimePassword::UserIdentifier).not_null())
                    .col(ColumnDef::new(OneTimePassword::Code).char_len(6).not_null())
                    .col(timestamp_with_time_zone(OneTimePassword::CreatedAt))
                    .col(timestamp_with_time_zone(OneTimePassword::UpdatedAt).null())
                    .to_owned(),
            )
            .await?;

        let db_backend = manager.get_database_backend();
        if db_backend == DatabaseBackend::Postgres {
            manager
                .create_foreign_key(
                    ForeignKey::create()
                        .name("fk_one_time_password_user_identifier")
                        .from(OneTimePassword::Table, OneTimePassword::UserIdentifier)
                        .to("users", "identifier")
                        .on_delete(ForeignKeyAction::Cascade)
                        .to_owned(),
                )
                .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(OneTimePassword::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum OneTimePassword {
    Table,
    Identifier,
    UserIdentifier,
    Code,
    CreatedAt,
    UpdatedAt,
}
