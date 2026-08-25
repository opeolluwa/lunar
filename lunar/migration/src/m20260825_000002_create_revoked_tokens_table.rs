use sea_orm_migration::{prelude::*, schema::*, sea_orm::DatabaseBackend};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RevokedToken::Table)
                    .if_not_exists()
                    .col(pk_uuid(RevokedToken::Identifier))
                    .col(ColumnDef::new(RevokedToken::Jti).uuid().not_null().unique_key())
                    .col(uuid(RevokedToken::UserIdentifier).not_null())
                    .col(timestamp_with_time_zone(RevokedToken::ExpiresAt))
                    .col(
                        timestamp_with_time_zone(RevokedToken::RevokedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        let db_backend = manager.get_database_backend();
        if db_backend == DatabaseBackend::Postgres {
            manager
                .create_foreign_key(
                    ForeignKey::create()
                        .name("fk_revoked_tokens_user_identifier")
                        .from(RevokedToken::Table, RevokedToken::UserIdentifier)
                        .to("users", "identifier")
                        .on_delete(ForeignKeyAction::Cascade)
                        .to_owned(),
                )
                .await?;
        }

        manager
            .create_index(
                Index::create()
                    .name("revoked_tokens_jti_idx")
                    .table(RevokedToken::Table)
                    .col(RevokedToken::Jti)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("revoked_tokens_expires_at_idx")
                    .table(RevokedToken::Table)
                    .col(RevokedToken::ExpiresAt)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RevokedToken::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum RevokedToken {
    Table,
    Identifier,
    Jti,
    UserIdentifier,
    ExpiresAt,
    RevokedAt,
}
