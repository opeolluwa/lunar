-- ============================================
-- 17886068412026032778860684120260327_000000_add_workspace_id_to_user_preference
-- ============================================

CREATE TABLE IF NOT EXISTS "user_preference_new" ( "identifier" uuid_text NOT NULL PRIMARY KEY, "first_name" varchar NOT NULL, "last_name" varchar NOT NULL, "email" varchar NOT NULL UNIQUE, "workspace_identifier" uuid_text, "created_at" timestamp_with_timezone_text NOT NULL, "updated_at" timestamp_with_timezone_text NOT NULL, FOREIGN KEY ("workspace_identifier") REFERENCES "workspaces" ("identifier") ON DELETE CASCADE );


                    INSERT INTO "user_preference_new" ("identifier", "first_name", "last_name", "email", "created_at", "updated_at")
                    SELECT "identifier", "first_name", "last_name", "email", "created_at", "updated_at" FROM "user_preference";

                    DROP TABLE "user_preference";
                    ALTER TABLE "user_preference_new" RENAME TO "user_preference";
                    ;

