// =============================================================================
// domain/src/repositories/storage_repository.rs: ストレージ操作トレイト
// =============================================================================
// ファイルストレージ（S3 等）の抽象インターフェース。
//
// ストレージ管理の設計:
// - ファイルの実体をオブジェクトストレージに保存
// - メタデータはデータベースで管理（FileReader/FileWriter）
// - storage_path でストレージ上のファイルを識別
//
// なぜトレイトで抽象化するか:
// - Clean Architecture: Application 層が Infrastructure 層に直接依存しない
// - テスト容易性: モック実装でユニットテストが可能
// - 交換可能性: S3 → GCS → Azure Blob など、実装を切り替え可能
//
// 実装例:
// - S3StorageService: AWS S3 / LocalStack 実装（infrastructure 層）
// =============================================================================

// -----------------------------------------------------------------------------
// 外部クレートのインポート
// -----------------------------------------------------------------------------

// async_trait: トレイト内で async fn を使用可能にする
use async_trait::async_trait;

// uuid: 一意識別子
use uuid::Uuid;

// 同じクレート内のエラー型
use crate::errors::DomainError;

// =============================================================================
// StorageOps トレイト
// =============================================================================

/// ストレージ操作トレイト
///
/// オブジェクトストレージ（S3、GCS など）の抽象インターフェース。
/// Application 層はこのトレイトに依存し、具象実装（S3StorageService 等）を知らない。
///
/// # storage_path の形式
/// `users/{user_id}/files/{uuid}/{filename}`
///
/// # セキュリティ
/// - user_id でユーザーごとに名前空間を分離
/// - UUID で推測困難なパスを生成
/// - ファイル名はサニタイズ済みであること
///
/// # 実装例
/// - `S3StorageService`: AWS S3 / LocalStack 実装（infrastructure 層）
#[async_trait]
pub trait StorageOps: Send + Sync {
    /// ファイルをアップロード
    ///
    /// # Arguments
    /// * `user_id` - アップロードするユーザーの UUID
    /// * `filename` - ファイル名（サニタイズ済み）
    /// * `content_type` - MIME タイプ
    /// * `data` - ファイルの内容（バイト列）
    ///
    /// # Returns
    /// * `Ok(String)` - storage_path（ストレージ上のキー）
    /// * `Err(DomainError::External)` - ストレージエラー
    ///
    /// # Example
    /// ```rust,ignore
    /// let storage_path = storage.upload(
    ///     user_id,
    ///     "image.png",
    ///     "image/png",
    ///     file_data,
    /// ).await?;
    /// // storage_path: "users/{user_id}/files/{uuid}/image.png"
    /// ```
    async fn upload(
        &self,
        user_id: Uuid,
        filename: &str,
        content_type: &str,
        data: Vec<u8>,
    ) -> Result<String, DomainError>;

    /// ファイルをダウンロード
    ///
    /// # Arguments
    /// * `storage_path` - ストレージ上のキー（upload 時に返された値）
    ///
    /// # Returns
    /// * `Ok(Vec<u8>)` - ファイルの内容
    /// * `Err(DomainError::External)` - ストレージエラー
    /// * `Err(DomainError::NotFound)` - ファイルが存在しない
    ///
    /// # Note
    /// アクセス制御は呼び出し側の責任。
    /// storage_path を知っているだけではダウンロードを許可しない設計にすること。
    async fn download(&self, storage_path: &str) -> Result<Vec<u8>, DomainError>;

    /// ファイルを削除
    ///
    /// # Arguments
    /// * `storage_path` - ストレージ上のキー（upload 時に返された値）
    ///
    /// # Returns
    /// * `Ok(())` - 削除成功（存在しない場合も成功として扱う - 冪等性）
    /// * `Err(DomainError::External)` - ストレージエラー
    ///
    /// # Note
    /// S3 の DeleteObject は冪等。存在しないキーを削除してもエラーにならない。
    async fn delete(&self, storage_path: &str) -> Result<(), DomainError>;
}
