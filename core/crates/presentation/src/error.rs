// =============================================================================
// presentation/src/error.rs: API エラー型
// =============================================================================
// ドメインエラーを HTTP レスポンスに変換する。
// axum の IntoResponse トレイトを実装することで、
// Result<T, ApiError> をハンドラの戻り値として使用できる。
//
// エラーマッピング:
// - DomainError::Validation → 400 Bad Request
// - DomainError::Authentication → 401 Unauthorized
// - DomainError::NotFound → 404 Not Found
// - DomainError::Duplicate → 409 Conflict
// - DomainError::Repository/Cache → 500 Internal Server Error
// =============================================================================

// -----------------------------------------------------------------------------
// 外部クレートのインポート
// -----------------------------------------------------------------------------

// axum: Web フレームワーク
// StatusCode: HTTP ステータスコード
// IntoResponse: レスポンス変換トレイト
// Response: HTTP レスポンス型
// Json: JSON レスポンスヘルパー
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

// domain: ドメイン層のエラー型
use domain::DomainError;

// thiserror: エラー型定義を簡略化するマクロ
// #[error("...")] でエラーメッセージを定義
use thiserror::Error;

// =============================================================================
// ApiError 列挙型
// =============================================================================

/// API エラー型
///
/// HTTP ステータスコードとエラーメッセージを保持する。
/// DomainError から自動変換される。
///
/// # derive マクロの説明
///
/// - `Debug`: `{:?}` フォーマットでデバッグ出力可能に
/// - `Error`: std::error::Error トレイトを自動実装（thiserror）
///
/// # 使用例
///
/// ```rust,ignore
/// // ハンドラ内で Result を返す
/// async fn handler() -> Result<Json<Todo>, ApiError> {
///     let todo = query.execute().await?; // DomainError → ApiError に自動変換
///     Ok(Json(todo))
/// }
/// ```
#[derive(Debug, Error)]
pub enum ApiError {
    /// 400 Bad Request: リクエストが不正
    ///
    /// バリデーションエラーやリクエスト形式エラーに使用。
    #[error("Bad Request: {0}")]
    BadRequest(String),

    /// 401 Unauthorized: 認証エラー
    ///
    /// JWT 無効、パスワード不正、トークン期限切れなどに使用。
    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    /// 404 Not Found: リソースが見つからない
    ///
    /// 存在しない TODO や User へのアクセスに使用。
    #[error("Not Found")]
    NotFound,

    /// 409 Conflict: 重複エラー
    ///
    /// メールアドレス重複などの一意制約違反に使用。
    #[error("Conflict: {0}")]
    Conflict(String),

    /// 500 Internal Server Error: 内部エラー
    ///
    /// DB エラー、キャッシュエラーなど予期しないエラーに使用。
    #[error("Internal Server Error: {0}")]
    Internal(String),
}

// =============================================================================
// From トレイト実装: DomainError → ApiError
// =============================================================================

/// DomainError から ApiError への変換
///
/// `From` トレイトを実装することで、`?` 演算子で自動変換される。
/// これにより、ハンドラ内で `domain_result?` と書くだけで
/// DomainError が ApiError に変換される。
impl From<DomainError> for ApiError {
    /// DomainError を ApiError に変換する
    ///
    /// # Arguments
    ///
    /// * `err` - ドメイン層のエラー
    ///
    /// # Returns
    ///
    /// 対応する ApiError
    fn from(err: DomainError) -> Self {
        // match 式でエラー種別に応じた HTTP エラーに変換
        match err {
            // バリデーションエラー → 400 Bad Request
            DomainError::Validation(msg) => ApiError::BadRequest(msg),

            // 認証エラー → 401 Unauthorized
            DomainError::Authentication(msg) => ApiError::Unauthorized(msg),

            // 見つからない → 404 Not Found
            DomainError::NotFound => ApiError::NotFound,

            // 重複エラー → 409 Conflict
            DomainError::Duplicate(msg) => ApiError::Conflict(msg),

            // DB エラー → 500 Internal Server Error
            DomainError::Repository(msg) => ApiError::Internal(format!("Database error: {}", msg)),

            // キャッシュエラー → 500 Internal Server Error
            DomainError::Cache(msg) => ApiError::Internal(format!("Cache error: {}", msg)),

            // 外部サービスエラー → 500 Internal Server Error
            DomainError::External(msg) => {
                ApiError::Internal(format!("External service error: {}", msg))
            }
        }
    }
}

// =============================================================================
// IntoResponse トレイト実装
// =============================================================================

/// ApiError を HTTP レスポンスに変換
///
/// axum の `IntoResponse` トレイトを実装することで、
/// `Result<T, ApiError>` をハンドラの戻り値として使用できる。
impl IntoResponse for ApiError {
    /// ApiError を HTTP レスポンスに変換する
    ///
    /// # Returns
    ///
    /// HTTP レスポンス（ステータスコード + JSON ボディ）
    fn into_response(self) -> Response {
        // match 式でステータスコードとメッセージを決定
        let (status, message) = match &self {
            // 400 Bad Request
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),

            // 401 Unauthorized
            ApiError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg.clone()),

            // 404 Not Found
            ApiError::NotFound => (StatusCode::NOT_FOUND, "not found".to_string()),

            // 409 Conflict
            ApiError::Conflict(msg) => (StatusCode::CONFLICT, msg.clone()),

            // 500 Internal Server Error
            ApiError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
        };

        // JSON 形式でエラーレスポンスを返す
        // {"error": "エラーメッセージ"}
        // serde_json::json! マクロで JSON オブジェクトを構築
        (status, Json(serde_json::json!({"error": message}))).into_response()
    }
}
