//! Double-write mirror between the legacy sea-orm tables (the app's read path)
//! and the offline-first Loomabase CRDT store (the sync domain).
//!
//! Every local mutation reaches the store through one of the `mirror_*`
//! functions: full-model writes (`sync_cells`) for create/update-style
//! operations and targeted operations (`tombstone`, `restore_row`, `transfer`)
//! for the rest. Mirrors are best-effort: a store failure is logged but never
//! fails the user-facing command, because the legacy write already succeeded.
//!
//! [`apply_all`] is the receive path. After a sync completes, the accumulated
//! `RowChange`s (including the `deleted` liveness register) are written back
//! into the sea-orm tables so edits made on other devices actually show up in
//! this app. Live rows are merged onto the existing model (or a fresh default
//! when the row is new) and upserted; tombstoned rows are removed without
//! creating a second recycle-bin entry, because the bin table syncs its own
//! rows.
//!
//! This file is the single place that maps entity fields onto contract
//! columns, so schema changes touch exactly here.

use std::collections::BTreeMap;
use std::sync::Arc;

use loomabase::client::SqliteClient;
use loomabase::crdt::{CrdtColumn, CrdtValue, RowChange};
use lunar::entities::sea_orm_active_enums::{ItemType, Priority, Tag};
use lunar::entities::{bookmark, notes, recycle_bin, reminder, snippets, todo, workspaces, workspace_profiles};
use lunar::sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use lunar::sea_orm::prelude::DateTimeWithTimeZone;
use lunar::repositories::{
    bookmarks::BookmarkRepositoryExt, notes::NotesRepositoryExt,
    recycle_bin::RecycleBinRepositoryExt, reminder::ReminderRepositoryExt,
    snippets::SnippetRepositoryExt, todo::TodoRepositoryExt, workspace::WorkspaceRepositoryExt,
    workspace_profiles::WorkspaceProfileRepositoryExt,
};
use uuid::Uuid;

use super::sync::SyncManager;

/// Contract table names, kept in sync with [`crate::state::sync::SyncManager`].
pub const TABLE_TODOS: &str = "todos";
pub const TABLE_NOTES: &str = "loomabase_notes";
pub const TABLE_BOOKMARKS: &str = "loomabase_bookmarks";
pub const TABLE_REMINDERS: &str = "loomabase_reminders";
pub const TABLE_SNIPPETS: &str = "loomabase_snippets";
pub const TABLE_WORKSPACES: &str = "loomabase_workspaces";
pub const TABLE_WORKSPACE_PROFILES: &str = "loomabase_workspace_profiles";
pub const TABLE_RECYCLE_BIN: &str = "loomabase_recycle_bin";

const KEY_WORKSPACE_IDENTIFIER: &str = "workspace_identifier";

fn db_error(error: lunar::sea_orm::DbErr) -> crate::errors::AppError {
    crate::errors::AppError::Kernel(lunar::error::LunarError::DbOperationError(error.to_string()))
}

/// Maps a recycle-bin item type back to its entity store table.
pub fn table_for_item_type(item_type: &ItemType) -> &'static str {
    match item_type {
        ItemType::Todo => TABLE_TODOS,
        ItemType::Note => TABLE_NOTES,
        ItemType::Bookmark => TABLE_BOOKMARKS,
        ItemType::Snippet => TABLE_SNIPPETS,
        ItemType::Reminder => TABLE_REMINDERS,
    }
}

// ---------------------------------------------------------------------------
// Cell encoding: model -> contract columns
// ---------------------------------------------------------------------------

fn push_optional(cells: &mut BTreeMap<String, CrdtValue>, key: &str, value: Option<String>) {
    if let Some(value) = value {
        cells.insert(key.to_owned(), CrdtValue::Text(value));
    }
}

fn timestamp(clock: &DateTimeWithTimeZone) -> String {
    clock.to_rfc3339()
}

fn priority_text(priority: &Priority) -> &'static str {
    match priority {
        Priority::High => "high",
        Priority::Medium => "medium",
        Priority::Low => "low",
    }
}

fn tag_text(tag: &Tag) -> &'static str {
    match tag {
        Tag::Development => "development",
        Tag::Inspiration => "inspiration",
        Tag::Design => "design",
        Tag::Research => "research",
    }
}

fn item_type_text(item_type: &ItemType) -> &'static str {
    match item_type {
        ItemType::Todo => "todo",
        ItemType::Note => "note",
        ItemType::Bookmark => "bookmark",
        ItemType::Snippet => "snippet",
        ItemType::Reminder => "reminder",
    }
}

fn todo_cells(model: &todo::Model) -> BTreeMap<String, CrdtValue> {
    let mut cells = BTreeMap::new();
    cells.insert("title".to_owned(), CrdtValue::Text(model.title.clone()));
    cells.insert("completed".to_owned(), CrdtValue::Boolean(model.done));
    cells.insert("priority".to_owned(), CrdtValue::Text(priority_text(&model.priority).to_owned()));
    push_optional(&mut cells, "description", model.description.clone());
    if let Some(due_date) = model.due_date {
        cells.insert("due_date".to_owned(), CrdtValue::Text(due_date.format("%Y-%m-%d").to_string()));
    }
    if let Some(due_time) = model.due_time {
        cells.insert("due_time".to_owned(), CrdtValue::Text(due_time.format("%H:%M:%S").to_string()));
    }
    cells.insert("created_at".to_owned(), CrdtValue::Text(timestamp(&model.created_at)));
    cells.insert("updated_at".to_owned(), CrdtValue::Text(timestamp(&model.updated_at)));
    push_optional(&mut cells, KEY_WORKSPACE_IDENTIFIER, model.workspace_identifier.map(|id| id.to_string()));
    cells
}

fn notes_cells(model: &notes::Model) -> BTreeMap<String, CrdtValue> {
    let mut cells = BTreeMap::new();
    cells.insert("title".to_owned(), CrdtValue::Text(model.title.clone()));
    cells.insert("content".to_owned(), CrdtValue::Text(model.content.clone()));
    if let Some(categories) = &model.categories {
        let json = serde_json::to_string(categories).unwrap_or_else(|_| "[]".into());
        cells.insert("categories".to_owned(), CrdtValue::Text(json));
    }
    cells.insert("created_at".to_owned(), CrdtValue::Text(timestamp(&model.created_at)));
    cells.insert("updated_at".to_owned(), CrdtValue::Text(timestamp(&model.updated_at)));
    push_optional(&mut cells, KEY_WORKSPACE_IDENTIFIER, model.workspace_identifier.map(|id| id.to_string()));
    cells
}

fn bookmark_cells(model: &bookmark::Model) -> BTreeMap<String, CrdtValue> {
    let mut cells = BTreeMap::new();
    cells.insert("title".to_owned(), CrdtValue::Text(model.title.clone()));
    cells.insert("url".to_owned(), CrdtValue::Text(model.url.clone()));
    cells.insert("tag".to_owned(), CrdtValue::Text(tag_text(&model.tag).to_owned()));
    cells.insert("created_at".to_owned(), CrdtValue::Text(timestamp(&model.created_at)));
    cells.insert("updated_at".to_owned(), CrdtValue::Text(timestamp(&model.updated_at)));
    push_optional(&mut cells, KEY_WORKSPACE_IDENTIFIER, model.workspace_identifier.map(|id| id.to_string()));
    cells
}

fn reminder_cells(model: &reminder::Model) -> BTreeMap<String, CrdtValue> {
    let mut cells = BTreeMap::new();
    cells.insert("title".to_owned(), CrdtValue::Text(model.title.clone()));
    cells.insert("recurring".to_owned(), CrdtValue::Boolean(model.recurring));
    cells.insert("remind_at".to_owned(), CrdtValue::Text(timestamp(&model.remind_at)));
    push_optional(&mut cells, "description", model.description.clone());
    push_optional(&mut cells, "recurrence_rule", model.recurrence_rule.clone());
    push_optional(&mut cells, "alarm_sound", model.alarm_sound.clone());
    cells.insert("created_at".to_owned(), CrdtValue::Text(timestamp(&model.created_at)));
    cells.insert("updated_at".to_owned(), CrdtValue::Text(timestamp(&model.updated_at)));
    push_optional(&mut cells, KEY_WORKSPACE_IDENTIFIER, model.workspace_identifier.map(|id| id.to_string()));
    cells
}

fn snippet_cells(model: &snippets::Model) -> BTreeMap<String, CrdtValue> {
    let mut cells = BTreeMap::new();
    cells.insert("code".to_owned(), CrdtValue::Text(model.code.clone()));
    cells.insert("is_pinned".to_owned(), CrdtValue::Boolean(model.is_pinned));
    push_optional(&mut cells, "title", model.title.clone());
    push_optional(&mut cells, "language", model.language.clone());
    push_optional(&mut cells, "description", model.description.clone());
    cells.insert("created_at".to_owned(), CrdtValue::Text(timestamp(&model.created_at)));
    cells.insert("updated_at".to_owned(), CrdtValue::Text(timestamp(&model.updated_at)));
    push_optional(&mut cells, KEY_WORKSPACE_IDENTIFIER, model.workspace_identifier.map(|id| id.to_string()));
    cells
}

fn workspace_cells(model: &workspaces::Model) -> BTreeMap<String, CrdtValue> {
    let mut cells = BTreeMap::new();
    cells.insert("name".to_owned(), CrdtValue::Text(model.name.clone()));
    cells.insert("description".to_owned(), CrdtValue::Text(model.description.clone()));
    cells.insert("is_default".to_owned(), CrdtValue::Boolean(model.is_default));
    cells.insert("is_hidden".to_owned(), CrdtValue::Boolean(model.is_hidden));
    cells.insert("is_secured".to_owned(), CrdtValue::Boolean(model.is_secured));
    cells.insert("created_at".to_owned(), CrdtValue::Text(timestamp(&model.created_at)));
    cells.insert("updated_at".to_owned(), CrdtValue::Text(timestamp(&model.updated_at)));
    push_optional(&mut cells, "password_hash", model.password_hash.clone());
    push_optional(&mut cells, "user_identifier", model.user_identifier.map(|id| id.to_string()));
    cells
}

fn workspace_profile_cells(model: &workspace_profiles::Model) -> BTreeMap<String, CrdtValue> {
    let mut cells = BTreeMap::new();
    cells.insert("first_name".to_owned(), CrdtValue::Text(model.first_name.clone()));
    cells.insert("last_name".to_owned(), CrdtValue::Text(model.last_name.clone()));
    cells.insert("created_at".to_owned(), CrdtValue::Text(timestamp(&model.created_at)));
    cells.insert("updated_at".to_owned(), CrdtValue::Text(timestamp(&model.updated_at)));
    push_optional(&mut cells, KEY_WORKSPACE_IDENTIFIER, model.workspace_identifier.map(|id| id.to_string()));
    push_optional(&mut cells, "profile_picture", model.profile_picture.clone());
    cells
}

fn recycle_bin_cells(model: &recycle_bin::Model) -> BTreeMap<String, CrdtValue> {
    let mut cells = BTreeMap::new();
    cells.insert("item_id".to_owned(), CrdtValue::Text(model.item_id.to_string()));
    cells.insert("item_type".to_owned(), CrdtValue::Text(item_type_text(&model.item_type).to_owned()));
    cells.insert("payload".to_owned(), CrdtValue::Text(model.payload.clone()));
    cells.insert("deleted_at".to_owned(), CrdtValue::Text(timestamp(&model.deleted_at)));
    push_optional(&mut cells, KEY_WORKSPACE_IDENTIFIER, model.workspace_identifier.map(|id| id.to_string()));
    cells
}

// ---------------------------------------------------------------------------
// Write mirror: model mutations -> store
// ---------------------------------------------------------------------------

/// Writes a full set of cells onto a store row, inserting it first if the row
/// is not live yet. The first cell key doubles as a cheap liveness probe.
/// Optional fields whose value is `None` are skipped (the contract does not
/// carry nulls), so clearing an optional column does not propagate.
async fn sync_cells(store: &SqliteClient, identifier: &Uuid, cells: BTreeMap<String, CrdtValue>) {
    let Some(probe) = cells.keys().next().cloned() else {
        return;
    };
    let id = identifier.to_string();

    let live = match store.get_cell(id.clone(), probe).await {
        Ok(Some(_)) => true,
        Ok(None) => false,
        Err(error) => {
            log::error!("[mirror] probe {id}: {error}");
            return;
        }
    };

    let result = if live {
        for (column, value) in &cells {
            if let Err(error) = store.set(id.clone(), column.clone(), value.clone()).await {
                log::error!("[mirror] set {column} on {id}: {error}");
                return;
            }
        }
        Ok(())
    } else {
        store.insert(id.clone(), cells).await
    };

    if let Err(error) = result {
        log::error!("[mirror] write {id}: {error}");
    }
}

pub async fn mirror_todo(sync: &SyncManager, model: &todo::Model) {
    let Some(store) = sync.client(TABLE_TODOS) else { return; };
    sync_cells(store, &model.identifier, todo_cells(model)).await;
}

pub async fn mirror_notes(sync: &SyncManager, model: &notes::Model) {
    let Some(store) = sync.client(TABLE_NOTES) else { return; };
    sync_cells(store, &model.identifier, notes_cells(model)).await;
}

pub async fn mirror_bookmark(sync: &SyncManager, model: &bookmark::Model) {
    let Some(store) = sync.client(TABLE_BOOKMARKS) else { return; };
    sync_cells(store, &model.identifier, bookmark_cells(model)).await;
}

pub async fn mirror_reminder(sync: &SyncManager, model: &reminder::Model) {
    let Some(store) = sync.client(TABLE_REMINDERS) else { return; };
    sync_cells(store, &model.identifier, reminder_cells(model)).await;
}

pub async fn mirror_snippet(sync: &SyncManager, model: &snippets::Model) {
    let Some(store) = sync.client(TABLE_SNIPPETS) else { return; };
    sync_cells(store, &model.identifier, snippet_cells(model)).await;
}

pub async fn mirror_workspace(sync: &SyncManager, model: &workspaces::Model) {
    let Some(store) = sync.client(TABLE_WORKSPACES) else { return; };
    sync_cells(store, &model.identifier, workspace_cells(model)).await;
}

pub async fn mirror_workspace_profile(sync: &SyncManager, model: &workspace_profiles::Model) {
    let Some(store) = sync.client(TABLE_WORKSPACE_PROFILES) else { return; };
    sync_cells(store, &model.identifier, workspace_profile_cells(model)).await;
}

pub async fn mirror_recycle_bin(sync: &SyncManager, model: &recycle_bin::Model) {
    let Some(store) = sync.client(TABLE_RECYCLE_BIN) else { return; };
    sync_cells(store, &model.identifier, recycle_bin_cells(model)).await;
}

/// Tombstones a store row (entity deleted or bin entry purged/restored away).
pub async fn tombstone(sync: &SyncManager, table: &str, identifier: &Uuid) {
    let Some(store) = sync.client(table) else { return; };
    if let Err(error) = store.delete(identifier.to_string()).await {
        log::warn!("[mirror] tombstone {identifier} in {table}: {error}");
    }
}

/// Un-tombstones a store row (entity restored from the recycled bin).
pub async fn restore_row(sync: &SyncManager, table: &str, identifier: &Uuid) {
    let Some(store) = sync.client(table) else { return; };
    if let Err(error) = store.restore(identifier.to_string()).await {
        log::warn!("[mirror] restore {identifier} in {table}: {error}");
    }
}

/// Moves a store row to another workspace.
pub async fn transfer(sync: &SyncManager, table: &str, identifier: &Uuid, target: &Uuid) {
    let Some(store) = sync.client(table) else { return; };
    if let Err(error) = store
        .set(
            identifier.to_string(),
            KEY_WORKSPACE_IDENTIFIER.to_owned(),
            CrdtValue::Text(target.to_string()),
        )
        .await
    {
        log::warn!("[mirror] transfer {identifier} in {table}: {error}");
    }
}

// ---------------------------------------------------------------------------
// Receive mirror: store changes (from sync) -> sea-orm tables
// ---------------------------------------------------------------------------

fn text_of(columns: &BTreeMap<String, CrdtColumn>, key: &str) -> Option<String> {
    match columns.get(key).map(|column| &column.value) {
        Some(CrdtValue::Text(value)) => Some(value.clone()),
        _ => None,
    }
}

fn boolean_of(columns: &BTreeMap<String, CrdtColumn>, key: &str) -> Option<bool> {
    match columns.get(key).map(|column| &column.value) {
        Some(CrdtValue::Boolean(value)) => Some(*value),
        _ => None,
    }
}

fn is_tombstone(columns: &BTreeMap<String, CrdtColumn>) -> bool {
    boolean_of(columns, "deleted") == Some(true)
}

fn parse_uuid(text: &str) -> Result<Uuid, String> {
    Uuid::parse_str(text).map_err(|error| format!("invalid identifier {text}: {error}"))
}

fn parse_timestamp(text: &str) -> Option<DateTimeWithTimeZone> {
    chrono::DateTime::parse_from_rfc3339(text).ok()
}

fn now() -> DateTimeWithTimeZone {
    chrono::Utc::now().fixed_offset()
}

/// Applies the changed cells of one row onto the stored model (or a fresh
/// default when the row is new). Returns `None` for tombstones (row must be
/// removed) and `Some(model)` for live rows.
async fn apply_todo_row(conn: &DatabaseConnection, row: &RowChange) -> Result<Option<todo::Model>, String> {
    if is_tombstone(&row.columns) {
        return Ok(None);
    }
    let identifier = parse_uuid(&row.todo_id)?;
    let existing = todo::Entity::find_by_id(identifier).one(conn).await.map_err(|e| e.to_string())?;
    let mut model = existing.unwrap_or(todo::Model {
        identifier,
        title: String::new(),
        description: None,
        due_date: None,
        due_time: None,
        priority: Priority::Medium,
        done: false,
        created_at: now(),
        updated_at: now(),
        workspace_identifier: None,
    });

    if let Some(title) = text_of(&row.columns, "title") {
        model.title = title;
    }
    if let Some(description) = text_of(&row.columns, "description") {
        model.description = Some(description);
    }
    if let Some(priority) = text_of(&row.columns, "priority") {
        model.priority = match priority.as_str() {
            "high" => Priority::High,
            "low" => Priority::Low,
            _ => Priority::Medium,
        };
    }
    if let Some(done) = boolean_of(&row.columns, "completed") {
        model.done = done;
    }
    if let Some(day) = text_of(&row.columns, "due_date") {
        model.due_date = chrono::NaiveDate::parse_from_str(&day, "%Y-%m-%d").ok();
    }
    if let Some(time) = text_of(&row.columns, "due_time") {
        model.due_time = chrono::NaiveTime::parse_from_str(&time, "%H:%M:%S").ok();
    }
    if let Some(created_at) = text_of(&row.columns, "created_at").and_then(|v| parse_timestamp(&v)) {
        model.created_at = created_at;
    }
    if let Some(updated_at) = text_of(&row.columns, "updated_at").and_then(|v| parse_timestamp(&v)) {
        model.updated_at = updated_at;
    }
    if let Some(workspace) = text_of(&row.columns, KEY_WORKSPACE_IDENTIFIER) {
        model.workspace_identifier = parse_uuid(&workspace).ok();
    }
    Ok(Some(model))
}

async fn apply_notes_row(conn: &DatabaseConnection, row: &RowChange) -> Result<Option<notes::Model>, String> {
    if is_tombstone(&row.columns) {
        return Ok(None);
    }
    let identifier = parse_uuid(&row.todo_id)?;
    let existing = notes::Entity::find_by_id(identifier).one(conn).await.map_err(|e| e.to_string())?;
    let mut model = existing.unwrap_or(notes::Model {
        identifier,
        title: String::new(),
        content: String::new(),
        categories: None,
        created_at: now(),
        updated_at: now(),
        workspace_identifier: None,
    });

    if let Some(title) = text_of(&row.columns, "title") {
        model.title = title;
    }
    if let Some(content) = text_of(&row.columns, "content") {
        model.content = content;
    }
    if let Some(categories) = text_of(&row.columns, "categories") {
        model.categories = serde_json::from_str(&categories).ok();
    }
    if let Some(created_at) = text_of(&row.columns, "created_at").and_then(|v| parse_timestamp(&v)) {
        model.created_at = created_at;
    }
    if let Some(updated_at) = text_of(&row.columns, "updated_at").and_then(|v| parse_timestamp(&v)) {
        model.updated_at = updated_at;
    }
    if let Some(workspace) = text_of(&row.columns, KEY_WORKSPACE_IDENTIFIER) {
        model.workspace_identifier = parse_uuid(&workspace).ok();
    }
    Ok(Some(model))
}

async fn apply_bookmark_row(conn: &DatabaseConnection, row: &RowChange) -> Result<Option<bookmark::Model>, String> {
    if is_tombstone(&row.columns) {
        return Ok(None);
    }
    let identifier = parse_uuid(&row.todo_id)?;
    let existing = bookmark::Entity::find_by_id(identifier).one(conn).await.map_err(|e| e.to_string())?;
    let mut model = existing.unwrap_or(bookmark::Model {
        identifier,
        title: String::new(),
        url: String::new(),
        tag: Tag::Research,
        created_at: now(),
        updated_at: now(),
        workspace_identifier: None,
    });

    if let Some(title) = text_of(&row.columns, "title") {
        model.title = title;
    }
    if let Some(url) = text_of(&row.columns, "url") {
        model.url = url;
    }
    if let Some(tag) = text_of(&row.columns, "tag") {
        model.tag = match tag.as_str() {
            "development" => Tag::Development,
            "inspiration" => Tag::Inspiration,
            "design" => Tag::Design,
            _ => Tag::Research,
        };
    }
    if let Some(created_at) = text_of(&row.columns, "created_at").and_then(|v| parse_timestamp(&v)) {
        model.created_at = created_at;
    }
    if let Some(updated_at) = text_of(&row.columns, "updated_at").and_then(|v| parse_timestamp(&v)) {
        model.updated_at = updated_at;
    }
    if let Some(workspace) = text_of(&row.columns, KEY_WORKSPACE_IDENTIFIER) {
        model.workspace_identifier = parse_uuid(&workspace).ok();
    }
    Ok(Some(model))
}

async fn apply_reminder_row(conn: &DatabaseConnection, row: &RowChange) -> Result<Option<reminder::Model>, String> {
    if is_tombstone(&row.columns) {
        return Ok(None);
    }
    let identifier = parse_uuid(&row.todo_id)?;
    let existing = reminder::Entity::find_by_id(identifier).one(conn).await.map_err(|e| e.to_string())?;
    let mut model = existing.unwrap_or(reminder::Model {
        identifier,
        title: String::new(),
        description: None,
        recurring: false,
        recurrence_rule: None,
        alarm_sound: None,
        remind_at: now(),
        created_at: now(),
        updated_at: now(),
        workspace_identifier: None,
    });

    if let Some(title) = text_of(&row.columns, "title") {
        model.title = title;
    }
    if let Some(description) = text_of(&row.columns, "description") {
        model.description = Some(description);
    }
    if let Some(recurring) = boolean_of(&row.columns, "recurring") {
        model.recurring = recurring;
    }
    if let Some(rule) = text_of(&row.columns, "recurrence_rule") {
        model.recurrence_rule = Some(rule);
    }
    if let Some(alarm) = text_of(&row.columns, "alarm_sound") {
        model.alarm_sound = Some(alarm);
    }
    if let Some(remind_at) = text_of(&row.columns, "remind_at").and_then(|v| parse_timestamp(&v)) {
        model.remind_at = remind_at;
    }
    if let Some(created_at) = text_of(&row.columns, "created_at").and_then(|v| parse_timestamp(&v)) {
        model.created_at = created_at;
    }
    if let Some(updated_at) = text_of(&row.columns, "updated_at").and_then(|v| parse_timestamp(&v)) {
        model.updated_at = updated_at;
    }
    if let Some(workspace) = text_of(&row.columns, KEY_WORKSPACE_IDENTIFIER) {
        model.workspace_identifier = parse_uuid(&workspace).ok();
    }
    Ok(Some(model))
}

async fn apply_snippet_row(conn: &DatabaseConnection, row: &RowChange) -> Result<Option<snippets::Model>, String> {
    if is_tombstone(&row.columns) {
        return Ok(None);
    }
    let identifier = parse_uuid(&row.todo_id)?;
    let existing = snippets::Entity::find_by_id(identifier).one(conn).await.map_err(|e| e.to_string())?;
    let mut model = existing.unwrap_or(snippets::Model {
        identifier,
        title: None,
        language: None,
        code: String::new(),
        description: None,
        is_pinned: false,
        created_at: now(),
        updated_at: now(),
        workspace_identifier: None,
    });

    if let Some(title) = text_of(&row.columns, "title") {
        model.title = Some(title);
    }
    if let Some(language) = text_of(&row.columns, "language") {
        model.language = Some(language);
    }
    if let Some(code) = text_of(&row.columns, "code") {
        model.code = code;
    }
    if let Some(description) = text_of(&row.columns, "description") {
        model.description = Some(description);
    }
    if let Some(is_pinned) = boolean_of(&row.columns, "is_pinned") {
        model.is_pinned = is_pinned;
    }
    if let Some(created_at) = text_of(&row.columns, "created_at").and_then(|v| parse_timestamp(&v)) {
        model.created_at = created_at;
    }
    if let Some(updated_at) = text_of(&row.columns, "updated_at").and_then(|v| parse_timestamp(&v)) {
        model.updated_at = updated_at;
    }
    if let Some(workspace) = text_of(&row.columns, KEY_WORKSPACE_IDENTIFIER) {
        model.workspace_identifier = parse_uuid(&workspace).ok();
    }
    Ok(Some(model))
}

async fn apply_workspace_row(conn: &DatabaseConnection, row: &RowChange) -> Result<Option<workspaces::Model>, String> {
    if is_tombstone(&row.columns) {
        return Ok(None);
    }
    let identifier = parse_uuid(&row.todo_id)?;
    let existing = workspaces::Entity::find_by_id(identifier).one(conn).await.map_err(|e| e.to_string())?;
    let mut model = existing.unwrap_or(workspaces::Model {
        identifier,
        name: String::new(),
        description: String::new(),
        created_at: now(),
        updated_at: now(),
        is_default: false,
        is_hidden: false,
        is_secured: false,
        password_hash: None,
        user_identifier: None,
    });

    if let Some(name) = text_of(&row.columns, "name") {
        model.name = name;
    }
    if let Some(description) = text_of(&row.columns, "description") {
        model.description = description;
    }
    if let Some(is_default) = boolean_of(&row.columns, "is_default") {
        model.is_default = is_default;
    }
    if let Some(is_hidden) = boolean_of(&row.columns, "is_hidden") {
        model.is_hidden = is_hidden;
    }
    if let Some(is_secured) = boolean_of(&row.columns, "is_secured") {
        model.is_secured = is_secured;
    }
    if let Some(hash) = text_of(&row.columns, "password_hash") {
        model.password_hash = Some(hash);
    }
    if let Some(user) = text_of(&row.columns, "user_identifier") {
        model.user_identifier = parse_uuid(&user).ok();
    }
    if let Some(created_at) = text_of(&row.columns, "created_at").and_then(|v| parse_timestamp(&v)) {
        model.created_at = created_at;
    }
    if let Some(updated_at) = text_of(&row.columns, "updated_at").and_then(|v| parse_timestamp(&v)) {
        model.updated_at = updated_at;
    }
    Ok(Some(model))
}

async fn apply_workspace_profile_row(
    conn: &DatabaseConnection,
    row: &RowChange,
) -> Result<Option<workspace_profiles::Model>, String> {
    if is_tombstone(&row.columns) {
        return Ok(None);
    }
    let identifier = parse_uuid(&row.todo_id)?;
    let existing = workspace_profiles::Entity::find_by_id(identifier).one(conn).await.map_err(|e| e.to_string())?;
    let mut model = existing.unwrap_or(workspace_profiles::Model {
        identifier,
        first_name: String::new(),
        last_name: String::new(),
        created_at: now(),
        updated_at: now(),
        workspace_identifier: None,
        profile_picture: None,
    });

    if let Some(first_name) = text_of(&row.columns, "first_name") {
        model.first_name = first_name;
    }
    if let Some(last_name) = text_of(&row.columns, "last_name") {
        model.last_name = last_name;
    }
    if let Some(profile_picture) = text_of(&row.columns, "profile_picture") {
        model.profile_picture = Some(profile_picture);
    }
    if let Some(created_at) = text_of(&row.columns, "created_at").and_then(|v| parse_timestamp(&v)) {
        model.created_at = created_at;
    }
    if let Some(updated_at) = text_of(&row.columns, "updated_at").and_then(|v| parse_timestamp(&v)) {
        model.updated_at = updated_at;
    }
    if let Some(workspace) = text_of(&row.columns, KEY_WORKSPACE_IDENTIFIER) {
        model.workspace_identifier = parse_uuid(&workspace).ok();
    }
    Ok(Some(model))
}

async fn apply_recycle_bin_row(
    conn: &DatabaseConnection,
    row: &RowChange,
) -> Result<Option<recycle_bin::Model>, String> {
    if is_tombstone(&row.columns) {
        return Ok(None);
    }
    let identifier = parse_uuid(&row.todo_id)?;
    let existing = recycle_bin::Entity::find_by_id(identifier).one(conn).await.map_err(|e| e.to_string())?;
    let mut model = existing.unwrap_or(recycle_bin::Model {
        identifier,
        item_id: Uuid::nil(),
        item_type: ItemType::Todo,
        payload: String::new(),
        deleted_at: now(),
        workspace_identifier: None,
    });

    if let Some(item_id) = text_of(&row.columns, "item_id") {
        if let Ok(item_id) = parse_uuid(&item_id) {
            model.item_id = item_id;
        }
    }
    if let Some(item_type) = text_of(&row.columns, "item_type") {
        model.item_type = match item_type.as_str() {
            "note" => ItemType::Note,
            "reminder" => ItemType::Reminder,
            "snippet" => ItemType::Snippet,
            "bookmark" => ItemType::Bookmark,
            _ => ItemType::Todo,
        };
    }
    if let Some(payload) = text_of(&row.columns, "payload") {
        model.payload = payload;
    }
    if let Some(deleted_at) = text_of(&row.columns, "deleted_at").and_then(|v| parse_timestamp(&v)) {
        model.deleted_at = deleted_at;
    }
    if let Some(workspace) = text_of(&row.columns, KEY_WORKSPACE_IDENTIFIER) {
        model.workspace_identifier = parse_uuid(&workspace).ok();
    }
    Ok(Some(model))
}

/// Executes the tombstones for a table (deletes the matching rows).
async fn execute_tombstones<E>(
    conn: &DatabaseConnection,
    tombstones: Vec<String>,
    build_delete: impl FnOnce(Vec<Uuid>) -> lunar::sea_orm::DeleteMany<E>,
) -> Result<(), String>
where
    E: EntityTrait,
{
    let ids: Vec<Uuid> = tombstones.iter().filter_map(|id| Uuid::parse_str(id).ok()).collect();
    if ids.is_empty() {
        return Ok(());
    }
    build_delete(ids).exec(conn).await.map(|_| ()).map_err(|error| error.to_string())
}

async fn save_todos(conn: &DatabaseConnection, live: Vec<todo::Model>, tombstones: Vec<String>) -> Result<(), String> {
    if !live.is_empty() {
        let repo = lunar::repositories::todo::TodoRepository::new(Arc::new(conn.clone()));
        let results = repo.upsert_many(live).await.map_err(|e| e.to_string())?;
        for result in results {
            if !result.success {
                log::error!("[sync] upsert todo {}: {:?}", result.identifier, result.error_message);
            }
        }
    }
    execute_tombstones(conn, tombstones, |ids| {
        todo::Entity::delete_many().filter(todo::Column::Identifier.is_in(ids))
    })
    .await
}

async fn save_notes(conn: &DatabaseConnection, live: Vec<notes::Model>, tombstones: Vec<String>) -> Result<(), String> {
    if !live.is_empty() {
        let repo = lunar::repositories::notes::NotesRepository::new(Arc::new(conn.clone()));
        let results = repo.upsert_many(live).await.map_err(|e| e.to_string())?;
        for result in results {
            if !result.success {
                log::error!("[sync] upsert note {}: {:?}", result.identifier, result.error_message);
            }
        }
    }
    execute_tombstones(conn, tombstones, |ids| {
        notes::Entity::delete_many().filter(notes::Column::Identifier.is_in(ids))
    })
    .await
}

async fn save_bookmarks(conn: &DatabaseConnection, live: Vec<bookmark::Model>, tombstones: Vec<String>) -> Result<(), String> {
    if !live.is_empty() {
        let repo = lunar::repositories::bookmarks::BookmarkRepository::new(Arc::new(conn.clone()));
        let results = repo.upsert_many(live).await.map_err(|e| e.to_string())?;
        for result in results {
            if !result.success {
                log::error!("[sync] upsert bookmark {}: {:?}", result.identifier, result.error_message);
            }
        }
    }
    execute_tombstones(conn, tombstones, |ids| {
        bookmark::Entity::delete_many().filter(bookmark::Column::Identifier.is_in(ids))
    })
    .await
}

async fn save_reminders(conn: &DatabaseConnection, live: Vec<reminder::Model>, tombstones: Vec<String>) -> Result<(), String> {
    if !live.is_empty() {
        let repo = lunar::repositories::reminder::ReminderRepository::new(Arc::new(conn.clone()));
        let results = repo.upsert_many(live).await.map_err(|e| e.to_string())?;
        for result in results {
            if !result.success {
                log::error!("[sync] upsert reminder {}: {:?}", result.identifier, result.error_message);
            }
        }
    }
    execute_tombstones(conn, tombstones, |ids| {
        reminder::Entity::delete_many().filter(reminder::Column::Identifier.is_in(ids))
    })
    .await
}

async fn save_snippets(conn: &DatabaseConnection, live: Vec<snippets::Model>, tombstones: Vec<String>) -> Result<(), String> {
    if !live.is_empty() {
        let repo = lunar::repositories::snippets::SnippetRepository::new(Arc::new(conn.clone()));
        let results = repo.upsert_many(live).await.map_err(|e| e.to_string())?;
        for result in results {
            if !result.success {
                log::error!("[sync] upsert snippet {}: {:?}", result.identifier, result.error_message);
            }
        }
    }
    execute_tombstones(conn, tombstones, |ids| {
        snippets::Entity::delete_many().filter(snippets::Column::Identifier.is_in(ids))
    })
    .await
}

async fn save_workspaces(conn: &DatabaseConnection, live: Vec<workspaces::Model>, tombstones: Vec<String>) -> Result<(), String> {
    if !live.is_empty() {
        let repo = lunar::repositories::workspace::WorkspaceRepository::new(Arc::new(conn.clone()));
        let results = repo.upsert_many(live).await.map_err(|e| e.to_string())?;
        for result in results {
            if !result.success {
                log::error!("[sync] upsert workspace {}: {:?}", result.identifier, result.error_message);
            }
        }
    }
    execute_tombstones(conn, tombstones, |ids| {
        workspaces::Entity::delete_many().filter(workspaces::Column::Identifier.is_in(ids))
    })
    .await
}

async fn save_workspace_profiles(
    conn: &DatabaseConnection,
    live: Vec<workspace_profiles::Model>,
    tombstones: Vec<String>,
) -> Result<(), String> {
    if !live.is_empty() {
        let repo = lunar::repositories::workspace_profiles::WorkspaceProfileRepository::new(Arc::new(conn.clone()));
        let results = repo.upsert_many(live)
            .await
            .map_err(|e| e.to_string())?;
        for result in results {
            if !result.success {
                log::error!("[sync] upsert workspace profile {}: {:?}", result.identifier, result.error_message);
            }
        }
    }
    execute_tombstones(conn, tombstones, |ids| {
        workspace_profiles::Entity::delete_many().filter(workspace_profiles::Column::Identifier.is_in(ids))
    })
    .await
}

async fn save_recycle_bin(conn: &DatabaseConnection, live: Vec<recycle_bin::Model>, tombstones: Vec<String>) -> Result<(), String> {
    if !live.is_empty() {
        let repo = lunar::repositories::recycle_bin::RecycleBinRepository::new(Arc::new(conn.clone()));
        let results = repo.upsert_many(live).await.map_err(|e| e.to_string())?;
        for result in results {
            if !result.success {
                log::error!("[sync] upsert recycle-bin row {}: {:?}", result.identifier, result.error_message);
            }
        }
    }
    execute_tombstones(conn, tombstones, |ids| {
        recycle_bin::Entity::delete_many().filter(recycle_bin::Column::Identifier.is_in(ids))
    })
    .await
}

/// Reproduces one table's converged store state into the sea-orm tables.
async fn apply_table(conn: &DatabaseConnection, table: &str, rows: &[RowChange]) -> Result<(), String> {
    match table {
        TABLE_TODOS => {
            let mut live = Vec::new();
            let mut tombstones = Vec::new();
            for row in rows {
                match apply_todo_row(conn, row).await {
                    Ok(Some(model)) => live.push(model),
                    Ok(None) => tombstones.push(row.todo_id.clone()),
                    Err(error) => log::debug!("[sync] skip row {}: {error}", row.todo_id),
                }
            }
            save_todos(conn, live, tombstones).await
        }
        TABLE_NOTES => {
            let mut live = Vec::new();
            let mut tombstones = Vec::new();
            for row in rows {
                match apply_notes_row(conn, row).await {
                    Ok(Some(model)) => live.push(model),
                    Ok(None) => tombstones.push(row.todo_id.clone()),
                    Err(error) => log::debug!("[sync] skip row {}: {error}", row.todo_id),
                }
            }
            save_notes(conn, live, tombstones).await
        }
        TABLE_BOOKMARKS => {
            let mut live = Vec::new();
            let mut tombstones = Vec::new();
            for row in rows {
                match apply_bookmark_row(conn, row).await {
                    Ok(Some(model)) => live.push(model),
                    Ok(None) => tombstones.push(row.todo_id.clone()),
                    Err(error) => log::debug!("[sync] skip row {}: {error}", row.todo_id),
                }
            }
            save_bookmarks(conn, live, tombstones).await
        }
        TABLE_REMINDERS => {
            let mut live = Vec::new();
            let mut tombstones = Vec::new();
            for row in rows {
                match apply_reminder_row(conn, row).await {
                    Ok(Some(model)) => live.push(model),
                    Ok(None) => tombstones.push(row.todo_id.clone()),
                    Err(error) => log::debug!("[sync] skip row {}: {error}", row.todo_id),
                }
            }
            save_reminders(conn, live, tombstones).await
        }
        TABLE_SNIPPETS => {
            let mut live = Vec::new();
            let mut tombstones = Vec::new();
            for row in rows {
                match apply_snippet_row(conn, row).await {
                    Ok(Some(model)) => live.push(model),
                    Ok(None) => tombstones.push(row.todo_id.clone()),
                    Err(error) => log::debug!("[sync] skip row {}: {error}", row.todo_id),
                }
            }
            save_snippets(conn, live, tombstones).await
        }
        TABLE_WORKSPACES => {
            let mut live = Vec::new();
            let mut tombstones = Vec::new();
            for row in rows {
                match apply_workspace_row(conn, row).await {
                    Ok(Some(model)) => live.push(model),
                    Ok(None) => tombstones.push(row.todo_id.clone()),
                    Err(error) => log::debug!("[sync] skip row {}: {error}", row.todo_id),
                }
            }
            save_workspaces(conn, live, tombstones).await
        }
        TABLE_WORKSPACE_PROFILES => {
            let mut live = Vec::new();
            let mut tombstones = Vec::new();
            for row in rows {
                match apply_workspace_profile_row(conn, row).await {
                    Ok(Some(model)) => live.push(model),
                    Ok(None) => tombstones.push(row.todo_id.clone()),
                    Err(error) => log::debug!("[sync] skip row {}: {error}", row.todo_id),
                }
            }
            save_workspace_profiles(conn, live, tombstones).await
        }
        TABLE_RECYCLE_BIN => {
            let mut live = Vec::new();
            let mut tombstones = Vec::new();
            for row in rows {
                match apply_recycle_bin_row(conn, row).await {
                    Ok(Some(model)) => live.push(model),
                    Ok(None) => tombstones.push(row.todo_id.clone()),
                    Err(error) => log::debug!("[sync] skip row {}: {error}", row.todo_id),
                }
            }
            save_recycle_bin(conn, live, tombstones).await
        }
        other => {
            log::debug!("[sync] no sea-orm sink for table {other}");
            Ok(())
        }
    }
}

/// Writes every accumulated remote change into the sea-orm tables. Tables are
/// applied in dependency order (workspaces before entities that reference
/// them); a failed table does not stop the rest.
pub async fn apply_all(conn: &DatabaseConnection, by_table: &BTreeMap<String, Vec<RowChange>>) {
    const ORDER: [&str; 8] = [
        TABLE_WORKSPACES,
        TABLE_WORKSPACE_PROFILES,
        TABLE_TODOS,
        TABLE_NOTES,
        TABLE_BOOKMARKS,
        TABLE_REMINDERS,
        TABLE_SNIPPETS,
        TABLE_RECYCLE_BIN,
    ];
    for table in ORDER {
        let Some(rows) = by_table.get(table) else { continue; };
        if rows.is_empty() {
            continue;
        }
        if let Err(error) = apply_table(conn, table, rows).await {
            log::error!("[sync] apply {table}: {error}");
        }
    }
}

// ---------------------------------------------------------------------------
// One-time seed of pre-existing data into the store (backfill)
// ---------------------------------------------------------------------------

/// Seeds every syncable store from the legacy tables on the first run after an
/// upgrade. Runs only when a marker file indicates it has not completed, and
/// only while the store has no synced history yet.
/// Mirrors every legacy row of every sync table into its store, regardless of
/// whether the store already has rows. Used to bootstrap a fresh store.
pub async fn backfill_all(
    sync_manager: &SyncManager,
    conn: &DatabaseConnection,
) -> Result<usize, crate::errors::AppError> {
    backfill_tables(sync_manager, conn, false).await
}

/// Mirrors legacy rows into any store table that is still empty. Runs on every
/// launch so a swapped-in or freshly copied database self-heals: existing
/// legacy rows are seeded whenever their store partition is empty, no matter
/// what happened on previous launches. Tables whose store already has rows are
/// left untouched.
pub async fn backfill_missing(
    sync_manager: &SyncManager,
    conn: &DatabaseConnection,
) -> Result<usize, crate::errors::AppError> {
    backfill_tables(sync_manager, conn, true).await
}

async fn backfill_tables(
    sync_manager: &SyncManager,
    conn: &DatabaseConnection,
    only_empty: bool,
) -> Result<usize, crate::errors::AppError> {
    use lunar::sea_orm::EntityTrait;

    let mut total = 0usize;

    if !only_empty || store_needs_seeding(sync_manager, TABLE_WORKSPACES).await {
        let rows = workspaces::Entity::find().all(conn).await.map_err(db_error)?;
        for model in &rows {
            mirror_workspace(sync_manager, model).await;
        }
        total += rows.len();
    }

    if !only_empty || store_needs_seeding(sync_manager, TABLE_WORKSPACE_PROFILES).await {
        let rows = workspace_profiles::Entity::find().all(conn).await.map_err(db_error)?;
        for model in &rows {
            mirror_workspace_profile(sync_manager, model).await;
        }
        total += rows.len();
    }

    if !only_empty || store_needs_seeding(sync_manager, TABLE_TODOS).await {
        let rows = todo::Entity::find().all(conn).await.map_err(db_error)?;
        for model in &rows {
            mirror_todo(sync_manager, model).await;
        }
        total += rows.len();
    }

    if !only_empty || store_needs_seeding(sync_manager, TABLE_NOTES).await {
        let rows = notes::Entity::find().all(conn).await.map_err(db_error)?;
        for model in &rows {
            mirror_notes(sync_manager, model).await;
        }
        total += rows.len();
    }

    if !only_empty || store_needs_seeding(sync_manager, TABLE_BOOKMARKS).await {
        let rows = bookmark::Entity::find().all(conn).await.map_err(db_error)?;
        for model in &rows {
            mirror_bookmark(sync_manager, model).await;
        }
        total += rows.len();
    }

    if !only_empty || store_needs_seeding(sync_manager, TABLE_REMINDERS).await {
        let rows = reminder::Entity::find().all(conn).await.map_err(db_error)?;
        for model in &rows {
            mirror_reminder(sync_manager, model).await;
        }
        total += rows.len();
    }

    if !only_empty || store_needs_seeding(sync_manager, TABLE_SNIPPETS).await {
        let rows = snippets::Entity::find().all(conn).await.map_err(db_error)?;
        for model in &rows {
            mirror_snippet(sync_manager, model).await;
        }
        total += rows.len();
    }

    if !only_empty || store_needs_seeding(sync_manager, TABLE_RECYCLE_BIN).await {
        let rows = recycle_bin::Entity::find().all(conn).await.map_err(db_error)?;
        for model in &rows {
            mirror_recycle_bin(sync_manager, model).await;
        }
        total += rows.len();
    }

    Ok(total)
}

/// Whether the given store table still needs to be seeded with legacy rows.
/// Store failures are best-effort like every mirror: logged, and the table is
/// skipped so an existing-but-unreadable store is never clobbered.
async fn store_needs_seeding(sync_manager: &SyncManager, store: &str) -> bool {
    match sync_manager.client(store) {
        Some(client) => match client.is_empty().await {
            Ok(empty) => empty,
            Err(error) => {
                log::error!("[backfill] emptiness check for {store}: {error}");
                false
            }
        },
        None => false,
    }
}