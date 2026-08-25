-- ============================================
-- m20260824_210439_create_user_table
-- ============================================

CREATE TABLE "users" ( "identifier" uuid PRIMARY KEY, "first_name" varchar NULL, "last_name" varchar NULL, "email" varchar NOT NULL UNIQUE, "is_active" bool NOT NULL DEFAULT FALSE, "profile_picture" varchar NULL, "username" varchar NULL, "created_at" timestamp without time zone NOT NULL DEFAULT CURRENT_TIMESTAMP, "updated_at" timestamp without time zone NULL );

