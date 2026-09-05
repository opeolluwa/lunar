-- ============================================
-- 20260224_220314_add_workspace_id_to_snippet_entities
-- ============================================

CREATE TABLE IF NOT EXISTS "snippets_new" ( "identifier" uuid_text NOT NULL PRIMARY KEY, "title" varchar NULL, "language" varchar NULL, "code" text NOT NULL, "description" text NULL, "is_pinned" boolean NOT NULL DEFAULT FALSE, "created_at" timestamp_with_timezone_text NOT NULL, "updated_at" timestamp_with_timezone_text NOT NULL, "workspace_identifier" uuid_text, FOREIGN KEY ("workspace_identifier") REFERENCES "workspaces" ("identifier") ON DELETE CASCADE );


                    INSERT INTO "snippets_new" ("identifier", "title", "code", "description", "language", "is_pinned", "created_at", "updated_at")

                    SELECT "identifier", "title", "code", "description", "language", "is_pinned", "created_at", "updated_at" FROM "snippets";
                    DROP TABLE "snippets";
                    ALTER TABLE "snippets_new" RENAME TO "snippets";
                    ;

