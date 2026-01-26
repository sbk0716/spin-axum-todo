-- =============================================================================
-- 20250127000000_create_users.up.sql: ローカル認証用ユーザーテーブル
-- =============================================================================
-- ローカル認証を採用し、パスワードハッシュを users テーブルで管理する。
-- Core 層で JWT を発行し、Edge 層で検証する。
-- =============================================================================

-- -----------------------------------------------------------------------------
-- users テーブルを作成
-- -----------------------------------------------------------------------------
CREATE TABLE users (
    -- 内部 ID（UUID v4）- JWT sub クレームとして使用
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- メールアドレス（ログイン用、一意）
    email TEXT NOT NULL UNIQUE,

    -- パスワードハッシュ（bcrypt）
    password_hash TEXT NOT NULL,

    -- 表示名（任意）
    display_name TEXT,

    -- メタデータ
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- email での検索用インデックス
CREATE INDEX idx_users_email ON users (email);

-- -----------------------------------------------------------------------------
-- todos テーブルの user_id を UUID に変更し、外部キー制約を追加
-- -----------------------------------------------------------------------------
-- 既存の user_id インデックスを削除
DROP INDEX IF EXISTS idx_todos_user_id;
DROP INDEX IF EXISTS idx_todos_user_id_created_at;

-- user_id を UUID 型に変更
-- 注: 既存データがある場合はエラーになる（開発環境では問題なし）
ALTER TABLE todos
ALTER COLUMN user_id TYPE UUID USING user_id::uuid;

-- 外部キー制約を追加（CASCADE 削除）
ALTER TABLE todos
ADD CONSTRAINT fk_todos_user_id
FOREIGN KEY (user_id) REFERENCES users (id)
ON DELETE CASCADE;

-- 新しいインデックスを作成
CREATE INDEX idx_todos_user_id ON todos (user_id);
CREATE INDEX idx_todos_user_id_created_at ON todos (user_id, created_at DESC);
