//! # ゲートウェイコンポーネント
//!
//! このコンポーネントは HTTP リクエストのエントリーポイントとして機能し、
//! 以下の責務を担います：
//!
//! ## 責務
//! 1. HTTP リクエストの受信
//! 2. auth コンポーネントを呼び出して JWT 認証を実行
//! 3. 認証成功時、コア層（axum）へリクエストをプロキシ
//! 4. 認証失敗時、401 Unauthorized レスポンスを返却
//!
//! ## アーキテクチャ
//! ```text
//! クライアント → [gateway] → [auth] (WIT)
//!                    ↓
//!              [コア層 axum]
//! ```

// =============================================================================
// 外部クレートのインポート
// =============================================================================

// JSON シリアライズ用
// Serialize: Rust 構造体から JSON への変換を自動生成
use serde::Serialize;

// UUID 生成用（リクエストID）
use uuid::Uuid;

// Spin SDK の HTTP 関連モジュール
// IntoResponse: レスポンスに変換可能な型を示すトレイト
// Method: HTTP メソッド（GET, POST など）
// Request: HTTP リクエストを表す構造体
// Response: HTTP レスポンスを表す構造体
use spin_sdk::http::{IntoResponse, Method, Request, Response};

// Spin の HTTP コンポーネント属性マクロ
// この属性を付けた関数が HTTP リクエストのハンドラーになる
use spin_sdk::http_component;

// =============================================================================
// WIT バインディングの生成
// =============================================================================

// wit_bindgen マクロで WIT インターフェースから Rust コードを自動生成
// world: このコンポーネントが実装する world 名（gateway-world）
// path: WIT ファイルが配置されているディレクトリへの相対パス
//
// gateway-world は authenticator インターフェースをインポートするため、
// auth コンポーネントの関数を呼び出すためのコードが生成される
wit_bindgen::generate!({
    world: "gateway-world",
    path: "../wit",
});

// WIT で定義された認証インターフェースをインポート
// verify_token: JWT トークンを検証する関数
// AuthResult: 認証結果を表す構造体
// これらは auth コンポーネントからエクスポートされ、
// Spin のコンポーネント合成機能により gateway から呼び出し可能
use demo::auth::authenticator::{verify_token, AuthResult};

// =============================================================================
// 定数定義
// =============================================================================

/// コア層（axum サーバー）の URL
///
/// エッジ層で認証が成功した後、このURLにリクエストをプロキシします。
/// 本番環境では環境変数や Spin の変数機能で設定することを推奨。
const CORE_URL: &str = "http://localhost:3001";

/// Edge 検証用シークレット
///
/// Core 層がリクエストが正当な Edge 層から来たことを検証するために使用。
/// 本番環境では必ず環境変数から取得すること。
const EDGE_SECRET: &str = "super-secret-edge-key";

/// 認証不要のパブリックパス
///
/// これらのパスは JWT 認証なしでコア層にプロキシされる。
const PUBLIC_PATHS: &[&str] = &["/api/auth/register", "/api/auth/login"];

// =============================================================================
// 構造体定義
// =============================================================================

/// エラーレスポンスのボディを表す構造体
///
/// 認証失敗やエラー時に返却する JSON レスポンスの形式を定義。
/// 例: {"error": "Missing token"}
#[derive(Serialize)]
struct ErrorResponse {
    /// エラーメッセージ
    error: String,
}

// =============================================================================
// HTTP リクエストハンドラー
// =============================================================================

/// HTTP リクエストを処理するメインハンドラー
///
/// Spin ランタイムから HTTP リクエストを受け取り、適切なレスポンスを返します。
///
/// # 処理フロー
/// 1. リクエストのパスをログ出力
/// 2. /api/* パスの場合、JWT 認証を実行
///    - 認証成功: コア層にプロキシ
///    - 認証失敗: 401 レスポンスを返却
/// 3. /api/* 以外のパスは 401 を返却
///
/// # 引数
/// * `req` - Spin SDK の Request 構造体
///
/// # 戻り値
/// * `impl IntoResponse` - HTTP レスポンスに変換可能な型
#[http_component]
async fn handle_request(req: Request) -> impl IntoResponse {
    // リクエストのパスを取得（例: "/api/users"）
    let path = req.path();

    // HTTP メソッドを文字列に変換（例: "GET", "POST"）
    let method = req.method().to_string();

    // デバッグ用ログ出力
    // Spin のログは .spin/logs/ ディレクトリに保存される
    println!("[Gateway] {} {}", method, path);

    // -------------------------------------------------------------------------
    // /health パスの処理（認証不要）
    // -------------------------------------------------------------------------
    // ヘルスチェックエンドポイントは認証をバイパスしてコア層に転送
    if path == "/health" {
        return proxy_health_check().await;
    }

    // -------------------------------------------------------------------------
    // パブリックパスの処理（認証不要）
    // -------------------------------------------------------------------------
    // 認証・登録エンドポイントは認証なしでコア層に転送
    if is_public_path(path) {
        println!("[Gateway] Public path, bypassing auth: {}", path);
        return proxy_to_core_public(&req).await;
    }

    // -------------------------------------------------------------------------
    // /api/* パスの処理（認証が必要）
    // -------------------------------------------------------------------------
    if path.starts_with("/api/") {
        // Authorization ヘッダーから Bearer トークンを抽出
        let token = extract_bearer_token(&req);

        // auth コンポーネントの verify_token 関数を呼び出し
        // WIT コンポーネント合成により、直接関数呼び出しが可能
        // 実行時には Spin がコンポーネント間の通信を処理
        let auth_result: AuthResult = verify_token(&token);

        // 認証失敗の場合
        if !auth_result.authenticated {
            // エラーメッセージを取得（なければデフォルトメッセージ）
            let error_msg = auth_result
                .error
                .unwrap_or_else(|| "Unauthorized".to_string());

            // 認証失敗をログ出力
            println!("[Gateway] Auth failed: {}", error_msg);

            // エラーレスポンスの JSON ボディを構築
            let body = serde_json::to_string(&ErrorResponse { error: error_msg }).unwrap();

            // 401 Unauthorized レスポンスを返却
            return Response::builder()
                .status(401) // HTTP ステータスコード 401
                .header("Content-Type", "application/json") // JSON レスポンス
                .body(body) // レスポンスボディ
                .build();
        }

        // 認証成功の場合
        // ユーザーIDを取得（なければ "unknown"）
        let user_id = auth_result
            .user_id
            .unwrap_or_else(|| "unknown".to_string());

        // 認証成功をログ出力
        println!("[Gateway] Auth success: user_id={}", user_id);

        // コア層にリクエストをプロキシ
        // await: 非同期処理の完了を待機
        return proxy_to_core(&req, &user_id).await;
    }

    // -------------------------------------------------------------------------
    // /api/* 以外のパスの処理
    // -------------------------------------------------------------------------
    // このゲートウェイは API リクエストのみを処理するため、
    // それ以外のパスは 401 エラーを返す

    // エラーレスポンスの JSON ボディを構築
    let body = serde_json::to_string(&ErrorResponse {
        error: "Unauthorized: Only /api/* paths are allowed".to_string(),
    })
    .unwrap();

    // 401 Unauthorized レスポンスを返却
    Response::builder()
        .status(401)
        .header("Content-Type", "application/json")
        .body(body)
        .build()
}

// =============================================================================
// ヘルパー関数
// =============================================================================

/// ヘルスチェックをコア層にプロキシする
///
/// 認証不要でコア層の /health エンドポイントにリクエストを転送。
/// ロードバランサーや Kubernetes のヘルスプローブ用。
///
/// # 戻り値
/// * `Response` - コア層からのヘルスチェックレスポンス
async fn proxy_health_check() -> Response {
    let url = format!("{}/health", CORE_URL);
    println!("[Gateway] Health check -> {}", url);

    let outbound_req = Request::builder()
        .method(Method::Get)
        .uri(&url)
        .build();

    match spin_sdk::http::send::<_, Response>(outbound_req).await {
        Ok(response) => {
            let status = *response.status();
            let body = response.into_body();
            Response::builder()
                .status(status)
                .header("Content-Type", "application/json")
                .body(body)
                .build()
        }
        Err(e) => {
            let body = serde_json::to_string(&ErrorResponse {
                error: format!("Health check failed: {}", e),
            })
            .unwrap();
            Response::builder()
                .status(503)
                .header("Content-Type", "application/json")
                .body(body)
                .build()
        }
    }
}

/// Authorization ヘッダーから Bearer トークンを抽出する
///
/// HTTP Authorization ヘッダーの形式: "Bearer <token>"
/// この関数は "Bearer " プレフィックスを除去し、トークン部分のみを返します。
///
/// # 引数
/// * `req` - HTTP リクエスト
///
/// # 戻り値
/// * `String` - トークン文字列（見つからない場合は空文字列）
///
/// # 例
/// ```text
/// Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
/// → "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
/// ```
fn extract_bearer_token(req: &Request) -> String {
    req.header("Authorization") // Authorization ヘッダーを取得（Option<HeaderValue>）
        .and_then(|h| h.as_str()) // 文字列スライスに変換（Option<&str>）
        .and_then(|auth| {
            // "Bearer " で始まる場合、その後ろの部分を取得
            if auth.starts_with("Bearer ") {
                // 7文字目以降を取得（"Bearer " は7文字）
                Some(auth[7..].to_string())
            } else {
                // "Bearer " で始まらない場合は None
                None
            }
        })
        .unwrap_or_default() // None の場合は空文字列を返す
}

/// コア層（axum サーバー）にリクエストをプロキシする
///
/// 認証成功後、元のリクエストをコア層に転送します。
/// 転送時に保持される情報：
/// - HTTP メソッド（GET, POST, PATCH, DELETE）
/// - リクエストボディ
/// - クエリパラメータ（例: `?completed=true`）
/// - Content-Type ヘッダー
///
/// 追加されるヘッダー：
/// - X-User-Id: 認証済みユーザーID
/// - X-Request-Id: リクエスト追跡用 UUID
/// - X-Edge-Verified: Edge 検証用シークレット（Defense in Depth）
///
/// # 引数
/// * `req` - 元の HTTP リクエスト
/// * `user_id` - 認証で取得したユーザーID
///
/// # 戻り値
/// * `Response` - コア層からのレスポンス、またはエラーレスポンス
///
/// # エラー処理
/// コア層への接続に失敗した場合、502 Bad Gateway を返します。
async fn proxy_to_core(req: &Request, user_id: &str) -> Response {
    // 元のリクエストのパスを取得
    let path = req.path();

    // クエリパラメータを取得
    let query = req.query();

    // コア層への完全な URL を構築（クエリパラメータを含む）
    // 例: "http://localhost:3001" + "/api/todos" + "?completed=false"
    let url = if query.is_empty() {
        format!("{}{}", CORE_URL, path)
    } else {
        format!("{}{}?{}", CORE_URL, path, query)
    };

    // リクエスト追跡用 UUID を生成
    let request_id = Uuid::new_v4().to_string();

    // プロキシ先をログ出力
    println!(
        "[Gateway] Proxying {} {} -> {} (request_id={})",
        req.method(),
        path,
        url,
        request_id
    );

    // 元のリクエストの Content-Type を取得（なければ application/json）
    let content_type = req
        .header("Content-Type")
        .and_then(|h| h.as_str())
        .unwrap_or("application/json");

    // 元のリクエストボディを取得
    let body = req.body().to_vec();

    // コア層へのリクエストを構築
    // 全 HTTP メソッドとリクエストボディを転送
    let outbound_req = Request::builder()
        .method(req.method().clone()) // 元のメソッドを維持（GET, POST, PATCH, DELETE）
        .uri(&url) // プロキシ先 URL
        .header("Content-Type", content_type) // Content-Type を転送
        .header("X-User-Id", user_id) // 認証済みユーザーID
        .header("X-Request-Id", &request_id) // リクエスト追跡用
        .header("X-Edge-Verified", EDGE_SECRET) // Edge 検証用（Defense in Depth）
        .body(body) // リクエストボディを転送
        .build();

    // Spin の outbound HTTP 機能でリクエストを送信
    // send::<_, Response>: 入力は任意、出力は Response 型を期待
    // await: 非同期処理の完了を待機
    match spin_sdk::http::send::<_, Response>(outbound_req).await {
        // 送信成功
        Ok(response) => {
            // レスポンスのステータスコードを取得
            // status() は &u16 を返すため、* でデリファレンス
            let status = *response.status();

            // レスポンスボディを取得
            // into_body() は response を消費して body を返す
            let body = response.into_body();

            // クライアントへのレスポンスを構築
            // X-Request-Id をレスポンスにも付与してトレーサビリティを確保
            Response::builder()
                .status(status) // コア層のステータスコードをそのまま使用
                .header("Content-Type", "application/json")
                .header("X-Request-Id", &request_id) // レスポンスにも付与
                .body(body) // コア層のボディをそのまま使用
                .build()
        }
        // 送信失敗（ネットワークエラー、タイムアウトなど）
        Err(e) => {
            // エラーをログ出力
            println!("[Gateway] Proxy error (request_id={}): {}", request_id, e);

            // エラーレスポンスの JSON ボディを構築
            let body = serde_json::to_string(&ErrorResponse {
                error: format!("Proxy error: {}", e),
            })
            .unwrap();

            // 502 Bad Gateway を返却
            // プロキシ先との通信に問題があったことを示す
            Response::builder()
                .status(502)
                .header("Content-Type", "application/json")
                .header("X-Request-Id", &request_id)
                .body(body)
                .build()
        }
    }
}

/// パブリックパス（認証不要）かどうかを判定
///
/// # 引数
/// * `path` - リクエストパス
///
/// # 戻り値
/// * `bool` - パブリックパスの場合 true
fn is_public_path(path: &str) -> bool {
    PUBLIC_PATHS.iter().any(|&p| path == p)
}

/// パブリックパス用のコア層プロキシ
///
/// 認証不要のリクエストをコア層に転送します。
/// X-User-Id ヘッダーは付与しません。
///
/// # 引数
/// * `req` - 元の HTTP リクエスト
///
/// # 戻り値
/// * `Response` - コア層からのレスポンス、またはエラーレスポンス
async fn proxy_to_core_public(req: &Request) -> Response {
    let path = req.path();
    let query = req.query();

    let url = if query.is_empty() {
        format!("{}{}", CORE_URL, path)
    } else {
        format!("{}{}?{}", CORE_URL, path, query)
    };

    let request_id = Uuid::new_v4().to_string();

    println!(
        "[Gateway] Proxying public {} {} -> {} (request_id={})",
        req.method(),
        path,
        url,
        request_id
    );

    let content_type = req
        .header("Content-Type")
        .and_then(|h| h.as_str())
        .unwrap_or("application/json");

    let body = req.body().to_vec();

    // パブリックリクエストには X-User-Id を付与しない
    let outbound_req = Request::builder()
        .method(req.method().clone())
        .uri(&url)
        .header("Content-Type", content_type)
        .header("X-Request-Id", &request_id)
        .body(body)
        .build();

    match spin_sdk::http::send::<_, Response>(outbound_req).await {
        Ok(response) => {
            let status = *response.status();
            let body = response.into_body();

            Response::builder()
                .status(status)
                .header("Content-Type", "application/json")
                .header("X-Request-Id", &request_id)
                .body(body)
                .build()
        }
        Err(e) => {
            println!("[Gateway] Proxy error (request_id={}): {}", request_id, e);

            let body = serde_json::to_string(&ErrorResponse {
                error: format!("Proxy error: {}", e),
            })
            .unwrap();

            Response::builder()
                .status(502)
                .header("Content-Type", "application/json")
                .header("X-Request-Id", &request_id)
                .body(body)
                .build()
        }
    }
}
