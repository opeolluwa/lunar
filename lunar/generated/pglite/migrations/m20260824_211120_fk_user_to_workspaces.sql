-- ============================================
-- m20260824_211120_fk_user_to_workspaces
-- ============================================

ALTER TABLE "workspaces" ADD CONSTRAINT "fk_user_workspace_identifier" FOREIGN KEY ("user_identifier") REFERENCES "users" ("identifier") ON DELETE CASCADE;

