use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TtsConfig::Table)
                    .if_not_exists()
                    .col(pk_uuid(TtsConfig::Identifier))
                    .col(uuid_null(TtsConfig::UserIdentifier))
                    .col(string(TtsConfig::Language))
                    .col(string(TtsConfig::VoiceActor))
                    .col(double(TtsConfig::Rate).default(0.8))
                    .col(double(TtsConfig::Pitch).default(1.2))
                    .col(double(TtsConfig::Volume).default(1.0))
                    .col(timestamp_with_time_zone(TtsConfig::CreatedAt))
                    .col(timestamp_with_time_zone(TtsConfig::UpdatedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TtsConfig::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum TtsConfig {
    Table,
    Identifier,
    UserIdentifier,
    Language,
    VoiceActor,
    Rate,
    Pitch,
    Volume,
    CreatedAt,
    UpdatedAt,
}