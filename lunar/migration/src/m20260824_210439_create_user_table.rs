use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260824_210439_create_user_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("users")
                    .col(ColumnDef::new("identifier").uuid().primary_key())
                    .col(string_null("first_name"))
                    .col(string("password"))
                    .col(string_null("last_name"))
                    .col(string("email").unique_key())
                    .col(boolean("is_active").default(false))
                    .col(string_null("profile_picture"))
                    .col(string_null("username"))
                    .col(date_time("created_at").default(Expr::current_timestamp()))
                    .col(date_time_null("updated_at"))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("users").to_owned())
            .await
    }
}
