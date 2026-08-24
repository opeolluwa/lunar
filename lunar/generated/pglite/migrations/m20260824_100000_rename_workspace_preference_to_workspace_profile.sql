-- ============================================
-- m20260824_100000_rename_workspace_preference_to_workspace_profile
-- ============================================

ALTER TABLE "workspace_preferences" RENAME TO "workspace_profiles";

ALTER TABLE "workspace_profiles" ADD COLUMN IF NOT EXISTS "profile_picture" varchar NULL;

