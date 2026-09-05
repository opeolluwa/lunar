-- ============================================
-- 20260224_221707_add_workspace_id_to_recycle_bin_entities
-- ============================================

CREATE TABLE IF NOT EXISTS "recycle_bin_new" ( "identifier" uuid_text NOT NULL PRIMARY KEY, "item_id" uuid_text NOT NULL, "item_type" enum_text NOT NULL, "workspace_identifier" uuid_text, "payload" text NOT NULL, "deleted_at" timestamp_with_timezone_text NOT NULL, FOREIGN KEY ("workspace_identifier") REFERENCES "workspaces" ("identifier") ON DELETE CASCADE );


                    INSERT INTO "recycle_bin_new" ("identifier", "item_id", "item_type", "payload", "deleted_at")

                    SELECT "identifier", "item_id", "item_type", "payload", "deleted_at" FROM "recycle_bin";

                    DROP TABLE "recycle_bin";

                    ALTER TABLE "recycle_bin_new" RENAME TO "recycle_bin";
                    ;

