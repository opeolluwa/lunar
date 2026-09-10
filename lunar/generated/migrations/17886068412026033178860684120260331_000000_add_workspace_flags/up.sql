-- ============================================
-- 17886068412026033178860684120260331_000000_add_workspace_flags
-- ============================================


                ALTER TABLE workspaces ADD COLUMN is_default BOOLEAN NOT NULL DEFAULT FALSE;
                ALTER TABLE workspaces ADD COLUMN is_hidden BOOLEAN NOT NULL DEFAULT FALSE;
                UPDATE workspaces SET is_default = TRUE WHERE name = 'default';
                ;

