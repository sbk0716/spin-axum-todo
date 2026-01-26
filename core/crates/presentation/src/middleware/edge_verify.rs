// =============================================================================
// presentation/src/middleware/edge_verify.rs: Edge 検証ミドルウェア
// =============================================================================
// Defense in Depth: Core 層でも Edge 層からのリクエストを検証する。
// X-Edge-Verified ヘッダーのシークレットを検証。
//
// セキュリティ目的:
// - Core API に直接アクセスする攻撃を防ぐ
// - Edge 層を経由しないリクエストを拒否
// - シークレットの一致で正当な Edge 層からのリクエストを確認
//
// 多層防御（Defense in Depth）:
// 1. Edge 層: JWT 検証、レート制限、WAF
// 2. Core 層: Edge 検証（このミドルウェア）、所有者チェック
//
// 設定:
// - 本番環境: EDGE_VERIFY_SECRET 環境変数で設定
// - 開発環境: 空にして検証をスキップ可能
// =============================================================================

// -----------------------------------------------------------------------------
// 外部クレートのインポート
// -----------------------------------------------------------------------------

// axum: Web フレームワーク
// body::Body: リクエスト/レスポンスボディ
// extract::State: ミドルウェア用状態抽出
// http::Request/StatusCode: HTTP リクエスト/ステータスコード
// middleware::from_fn_with_state: 状態付きミドルウェア構築
// middleware::Next: 次のミドルウェア/ハンドラ
// response::IntoResponse/Response: レスポンス変換
// Router: ルーターオブジェクト
use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::{from_fn_with_state, Next},
    response::{IntoResponse, Response},
    Router,
};

// =============================================================================
// EdgeVerifyState 構造体
// =============================================================================

/// Edge 検証用のシークレットを保持する状態
///
/// ミドルウェアで使用するシークレット値を保持する。
/// `from_fn_with_state` でミドルウェアに注入される。
///
/// # derive マクロ
///
/// - `Clone`: 複製可能（ミドルウェアで必要）
#[derive(Clone)]
pub struct EdgeVerifyState {
    /// Edge 検証用シークレット
    ///
    /// Edge 層が設定する X-Edge-Verified ヘッダーの値と一致する必要がある。
    /// 本番環境では長い乱数文字列を使用。
    pub secret: String,
}

// =============================================================================
// edge_verify ミドルウェア関数
// =============================================================================

/// Edge 検証ミドルウェア
///
/// X-Edge-Verified ヘッダーを検証し、正当な Edge 層からのリクエストであることを確認。
/// これにより、Core 層に直接アクセスしようとする攻撃を防ぐ。
///
/// # Arguments
///
/// * `state` - EdgeVerifyState（シークレットを保持）
/// * `request` - HTTP リクエスト
/// * `next` - 次のミドルウェア/ハンドラ
///
/// # Returns
///
/// * 検証成功: 次のミドルウェア/ハンドラの結果
/// * 検証失敗: 403 Forbidden
///
/// # 処理フロー
///
/// 1. X-Edge-Verified ヘッダーを取得
/// 2. シークレットと比較
/// 3. 一致: next.run() で次に進む
/// 4. 不一致/なし: 403 Forbidden を返す
async fn edge_verify(
    State(state): State<EdgeVerifyState>, // ミドルウェア用状態を抽出
    request: Request<Body>,               // HTTP リクエスト
    next: Next,                           // 次のミドルウェア/ハンドラ
) -> Response {
    // -------------------------------------------------------------------------
    // X-Edge-Verified ヘッダーの取得
    // -------------------------------------------------------------------------
    // Option チェーンで安全にヘッダー値を取得
    let edge_verified = request
        .headers() // HeaderMap を取得
        .get("X-Edge-Verified") // ヘッダー値を取得（Option<&HeaderValue>）
        .and_then(|v| v.to_str().ok()); // &str に変換（無効な UTF-8 なら None）

    // -------------------------------------------------------------------------
    // X-Request-Id ヘッダーの取得（ログ用）
    // -------------------------------------------------------------------------
    // デバッグ用にリクエスト ID を取得
    // 無い場合は "unknown" を使用
    let request_id = request
        .headers() // HeaderMap を取得
        .get("X-Request-Id") // ヘッダー値を取得
        .and_then(|v| v.to_str().ok()) // &str に変換
        .unwrap_or("unknown"); // デフォルト値

    // -------------------------------------------------------------------------
    // 検証と分岐
    // -------------------------------------------------------------------------
    match edge_verified {
        // シークレットが一致する場合: 検証成功
        Some(secret) if secret == state.secret => {
            // 次のミドルウェア/ハンドラに進む
            // next.run(request) は async で実行される
            next.run(request).await
        }

        // シークレットが一致しない場合: 検証失敗
        Some(_) => {
            // 警告ログ: シークレット不一致を記録
            tracing::warn!(
                request_id = %request_id,      // Display フォーマット
                "Edge verification failed: invalid secret"
            );

            // 403 Forbidden を返す
            // タプル (StatusCode, &str) は IntoResponse を実装
            (
                StatusCode::FORBIDDEN,
                "Forbidden: Invalid edge verification",
            )
                .into_response() // Response に変換
        }

        // ヘッダーが無い場合: 検証失敗
        None => {
            // 警告ログ: ヘッダー欠落を記録
            tracing::warn!(
                request_id = %request_id,
                "Edge verification failed: missing X-Edge-Verified header"
            );

            // 403 Forbidden を返す
            (
                StatusCode::FORBIDDEN,
                "Forbidden: Missing edge verification",
            )
                .into_response()
        }
    }
}

// =============================================================================
// with_edge_verify 関数
// =============================================================================

/// Edge 検証ミドルウェアを Router に適用する
///
/// Router::route_layer を使用して、指定された Router の全ルートに
/// Edge 検証ミドルウェアを適用する。
///
/// # Type Parameters
///
/// * `S` - Router の状態型
///   - `Clone`: 複製可能（ミドルウェアで必要）
///   - `Send + Sync`: スレッド安全（非同期処理で必要）
///   - `'static`: 静的ライフタイム（axum で必要）
///
/// # Arguments
///
/// * `router` - ミドルウェアを適用する Router
/// * `secret` - Edge 検証用シークレット
///
/// # Returns
///
/// ミドルウェアが適用された Router
///
/// # 使用例
///
/// ```rust,ignore
/// let protected_routes = Router::new()
///     .route("/api/todos", get(list_todos));
///
/// let protected_routes = with_edge_verify(protected_routes, "my-secret".to_string());
/// ```
pub fn with_edge_verify<S: Clone + Send + Sync + 'static>(
    router: Router<S>, // 適用対象の Router
    secret: String,    // Edge 検証用シークレット
) -> Router<S> {
    // route_layer: Router の全ルートにミドルウェアを適用
    // from_fn_with_state: 状態付きの関数をミドルウェアに変換
    // EdgeVerifyState: シークレットを保持する状態
    // edge_verify: ミドルウェア関数
    router.route_layer(from_fn_with_state(
        EdgeVerifyState { secret }, // シークレットを状態に設定
        edge_verify,                // ミドルウェア関数
    ))
}
