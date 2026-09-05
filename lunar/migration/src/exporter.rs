use std::{fs, path::Path};

use sea_orm::{DbBackend, MockDatabase};
use sea_orm_migration::{manager::SchemaManager, prelude::*};

use crate::Migrator;

pub async fn export_sql(output_dir: impl AsRef<Path>, backend: DbBackend) -> Result<(), DbErr> {
    let output_dir = output_dir.as_ref();

    fs::create_dir_all(output_dir).map_err(|e| DbErr::Custom(e.to_string()))?;

    generate_sql(backend, output_dir).await?;

    Ok(())
}

async fn generate_sql(backend: DbBackend, output_dir: &Path) -> Result<(), DbErr> {
    for migration in Migrator::migrations() {
        let name = migration.name();

        println!("Generating {name}");

        let db = MockDatabase::new(backend)
            .append_exec_results((0..10_000).map(|_| Default::default()))
            .into_connection();

        let manager = SchemaManager::new(&db);

        migration.up(&manager).await?;

        let transactions = db.into_transaction_log();

        let mut sql = String::new();

        sql.push_str("-- ============================================\n");
        sql.push_str(&format!("-- {name}\n"));
        sql.push_str("-- ============================================\n\n");

        for transaction in transactions {
            for statement in transaction.statements() {
                sql.push_str(&statement.sql);
                sql.push_str(";\n\n");
            }
        }

        let filename = format!("{name}.sql");

        let path = output_dir.join(filename);

        fs::write(&path, sql).map_err(|e| DbErr::Custom(e.to_string()))?;

        println!("  -> {}", path.display());
    }

    Ok(())
}
