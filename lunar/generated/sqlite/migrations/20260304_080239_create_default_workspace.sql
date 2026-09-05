-- ============================================
-- 20260304_080239_create_default_workspace
-- ============================================


                INSERT INTO workspaces (identifier, name, description, created_at, updated_at)
                SELECT X'85b49460f44a47f4930a8d0b45ec34be', 'default', 'Default workspace', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP
                WHERE NOT EXISTS (
                    SELECT 1 FROM workspaces WHERE name = 'default'
                );
                ;


                UPDATE todo SET workspace_identifier = X'85b49460f44a47f4930a8d0b45ec34be' WHERE workspace_identifier IS NULL;
                UPDATE notes SET workspace_identifier = X'85b49460f44a47f4930a8d0b45ec34be' WHERE workspace_identifier IS NULL;
                UPDATE bookmark SET workspace_identifier = X'85b49460f44a47f4930a8d0b45ec34be' WHERE workspace_identifier IS NULL;
                UPDATE recycle_bin SET workspace_identifier = X'85b49460f44a47f4930a8d0b45ec34be' WHERE workspace_identifier IS NULL;
                UPDATE reminder SET workspace_identifier = X'85b49460f44a47f4930a8d0b45ec34be' WHERE workspace_identifier IS NULL;
                UPDATE snippets SET workspace_identifier = X'85b49460f44a47f4930a8d0b45ec34be' WHERE workspace_identifier IS NULL;
                ;

