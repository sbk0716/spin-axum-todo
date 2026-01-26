-- =============================================================================
-- 20250126000000_add_user_id.down.sql: マルチテナント対応のロールバック
-- =============================================================================

-- インデックスを削除
DROP INDEX IF EXISTS idx_todos_user_id_created_at;
DROP INDEX IF EXISTS idx_todos_user_id;

-- user_id カラムを削除
ALTER TABLE todos
DROP COLUMN IF EXISTS user_id;
