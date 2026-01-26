-- =============================================================================
-- 20250125000000_create_todos.up.sql: マイグレーション（UP）
-- =============================================================================
-- このファイルはデータベーススキーマを作成するマイグレーションファイル。
-- sqlx migrate run コマンドで実行される。
--
-- ファイル名の形式: {タイムスタンプ}_{説明}.up.sql
-- - タイムスタンプ: マイグレーションの実行順序を決定
-- - .up.sql: スキーマを適用する方向（対になる .down.sql でロールバック）
-- =============================================================================

-- -----------------------------------------------------------------------------
-- todos テーブル: TODO アイテムを保存
-- -----------------------------------------------------------------------------
CREATE TABLE todos (
    -- id: 主キー（UUID 形式）
    -- UUID を使う利点:
    -- - 分散システムでも衝突しにくい
    -- - 連番と違い、ID から作成順が推測されにくい（セキュリティ）
    -- PRIMARY KEY: この列がテーブルの一意識別子であることを示す
    id UUID PRIMARY KEY,

    -- title: TODO のタイトル（必須）
    -- TEXT: 可変長文字列（PostgreSQL では VARCHAR と同等）
    -- NOT NULL: NULL 値を許可しない
    title TEXT NOT NULL,

    -- description: TODO の詳細説明（任意）
    -- NOT NULL がないため NULL を許可（Rust では Option<String> に対応）
    description TEXT,

    -- completed: 完了フラグ
    -- BOOLEAN: 真偽値（true または false）
    -- DEFAULT false: 明示的に指定しない場合は false（未完了）
    completed BOOLEAN NOT NULL DEFAULT false,

    -- created_at: 作成日時
    -- TIMESTAMPTZ: タイムゾーン付きタイムスタンプ（内部的にはUTCで保存）
    -- DEFAULT now(): 明示的に指定しない場合は現在時刻
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    -- updated_at: 最終更新日時
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- -----------------------------------------------------------------------------
-- インデックス: クエリの高速化
-- -----------------------------------------------------------------------------

-- idx_todos_completed: completed でのフィルタリングを高速化
-- GET /todos?completed=true のようなクエリで使用される
-- インデックスがないと全行スキャン（遅い）、あればインデックススキャン（速い）
CREATE INDEX idx_todos_completed ON todos (completed);

-- idx_todos_created_at: created_at での並び替えを高速化
-- DESC: 降順（新しい順）でインデックスを作成
-- ORDER BY created_at DESC のクエリで効率的にソートできる
CREATE INDEX idx_todos_created_at ON todos (created_at DESC);
