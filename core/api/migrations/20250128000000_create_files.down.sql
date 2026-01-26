-- =============================================================================
-- files テーブルのロールバック
-- =============================================================================

DROP INDEX IF EXISTS idx_files_storage_path;
DROP INDEX IF EXISTS idx_files_todo_id;
DROP TABLE IF EXISTS files;
