-- ============================================
-- 17886068432026082678860684320260826_000000_add_user_identifier_to_workspaces
-- ============================================


                PRAGMA foreign_keys = OFF;

                CREATE TABLE workspaces_new (
                    identifier BLOB PRIMARY KEY NOT NULL,
                    name TEXT NOT NULL,
                    description TEXT NOT NULL,
                    is_default BOOLEAN NOT NULL DEFAULT FALSE,
                    is_hidden BOOLEAN NOT NULL DEFAULT FALSE,
                    is_secured BOOLEAN NOT NULL DEFAULT FALSE,
                    password_hash TEXT,
                    created_at TEXT NOT NULL,
                    updated_at TEXT NOT NULL,
                    user_identifier BLOB
                );

                INSERT INTO workspaces_new (identifier, name, description, is_default, is_hidden, is_secured, password_hash, created_at, updated_at, user_identifier)
                SELECT identifier, name, description, is_default, is_hidden, is_secured, password_hash, created_at, updated_at, NULL FROM workspaces;

                DROP TABLE workspaces;

                ALTER TABLE workspaces_new RENAME TO workspaces;

                PRAGMA foreign_keys = ON;

                DROP TRIGGER IF EXISTS workspaces_sync_insert;
                DROP TRIGGER IF EXISTS workspaces_sync_update;
                DROP TRIGGER IF EXISTS workspaces_sync_delete;

                CREATE TRIGGER IF NOT EXISTS workspaces_sync_insert
                AFTER INSERT ON workspaces
                BEGIN
                INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
                VALUES (randomblob(16), 'workspaces', lower(hex(NEW.identifier)), 'INSERT', datetime('now'));
                END;

                CREATE TRIGGER IF NOT EXISTS workspaces_sync_update
                AFTER UPDATE ON workspaces
                BEGIN
                INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
                VALUES (randomblob(16), 'workspaces', lower(hex(NEW.identifier)), 'UPDATE', datetime('now'));
                END;

                CREATE TRIGGER IF NOT EXISTS workspaces_sync_delete
                AFTER DELETE ON workspaces
                BEGIN
                INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
                VALUES (randomblob(16), 'workspaces', lower(hex(OLD.identifier)), 'DELETE', datetime('now'));
                END;
                ;

