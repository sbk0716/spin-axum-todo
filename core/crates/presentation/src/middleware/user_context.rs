// =============================================================================
// presentation/src/middleware/user_context.rs: UserContext エクストラクタ
// =============================================================================
// Edge 層から転送された X-User-Id ヘッダーからユーザー情報を抽出。
// ハンドラで認証済みユーザーの情報を取得するために使用。
//
// 認証フロー:
// 1. クライアントが JWT トークン付きでリクエスト
// 2. Edge 層が JWT を検証し、user_id を X-User-Id ヘッダーに設定
// 3. Core 層（このモジュール）が X-User-Id を抽出
// 4. ハンドラで UserContext として利用可能
//
// セキュリティ:
// - X-User-Id は Edge 層でのみ設定される想定
// - Edge 検証ミドルウェアと組み合わせて使用
// - 外部から直接 X-User-Id を設定しても Edge 検証で弾かれる
// =============================================================================

// -----------------------------------------------------------------------------
// 外部クレートのインポート
// -----------------------------------------------------------------------------

// axum: Web フレームワーク
// extract::FromRequestParts: カスタムエクストラクタを定義するトレイト
// http::request::Parts: リクエストのヘッダー部分
// http::StatusCode: HTTP ステータスコード
use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};

// uuid: 一意識別子
use uuid::Uuid;

// =============================================================================
// UserContext 構造体
// =============================================================================

/// ユーザーコンテキスト
///
/// Edge 層で認証された後、X-User-Id ヘッダーから抽出されたユーザー情報。
/// ハンドラで `UserContext` を引数に取ることで、認証済みユーザーの ID を取得できる。
///
/// # derive マクロ
///
/// - `Debug`: デバッグ出力可能（ログ用）
/// - `Clone`: 複製可能（ハンドラ間で共有可能）
///
/// # 使用例
///
/// ```rust,ignore
/// // ハンドラの引数として使用
/// async fn handler(user: UserContext, ...) -> impl IntoResponse {
///     let user_id = user.user_id; // 認証済みユーザー ID
///     // user_id を使って TODO をフィルタリング
/// }
/// ```
///
/// # セキュリティ
///
/// このエクストラクタは X-User-Id ヘッダーが存在し、有効な UUID である
/// ことを検証する。ヘッダーが無い場合や不正な形式の場合は 401 Unauthorized
/// を返す。
#[derive(Debug, Clone)]
pub struct UserContext {
    /// 認証済みユーザーの ID（UUID）
    ///
    /// Edge 層の JWT 検証から取得した user_id。
    /// この ID を使用して、ユーザーは自分の TODO のみにアクセス可能。
    pub user_id: Uuid,

    /// リクエスト追跡用 ID（オプショナル）
    ///
    /// X-Request-Id ヘッダーから取得。
    /// 分散トレーシングやデバッグに使用。
    pub request_id: Option<String>,
}

// =============================================================================
// FromRequestParts トレイト実装
// =============================================================================

/// FromRequestParts トレイトの実装
///
/// axum のエクストラクタシステムにより、ハンドラの引数として
/// UserContext を指定するだけで自動的にこの実装が呼び出される。
///
/// # 型パラメータ
///
/// - `S`: axum の State 型。この実装では使用しないが、
///   `Send + Sync` 制約が必要。
///
/// # 非同期
///
/// Rust 2024 Edition 以降、トレイト内の async fn がサポートされた。
/// これにより async_trait マクロなしで async fn を定義可能。
impl<S> FromRequestParts<S> for UserContext
where
    S: Send + Sync, // 状態はスレッド安全である必要がある
{
    /// エクストラクション失敗時のエラー型
    ///
    /// (StatusCode, &'static str) のタプルで、
    /// ステータスコードとエラーメッセージを返す。
    type Rejection = (StatusCode, &'static str);

    /// リクエストからユーザーコンテキストを抽出する
    ///
    /// # Arguments
    ///
    /// * `parts` - HTTP リクエストのヘッダー部分（可変参照）
    /// * `_state` - アプリケーション状態（未使用）
    ///
    /// # Returns
    ///
    /// * `Ok(UserContext)` - 抽出成功
    /// * `Err((StatusCode, &str))` - 抽出失敗（401 Unauthorized）
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // ---------------------------------------------------------------------
        // X-User-Id ヘッダーの抽出と検証
        // ---------------------------------------------------------------------
        // Option チェーンで安全にヘッダー値を取得
        let user_id = parts
            .headers // HeaderMap を取得
            .get("X-User-Id") // ヘッダー値を取得（Option<&HeaderValue>）
            .and_then(|v| v.to_str().ok()) // &str に変換（無効な UTF-8 なら None）
            .and_then(|s| Uuid::parse_str(s).ok()); // UUID にパース（無効な形式なら None）

        // ---------------------------------------------------------------------
        // X-Request-Id ヘッダーの抽出（オプショナル）
        // ---------------------------------------------------------------------
        // 分散トレーシング用の ID。無くても OK。
        let request_id = parts
            .headers // HeaderMap を取得
            .get("X-Request-Id") // ヘッダー値を取得
            .and_then(|v| v.to_str().ok()) // &str に変換
            .map(|s| s.to_string()); // String に変換

        // ---------------------------------------------------------------------
        // 結果の判定
        // ---------------------------------------------------------------------
        match user_id {
            // 有効な user_id が取得できた場合
            Some(user_id) => {
                // 構造化ログ: ユーザー ID とリクエスト ID を記録
                tracing::debug!(
                    user_id = %user_id,        // Display フォーマットで出力
                    request_id = ?request_id,  // Debug フォーマットで出力
                    "User context extracted"
                );

                // UserContext を構築して返す
                Ok(UserContext {
                    user_id,
                    request_id,
                })
            }
            // user_id が取得できなかった場合
            None => {
                // 警告ログ: 認証失敗を記録
                tracing::warn!(
                    request_id = ?request_id,
                    "Missing or invalid X-User-Id header"
                );

                // 401 Unauthorized を返す
                Err((
                    StatusCode::UNAUTHORIZED,
                    "Missing or invalid user identification",
                ))
            }
        }
    }
}
