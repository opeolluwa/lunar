-- ============================================
-- 178860563620260528_132342_notification
-- ============================================

CREATE TABLE IF NOT EXISTS "notifications" ( "identifier" uuid_text NOT NULL PRIMARY KEY, "title" varchar NOT NULL, "body" varchar NOT NULL, "notification_type" enum_text NOT NULL DEFAULT 'generic', "created_at" timestamp_with_timezone_text NOT NULL, "updated_at" timestamp_with_timezone_text NOT NULL );

