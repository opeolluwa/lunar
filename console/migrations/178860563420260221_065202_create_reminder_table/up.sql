-- ============================================
-- 178860563420260221_065202_create_reminder_table
-- ============================================

CREATE TABLE IF NOT EXISTS "reminder" ( "identifier" uuid_text NOT NULL PRIMARY KEY, "title" varchar NOT NULL, "description" text NULL, "recurring" boolean NOT NULL DEFAULT FALSE, "recurrence_rule" varchar NULL, "alarm_sound" varchar NULL, "remind_at" timestamp_with_timezone_text NOT NULL, "created_at" timestamp_with_timezone_text NOT NULL, "updated_at" timestamp_with_timezone_text NOT NULL );

