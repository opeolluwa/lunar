-- ============================================
-- 20260221_065938_create_user_preference_table
-- ============================================

CREATE TABLE IF NOT EXISTS "user_preference" ( "identifier" uuid_text NOT NULL PRIMARY KEY, "first_name" varchar NOT NULL, "last_name" varchar NOT NULL, "email" varchar NOT NULL UNIQUE, "created_at" timestamp_with_timezone_text NOT NULL, "updated_at" timestamp_with_timezone_text NOT NULL );

