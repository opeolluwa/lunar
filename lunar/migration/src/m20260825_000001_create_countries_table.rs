use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Country::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Country::Identifier)
                            .char_len(26)
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Country::CurrencyCode).string_len(10).not_null())
                    .col(ColumnDef::new(Country::Currency).string_len(100).not_null())
                    .col(ColumnDef::new(Country::Country).string_len(100).not_null())
                    .col(text_null(Country::Flag))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Country::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Country {
    Table,
    Identifier,
    CurrencyCode,
    Currency,
    Country,
    Flag,
}
