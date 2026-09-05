-- ============================================
-- 20260218_204212_create_bookmarks_table
-- ============================================

CREATE TABLE IF NOT EXISTS "bookmark" ( "identifier" uuid_text NOT NULL PRIMARY KEY, "title" varchar NOT NULL, "url" varchar NOT NULL, "tag" enum_text NOT NULL DEFAULT 'development', "created_at" timestamp_with_timezone_text NOT NULL, "updated_at" timestamp_with_timezone_text NOT NULL );

