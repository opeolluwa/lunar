-- ============================================
-- m20260825_000002_create_revoked_tokens_table
-- ============================================

CREATE TABLE IF NOT EXISTS "revoked_token" ( "identifier" uuid NOT NULL PRIMARY KEY, "jti" uuid NOT NULL UNIQUE, "user_identifier" uuid NOT NULL, "expires_at" timestamp with time zone NOT NULL, "revoked_at" timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP );

ALTER TABLE "revoked_token" ADD CONSTRAINT "fk_revoked_tokens_user_identifier" FOREIGN KEY ("user_identifier") REFERENCES "users" ("identifier") ON DELETE CASCADE;

CREATE INDEX "revoked_tokens_jti_idx" ON "revoked_token" ("jti");

CREATE INDEX "revoked_tokens_expires_at_idx" ON "revoked_token" ("expires_at");

