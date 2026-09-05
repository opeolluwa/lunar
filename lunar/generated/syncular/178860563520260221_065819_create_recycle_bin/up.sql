-- ============================================
-- 178860563520260221_065819_create_recycle_bin
-- ============================================

CREATE TABLE IF NOT EXISTS "recycle_bin" ( "identifier" uuid_text NOT NULL PRIMARY KEY, "item_id" uuid_text NOT NULL, "item_type" enum_text NOT NULL, "payload" text NOT NULL, "deleted_at" timestamp_with_timezone_text NOT NULL );

