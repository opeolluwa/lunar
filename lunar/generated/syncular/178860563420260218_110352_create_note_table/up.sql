-- ============================================
-- 178860563420260218_110352_create_note_table
-- ============================================

CREATE TABLE IF NOT EXISTS "notes" ( "identifier" uuid_text NOT NULL PRIMARY KEY, "title" varchar NOT NULL, "content" text NOT NULL, "categories" json_text NOT NULL, "created_at" timestamp_with_timezone_text NOT NULL, "updated_at" timestamp_with_timezone_text NOT NULL );

