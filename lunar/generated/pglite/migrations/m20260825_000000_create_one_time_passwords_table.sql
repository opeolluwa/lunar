-- ============================================
-- m20260825_000000_create_one_time_passwords_table
-- ============================================

CREATE TABLE IF NOT EXISTS "one_time_password" ( "identifier" uuid NOT NULL PRIMARY KEY, "user_identifier" uuid NOT NULL, "code" char(6) NOT NULL, "created_at" timestamp with time zone NOT NULL, "updated_at" timestamp with time zone NULL );

ALTER TABLE "one_time_password" ADD CONSTRAINT "fk_one_time_password_user_identifier" FOREIGN KEY ("user_identifier") REFERENCES "users" ("identifier") ON DELETE CASCADE;

