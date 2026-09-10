-- ============================================
-- 17886068422026082478860684220260824_210439_create_user_table
-- ============================================

CREATE TABLE "users" ( "identifier" uuid_text PRIMARY KEY, "first_name" varchar NULL, "password" varchar NOT NULL, "last_name" varchar NULL, "email" varchar NOT NULL UNIQUE, "is_active" boolean NOT NULL DEFAULT FALSE, "profile_picture" varchar NULL, "username" varchar NULL, "created_at" datetime_text NOT NULL DEFAULT CURRENT_TIMESTAMP, "updated_at" datetime_text NULL );

