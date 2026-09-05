-- ============================================
-- 178860563520260224_221502_add_workspace_id_to_bookmark_entities
-- ============================================

CREATE TABLE IF NOT EXISTS "bookmarks_new" ( "identifier" uuid_text NOT NULL PRIMARY KEY, "title" varchar NOT NULL, "url" varchar NOT NULL, "tag" enum_text NOT NULL DEFAULT 'development', "workspace_identifier" uuid_text, "created_at" timestamp_with_timezone_text NOT NULL, "updated_at" timestamp_with_timezone_text NOT NULL, FOREIGN KEY ("workspace_identifier") REFERENCES "workspaces" ("identifier") ON DELETE CASCADE );


                    INSERT INTO "bookmarks_new" ("identifier", "title", "url", "created_at", "updated_at")

                    SELECT "identifier", "title", "url", "created_at", "updated_at" FROM "bookmark";
                    
                    DROP TABLE "bookmark";
                    ALTER TABLE "bookmarks_new" RENAME TO "bookmark";
                    ;

