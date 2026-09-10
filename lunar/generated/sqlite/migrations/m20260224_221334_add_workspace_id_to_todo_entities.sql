-- ============================================
-- m20260224_221334_add_workspace_id_to_todo_entities
-- ============================================

CREATE TABLE IF NOT EXISTS "todo_new" ( "identifier" uuid_text NOT NULL PRIMARY KEY, "title" varchar NOT NULL, "description" text NULL, "due_date" date_text NULL, "priority" enum_text NOT NULL DEFAULT 'medium', "created_at" timestamp_with_timezone_text NOT NULL, "updated_at" timestamp_with_timezone_text NOT NULL, "due_time" time_text NULL, "workspace_identifier" uuid_text, "done" boolean NOT NULL DEFAULT FALSE, FOREIGN KEY ("workspace_identifier") REFERENCES "workspaces" ("identifier") ON DELETE CASCADE );


                     INSERT INTO "todo_new" ("identifier", "title", "description", "done", "created_at", "updated_at")

                     SELECT "identifier", "title", "description", "done", "created_at", "updated_at" FROM "todo";

                     DROP TABLE "todo";
                     ALTER TABLE "todo_new" RENAME TO "todo";
                        ;

