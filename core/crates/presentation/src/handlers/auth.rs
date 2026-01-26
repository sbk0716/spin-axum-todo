// =============================================================================
// presentation/src/handlers/auth.rs: 認証ハンドラ
// =============================================================================
// ユーザー登録とログインを処理する。
// これらのエンドポイントは認証不要（パブリック）。
//
// エンドポイント:
// - POST /api/auth/register - ユーザー登録
// - POST /api/auth/login    - ログイン（JWT 発行）
//
// 統一 CQRS パターン:
// - UserReader: ログイン認証（メールでユーザー検索）
// - UserWriter: ユーザー登録（新規ユーザー作成）
//
// セキュリティ:
// - パスワードは bcrypt でハッシュ化（AuthService 内で処理）
// - JWT トークンを発行（有効期間は設定可能）
// =============================================================================

// -----------------------------------------------------------------------------
// 外部クレートのインポート
// -----------------------------------------------------------------------------

// std::sync::Arc: スレッド安全な参照カウントポインタ
// 複数のリクエスト間で AppState を共有するために使用
use std::sync::Arc;

// axum: Web フレームワーク
// extract::State: 状態を抽出するエクストラクタ
// http::StatusCode: HTTP ステータスコード
// response::IntoResponse: レスポンス変換トレイト
// Json: JSON リクエスト/レスポンス
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

// domain: ドメイン層のトレイト（ジェネリクス制約用）
use domain::{TodoCacheOps, TodoReader, TodoWriter, UserReader, UserWriter};

// application: Application 層の DTO
use application::dto::{LoginRequest, RegisterRequest, TokenResponse, UserResponse};

// crate: このクレート内のモジュール
use crate::error::ApiError; // API エラー型
use crate::state::AppState; // アプリケーション状態

// =============================================================================
// register ハンドラ
// =============================================================================

/// ユーザー登録
///
/// POST /api/auth/register
///
/// # Request Body
///
/// ```json
/// {
///     "email": "user@example.com",
///     "password": "password123",
///     "display_name": "User Name"  // Optional
/// }
/// ```
///
/// # Response (201 Created)
///
/// ```json
/// {
///     "id": "uuid",
///     "email": "user@example.com",
///     "display_name": "User Name",
///     "created_at": "2024-01-01T00:00:00Z"
/// }
/// ```
///
/// # Errors
///
/// - 400 Bad Request: バリデーションエラー（メール形式不正、パスワード短すぎなど）
/// - 409 Conflict: メールアドレスが既に使用されている
///
/// # Type Parameters
///
/// ジェネリクスは AppState の型パラメータを引き継ぐ。
/// 実際の実装は依存性注入で決定される。
pub async fn register<
    TW: TodoWriter,  // TODO 書き込み（このハンドラでは未使用）
    TR: TodoReader,  // TODO 読み取り（このハンドラでは未使用）
    C: TodoCacheOps, // キャッシュ操作（このハンドラでは未使用）
    UR: UserReader,  // ユーザー読み取り（重複チェック）
    UW: UserWriter,  // ユーザー書き込み（新規作成）
>(
    // State エクストラクタ: AppState を Arc でラップして取得
    // Arc により複数リクエストで状態を共有可能
    State(state): State<Arc<AppState<TW, TR, C, UR, UW>>>,
    // Json エクストラクタ: リクエストボディを RegisterRequest にデシリアライズ
    // デシリアライズ失敗時は 400 Bad Request が自動返却される
    Json(req): Json<RegisterRequest>,
) -> Result<impl IntoResponse, ApiError> {
    // AuthService の register メソッドを呼び出し
    // - パスワードの bcrypt ハッシュ化
    // - ユーザーの作成（UserWriter 使用）
    // エラー時は `?` で早期リターン（DomainError → ApiError に自動変換）
    let user = state
        .auth_service // AuthService を取得
        .register(&req.email, &req.password, req.display_name) // 登録処理
        .await?; // 非同期待機 + エラー伝播

    // 成功時: 201 Created + ユーザー情報
    // User を UserResponse に変換してレスポンス
    Ok((StatusCode::CREATED, Json(UserResponse::from(user))))
}

// =============================================================================
// login ハンドラ
// =============================================================================

/// ログイン
///
/// POST /api/auth/login
///
/// # Request Body
///
/// ```json
/// {
///     "email": "user@example.com",
///     "password": "password123"
/// }
/// ```
///
/// # Response (200 OK)
///
/// ```json
/// {
///     "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
/// }
/// ```
///
/// # Errors
///
/// - 401 Unauthorized: 認証失敗（メールまたはパスワードが不正）
///
/// # Security
///
/// - パスワードは bcrypt で検証（平文比較ではない）
/// - 成功時に JWT トークンを発行
/// - トークンには user_id と有効期限が含まれる
pub async fn login<
    TW: TodoWriter,  // TODO 書き込み（このハンドラでは未使用）
    TR: TodoReader,  // TODO 読み取り（このハンドラでは未使用）
    C: TodoCacheOps, // キャッシュ操作（このハンドラでは未使用）
    UR: UserReader,  // ユーザー読み取り（メールで検索）
    UW: UserWriter,  // ユーザー書き込み（このハンドラでは未使用）
>(
    // State エクストラクタ: AppState を Arc でラップして取得
    State(state): State<Arc<AppState<TW, TR, C, UR, UW>>>,
    // Json エクストラクタ: リクエストボディを LoginRequest にデシリアライズ
    Json(req): Json<LoginRequest>,
) -> Result<impl IntoResponse, ApiError> {
    // AuthService の login メソッドを呼び出し
    // - メールでユーザー検索（UserReader 使用）
    // - パスワードの bcrypt 検証
    // - JWT トークン生成
    // エラー時は `?` で早期リターン（DomainError → ApiError に自動変換）
    let token = state
        .auth_service // AuthService を取得
        .login(&req.email, &req.password) // ログイン処理
        .await?; // 非同期待機 + エラー伝播

    // 成功時: 200 OK + JWT トークン
    // TokenResponse::new でレスポンス用構造体を作成
    Ok((StatusCode::OK, Json(TokenResponse::new(token))))
}
