-- ============================================
-- 178860563520260403_000000_add_workspace_security
-- ============================================


                ALTER TABLE workspaces ADD COLUMN is_secured BOOLEAN NOT NULL DEFAULT FALSE;
                ALTER TABLE workspaces ADD COLUMN password_hash TEXT;
                ;

