-- =============================================================================
-- 20250127000000_create_users.down.sql: ロールバック
-- =============================================================================

-- インデックスを削除
DROP INDEX IF EXISTS idx_todos_user_id;
DROP INDEX IF EXISTS idx_todos_user_id_created_at;

-- 外部キー制約を削除
ALTER TABLE todos DROP CONSTRAINT IF EXISTS fk_todos_user_id;

-- user_id を TEXT 型に戻す
ALTER TABLE todos
ALTER COLUMN user_id TYPE TEXT;

-- インデックスを再作成（元の形式）
CREATE INDEX idx_todos_user_id ON todos (user_id);
CREATE INDEX idx_todos_user_id_created_at ON todos (user_id, created_at DESC);

-- users テーブルを削除
DROP TABLE IF EXISTS users;
