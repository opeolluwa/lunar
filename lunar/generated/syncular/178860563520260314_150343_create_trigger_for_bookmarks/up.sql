-- ============================================
-- 178860563520260314_150343_create_trigger_for_bookmarks
-- ============================================



-- BOOKMARK
CREATE TRIGGER bookmark_sync_insert
AFTER INSERT ON bookmark
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (lower(hex(randomblob(16))), 'bookmark', NEW.identifier, 'INSERT', datetime('now'));
END;

CREATE TRIGGER bookmark_sync_update
AFTER UPDATE ON bookmark
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (lower(hex(randomblob(16))), 'bookmark', NEW.identifier, 'UPDATE', datetime('now'));
END;

CREATE TRIGGER bookmark_sync_delete
AFTER DELETE ON bookmark
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (lower(hex(randomblob(16))), 'bookmark', OLD.identifier, 'DELETE', datetime('now'));
END;


-- NOTES
CREATE TRIGGER notes_sync_insert
AFTER INSERT ON notes
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (lower(hex(randomblob(16))), 'notes', NEW.identifier, 'INSERT', datetime('now'));
END;

CREATE TRIGGER notes_sync_update
AFTER UPDATE ON notes
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (lower(hex(randomblob(16))), 'notes', NEW.identifier, 'UPDATE', datetime('now'));
END;

CREATE TRIGGER notes_sync_delete
AFTER DELETE ON notes
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (lower(hex(randomblob(16))), 'notes', OLD.identifier, 'DELETE', datetime('now'));
END;


-- RECYCLE_BIN
CREATE TRIGGER recycle_bin_sync_insert
AFTER INSERT ON recycle_bin
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (lower(hex(randomblob(16))), 'recycle_bin', NEW.identifier, 'INSERT', datetime('now'));
END;

CREATE TRIGGER recycle_bin_sync_update
AFTER UPDATE ON recycle_bin
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (lower(hex(randomblob(16))), 'recycle_bin', NEW.identifier, 'UPDATE', datetime('now'));
END;

CREATE TRIGGER recycle_bin_sync_delete
AFTER DELETE ON recycle_bin
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (lower(hex(randomblob(16))), 'recycle_bin', OLD.identifier, 'DELETE', datetime('now'));
END;


-- REMINDER
CREATE TRIGGER reminder_sync_insert
AFTER INSERT ON reminder
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (lower(hex(randomblob(16))), 'reminder', NEW.identifier, 'INSERT', datetime('now'));
END;

CREATE TRIGGER reminder_sync_update
AFTER UPDATE ON reminder
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (lower(hex(randomblob(16))), 'reminder', NEW.identifier, 'UPDATE', datetime('now'));
END;

CREATE TRIGGER reminder_sync_delete
AFTER DELETE ON reminder
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (lower(hex(randomblob(16))), 'reminder', OLD.identifier, 'DELETE', datetime('now'));
END;


-- SNIPPETS
CREATE TRIGGER snippets_sync_insert
AFTER INSERT ON snippets
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (lower(hex(randomblob(16))), 'snippets', NEW.identifier, 'INSERT', datetime('now'));
END;

CREATE TRIGGER snippets_sync_update
AFTER UPDATE ON snippets
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (lower(hex(randomblob(16))), 'snippets', NEW.identifier, 'UPDATE', datetime('now'));
END;

CREATE TRIGGER snippets_sync_delete
AFTER DELETE ON snippets
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (lower(hex(randomblob(16))), 'snippets', OLD.identifier, 'DELETE', datetime('now'));
END;


-- TODO
CREATE TRIGGER todo_sync_insert
AFTER INSERT ON todo
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (lower(hex(randomblob(16))), 'todo', NEW.identifier, 'INSERT', datetime('now'));
END;

CREATE TRIGGER todo_sync_update
AFTER UPDATE ON todo
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (lower(hex(randomblob(16))), 'todo', NEW.identifier, 'UPDATE', datetime('now'));
END;

CREATE TRIGGER todo_sync_delete
AFTER DELETE ON todo
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (lower(hex(randomblob(16))), 'todo', OLD.identifier, 'DELETE', datetime('now'));
END;


-- WORKSPACES
CREATE TRIGGER workspaces_sync_insert
AFTER INSERT ON workspaces
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (lower(hex(randomblob(16))), 'workspaces', NEW.identifier, 'INSERT', datetime('now'));
END;

CREATE TRIGGER workspaces_sync_update
AFTER UPDATE ON workspaces
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (lower(hex(randomblob(16))), 'workspaces', NEW.identifier, 'UPDATE', datetime('now'));
END;

CREATE TRIGGER workspaces_sync_delete
AFTER DELETE ON workspaces
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (lower(hex(randomblob(16))), 'workspaces', OLD.identifier, 'DELETE', datetime('now'));
END;


-- USER_PREFERENCE
CREATE TRIGGER user_preference_sync_insert
AFTER INSERT ON user_preference
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (lower(hex(randomblob(16))), 'user_preference', NEW.identifier, 'INSERT', datetime('now'));
END;

CREATE TRIGGER user_preference_sync_update
AFTER UPDATE ON user_preference
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (lower(hex(randomblob(16))), 'user_preference', NEW.identifier, 'UPDATE', datetime('now'));
END;

CREATE TRIGGER user_preference_sync_delete
AFTER DELETE ON user_preference
BEGIN
INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
VALUES (lower(hex(randomblob(16))), 'user_preference', OLD.identifier, 'DELETE', datetime('now'));
END;

;

