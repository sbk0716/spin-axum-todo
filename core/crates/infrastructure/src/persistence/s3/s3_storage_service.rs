// =============================================================================
// infrastructure/src/persistence/s3/s3_storage_service.rs
// =============================================================================
// AWS S3（または LocalStack）を使用したファイルストレージサービス。
//
// 機能:
// - ファイルのアップロード（upload）
// - ファイルのダウンロード（download）
// - ファイルの削除（delete）
// - バケットの存在確認と作成（ensure_bucket_exists）
//
// S3 キーフォーマット:
// - `users/{user_id}/files/{file_uuid}/{filename}`
// - ユーザーごとにファイルを分離
// - UUID でファイルの一意性を保証
//
// 環境変数:
// - S3_ENDPOINT_URL: カスタムエンドポイント（LocalStack 用）
// - S3_BUCKET: バケット名（デフォルト: todo-files）
// - AWS_DEFAULT_REGION: リージョン
// - AWS_ACCESS_KEY_ID: アクセスキー
// - AWS_SECRET_ACCESS_KEY: シークレットキー
// =============================================================================

// -----------------------------------------------------------------------------
// 外部クレートのインポート
// -----------------------------------------------------------------------------

// async_trait: トレイト内で async fn を使用可能にする
use async_trait::async_trait;

// aws_sdk_s3: AWS S3 クライアント
use aws_sdk_s3::{primitives::ByteStream, Client};

// domain: ドメイン層の型をインポート
use domain::{DomainError, StorageOps};

// tracing: 構造化ログライブラリ
use tracing::{debug, info, warn};

// uuid: 一意識別子ライブラリ
use uuid::Uuid;

// =============================================================================
// S3StorageService 構造体
// =============================================================================

/// S3 ストレージサービス
///
/// # 責務
///
/// - ファイルのアップロード（S3 PUT）
/// - ファイルのダウンロード（S3 GET）
/// - ファイルの削除（S3 DELETE）
///
/// # Clone
///
/// `Client` は内部で Arc を使用しているため、Clone は安価。
/// 複数のハンドラ間で共有可能。
#[derive(Clone)]
pub struct S3StorageService {
    /// S3 クライアント
    client: Client,
    /// バケット名
    bucket: String,
}

impl S3StorageService {
    /// 新しい S3StorageService を作成する
    ///
    /// # Arguments
    ///
    /// * `client` - S3 クライアント
    /// * `bucket` - バケット名
    pub fn new(client: Client, bucket: String) -> Self {
        Self { client, bucket }
    }

    /// Config から S3StorageService を初期化する
    ///
    /// # Arguments
    ///
    /// * `bucket` - S3 バケット名
    /// * `endpoint_url` - カスタムエンドポイント URL（LocalStack 用、None の場合は AWS 標準）
    ///
    /// # 使用例
    ///
    /// ```rust,ignore
    /// let config = AppConfig::from_env()?;
    /// let storage = S3StorageService::from_config(
    ///     &config.s3.bucket,
    ///     config.s3.endpoint_url.as_deref(),
    /// ).await?;
    /// ```
    ///
    /// # LocalStack 対応
    ///
    /// `endpoint_url` が指定されている場合、カスタムエンドポイントを使用。
    /// `force_path_style(true)` を設定して LocalStack との互換性を確保。
    pub async fn from_config(
        bucket: &str,
        endpoint_url: Option<&str>,
    ) -> Result<Self, DomainError> {
        let bucket = bucket.to_string();

        // AWS 設定をロード
        let sdk_config = aws_config::from_env().load().await;

        // S3 クライアントを作成
        let client = if let Some(endpoint) = endpoint_url {
            info!(endpoint = %endpoint, bucket = %bucket, "Initializing S3 with custom endpoint (LocalStack)");
            // LocalStack 用（カスタムエンドポイント + パススタイル強制）
            let s3_config = aws_sdk_s3::config::Builder::from(&sdk_config)
                .endpoint_url(endpoint)
                .force_path_style(true)
                .build();
            Client::from_conf(s3_config)
        } else {
            info!(bucket = %bucket, "Initializing S3 with AWS default endpoint");
            // 本番用（AWS 標準）
            Client::new(&sdk_config)
        };

        Ok(Self { client, bucket })
    }

    /// バケットが存在することを確認し、なければ作成する
    ///
    /// # Returns
    ///
    /// * `Ok(())` - バケットが存在する、または作成成功
    /// * `Err(DomainError::External)` - バケット作成失敗
    ///
    /// # Note
    ///
    /// LocalStack ではバケットを自動作成する必要がある。
    /// 本番環境では事前にバケットを作成しておくことを推奨。
    pub async fn ensure_bucket_exists(&self) -> Result<(), DomainError> {
        debug!(bucket = %self.bucket, "Checking if bucket exists");

        // バケットの存在確認（HEAD Bucket）
        match self.client.head_bucket().bucket(&self.bucket).send().await {
            Ok(_) => {
                info!(bucket = %self.bucket, "Bucket exists");
                Ok(())
            }
            Err(e) => {
                // バケットが存在しない場合は作成を試みる
                warn!(bucket = %self.bucket, error = %e, "Bucket check failed, attempting to create...");

                match self
                    .client
                    .create_bucket()
                    .bucket(&self.bucket)
                    .send()
                    .await
                {
                    Ok(_) => {
                        info!(bucket = %self.bucket, "Bucket created successfully");
                        Ok(())
                    }
                    Err(create_err) => {
                        // BucketAlreadyOwnedByYou または BucketAlreadyExists は成功として扱う
                        let error_str = create_err.to_string();
                        if error_str.contains("BucketAlreadyOwnedByYou")
                            || error_str.contains("BucketAlreadyExists")
                        {
                            info!(bucket = %self.bucket, "Bucket already exists (confirmed via create attempt)");
                            Ok(())
                        } else {
                            Err(DomainError::External(format!(
                                "Failed to create bucket: {}",
                                create_err
                            )))
                        }
                    }
                }
            }
        }
    }

    /// ファイルを S3 にアップロードする
    ///
    /// # Arguments
    ///
    /// * `user_id` - ファイル所有者のユーザー ID
    /// * `filename` - 元のファイル名
    /// * `content_type` - MIME タイプ（例: "image/png"）
    /// * `data` - ファイルデータ
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - S3 キー（storage_path として DB に保存）
    /// * `Err(DomainError::External)` - アップロード失敗
    ///
    /// # S3 Key Format
    ///
    /// `users/{user_id}/files/{file_uuid}/{filename}`
    ///
    /// 例: `users/550e8400.../files/7c9e6679.../image.png`
    pub async fn upload(
        &self,
        user_id: Uuid,
        filename: &str,
        content_type: &str,
        data: Vec<u8>,
    ) -> Result<String, DomainError> {
        // S3 キーを生成
        // ユーザー ID + UUID でファイルの一意性を保証
        let file_uuid = Uuid::new_v4();
        let key = format!("users/{}/files/{}/{}", user_id, file_uuid, filename);

        debug!(
            user_id = %user_id,
            key = %key,
            content_type = %content_type,
            size = data.len(),
            "Uploading file to S3"
        );

        // PUT Object
        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(&key)
            .content_type(content_type)
            .body(ByteStream::from(data))
            .send()
            .await
            .map_err(|e| DomainError::External(format!("S3 upload failed: {}", e)))?;

        info!(key = %key, "File uploaded to S3");

        Ok(key)
    }

    /// S3 からファイルをダウンロードする
    ///
    /// # Arguments
    ///
    /// * `storage_path` - S3 キー（DB に保存された値）
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<u8>)` - ファイルデータ
    /// * `Err(DomainError::External)` - ダウンロード失敗
    /// * `Err(DomainError::NotFound)` - ファイルが存在しない
    pub async fn download(&self, storage_path: &str) -> Result<Vec<u8>, DomainError> {
        debug!(key = %storage_path, "Downloading file from S3");

        // GET Object
        let response = self
            .client
            .get_object()
            .bucket(&self.bucket)
            .key(storage_path)
            .send()
            .await
            .map_err(|e| {
                // NoSuchKey エラーは NotFound として扱う
                let error_str = e.to_string();
                if error_str.contains("NoSuchKey") || error_str.contains("not found") {
                    DomainError::NotFound
                } else {
                    DomainError::External(format!("S3 download failed: {}", e))
                }
            })?;

        // ボディを読み取り
        let data = response
            .body
            .collect()
            .await
            .map_err(|e| DomainError::External(format!("Failed to read S3 response body: {}", e)))?
            .into_bytes()
            .to_vec();

        debug!(key = %storage_path, size = data.len(), "File downloaded from S3");

        Ok(data)
    }

    /// S3 からファイルを削除する
    ///
    /// # Arguments
    ///
    /// * `storage_path` - S3 キー（DB に保存された値）
    ///
    /// # Returns
    ///
    /// * `Ok(())` - 削除成功（ファイルが存在しなくても成功）
    /// * `Err(DomainError::External)` - 削除失敗
    ///
    /// # Note
    ///
    /// S3 の DELETE は冪等（ファイルが存在しなくてもエラーにならない）。
    pub async fn delete(&self, storage_path: &str) -> Result<(), DomainError> {
        debug!(key = %storage_path, "Deleting file from S3");

        // DELETE Object
        self.client
            .delete_object()
            .bucket(&self.bucket)
            .key(storage_path)
            .send()
            .await
            .map_err(|e| DomainError::External(format!("S3 delete failed: {}", e)))?;

        info!(key = %storage_path, "File deleted from S3");

        Ok(())
    }

    /// バケット名を取得する
    pub fn bucket(&self) -> &str {
        &self.bucket
    }
}

// =============================================================================
// StorageOps トレイト実装
// =============================================================================
// Clean Architecture に従い、Domain 層で定義された StorageOps トレイトを実装。
// これにより Application 層は S3StorageService を直接参照せず、
// StorageOps トレイトに依存できる。
// =============================================================================

#[async_trait]
impl StorageOps for S3StorageService {
    async fn upload(
        &self,
        user_id: Uuid,
        filename: &str,
        content_type: &str,
        data: Vec<u8>,
    ) -> Result<String, DomainError> {
        // 既存の upload メソッドに委譲
        S3StorageService::upload(self, user_id, filename, content_type, data).await
    }

    async fn download(&self, storage_path: &str) -> Result<Vec<u8>, DomainError> {
        // 既存の download メソッドに委譲
        S3StorageService::download(self, storage_path).await
    }

    async fn delete(&self, storage_path: &str) -> Result<(), DomainError> {
        // 既存の delete メソッドに委譲
        S3StorageService::delete(self, storage_path).await
    }
}
