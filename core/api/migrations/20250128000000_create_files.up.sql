-- =============================================================================
-- files テーブル: TODO に添付されるファイルのメタデータ
-- =============================================================================
-- トランザクション管理のデモ用テーブル。
-- TODO + ファイルを1トランザクションで作成することで、アトミック操作を実現。
-- =============================================================================

CREATE TABLE files (
    -- 主キー（UUID）
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- 紐づく TODO の ID（外部キー、CASCADE 削除）
    -- TODO が削除されると、紐づくファイルも自動削除される
    todo_id UUID NOT NULL REFERENCES todos(id) ON DELETE CASCADE,

    -- 元のファイル名（ユーザーがアップロードした時の名前）
    filename TEXT NOT NULL,

    -- MIME タイプ（例: image/png, application/pdf）
    mime_type TEXT NOT NULL,

    -- ファイルサイズ（バイト）
    size_bytes BIGINT NOT NULL,

    -- ストレージ上のファイルパス（実際のファイル保存先）
    storage_path TEXT NOT NULL,

    -- 作成日時
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- -----------------------------------------------------------------------------
-- インデックス
-- -----------------------------------------------------------------------------

-- todo_id でのフィルタリング高速化（TODO に紐づくファイル一覧取得用）
CREATE INDEX idx_files_todo_id ON files (todo_id);

-- storage_path の一意性保証（同一ファイルパスの重複防止）
CREATE UNIQUE INDEX idx_files_storage_path ON files (storage_path);
