-- ============================================
-- 17886068432026090178860684320260901_000000_rebuild_todo_with_due_time
-- ============================================


                PRAGMA foreign_keys = OFF;

                CREATE TABLE IF NOT EXISTS todo_new (
                    identifier uuid_text NOT NULL PRIMARY KEY,
                    title varchar NOT NULL,
                    description text NULL,
                    due_date date_text NULL,
                    priority enum_text NOT NULL DEFAULT 'medium',
                    done boolean NOT NULL DEFAULT FALSE,
                    created_at timestamp_with_timezone_text NOT NULL,
                    updated_at timestamp_with_timezone_text NOT NULL,
                    due_time time_text NULL,
                    workspace_identifier uuid_text NULL
                );

                INSERT INTO todo_new (identifier, title, description, due_date, priority, done, created_at, updated_at, due_time, workspace_identifier)
                SELECT identifier, title, description, due_date, priority, done, created_at, updated_at, due_time, workspace_identifier FROM todo;

                DROP TABLE todo;

                ALTER TABLE todo_new RENAME TO todo;

                PRAGMA foreign_keys = ON;

                DROP TRIGGER IF EXISTS todo_sync_insert;
                DROP TRIGGER IF EXISTS todo_sync_update;
                DROP TRIGGER IF EXISTS todo_sync_delete;

                CREATE TRIGGER IF NOT EXISTS todo_sync_insert
                AFTER INSERT ON todo
                BEGIN
                INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
                VALUES (randomblob(16), 'todo', lower(hex(NEW.identifier)), 'INSERT', datetime('now'));
                END;

                CREATE TRIGGER IF NOT EXISTS todo_sync_update
                AFTER UPDATE ON todo
                BEGIN
                INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
                VALUES (randomblob(16), 'todo', lower(hex(NEW.identifier)), 'UPDATE', datetime('now'));
                END;

                CREATE TRIGGER IF NOT EXISTS todo_sync_delete
                AFTER DELETE ON todo
                BEGIN
                INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, created_at)
                VALUES (randomblob(16), 'todo', lower(hex(OLD.identifier)), 'DELETE', datetime('now'));
                END;
                ;

