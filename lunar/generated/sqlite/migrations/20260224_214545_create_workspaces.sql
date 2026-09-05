-- ============================================
-- 20260224_214545_create_workspaces
-- ============================================

CREATE TABLE IF NOT EXISTS "workspaces" ( "name" varchar NOT NULL, "description" varchar NOT NULL, "created_at" timestamp_with_timezone_text NOT NULL, "updated_at" timestamp_with_timezone_text NOT NULL, "identifier" uuid_text NOT NULL PRIMARY KEY );

