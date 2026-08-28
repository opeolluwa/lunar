//! The shared Loomabase synchronization contract between the desktop client
//! (`console`) and the orchard server.
//!
//! The contract table names intentionally avoid colliding with the legacy
//! sea-orm entity tables (which use `identifier` primary keys). Each table is
//! a standalone CRDT store with an implicit `id TEXT` primary key and the
//! reserved `deleted` liveness register; every other column is application data
//! that converges independently under last-writer-wins ordering.

use loomabase::schema::{ColumnDef, ColumnType, Contract, TableDef};
use loomabase::Result;

const WORKSPACE_IDENTIFIER: &str = "workspace_identifier";

/// Builds the canonical contract covering every syncable application entity.
/// Must remain byte-for-byte identical between client and server: the schema
/// fingerprints are derived from the table names and their column layouts.
pub fn contract() -> Result<Contract> {
    Contract::new(vec![
        todos_table()?,
        notes_table()?,
        bookmarks_table()?,
        reminders_table()?,
        snippets_table()?,
        workspaces_table()?,
        workspace_profiles_table()?,
        recycle_bin_table()?,
        sync_queue_table()?,
    ])
}

/// Resolves a `TableDef` from the contract by its schema fingerprint.
pub fn table_by_fingerprint(fingerprint: u64) -> Option<TableDef> {
    contract()
        .ok()?
        .tables()
        .iter()
        .find(|table| table.fingerprint() == fingerprint)
        .cloned()
}

fn timestamp_column(column: &str) -> ColumnDef {
    ColumnDef::new(column, ColumnType::Text)
}

fn optional_column(column: &str) -> ColumnDef {
    ColumnDef::new(column, ColumnType::Text)
}

fn todos_table() -> Result<TableDef> {
    TableDef::new(
        "todos",
        vec![
            ColumnDef::new("title", ColumnType::Text),
            // The app's `done` flag maps onto the canonical Loomabase register.
            ColumnDef::new("completed", ColumnType::Boolean),
            optional_column("description"),
            optional_column("due_date"),
            optional_column("due_time"),
            ColumnDef::new("priority", ColumnType::Text),
            timestamp_column("created_at"),
            timestamp_column("updated_at"),
            optional_column(WORKSPACE_IDENTIFIER),
        ],
    )
}

fn notes_table() -> Result<TableDef> {
    TableDef::new(
        "loomabase_notes",
        vec![
            ColumnDef::new("title", ColumnType::Text),
            ColumnDef::new("content", ColumnType::Text),
            optional_column("categories"),
            timestamp_column("created_at"),
            timestamp_column("updated_at"),
            optional_column(WORKSPACE_IDENTIFIER),
        ],
    )
}

fn bookmarks_table() -> Result<TableDef> {
    TableDef::new(
        "loomabase_bookmarks",
        vec![
            ColumnDef::new("title", ColumnType::Text),
            ColumnDef::new("url", ColumnType::Text),
            ColumnDef::new("tag", ColumnType::Text),
            timestamp_column("created_at"),
            timestamp_column("updated_at"),
            optional_column(WORKSPACE_IDENTIFIER),
        ],
    )
}

fn reminders_table() -> Result<TableDef> {
    TableDef::new(
        "loomabase_reminders",
        vec![
            ColumnDef::new("title", ColumnType::Text),
            optional_column("description"),
            ColumnDef::new("recurring", ColumnType::Boolean),
            optional_column("recurrence_rule"),
            optional_column("alarm_sound"),
            ColumnDef::new("remind_at", ColumnType::Text),
            timestamp_column("created_at"),
            timestamp_column("updated_at"),
            optional_column(WORKSPACE_IDENTIFIER),
        ],
    )
}

fn snippets_table() -> Result<TableDef> {
    TableDef::new(
        "loomabase_snippets",
        vec![
            optional_column("title"),
            optional_column("language"),
            ColumnDef::new("code", ColumnType::Text),
            optional_column("description"),
            ColumnDef::new("is_pinned", ColumnType::Boolean),
            timestamp_column("created_at"),
            timestamp_column("updated_at"),
            optional_column(WORKSPACE_IDENTIFIER),
        ],
    )
}

fn workspaces_table() -> Result<TableDef> {
    TableDef::new(
        "loomabase_workspaces",
        vec![
            ColumnDef::new("name", ColumnType::Text),
            ColumnDef::new("description", ColumnType::Text),
            timestamp_column("created_at"),
            timestamp_column("updated_at"),
            ColumnDef::new("is_default", ColumnType::Boolean),
            ColumnDef::new("is_hidden", ColumnType::Boolean),
            ColumnDef::new("is_secured", ColumnType::Boolean),
            optional_column("password_hash"),
            optional_column("user_identifier"),
        ],
    )
}

fn workspace_profiles_table() -> Result<TableDef> {
    TableDef::new(
        "loomabase_workspace_profiles",
        vec![
            ColumnDef::new("first_name", ColumnType::Text),
            ColumnDef::new("last_name", ColumnType::Text),
            timestamp_column("created_at"),
            timestamp_column("updated_at"),
            optional_column(WORKSPACE_IDENTIFIER),
            optional_column("profile_picture"),
        ],
    )
}

fn recycle_bin_table() -> Result<TableDef> {
    TableDef::new(
        "loomabase_recycle_bin",
        vec![
            ColumnDef::new("item_id", ColumnType::Text),
            ColumnDef::new("item_type", ColumnType::Text),
            ColumnDef::new("payload", ColumnType::Text),
            timestamp_column("deleted_at"),
            optional_column(WORKSPACE_IDENTIFIER),
        ],
    )
}

fn sync_queue_table() -> Result<TableDef> {
    TableDef::new(
        "loomabase_sync_queue",
        vec![
            ColumnDef::new("table_name", ColumnType::Text),
            ColumnDef::new("record_identifier", ColumnType::Text),
            ColumnDef::new("operation", ColumnType::Text),
            timestamp_column("created_at"),
        ],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contract_builds_with_unique_tables() {
        let contract = contract().expect("contract must build");
        let mut names: Vec<&str> = contract.tables().iter().map(TableDef::name).collect();
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), contract.tables().len(), "tables must be unique");
    }

    #[test]
    fn fingerprints_resolve() {
        let contract = contract().expect("contract must build");
        for table in contract.tables() {
            assert_eq!(
                table_by_fingerprint(table.fingerprint()).map(|t| t.name().to_owned()),
                Some(table.name().to_owned())
            );
        }
    }
}