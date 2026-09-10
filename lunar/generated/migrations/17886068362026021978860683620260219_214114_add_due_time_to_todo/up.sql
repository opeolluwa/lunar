-- ============================================
-- 17886068362026021978860683620260219_214114_add_due_time_to_todo
-- ============================================

ALTER TABLE "todo" ADD COLUMN IF NOT EXISTS "due_time" time NULL;

