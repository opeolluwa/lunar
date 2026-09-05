-- ============================================
-- 20260824_000000_create_workspace_members
-- ============================================

CREATE TABLE IF NOT EXISTS "workspace_members" ( "identifier" uuid_text NOT NULL PRIMARY KEY, "member_email" varchar NOT NULL, "role" varchar NOT NULL, "user_identifier" uuid_text NULL, "created_at" timestamp_with_timezone_text NOT NULL, "updated_at" timestamp_with_timezone_text NOT NULL, "workspace_identifier" uuid_text NOT NULL, FOREIGN KEY ("workspace_identifier") REFERENCES "workspaces" ("identifier") ON DELETE CASCADE );

CREATE UNIQUE INDEX "idx_workspace_members_workspace_email" ON "workspace_members" ("workspace_identifier", "member_email");

