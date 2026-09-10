-- ============================================
-- m20260528_200000_add_workspace_and_is_read_to_notifications
-- ============================================


                    DROP TABLE "notifications";             
                    ;

CREATE TABLE IF NOT EXISTS "notifications" ( "identifier" uuid_text NOT NULL PRIMARY KEY, "title" varchar NOT NULL, "body" varchar NOT NULL, "notification_type" enum_text NOT NULL DEFAULT 'generic', "is_read" boolean NOT NULL DEFAULT FALSE, "workspace_identifier" uuid_text, "created_at" timestamp_with_timezone_text NOT NULL, "updated_at" timestamp_with_timezone_text NOT NULL, FOREIGN KEY ("workspace_identifier") REFERENCES "workspaces" ("identifier") ON DELETE CASCADE );

