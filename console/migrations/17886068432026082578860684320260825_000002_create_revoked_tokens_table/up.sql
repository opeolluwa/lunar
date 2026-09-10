-- ============================================
-- 17886068432026082578860684320260825_000002_create_revoked_tokens_table
-- ============================================

CREATE TABLE IF NOT EXISTS "revoked_token" ( "identifier" uuid_text NOT NULL PRIMARY KEY, "jti" uuid_text NOT NULL UNIQUE, "user_identifier" uuid_text NOT NULL, "expires_at" timestamp_with_timezone_text NOT NULL, "revoked_at" timestamp_with_timezone_text NOT NULL DEFAULT CURRENT_TIMESTAMP );

CREATE INDEX "revoked_tokens_jti_idx" ON "revoked_token" ("jti");

CREATE INDEX "revoked_tokens_expires_at_idx" ON "revoked_token" ("expires_at");

