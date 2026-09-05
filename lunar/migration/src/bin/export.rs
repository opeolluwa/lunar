use std::env;
use sea_orm::DbBackend;

#[tokio::main]
async fn main() -> Result<(), sea_orm_migration::prelude::DbErr> {
    let output = env::args()
        .nth(1)
        .expect("usage: migration-export <output-dir>");

    let backend = env::args()
        .nth(2)
        .expect("usage: migration-export <backend>");

    let backend = match backend.as_str() {
        "postgres" => DbBackend::Postgres,
        "mysql" => DbBackend::MySql,
        "sqlite" => DbBackend::Sqlite,
        _ => return Err(sea_orm_migration::prelude::DbErr::Custom("invalid backend".to_string())),
    };

    migration::exporter::export_sql(output, backend).await?;

    Ok(())
}
