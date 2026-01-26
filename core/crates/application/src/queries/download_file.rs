// =============================================================================
// application/src/queries/download_file.rs: ファイルダウンロードクエリ
// =============================================================================
// ファイルをストレージからダウンロードするユースケース。
//
// Clean Architecture:
// - Presentation 層から呼び出される
// - Domain 層の FileReader, TodoReader, StorageOps トレイトに依存
// - アクセス制御: TODO の所有者のみがファイルをダウンロード可能
//
// 処理フロー:
// 1. ファイルメタデータを取得（FileReader 経由）
// 2. 親 TODO の所有者を確認（TodoReader 経由）
// 3. ストレージからダウンロード（StorageOps 経由）
// 4. ログ出力して結果を返す
// =============================================================================

// -----------------------------------------------------------------------------
// 標準ライブラリのインポート
// -----------------------------------------------------------------------------

use std::sync::Arc;

// -----------------------------------------------------------------------------
// 外部クレートのインポート
// -----------------------------------------------------------------------------

use domain::{DomainError, FileReader, StorageOps, TodoReader};
use tracing::info;
use uuid::Uuid;

// =============================================================================
// DownloadFileResult 構造体
// =============================================================================

/// ファイルダウンロード結果 DTO
///
/// ダウンロード成功時に返される情報。
/// Presentation 層でレスポンスヘッダーを設定するために使用。
#[derive(Debug, Clone)]
pub struct DownloadFileResult {
    /// ファイルデータ
    pub data: Vec<u8>,
    /// 元のファイル名
    pub filename: String,
    /// MIME タイプ
    pub mime_type: String,
}

// =============================================================================
// DownloadFileQuery 構造体
// =============================================================================

/// ファイルダウンロードクエリ
///
/// # ジェネリクス
///
/// - `TR: TodoReader` - TODO 読み取りの型（所有者確認用）
/// - `S: StorageOps` - ストレージ操作の型
///
/// # トレイトオブジェクト
///
/// - `file_reader: Arc<dyn FileReader>` - AppState と同じくトレイトオブジェクトを使用
///
/// # アクセス制御
///
/// ファイルの所有者（= 親 TODO の所有者）のみがダウンロード可能。
/// これにより他ユーザーのファイルへの不正アクセスを防止。
pub struct DownloadFileQuery<TR: TodoReader, S: StorageOps> {
    /// ファイルメタデータ読み取り（トレイトオブジェクト）
    file_reader: Arc<dyn FileReader>,
    /// TODO 読み取り（所有者確認用）
    todo_reader: Arc<TR>,
    /// ストレージ操作
    storage: Arc<S>,
}

// -----------------------------------------------------------------------------
// Clone トレイト実装
// -----------------------------------------------------------------------------

impl<TR: TodoReader, S: StorageOps> Clone for DownloadFileQuery<TR, S> {
    fn clone(&self) -> Self {
        Self {
            file_reader: Arc::clone(&self.file_reader),
            todo_reader: Arc::clone(&self.todo_reader),
            storage: Arc::clone(&self.storage),
        }
    }
}

// -----------------------------------------------------------------------------
// DownloadFileQuery の実装
// -----------------------------------------------------------------------------

impl<TR: TodoReader, S: StorageOps> DownloadFileQuery<TR, S> {
    /// 新しいクエリを作成
    ///
    /// # Arguments
    /// * `file_reader` - FileReader のトレイトオブジェクト
    /// * `todo_reader` - TodoReader の共有参照
    /// * `storage` - StorageOps の共有参照
    pub fn new(file_reader: Arc<dyn FileReader>, todo_reader: Arc<TR>, storage: Arc<S>) -> Self {
        Self {
            file_reader,
            todo_reader,
            storage,
        }
    }

    /// ファイルをダウンロードする
    ///
    /// # Arguments
    /// * `file_id` - ダウンロードするファイルの ID
    /// * `user_id` - リクエストしたユーザーの ID
    ///
    /// # Returns
    /// * `Ok(DownloadFileResult)` - ダウンロード成功
    /// * `Err(DomainError::NotFound)` - ファイルまたは TODO が見つからない
    /// * `Err(DomainError::External)` - ストレージエラー
    ///
    /// # アクセス制御
    /// user_id が親 TODO の所有者でない場合、NotFound を返す。
    pub async fn execute(
        &self,
        file_id: Uuid,
        user_id: Uuid,
    ) -> Result<DownloadFileResult, DomainError> {
        // 1. ファイルメタデータを取得
        let file = self
            .file_reader
            .find_by_id(file_id)
            .await?
            .ok_or(DomainError::NotFound)?;

        // 2. 親 TODO を取得して所有者を確認
        // find_by_id(todo_id, user_id) は user_id が所有者でない場合 None を返す
        let _todo = self
            .todo_reader
            .find_by_id(file.todo_id, user_id)
            .await?
            .ok_or(DomainError::NotFound)?;

        // 3. ストレージからダウンロード
        let data = self.storage.download(&file.storage_path).await?;

        // 4. ログ出力
        info!(
            file_id = %file_id,
            user_id = %user_id,
            filename = %file.filename,
            size = data.len(),
            "File downloaded successfully"
        );

        Ok(DownloadFileResult {
            data,
            filename: file.filename,
            mime_type: file.mime_type,
        })
    }
}
