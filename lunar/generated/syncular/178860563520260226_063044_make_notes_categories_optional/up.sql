-- ============================================
-- 178860563520260226_063044_make_notes_categories_optional
-- ============================================

CREATE TABLE IF NOT EXISTS "notes_new" ( "identifier" uuid_text NOT NULL PRIMARY KEY, "title" varchar NOT NULL, "content" text NOT NULL, "categories" json_text NULL, "created_at" timestamp_with_timezone_text NOT NULL, "updated_at" timestamp_with_timezone_text NOT NULL, "workspace_identifier" uuid_text, FOREIGN KEY ("workspace_identifier") REFERENCES "workspaces" ("identifier") ON DELETE CASCADE );


                     INSERT INTO "notes_new" ("identifier", "title", "content", "categories", "created_at", "updated_at")

                     SELECT "identifier", "title", "content", "categories", "created_at", "updated_at" FROM "notes";

                     DROP TABLE "notes";
                     ALTER TABLE "notes_new" RENAME TO "notes";
                        ;

