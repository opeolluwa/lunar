-- ============================================
-- 178860563520260224_221622_add_workspace_id_to_reminder_entities
-- ============================================

CREATE TABLE IF NOT EXISTS "reminders_new" ( "identifier" uuid_text NOT NULL PRIMARY KEY, "title" varchar NOT NULL, "description" text NULL, "recurring" boolean NOT NULL DEFAULT FALSE, "recurrence_rule" varchar NULL, "alarm_sound" varchar NULL, "workspace_identifier" uuid_text, "remind_at" timestamp_with_timezone_text NOT NULL, "created_at" timestamp_with_timezone_text NOT NULL, "updated_at" timestamp_with_timezone_text NOT NULL, FOREIGN KEY ("workspace_identifier") REFERENCES "workspaces" ("identifier") ON DELETE CASCADE );


                    INSERT INTO "reminders_new" ("identifier", "title", "description", "recurring", "recurrence_rule", "alarm_sound", "remind_at", "created_at", "updated_at")

                    SELECT "identifier", "title", "description", "recurring", "recurrence_rule", "alarm_sound", "remind_at", "created_at", "updated_at" FROM "reminder";
                    
                    DROP TABLE "reminder";

                    ALTER TABLE "reminders_new" RENAME TO "reminder";
                    ;

