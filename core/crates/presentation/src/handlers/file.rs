// =============================================================================
// presentation/src/handlers/file.rs: ファイルハンドラ
// =============================================================================
// ファイルのアップロード、ダウンロード、削除を処理する。
// Clean Architecture に従い、Application 層のユースケースを経由して操作を行う。
//
// エンドポイント:
// - POST /api/files/upload      - ファイルをアップロード
// - GET /api/files/:id/download - ファイルをダウンロード
// - DELETE /api/files/:id       - ファイルを削除
//
// セキュリティ:
// - Edge 検証 + 認証が必要
// - ファイル名、サイズ、MIME タイプのバリデーション（Application 層で実行）
// - 所有者確認（Application 層で実行）
// =============================================================================

// -----------------------------------------------------------------------------
// 外部クレートのインポート
// -----------------------------------------------------------------------------

// axum: Web フレームワーク
use axum::{
    body::Body,
    extract::{Multipart, Path, State},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    Json,
};

// serde: シリアライズ
use serde::Serialize;

// uuid: 一意識別子
use uuid::Uuid;

// domain: ドメイン層の型とトレイト
use domain::{StorageOps, TodoCacheOps, TodoReader, TodoWriter, UserReader, UserWriter};

// crate: このクレート内のモジュール
use crate::error::ApiError;
use crate::middleware::UserContext;
use crate::state::AppState;

// =============================================================================
// FileUploadResponse
// =============================================================================

/// ファイルアップロードレスポンス
///
/// アップロード成功時に返される情報。
/// クライアントはこの情報を使って TODO + ファイル同時作成を行う。
#[derive(Serialize)]
pub struct FileUploadResponse {
    /// S3 キー（storage_path）
    pub storage_path: String,
    /// 元のファイル名
    pub filename: String,
    /// MIME タイプ
    pub mime_type: String,
    /// ファイルサイズ（バイト）
    pub size_bytes: i64,
}

// =============================================================================
// upload_file ハンドラ
// =============================================================================

/// ファイルをアップロード
///
/// POST /api/files/upload
///
/// # Request
///
/// Content-Type: multipart/form-data
///
/// ```text
/// --boundary
/// Content-Disposition: form-data; name="file"; filename="image.png"
/// Content-Type: image/png
///
/// <binary data>
/// --boundary--
/// ```
///
/// # Response (201 Created)
///
/// ```json
/// {
///     "storage_path": "users/{user_id}/files/{uuid}/image.png",
///     "filename": "image.png",
///     "mime_type": "image/png",
///     "size_bytes": 12345
/// }
/// ```
///
/// # Errors
///
/// - 400 Bad Request: バリデーションエラー（ファイル名不正、サイズ超過など）
/// - 500 Internal Server Error: ストレージアップロード失敗
///
/// # Clean Architecture
///
/// Handler → UploadFileCommand → StorageOps
/// バリデーションは Application 層（UploadFileCommand）で実行される。
pub async fn upload_file<
    TW: TodoWriter + 'static,
    TR: TodoReader + 'static,
    C: TodoCacheOps + 'static,
    UR: UserReader + 'static,
    UW: UserWriter + 'static,
    S: StorageOps + 'static,
>(
    // State エクストラクタ: AppState を取得（axum 推奨）
    // axum が各リクエストで state.clone() を呼び出す
    State(state): State<AppState<TW, TR, C, UR, UW, S>>,
    user: UserContext,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, ApiError> {
    // multipart からファイルを取得（最初のフィールドのみ処理）
    let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| ApiError::BadRequest(format!("Failed to read multipart field: {}", e)))?
    else {
        // ファイルが提供されなかった場合
        return Err(ApiError::BadRequest("No file provided".to_string()));
    };

    // ファイル名を取得（必須）
    let filename = field
        .file_name()
        .ok_or_else(|| ApiError::BadRequest("Missing filename".to_string()))?
        .to_string();

    // Content-Type を取得（デフォルト: application/octet-stream）
    let content_type = field
        .content_type()
        .unwrap_or("application/octet-stream")
        .to_string();

    // ファイルデータを読み取り
    let data = field
        .bytes()
        .await
        .map_err(|e| ApiError::BadRequest(format!("Failed to read file data: {}", e)))?;

    // Application 層のコマンドを呼び出し
    // バリデーション（ファイル名、サイズ、MIME タイプ）は UploadFileCommand 内で実行
    let result = state
        .upload_file
        .execute(user.user_id, &filename, &content_type, data.to_vec())
        .await?;

    // レスポンスを返す
    Ok((
        StatusCode::CREATED,
        Json(FileUploadResponse {
            storage_path: result.storage_path,
            filename: result.filename,
            mime_type: result.mime_type,
            size_bytes: result.size_bytes,
        }),
    ))
}

// =============================================================================
// download_file ハンドラ
// =============================================================================

/// ファイルをダウンロード
///
/// GET /api/files/:id/download
///
/// # Path Parameters
///
/// - `id` - ファイル ID（UUID）
///
/// # Response
///
/// - 200 OK: ファイルデータ（Content-Type: 保存時の MIME タイプ）
/// - 404 Not Found: ファイルが見つからない、または所有者ではない
///
/// # Security
///
/// ファイル所有者の確認は Application 層（DownloadFileQuery）で行われる。
/// ファイルが紐付く TODO の所有者と一致する必要がある。
///
/// # Clean Architecture
///
/// Handler → DownloadFileQuery → FileReader + TodoReader + StorageOps
pub async fn download_file<
    TW: TodoWriter + 'static,
    TR: TodoReader + 'static,
    C: TodoCacheOps + 'static,
    UR: UserReader + 'static,
    UW: UserWriter + 'static,
    S: StorageOps + 'static,
>(
    // State エクストラクタ: AppState を取得（axum 推奨）
    // axum が各リクエストで state.clone() を呼び出す
    State(state): State<AppState<TW, TR, C, UR, UW, S>>,
    user: UserContext,
    Path(id): Path<Uuid>,
) -> Result<Response, ApiError> {
    // Application 層のクエリを呼び出し
    // ファイルメタデータ取得、所有者確認、ストレージダウンロードは
    // DownloadFileQuery 内で実行
    let result = state.download_file.execute(id, user.user_id).await?;

    // レスポンスを構築
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, &result.mime_type)
        .header(
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}\"", result.filename),
        )
        .body(Body::from(result.data))
        .map_err(|e| ApiError::Internal(format!("Failed to build response: {}", e)))?;

    Ok(response)
}

// =============================================================================
// delete_file ハンドラ
// =============================================================================

/// ファイルを削除
///
/// DELETE /api/files/:id
///
/// # Path Parameters
///
/// - `id` - ファイル ID（UUID）
///
/// # Response
///
/// - 204 No Content: 削除成功
/// - 404 Not Found: ファイルが見つからない、または所有者ではない
///
/// # Security
///
/// ファイル所有者の確認は Application 層（DeleteFileCommand）で行われる。
///
/// # Clean Architecture
///
/// Handler → DeleteFileCommand → FileReader + FileWriter + TodoReader + StorageOps
pub async fn delete_file<
    TW: TodoWriter + 'static,
    TR: TodoReader + 'static,
    C: TodoCacheOps + 'static,
    UR: UserReader + 'static,
    UW: UserWriter + 'static,
    S: StorageOps + 'static,
>(
    // State エクストラクタ: AppState を取得（axum 推奨）
    // axum が各リクエストで state.clone() を呼び出す
    State(state): State<AppState<TW, TR, C, UR, UW, S>>,
    user: UserContext,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    // Application 層のコマンドを呼び出し
    // ファイルメタデータ取得、所有者確認、ストレージ削除、DB 削除は
    // DeleteFileCommand 内で実行
    state.delete_file.execute(id, user.user_id).await?;

    Ok(StatusCode::NO_CONTENT)
}
