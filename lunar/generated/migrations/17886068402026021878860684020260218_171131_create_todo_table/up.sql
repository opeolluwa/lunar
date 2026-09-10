-- ============================================
-- 17886068402026021878860684020260218_171131_create_todo_table
-- ============================================

CREATE TABLE IF NOT EXISTS "todo" ( "identifier" uuid_text NOT NULL PRIMARY KEY, "title" varchar NOT NULL, "description" text NULL, "due_date" date_text NULL, "priority" enum_text NOT NULL DEFAULT 'medium', "done" boolean NOT NULL DEFAULT FALSE, "created_at" timestamp_with_timezone_text NOT NULL, "updated_at" timestamp_with_timezone_text NOT NULL );

