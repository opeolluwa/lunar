-- ============================================
-- 17886068402026021778860684020260217_143820_create_snippet_table
-- ============================================

CREATE TABLE IF NOT EXISTS "snippets" ( "identifier" uuid_text NOT NULL PRIMARY KEY, "title" varchar NULL, "language" varchar NULL, "code" text NOT NULL, "description" text NULL, "is_pinned" boolean NOT NULL DEFAULT FALSE, "created_at" timestamp_with_timezone_text NOT NULL, "updated_at" timestamp_with_timezone_text NOT NULL );

