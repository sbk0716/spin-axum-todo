// =============================================================================
// presentation/src/handlers/batch.rs: バッチ操作ハンドラ
// =============================================================================
// 複数 TODO の一括作成や TODO + ファイル同時作成を処理する。
// トランザクション管理は TransactionalTodoService に委譲する。
//
// エンドポイント:
// - POST /api/todos/batch      - 複数 TODO を一括作成
// - POST /api/todos/with-files - TODO + ファイルを同時作成
//
// トランザクション保証:
// - いずれかの操作が失敗した場合、全てロールバック
// - 部分的な成功は発生しない（All or Nothing）
//
// バリデーション:
// - ハンドラ内でバリデーションを実行
// - Domain 層の validate_* メソッドを使用
// =============================================================================

// -----------------------------------------------------------------------------
// 外部クレートのインポート
// -----------------------------------------------------------------------------

// std::sync::Arc: スレッド安全な参照カウントポインタ
use std::sync::Arc;

// axum: Web フレームワーク
// extract::State: アプリケーション状態の抽出
// http::StatusCode: HTTP ステータスコード
// response::IntoResponse: レスポンス変換トレイト
// Json: JSON リクエスト/レスポンス
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

// domain: ドメイン層の型とトレイト
// File: ファイルエンティティ（バリデーション用）
// Todo: TODO エンティティ（バリデーション用）
// TodoCacheOps: キャッシュ操作トレイト
// TodoReader/Writer: TODO 読み書きトレイト
// UserReader/Writer: ユーザー読み書きトレイト
use domain::{File, Todo, TodoCacheOps, TodoReader, TodoWriter, UserReader, UserWriter};

// infrastructure: Infrastructure 層の型
// FileInput: ファイル作成入力パラメータ
use infrastructure::FileInput;

// application: Application 層の DTO
use application::dto::{
    BatchCreateTodosRequest,    // バッチ作成リクエスト
    CreateTodoWithFilesRequest, // TODO + ファイル作成リクエスト
    FileResponse,               // ファイルレスポンス
    TodoWithFilesResponse,      // TODO + ファイルレスポンス
};

// crate: このクレート内のモジュール
use crate::error::ApiError; // API エラー型
use crate::middleware::UserContext; // 認証済みユーザー情報
use crate::state::AppState; // アプリケーション状態

// =============================================================================
// batch_create_todos ハンドラ
// =============================================================================

/// バッチ TODO 作成
///
/// POST /api/todos/batch
///
/// # Request Body
///
/// ```json
/// {
///     "todos": [
///         {"title": "タスク1", "description": "説明1"},
///         {"title": "タスク2", "description": null},
///         {"title": "タスク3"}
///     ]
/// }
/// ```
///
/// # Response (201 Created)
///
/// ```json
/// [
///     {
///         "id": "uuid",
///         "user_id": "uuid",
///         "title": "タスク1",
///         "description": "説明1",
///         "completed": false,
///         "created_at": "2024-01-01T00:00:00Z",
///         "updated_at": "2024-01-01T00:00:00Z"
///     },
///     ...
/// ]
/// ```
///
/// # Errors
///
/// - 400 Bad Request: バリデーションエラー（空配列、タイトル不正など）
///
/// # トランザクション
///
/// 複数の TODO を1トランザクションで作成する。
/// いずれかが失敗した場合、全てロールバックされる。
pub async fn batch_create_todos<
    TW: TodoWriter,  // TODO 書き込み（未使用、TransactionalTodoService 使用）
    TR: TodoReader,  // TODO 読み取り（未使用）
    C: TodoCacheOps, // キャッシュ操作（未使用）
    UR: UserReader,  // ユーザー読み取り（未使用）
    UW: UserWriter,  // ユーザー書き込み（未使用）
>(
    // UserContext エクストラクタ: 認証済みユーザー情報
    user: UserContext,
    // State エクストラクタ: AppState を取得
    State(state): State<Arc<AppState<TW, TR, C, UR, UW>>>,
    // Json エクストラクタ: リクエストボディを BatchCreateTodosRequest にデシリアライズ
    Json(req): Json<BatchCreateTodosRequest>,
) -> Result<impl IntoResponse, ApiError> {
    // -------------------------------------------------------------------------
    // バリデーション: 空配列チェック
    // -------------------------------------------------------------------------
    // 空配列の場合は 400 Bad Request を返す
    if req.todos.is_empty() {
        return Err(ApiError::BadRequest("todos cannot be empty".to_string()));
    }

    // -------------------------------------------------------------------------
    // バリデーション: 各 TODO のタイトル
    // -------------------------------------------------------------------------
    // Domain 層の validate_title を使用して検証
    // エラーの場合は早期リターン（DomainError → ApiError に自動変換）
    for todo in &req.todos {
        Todo::validate_title(&todo.title)?; // 1-200文字のチェック
    }

    // -------------------------------------------------------------------------
    // リクエストを内部形式に変換
    // -------------------------------------------------------------------------
    // BatchCreateTodosRequest の todos を (title, description) のタプルに変換
    // into_iter(): 所有権を移動するイテレータ（req.todos を消費）
    // map(): 各要素を変換
    // collect(): Vec に収集
    let todos: Vec<(String, Option<String>)> = req
        .todos
        .into_iter() // イテレータに変換
        .map(|t| (t.title, t.description)) // (title, description) タプルに変換
        .collect(); // Vec に収集

    // -------------------------------------------------------------------------
    // TransactionalTodoService で一括作成
    // -------------------------------------------------------------------------
    // batch_create: トランザクション内で複数 TODO を作成
    // 失敗時は全てロールバック
    let created: Vec<Todo> = state
        .batch_service // TransactionalTodoService を取得
        .batch_create(user.user_id, todos) // バッチ作成実行
        .await?; // 非同期待機 + エラー伝播

    // 成功時: 201 Created + 作成された TODO 配列
    Ok((StatusCode::CREATED, Json(created)))
}

// =============================================================================
// create_todo_with_files ハンドラ
// =============================================================================

/// TODO + ファイル同時作成
///
/// POST /api/todos/with-files
///
/// # Request Body
///
/// ```json
/// {
///     "title": "ドキュメント作成",
///     "description": "仕様書を作成する",
///     "files": [
///         {
///             "filename": "spec.pdf",
///             "mime_type": "application/pdf",
///             "size_bytes": 1024000,
///             "storage_path": "uploads/2024/01/spec.pdf"
///         }
///     ]
/// }
/// ```
///
/// # Response (201 Created)
///
/// ```json
/// {
///     "todo": {
///         "id": "uuid",
///         "user_id": "uuid",
///         "title": "ドキュメント作成",
///         "description": "仕様書を作成する",
///         "completed": false,
///         "created_at": "2024-01-01T00:00:00Z",
///         "updated_at": "2024-01-01T00:00:00Z"
///     },
///     "files": [
///         {
///             "id": "uuid",
///             "filename": "spec.pdf",
///             "mime_type": "application/pdf",
///             "size_bytes": 1024000,
///             "storage_path": "uploads/2024/01/spec.pdf",
///             "created_at": "2024-01-01T00:00:00Z"
///         }
///     ]
/// }
/// ```
///
/// # Errors
///
/// - 400 Bad Request: バリデーションエラー
///
/// # トランザクション
///
/// TODO とファイルメタデータを1トランザクションで作成する。
/// いずれかが失敗した場合、全てロールバックされる。
///
/// # Note
///
/// ファイル本体は事前にストレージにアップロード済みの前提。
/// このエンドポイントはメタデータのみを DB に登録する。
pub async fn create_todo_with_files<
    TW: TodoWriter,  // TODO 書き込み（未使用、TransactionalTodoService 使用）
    TR: TodoReader,  // TODO 読み取り（未使用）
    C: TodoCacheOps, // キャッシュ操作（未使用）
    UR: UserReader,  // ユーザー読み取り（未使用）
    UW: UserWriter,  // ユーザー書き込み（未使用）
>(
    // UserContext エクストラクタ: 認証済みユーザー情報
    user: UserContext,
    // State エクストラクタ: AppState を取得
    State(state): State<Arc<AppState<TW, TR, C, UR, UW>>>,
    // Json エクストラクタ: リクエストボディを CreateTodoWithFilesRequest にデシリアライズ
    Json(req): Json<CreateTodoWithFilesRequest>,
) -> Result<impl IntoResponse, ApiError> {
    // -------------------------------------------------------------------------
    // バリデーション: TODO タイトル
    // -------------------------------------------------------------------------
    // Domain 層の validate_title を使用して検証
    // バリデーション済みのタイトルを取得（トリム済み）
    let title = Todo::validate_title(&req.title)?;

    // -------------------------------------------------------------------------
    // バリデーション: ファイル情報
    // -------------------------------------------------------------------------
    // 各ファイルの filename, mime_type, size_bytes を検証
    // Vec::with_capacity: 事前にキャパシティを確保（効率化）
    let mut file_inputs = Vec::with_capacity(req.files.len());

    for f in req.files {
        // ファイル名のバリデーション（空でないこと、特殊文字チェック）
        let filename = File::validate_filename(&f.filename)?;

        // MIME タイプのバリデーション（形式チェック）
        let mime_type = File::validate_mime_type(&f.mime_type)?;

        // ファイルサイズのバリデーション（正の値チェック）
        File::validate_size(f.size_bytes)?;

        // FileInput 構造体に変換
        // Infrastructure 層の TransactionalTodoService で使用
        file_inputs.push(FileInput {
            filename,                     // バリデーション済みファイル名
            mime_type,                    // バリデーション済み MIME タイプ
            size_bytes: f.size_bytes,     // ファイルサイズ
            storage_path: f.storage_path, // ストレージパス（S3 キーなど）
        });
    }

    // -------------------------------------------------------------------------
    // TransactionalTodoService で一括作成
    // -------------------------------------------------------------------------
    // create_with_files: トランザクション内で TODO + ファイルを作成
    // 失敗時は全てロールバック
    let (todo, files): (Todo, Vec<File>) = state
        .batch_service // TransactionalTodoService を取得
        .create_with_files(
            // TODO + ファイル作成
            user.user_id,    // ユーザー ID
            title,           // バリデーション済みタイトル
            req.description, // 説明（Option）
            file_inputs,     // ファイル入力リスト
        )
        .await?; // 非同期待機 + エラー伝播

    // -------------------------------------------------------------------------
    // レスポンス構築
    // -------------------------------------------------------------------------
    // TodoWithFilesResponse: TODO とファイルの両方を含むレスポンス
    let response = TodoWithFilesResponse {
        todo,                                                       // TODO エンティティ
        files: files.into_iter().map(FileResponse::from).collect(), // File → FileResponse に変換
    };

    // 成功時: 201 Created + レスポンス
    Ok((StatusCode::CREATED, Json(response)))
}
