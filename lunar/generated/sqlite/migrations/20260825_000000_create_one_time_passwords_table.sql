-- ============================================
-- 20260825_000000_create_one_time_passwords_table
-- ============================================

CREATE TABLE IF NOT EXISTS "one_time_password" ( "identifier" uuid_text NOT NULL PRIMARY KEY, "user_identifier" uuid_text NOT NULL, "code" char(6) NOT NULL, "created_at" timestamp_with_timezone_text NOT NULL, "updated_at" timestamp_with_timezone_text NULL );

