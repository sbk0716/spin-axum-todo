// =============================================================================
// presentation/src/routes.rs: ルーティング設定
// =============================================================================
// axum Router を構築し、エンドポイントとハンドラを紐付ける。
// Edge 層からのリクエストを検証し、/api/* ルートを保護する。
//
// ルート構成:
// - /health              - ヘルスチェック（認証不要）
// - /api/auth/register   - ユーザー登録（認証不要）
// - /api/auth/login      - ログイン（認証不要）
// - /api/todos/*         - TODO 操作（Edge 検証 + 認証必須）
// - /api/files/*         - ファイル操作（Edge 検証 + 認証必須）
//
// 統一 CQRS パターン:
// - TW: TodoWriter（Commands 用）
// - TR: TodoReader（Queries 用）
// - C: TodoCacheOps（キャッシュ操作用）
// - UR: UserReader（Queries 用）
// - UW: UserWriter（Commands 用）
// - S: StorageOps（ファイルストレージ操作用）
// =============================================================================

// -----------------------------------------------------------------------------
// 外部クレートのインポート
// -----------------------------------------------------------------------------

// std::sync::Arc: スレッド安全な参照カウントポインタ
// 複数ハンドラ間で状態を共有するために使用
use std::sync::Arc;

// axum: Web フレームワーク
// routing: ルーティングヘルパー（get, post, delete など）
// Router: ルーターオブジェクト
use axum::{
    routing::{delete, get, post},
    Router,
};

// domain: ドメイン層のトレイト
use domain::{StorageOps, TodoCacheOps, TodoReader, TodoWriter, UserReader, UserWriter};

// crate: このクレート内のモジュール
use crate::handlers::{
    batch_create_todos, create_todo, create_todo_with_files, delete_file, delete_todo,
    download_file, get_todo, healthz, list_todos, login, register, update_todo, upload_file,
};
use crate::middleware::with_edge_verify;
use crate::state::AppState;

// =============================================================================
// create_router 関数
// =============================================================================

/// ルーターを作成する
///
/// # Arguments
///
/// * `state` - アプリケーション状態（Arc でラップ）
/// * `edge_secret` - Edge 検証用シークレット（None の場合は検証をスキップ）
///
/// # Returns
///
/// 設定済みの axum Router
///
/// # Type Parameters
///
/// * `TW` - TodoWriter 実装（Commands 用）
/// * `TR` - TodoReader 実装（Queries 用）
/// * `C` - TodoCacheOps 実装（キャッシュ操作用）
/// * `UR` - UserReader 実装（Queries 用）
/// * `UW` - UserWriter 実装（Commands 用）
/// * `S` - StorageOps 実装（ファイルストレージ操作用）
///
/// # Architecture
///
/// ```text
/// /health              - 認証不要（ヘルスチェック）
/// /api/auth/register   - 認証不要（ユーザー登録）
/// /api/auth/login      - 認証不要（ログイン）
/// /api/todos/*         - Edge 検証 + UserContext 必須
/// ```
///
/// # ジェネリクスの制約
///
/// - `TW: TodoWriter + 'static`: TODO 書き込み操作を提供、静的ライフタイム
/// - `TR: TodoReader + 'static`: TODO 読み取り操作を提供、静的ライフタイム
/// - `C: TodoCacheOps + 'static`: キャッシュ操作を提供、静的ライフタイム
/// - `UR: UserReader + 'static`: ユーザー読み取り操作を提供、静的ライフタイム
/// - `UW: UserWriter + 'static`: ユーザー書き込み操作を提供、静的ライフタイム
/// - `S: StorageOps + 'static`: ストレージ操作を提供、静的ライフタイム
///
/// `'static` 制約は、これらの型が任意の長さのライフタイムを持てることを保証。
/// axum のハンドラは 'static 境界を要求するため必要。
pub fn create_router<
    TW: TodoWriter + 'static,
    TR: TodoReader + 'static,
    C: TodoCacheOps + 'static,
    UR: UserReader + 'static,
    UW: UserWriter + 'static,
    S: StorageOps + 'static,
>(
    state: Arc<AppState<TW, TR, C, UR, UW, S>>, // Arc で状態を共有
    edge_secret: Option<String>,                // Option: None なら検証をスキップ
) -> Router {
    // -------------------------------------------------------------------------
    // 認証ルート（Edge 検証不要、パブリック）
    // -------------------------------------------------------------------------
    // ユーザー登録とログインは認証なしでアクセス可能
    let auth_routes = Router::new()
        // POST /api/auth/register - ユーザー登録
        .route("/register", post(register::<TW, TR, C, UR, UW, S>))
        // POST /api/auth/login - ログイン
        .route("/login", post(login::<TW, TR, C, UR, UW, S>));

    // -------------------------------------------------------------------------
    // TODO ルート（Edge 検証が必要）
    // -------------------------------------------------------------------------
    // 認証済みユーザーのみアクセス可能
    let todo_routes = Router::new()
        // GET /api/todos - TODO 一覧取得
        // POST /api/todos - TODO 作成
        .route(
            "/",
            get(list_todos::<TW, TR, C, UR, UW, S>).post(create_todo::<TW, TR, C, UR, UW, S>),
        )
        // GET /api/todos/{id} - TODO 詳細取得
        // PATCH /api/todos/{id} - TODO 更新
        // DELETE /api/todos/{id} - TODO 削除
        .route(
            "/{id}", // {id} はパスパラメータ（UUID）
            get(get_todo::<TW, TR, C, UR, UW, S>)
                .patch(update_todo::<TW, TR, C, UR, UW, S>)
                .delete(delete_todo::<TW, TR, C, UR, UW, S>),
        )
        // POST /api/todos/batch - バッチ作成（トランザクション対応）
        .route("/batch", post(batch_create_todos::<TW, TR, C, UR, UW, S>))
        // POST /api/todos/with-files - TODO + ファイル同時作成
        .route(
            "/with-files",
            post(create_todo_with_files::<TW, TR, C, UR, UW, S>),
        );

    // -------------------------------------------------------------------------
    // ファイルルート（Edge 検証が必要）
    // -------------------------------------------------------------------------
    // ファイルのアップロード、ダウンロード、削除
    let file_routes = Router::new()
        // POST /api/files/upload - ファイルアップロード
        .route("/upload", post(upload_file::<TW, TR, C, UR, UW, S>))
        // GET /api/files/{id}/download - ファイルダウンロード
        .route("/{id}/download", get(download_file::<TW, TR, C, UR, UW, S>))
        // DELETE /api/files/{id} - ファイル削除
        .route("/{id}", delete(delete_file::<TW, TR, C, UR, UW, S>));

    // -------------------------------------------------------------------------
    // Edge 検証ミドルウェアを適用
    // -------------------------------------------------------------------------
    // edge_secret が設定されている場合のみ Edge 検証を有効化
    let (todo_routes, file_routes) = if let Some(secret) = edge_secret {
        // 本番モード: Edge 検証を有効化
        tracing::info!("Edge verification enabled for /api/todos/* and /api/files/* routes");
        (
            with_edge_verify(todo_routes, secret.clone()),
            with_edge_verify(file_routes, secret),
        )
    } else {
        // 開発モード: Edge 検証をスキップ（警告を出力）
        tracing::warn!("Edge verification disabled - running in development mode");
        (todo_routes, file_routes)
    };

    // -------------------------------------------------------------------------
    // ルーターを組み立てて返す
    // -------------------------------------------------------------------------
    Router::new()
        // ヘルスチェック（認証不要、Edge 検証不要）
        // Kubernetes の liveness/readiness probe などで使用
        .route("/health", get(healthz))
        // 認証ルート（認証不要、Edge 検証不要）
        // /api/auth/* にネスト
        .nest("/api/auth", auth_routes)
        // TODO ルート（Edge 検証あり）
        // /api/todos/* にネスト
        .nest("/api/todos", todo_routes)
        // ファイルルート（Edge 検証あり）
        // /api/files/* にネスト
        .nest("/api/files", file_routes)
        // with_state: 状態をルーターに関連付け
        // ハンドラ内で State<Arc<AppState<...>>> として取得可能
        .with_state(state)
}
