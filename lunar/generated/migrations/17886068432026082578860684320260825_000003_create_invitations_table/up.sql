-- ============================================
-- 17886068432026082578860684320260825_000003_create_invitations_table
-- ============================================

CREATE TABLE IF NOT EXISTS "invitation" ( "identifier" uuid_text NOT NULL PRIMARY KEY, "workspace_identifier" uuid_text NOT NULL, "email" varchar NOT NULL, "first_name" varchar NULL, "last_name" varchar NULL, "token" varchar NOT NULL UNIQUE, "status" varchar NOT NULL DEFAULT 'pending', "expires_at" timestamp_with_timezone_text NOT NULL, "created_at" timestamp_with_timezone_text NOT NULL );

