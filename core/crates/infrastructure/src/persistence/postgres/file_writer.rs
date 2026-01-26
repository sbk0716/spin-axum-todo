// =============================================================================
// infrastructure/src/persistence/postgres/file_writer.rs: ファイル書き込み実装
// =============================================================================
// PostgreSQL を使用した FileWriter の実装。
// ファイルメタデータの作成・削除を担当。
// Writer DB プールと組み合わせることでレプリケーション対応可能。
//
// 注意:
// - このリポジトリはファイルメタデータ（DB レコード）のみを扱う
// - 実際のファイル本体のアップロード/削除は別途ストレージサービスが必要
// - メタデータとファイル本体の整合性には注意が必要
// =============================================================================

// -----------------------------------------------------------------------------
// 外部クレートのインポート
// -----------------------------------------------------------------------------

// async_trait: async fn を含むトレイトを定義可能にする
use async_trait::async_trait;

// chrono: 日付・時刻ライブラリ
use chrono::{DateTime, Utc};

// domain: ドメイン層の型をインポート
use domain::{DomainError, File, FileWriter};

// sqlx: PostgreSQL クライアントライブラリ
use sqlx::{FromRow, PgPool};

// tracing: 構造化ログライブラリ
use tracing::debug;

// uuid: 一意識別子ライブラリ
use uuid::Uuid;

// =============================================================================
// PostgresFileWriter 構造体
// =============================================================================

/// PostgreSQL への接続を管理するファイル書き込みリポジトリ
///
/// # 責務
///
/// - ファイルメタデータの作成
/// - ファイルメタデータの削除
///
/// # ファイルアップロードの流れ
///
/// 1. ファイル本体をストレージ（S3 など）にアップロード
/// 2. アップロード成功後、このリポジトリでメタデータを作成
/// 3. メタデータ作成失敗時はストレージからも削除（補償トランザクション）
pub struct PostgresFileWriter {
    /// PostgreSQL 接続プール（Writer 用）
    pool: PgPool,
}

impl PostgresFileWriter {
    /// 新しい PostgresFileWriter を作成する
    ///
    /// # Arguments
    ///
    /// * `pool` - PostgreSQL 接続プール（Writer プール）
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

// =============================================================================
// FileRow 構造体（内部用）
// =============================================================================

/// データベース行からのマッピング用構造体
///
/// RETURNING 句の結果を受け取るために使用。
#[derive(FromRow)]
struct FileRow {
    /// ファイルの一意識別子
    id: Uuid,
    /// 紐付く TODO の ID（外部キー）
    todo_id: Uuid,
    /// ファイル名（元のファイル名）
    filename: String,
    /// MIME タイプ
    mime_type: String,
    /// ファイルサイズ（バイト）
    size_bytes: i64,
    /// ストレージ上のパス
    storage_path: String,
    /// 作成日時
    created_at: DateTime<Utc>,
}

/// FileRow から domain::File への変換
impl From<FileRow> for File {
    fn from(row: FileRow) -> Self {
        File::from_raw(
            row.id,
            row.todo_id,
            row.filename,
            row.mime_type,
            row.size_bytes,
            row.storage_path,
            row.created_at,
        )
    }
}

// =============================================================================
// FileWriter トレイト実装
// =============================================================================

/// FileWriter トレイトの PostgreSQL 実装
#[async_trait]
impl FileWriter for PostgresFileWriter {
    /// 新しいファイルを作成する
    ///
    /// # Arguments
    ///
    /// * `file` - 作成するファイルエンティティ
    ///
    /// # Returns
    ///
    /// * `Ok(File)` - 作成されたファイル
    /// * `Err(DomainError)` - DB エラー
    ///
    /// # Note
    ///
    /// この操作の前にファイル本体をストレージにアップロード済みであること。
    /// todo_id は外部キー制約があるため、存在する TODO の ID である必要がある。
    async fn create(&self, file: &File) -> Result<File, DomainError> {
        // 構造化ログ: ファイル ID と TODO ID を記録
        debug!(file_id = %file.id, todo_id = %file.todo_id, "Creating file in PostgreSQL");

        // INSERT ... RETURNING で挿入と取得を同時に実行
        let row: FileRow = sqlx::query_as(
            r#"
            INSERT INTO files (id, todo_id, filename, mime_type, size_bytes, storage_path, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, todo_id, filename, mime_type, size_bytes, storage_path, created_at
            "#,
        )
        .bind(file.id) // $1: 事前生成した UUID
        .bind(file.todo_id) // $2: 親 TODO の ID（外部キー）
        .bind(&file.filename) // $3: ファイル名
        .bind(&file.mime_type) // $4: MIME タイプ
        .bind(file.size_bytes) // $5: サイズ（バイト）
        .bind(&file.storage_path) // $6: ストレージパス
        .bind(file.created_at) // $7: 作成日時
        .fetch_one(&self.pool) // 1行取得
        .await
        .map_err(|e| DomainError::Repository(e.to_string()))?;

        // FileRow → File に変換
        Ok(row.into())
    }

    /// ファイルを削除する
    ///
    /// # Arguments
    ///
    /// * `id` - 削除するファイルの UUID
    ///
    /// # Returns
    ///
    /// * `Ok(true)` - 削除成功（1行削除）
    /// * `Ok(false)` - 該当なし（0行削除）
    /// * `Err(DomainError)` - DB エラー
    ///
    /// # Warning
    ///
    /// メタデータ削除後、storage_path のファイル本体も
    /// ストレージから削除する必要がある。
    async fn delete(&self, id: Uuid) -> Result<bool, DomainError> {
        // 構造化ログ: ファイル ID を記録
        debug!(file_id = %id, "Deleting file from PostgreSQL");

        // DELETE 文を実行
        let result = sqlx::query(
            r#"
            DELETE FROM files
            WHERE id = $1
            "#,
        )
        .bind(id) // $1: 削除対象の ID
        .execute(&self.pool) // 実行
        .await
        .map_err(|e| DomainError::Repository(e.to_string()))?;

        // rows_affected(): 削除された行数
        Ok(result.rows_affected() > 0)
    }

    /// TODO に紐付く全ファイルを削除する
    ///
    /// TODO 削除時に呼び出される。
    ///
    /// # Arguments
    ///
    /// * `todo_id` - 親 TODO の UUID
    ///
    /// # Returns
    ///
    /// * `Ok(u64)` - 削除されたファイル数
    /// * `Err(DomainError)` - DB エラー
    ///
    /// # Warning
    ///
    /// 削除前に find_by_todo_id でファイル一覧を取得し、
    /// 各ファイルの storage_path を記録しておくこと。
    /// メタデータ削除後、ストレージからも削除する必要がある。
    async fn delete_by_todo_id(&self, todo_id: Uuid) -> Result<u64, DomainError> {
        // 構造化ログ: TODO ID を記録
        debug!(todo_id = %todo_id, "Deleting all files for todo from PostgreSQL");

        // TODO ID に紐付く全ファイルを削除
        let result = sqlx::query(
            r#"
            DELETE FROM files
            WHERE todo_id = $1
            "#,
        )
        .bind(todo_id) // $1: TODO ID
        .execute(&self.pool) // 実行
        .await
        .map_err(|e| DomainError::Repository(e.to_string()))?;

        // 削除されたファイル数を返す
        Ok(result.rows_affected())
    }
}
