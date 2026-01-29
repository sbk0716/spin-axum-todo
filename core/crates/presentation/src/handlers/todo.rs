// =============================================================================
// presentation/src/handlers/todo.rs: TODO ハンドラ
// =============================================================================
// TODO 関連の HTTP リクエストを処理し、Commands/Queries を呼び出す。
// ハンドラは「薄く」保ち、ビジネスロジックは application 層に委譲する。
//
// エンドポイント:
// - GET    /api/todos       - 一覧取得（フィルタ可能）
// - POST   /api/todos       - 作成
// - GET    /api/todos/{id}  - 詳細取得
// - PATCH  /api/todos/{id}  - 更新
// - DELETE /api/todos/{id}  - 削除
//
// 統一 CQRS パターン:
// - 状態変更操作（POST, PATCH, DELETE）: Commands を使用（+ キャッシュ操作）
// - 参照操作（GET）: Queries を使用
//
// マルチテナント対応:
// 全てのハンドラは UserContext から user_id を取得し、
// ユーザーは自分の TODO のみにアクセス可能。
// =============================================================================

// -----------------------------------------------------------------------------
// 外部クレートのインポート
// -----------------------------------------------------------------------------

// axum: Web フレームワーク
// Path: URL パスパラメータの抽出（例: /todos/{id} の id）
// Query: クエリパラメータの抽出（例: ?completed=true）
// State: アプリケーション状態の抽出
// StatusCode: HTTP ステータスコード
// IntoResponse: レスポンス変換トレイト
// Json: JSON リクエスト/レスポンス
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

// domain: ドメイン層の型とトレイト
// TodoCacheOps: キャッシュ操作トレイト
// TodoFilter: TODO 検索フィルタ（ビルダーパターン）
// TodoReader/Writer: TODO 読み書きトレイト
// UserReader/Writer: ユーザー読み書きトレイト
use domain::{
    StorageOps, TodoCacheOps, TodoFilter, TodoReader, TodoWriter, UserReader, UserWriter,
};

// serde: シリアライズ/デシリアライズ
// Deserialize: JSON → 構造体 変換
use serde::Deserialize;

// uuid: 一意識別子
use uuid::Uuid;

// application: Application 層の DTO
use application::dto::{CreateTodoDto, UpdateTodoDto};

// crate: このクレート内のモジュール
use crate::error::ApiError; // API エラー型
use crate::middleware::UserContext; // 認証済みユーザー情報
use crate::state::AppState; // アプリケーション状態

// =============================================================================
// リクエスト構造体定義
// =============================================================================

/// 一覧取得のクエリパラメータ
///
/// GET /api/todos?completed=true のようなクエリパラメータを受け取る。
///
/// # derive マクロ
///
/// - `Debug`: デバッグ出力可能（ログ用）
/// - `Deserialize`: URL クエリパラメータからデシリアライズ可能
#[derive(Debug, Deserialize)]
pub struct ListQuery {
    /// 完了状態でフィルタリング（任意）
    ///
    /// - `None`: フィルタなし（全件取得）
    /// - `Some(true)`: 完了済みのみ
    /// - `Some(false)`: 未完了のみ
    pub completed: Option<bool>,
}

/// TODO 作成リクエスト
///
/// POST /api/todos のリクエストボディを受け取る。
///
/// # derive マクロ
///
/// - `Debug`: デバッグ出力可能（ログ用）
/// - `Deserialize`: JSON からデシリアライズ可能
#[derive(Debug, Deserialize)]
pub struct CreateTodoRequest {
    /// TODO のタイトル（必須、1-200文字）
    pub title: String,
    /// TODO の詳細説明（任意）
    pub description: Option<String>,
}

/// TODO 更新リクエスト
///
/// PATCH /api/todos/{id} のリクエストボディを受け取る。
/// 全てのフィールドが Option で、指定されたフィールドのみ更新される。
///
/// # derive マクロ
///
/// - `Debug`: デバッグ出力可能（ログ用）
/// - `Deserialize`: JSON からデシリアライズ可能
#[derive(Debug, Deserialize)]
pub struct UpdateTodoRequest {
    /// 新しいタイトル（任意、1-200文字）
    pub title: Option<String>,
    /// 新しい説明（任意）
    pub description: Option<String>,
    /// 完了状態（任意）
    pub completed: Option<bool>,
}

// =============================================================================
// list_todos ハンドラ
// =============================================================================

/// TODO 一覧取得
///
/// GET /api/todos
/// GET /api/todos?completed=true
/// GET /api/todos?completed=false
///
/// # Response (200 OK)
///
/// ```json
/// [
///     {
///         "id": "uuid",
///         "user_id": "uuid",
///         "title": "タイトル",
///         "description": "説明",
///         "completed": false,
///         "created_at": "2024-01-01T00:00:00Z",
///         "updated_at": "2024-01-01T00:00:00Z"
///     }
/// ]
/// ```
///
/// # Note
///
/// 一覧取得はキャッシュしない（フィルタ条件が多様なため）。
pub async fn list_todos<
    TW: TodoWriter,  // TODO 書き込み（このハンドラでは未使用）
    TR: TodoReader,  // TODO 読み取り（一覧取得）
    C: TodoCacheOps, // キャッシュ操作（このハンドラでは未使用）
    UR: UserReader,  // ユーザー読み取り（未使用）
    UW: UserWriter,  // ユーザー書き込み（未使用）
    S: StorageOps,   // ストレージ操作（未使用）
>(
    // UserContext エクストラクタ: X-User-Id ヘッダーから抽出
    user: UserContext,
    // State エクストラクタ: AppState を取得
    State(state): State<AppState<TW, TR, C, UR, UW, S>>,
    // Query エクストラクタ: クエリパラメータを ListQuery にデシリアライズ
    Query(query): Query<ListQuery>,
) -> Result<impl IntoResponse, ApiError> {
    // TodoFilter を構築
    // - user_id: 自分の TODO のみを取得（マルチテナント対応）
    // - with_completed: 完了状態フィルタ（Option<bool>）
    let filter = TodoFilter::new(user.user_id) // user_id でフィルタ
        .with_completed(query.completed); // 完了状態フィルタを追加

    // ListTodosQuery を実行
    // エラー時は `?` で早期リターン（DomainError → ApiError に自動変換）
    let todos = state.list_todos.execute(filter).await?;

    // 成功時: 200 OK + TODO 配列
    Ok((StatusCode::OK, Json(todos)))
}

// =============================================================================
// create_todo ハンドラ
// =============================================================================

/// TODO 作成
///
/// POST /api/todos
///
/// # Request Body
///
/// ```json
/// {
///     "title": "買い物",
///     "description": "牛乳と卵を買う"
/// }
/// ```
///
/// # Response (201 Created)
///
/// ```json
/// {
///     "id": "uuid",
///     "user_id": "uuid",
///     "title": "買い物",
///     "description": "牛乳と卵を買う",
///     "completed": false,
///     "created_at": "2024-01-01T00:00:00Z",
///     "updated_at": "2024-01-01T00:00:00Z"
/// }
/// ```
///
/// # キャッシュ
///
/// Write-Through: 作成後にキャッシュにも保存される。
pub async fn create_todo<
    TW: TodoWriter,  // TODO 書き込み（作成）
    TR: TodoReader,  // TODO 読み取り（未使用）
    C: TodoCacheOps, // キャッシュ操作（Write-Through）
    UR: UserReader,  // ユーザー読み取り（未使用）
    UW: UserWriter,  // ユーザー書き込み（未使用）
    S: StorageOps,   // ストレージ操作（未使用）
>(
    // UserContext エクストラクタ: 認証済みユーザー情報
    user: UserContext,
    // State エクストラクタ: AppState を取得
    State(state): State<AppState<TW, TR, C, UR, UW, S>>,
    // Json エクストラクタ: リクエストボディを CreateTodoRequest にデシリアライズ
    Json(req): Json<CreateTodoRequest>,
) -> Result<impl IntoResponse, ApiError> {
    // リクエストを DTO に変換
    // DTO は Application 層で使用する内部表現
    let dto = CreateTodoDto {
        title: req.title,             // タイトル
        description: req.description, // 説明（Option）
    };

    // CreateTodoCommand を実行
    // - バリデーション
    // - DB 保存
    // - キャッシュ保存（Write-Through）
    let todo = state.create_todo.execute(user.user_id, dto).await?;

    // 成功時: 201 Created + 作成された TODO
    Ok((StatusCode::CREATED, Json(todo)))
}

// =============================================================================
// get_todo ハンドラ
// =============================================================================

/// TODO 詳細取得
///
/// GET /api/todos/{id}
///
/// # Response (200 OK)
///
/// ```json
/// {
///     "id": "uuid",
///     "user_id": "uuid",
///     "title": "タイトル",
///     "description": "説明",
///     "completed": false,
///     "created_at": "2024-01-01T00:00:00Z",
///     "updated_at": "2024-01-01T00:00:00Z"
/// }
/// ```
///
/// # Errors
///
/// - 404 Not Found: 指定された ID の TODO が存在しないか、他ユーザーの TODO
///
/// # キャッシュ
///
/// CachedTodoReader を使用している場合、キャッシュから取得される。
pub async fn get_todo<
    TW: TodoWriter,  // TODO 書き込み（未使用）
    TR: TodoReader,  // TODO 読み取り（詳細取得）
    C: TodoCacheOps, // キャッシュ操作（Reader 側で処理）
    UR: UserReader,  // ユーザー読み取り（未使用）
    UW: UserWriter,  // ユーザー書き込み（未使用）
    S: StorageOps,   // ストレージ操作（未使用）
>(
    // UserContext エクストラクタ: 認証済みユーザー情報
    user: UserContext,
    // State エクストラクタ: AppState を取得
    State(state): State<AppState<TW, TR, C, UR, UW, S>>,
    // Path エクストラクタ: URL パスから id を抽出
    // /api/todos/{id} の {id} 部分が Uuid としてパースされる
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    // GetTodoQuery を実行
    // - id: 取得対象の TODO ID
    // - user_id: 所有者チェック（他ユーザーの TODO は NotFound）
    let todo = state.get_todo.execute(id, user.user_id).await?;

    // 成功時: 200 OK + TODO
    Ok((StatusCode::OK, Json(todo)))
}

// =============================================================================
// update_todo ハンドラ
// =============================================================================

/// TODO 更新
///
/// PATCH /api/todos/{id}
///
/// # Request Body
///
/// ```json
/// {
///     "title": "新しいタイトル",
///     "description": "新しい説明",
///     "completed": true
/// }
/// ```
///
/// # Response (200 OK)
///
/// 更新後の TODO を返す（get_todo と同じ形式）。
///
/// # Errors
///
/// - 400 Bad Request: バリデーションエラー
/// - 404 Not Found: 指定された ID の TODO が存在しないか、他ユーザーの TODO
///
/// # キャッシュ
///
/// Cache Invalidation: 更新時にキャッシュが無効化される。
pub async fn update_todo<
    TW: TodoWriter,  // TODO 書き込み（更新）
    TR: TodoReader,  // TODO 読み取り（未使用）
    C: TodoCacheOps, // キャッシュ操作（Cache Invalidation）
    UR: UserReader,  // ユーザー読み取り（未使用）
    UW: UserWriter,  // ユーザー書き込み（未使用）
    S: StorageOps,   // ストレージ操作（未使用）
>(
    // UserContext エクストラクタ: 認証済みユーザー情報
    user: UserContext,
    // State エクストラクタ: AppState を取得
    State(state): State<AppState<TW, TR, C, UR, UW, S>>,
    // Path エクストラクタ: URL パスから id を抽出
    Path(id): Path<Uuid>,
    // Json エクストラクタ: リクエストボディを UpdateTodoRequest にデシリアライズ
    Json(req): Json<UpdateTodoRequest>,
) -> Result<impl IntoResponse, ApiError> {
    // リクエストを DTO に変換
    // 全フィールドが Option（指定されたフィールドのみ更新）
    let dto = UpdateTodoDto {
        title: req.title,             // 新しいタイトル（Option）
        description: req.description, // 新しい説明（Option）
        completed: req.completed,     // 新しい完了状態（Option）
    };

    // UpdateTodoCommand を実行
    // - バリデーション
    // - 所有者チェック
    // - DB 更新
    // - キャッシュ無効化
    let todo = state.update_todo.execute(id, user.user_id, dto).await?;

    // 成功時: 200 OK + 更新後の TODO
    Ok((StatusCode::OK, Json(todo)))
}

// =============================================================================
// delete_todo ハンドラ
// =============================================================================

/// TODO 削除
///
/// DELETE /api/todos/{id}
///
/// # Response (204 No Content)
///
/// ボディなし。
///
/// # Errors
///
/// - 404 Not Found: 指定された ID の TODO が存在しないか、他ユーザーの TODO
///
/// # キャッシュ
///
/// Cache Invalidation: 削除時にキャッシュが無効化される。
///
/// # Note
///
/// この操作は取り消せない。関連するファイルも削除される（CASCADE）。
pub async fn delete_todo<
    TW: TodoWriter,  // TODO 書き込み（削除）
    TR: TodoReader,  // TODO 読み取り（未使用）
    C: TodoCacheOps, // キャッシュ操作（Cache Invalidation）
    UR: UserReader,  // ユーザー読み取り（未使用）
    UW: UserWriter,  // ユーザー書き込み（未使用）
    S: StorageOps,   // ストレージ操作（未使用）
>(
    // UserContext エクストラクタ: 認証済みユーザー情報
    user: UserContext,
    // State エクストラクタ: AppState を取得
    State(state): State<AppState<TW, TR, C, UR, UW, S>>,
    // Path エクストラクタ: URL パスから id を抽出
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    // DeleteTodoCommand を実行
    // - 所有者チェック
    // - DB 削除
    // - キャッシュ無効化
    state.delete_todo.execute(id, user.user_id).await?;

    // 成功時: 204 No Content（ボディなし）
    Ok(StatusCode::NO_CONTENT)
}
