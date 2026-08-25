-- ============================================
-- m20260824_210059_link_user_to_workspaces
-- ============================================

ALTER TABLE "workspaces" ADD COLUMN "user_identifier" uuid;

