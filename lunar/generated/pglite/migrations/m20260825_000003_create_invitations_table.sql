-- ============================================
-- m20260825_000003_create_invitations_table
-- ============================================

CREATE TABLE IF NOT EXISTS "invitation" ( "identifier" uuid NOT NULL PRIMARY KEY, "workspace_identifier" uuid NOT NULL, "email" varchar NOT NULL, "first_name" varchar NULL, "last_name" varchar NULL, "token" varchar NOT NULL UNIQUE, "status" varchar NOT NULL DEFAULT 'pending', "expires_at" timestamp with time zone NOT NULL, "created_at" timestamp with time zone NOT NULL );

ALTER TABLE "invitation" ADD CONSTRAINT "fk_invitations_workspace_identifier" FOREIGN KEY ("workspace_identifier") REFERENCES "workspaces" ("identifier") ON DELETE CASCADE;

