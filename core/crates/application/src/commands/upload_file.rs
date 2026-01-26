// =============================================================================
// application/src/commands/upload_file.rs: ファイルアップロードコマンド
// =============================================================================
// ファイルをストレージにアップロードするユースケース。
//
// Clean Architecture:
// - Presentation 層から呼び出される
// - Domain 層の StorageOps トレイトに依存（具象実装を知らない）
// - Domain 層のバリデーションロジックを使用
//
// 処理フロー:
// 1. ファイル名、サイズ、MIME タイプのバリデーション（Domain 層）
// 2. ストレージにアップロード（StorageOps 経由）
// 3. ログ出力して結果を返す
// =============================================================================

// -----------------------------------------------------------------------------
// 標準ライブラリのインポート
// -----------------------------------------------------------------------------

use std::sync::Arc;

// -----------------------------------------------------------------------------
// 外部クレートのインポート
// -----------------------------------------------------------------------------

use domain::{DomainError, File, StorageOps};
use tracing::info;
use uuid::Uuid;

// =============================================================================
// UploadFileResult 構造体
// =============================================================================

/// ファイルアップロード結果 DTO
///
/// アップロード成功時に返される情報。
/// クライアントはこの情報を使って TODO + ファイル同時作成を行う。
#[derive(Debug, Clone)]
pub struct UploadFileResult {
    /// S3 キー（storage_path）
    pub storage_path: String,
    /// 元のファイル名（バリデーション済み）
    pub filename: String,
    /// MIME タイプ（バリデーション済み）
    pub mime_type: String,
    /// ファイルサイズ（バイト）
    pub size_bytes: i64,
}

// =============================================================================
// UploadFileCommand 構造体
// =============================================================================

/// ファイルアップロードコマンド
///
/// # ジェネリクス
///
/// - `S: StorageOps` - ストレージ操作の型（S3StorageService など）
///
/// # 責務
///
/// - ファイルのバリデーション（Domain 層に委譲）
/// - ストレージへのアップロード
pub struct UploadFileCommand<S: StorageOps> {
    /// ストレージ操作（Arc でラップして共有可能に）
    storage: Arc<S>,
}

// -----------------------------------------------------------------------------
// Clone トレイト実装
// -----------------------------------------------------------------------------

impl<S: StorageOps> Clone for UploadFileCommand<S> {
    fn clone(&self) -> Self {
        Self {
            storage: Arc::clone(&self.storage),
        }
    }
}

// -----------------------------------------------------------------------------
// UploadFileCommand の実装
// -----------------------------------------------------------------------------

impl<S: StorageOps> UploadFileCommand<S> {
    /// 新しいコマンドを作成
    ///
    /// # Arguments
    /// * `storage` - StorageOps の共有参照（Arc でラップ）
    pub fn new(storage: Arc<S>) -> Self {
        Self { storage }
    }

    /// ファイルをアップロードする
    ///
    /// # Arguments
    /// * `user_id` - アップロードするユーザーの ID
    /// * `filename` - 元のファイル名
    /// * `content_type` - MIME タイプ
    /// * `data` - ファイルの内容（バイト列）
    ///
    /// # Returns
    /// * `Ok(UploadFileResult)` - アップロード成功
    /// * `Err(DomainError::Validation)` - バリデーションエラー
    /// * `Err(DomainError::External)` - ストレージエラー
    pub async fn execute(
        &self,
        user_id: Uuid,
        filename: &str,
        content_type: &str,
        data: Vec<u8>,
    ) -> Result<UploadFileResult, DomainError> {
        // 1. バリデーション（Domain 層のロジックを使用）
        let validated_filename = File::validate_filename(filename)?;
        let size_bytes = data.len() as i64;
        File::validate_size(size_bytes)?;
        let validated_mime_type = File::validate_mime_type(content_type)?;

        // 2. ストレージにアップロード
        let storage_path = self
            .storage
            .upload(user_id, &validated_filename, &validated_mime_type, data)
            .await?;

        // 3. ログ出力
        info!(
            user_id = %user_id,
            filename = %validated_filename,
            storage_path = %storage_path,
            size_bytes = size_bytes,
            "File uploaded successfully"
        );

        Ok(UploadFileResult {
            storage_path,
            filename: validated_filename,
            mime_type: validated_mime_type,
            size_bytes,
        })
    }
}
