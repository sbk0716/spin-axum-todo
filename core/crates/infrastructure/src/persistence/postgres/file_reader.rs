// =============================================================================
// infrastructure/src/persistence/postgres/file_reader.rs: ファイル読み取り実装
// =============================================================================
// PostgreSQL を使用した FileReader の実装。
// ファイルメタデータの取得を担当。
// Reader DB プールと組み合わせることでレプリケーション対応可能。
//
// 注意:
// - このリポジトリはファイルメタデータ（DB レコード）のみを扱う
// - 実際のファイル本体は storage_path で示される場所に保存される
// - ファイル本体のダウンロードは別途ストレージサービスが必要
// =============================================================================

// -----------------------------------------------------------------------------
// 外部クレートのインポート
// -----------------------------------------------------------------------------

// async_trait: async fn を含むトレイトを定義可能にする
use async_trait::async_trait;

// chrono: 日付・時刻ライブラリ
use chrono::{DateTime, Utc};

// domain: ドメイン層の型をインポート
use domain::{DomainError, File, FileReader};

// sqlx: PostgreSQL クライアントライブラリ
use sqlx::{FromRow, PgPool};

// tracing: 構造化ログライブラリ
use tracing::debug;

// uuid: 一意識別子ライブラリ
use uuid::Uuid;

// =============================================================================
// PostgresFileReader 構造体
// =============================================================================

/// PostgreSQL への接続を管理するファイル読み取りリポジトリ
///
/// # 責務
///
/// - ファイルメタデータの読み取り
/// - TODO に紐付くファイル一覧の取得
///
/// # Note
///
/// ファイル本体のダウンロードには storage_path を使用して
/// 別途ストレージサービス（S3 など）にアクセスする。
pub struct PostgresFileReader {
    /// PostgreSQL 接続プール（Reader 用推奨）
    pool: PgPool,
}

impl PostgresFileReader {
    /// 新しい PostgresFileReader を作成する
    ///
    /// # Arguments
    ///
    /// * `pool` - PostgreSQL 接続プール（Reader プール推奨）
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

// =============================================================================
// FileRow 構造体（内部用）
// =============================================================================

/// データベース行からのマッピング用構造体
///
/// SQLx の `query_as` で使用する内部構造体。
#[derive(FromRow)]
struct FileRow {
    /// ファイルの一意識別子
    id: Uuid,
    /// 紐付く TODO の ID（外部キー）
    todo_id: Uuid,
    /// ファイル名（元のファイル名）
    filename: String,
    /// MIME タイプ（例: "image/png"）
    mime_type: String,
    /// ファイルサイズ（バイト）
    /// i64: PostgreSQL の BIGINT に対応
    size_bytes: i64,
    /// ストレージ上のパス（S3 キーなど）
    storage_path: String,
    /// 作成日時
    created_at: DateTime<Utc>,
}

/// FileRow から domain::File への変換
impl From<FileRow> for File {
    fn from(row: FileRow) -> Self {
        File::from_raw(
            row.id,           // UUID: 主キー
            row.todo_id,      // UUID: 親 TODO
            row.filename,     // String: ファイル名
            row.mime_type,    // String: MIME タイプ
            row.size_bytes,   // i64: サイズ
            row.storage_path, // String: ストレージパス
            row.created_at,   // DateTime<Utc>: 作成日時
        )
    }
}

// =============================================================================
// FileReader トレイト実装
// =============================================================================

/// FileReader トレイトの PostgreSQL 実装
#[async_trait]
impl FileReader for PostgresFileReader {
    /// ID でファイルを検索する
    ///
    /// # Arguments
    ///
    /// * `id` - 検索するファイルの UUID
    ///
    /// # Returns
    ///
    /// * `Ok(Some(file))` - 見つかった場合
    /// * `Ok(None)` - 見つからない場合
    /// * `Err(DomainError)` - DB エラーの場合
    ///
    /// # Note
    ///
    /// ファイル本体のダウンロードには、返された File の
    /// storage_path を使用してストレージにアクセスする。
    async fn find_by_id(&self, id: Uuid) -> Result<Option<File>, DomainError> {
        // 構造化ログ: ファイル ID を記録
        debug!(file_id = %id, "Finding file by ID in PostgreSQL");

        // UUID で検索（PRIMARY KEY）
        let row: Option<FileRow> = sqlx::query_as(
            r#"
            SELECT id, todo_id, filename, mime_type, size_bytes, storage_path, created_at
            FROM files
            WHERE id = $1
            "#,
        )
        .bind(id) // $1: ファイル ID
        .fetch_optional(&self.pool) // 0件 or 1件を取得
        .await // 非同期実行
        .map_err(|e| DomainError::Repository(e.to_string()))?; // エラー変換

        // Option<FileRow> → Option<File> に変換
        Ok(row.map(Into::into))
    }

    /// TODO ID でファイル一覧を取得する
    ///
    /// # Arguments
    ///
    /// * `todo_id` - 親 TODO の UUID
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<File>)` - ファイルのリスト（0件も可）
    /// * `Err(DomainError)` - DB エラーの場合
    ///
    /// # Note
    ///
    /// 結果は created_at ASC でソート（古い順）。
    /// TODO 詳細画面でファイル一覧を表示する際に使用。
    async fn find_by_todo_id(&self, todo_id: Uuid) -> Result<Vec<File>, DomainError> {
        // 構造化ログ: TODO ID を記録
        debug!(todo_id = %todo_id, "Finding files by todo_id in PostgreSQL");

        // TODO ID で検索（外部キー）
        let rows: Vec<FileRow> = sqlx::query_as(
            r#"
            SELECT id, todo_id, filename, mime_type, size_bytes, storage_path, created_at
            FROM files
            WHERE todo_id = $1
            ORDER BY created_at ASC
            "#,
        )
        .bind(todo_id) // $1: TODO ID
        .fetch_all(&self.pool) // 全件取得
        .await // 非同期実行
        .map_err(|e| DomainError::Repository(e.to_string()))?; // エラー変換

        // Vec<FileRow> → Vec<File> に変換
        Ok(rows.into_iter().map(Into::into).collect())
    }
}
