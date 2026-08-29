-- ============================================
-- m20260826_000000_add_user_identifier_to_workspaces
-- ============================================


                ALTER TABLE "workspaces" ADD COLUMN IF NOT EXISTS "user_identifier" uuid;
                ;

