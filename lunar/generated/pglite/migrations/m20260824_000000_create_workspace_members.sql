-- ============================================
-- m20260824_000000_create_workspace_members
-- ============================================

CREATE TABLE IF NOT EXISTS "workspace_members" ( "identifier" uuid NOT NULL PRIMARY KEY, "member_email" varchar NOT NULL, "role" varchar NOT NULL, "user_identifier" uuid NULL, "created_at" timestamp with time zone NOT NULL, "updated_at" timestamp with time zone NOT NULL, "workspace_identifier" uuid NOT NULL, CONSTRAINT "fk_workspace_members_workspace_identifier" FOREIGN KEY ("workspace_identifier") REFERENCES "workspaces" ("identifier") ON DELETE CASCADE );

CREATE UNIQUE INDEX "idx_workspace_members_workspace_email" ON "workspace_members" ("workspace_identifier", "member_email");

