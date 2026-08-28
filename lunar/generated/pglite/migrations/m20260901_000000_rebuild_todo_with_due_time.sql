-- ============================================
-- m20260901_000000_rebuild_todo_with_due_time
-- ============================================


                ALTER TABLE "todo" ADD COLUMN IF NOT EXISTS "due_time" time;
                ;

