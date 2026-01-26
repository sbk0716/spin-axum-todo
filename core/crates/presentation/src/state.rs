// =============================================================================
// presentation/src/state.rs: アプリケーション状態
// =============================================================================
// ユースケースインスタンスを保持し、各ハンドラで共有するための構造体。
// 依存性注入（DI）コンテナとして機能する。
//
// 統一 CQRS パターン:
// - TODO Commands: TodoWriter を使用 + キャッシュ無効化/更新
// - TODO Queries: TodoReader を使用
// - User Commands: UserWriter を使用
// - User Queries: UserReader を使用
//
// キャッシュ戦略:
// - Commands: Write-Through（作成時）/ Cache Invalidation（更新・削除時）
// - Queries: CachedTodoReader でキャッシュ済み（ここでは直接使用しない）
// =============================================================================

// -----------------------------------------------------------------------------
// 外部クレートのインポート
// -----------------------------------------------------------------------------

// std::sync::Arc: スレッド安全な参照カウントポインタ
// リポジトリを複数のユースケース間で共有するために使用
use std::sync::Arc;

// application: Application 層のユースケース
use application::{
    // Services
    services::AuthService,
    // Commands（状態変更操作 - Writer DB プール使用）
    CreateTodoCommand,
    DeleteTodoCommand,
    // Queries（参照操作 - Reader DB プール使用）
    GetTodoQuery,
    ListTodosQuery,
    UpdateTodoCommand,
};

// domain: ドメイン層のトレイト
use domain::{TodoCacheOps, TodoReader, TodoWriter, UserReader, UserWriter};

// infrastructure: Infrastructure 層のサービス
use infrastructure::TransactionalTodoService;

// =============================================================================
// AppState 構造体
// =============================================================================

/// アプリケーション状態
///
/// ユースケースインスタンスを保持し、axum の State エクストラクタ経由で
/// 各ハンドラに渡される。
///
/// # ジェネリクス
///
/// - `TW: TodoWriter` - TODO 書き込み実装（Commands 用）
/// - `TR: TodoReader` - TODO 読み取り実装（Queries 用）
/// - `C: TodoCacheOps` - キャッシュ操作実装（Write-Through/無効化用）
/// - `UR: UserReader` - ユーザー読み取り実装（Queries 用）
/// - `UW: UserWriter` - ユーザー書き込み実装（Commands 用）
///
/// # 使用例
///
/// ```rust,ignore
/// // ハンドラ内で State エクストラクタを使用
/// async fn handler(
///     State(state): State<Arc<AppState<TW, TR, C, UR, UW>>>,
/// ) -> Result<Json<Todo>, ApiError> {
///     let todo = state.get_todo.execute(id, user_id).await?;
///     Ok(Json(todo))
/// }
/// ```
pub struct AppState<TW: TodoWriter, TR: TodoReader, C: TodoCacheOps, UR: UserReader, UW: UserWriter>
{
    /// 認証サービス（UserReader + UserWriter を使用）
    ///
    /// ログイン、登録、JWT 検証を担当
    pub auth_service: AuthService<UR, UW>,

    // -------------------------------------------------------------------------
    // TODO Commands（状態変更操作 - Writer DB プール使用 + キャッシュ操作）
    // -------------------------------------------------------------------------
    /// TODO 作成コマンド
    ///
    /// Write-Through: 作成後にキャッシュにも保存
    pub create_todo: CreateTodoCommand<TW, C>,

    /// TODO 更新コマンド
    ///
    /// Cache Invalidation: 更新時にキャッシュを無効化
    pub update_todo: UpdateTodoCommand<TW, C>,

    /// TODO 削除コマンド
    ///
    /// Cache Invalidation: 削除時にキャッシュを無効化
    pub delete_todo: DeleteTodoCommand<TW, C>,

    // -------------------------------------------------------------------------
    // TODO Queries（参照操作 - Reader DB プール使用）
    // -------------------------------------------------------------------------
    /// TODO 取得クエリ
    ///
    /// Note: CachedTodoReader を使用する場合、キャッシュは Reader 側で処理
    pub get_todo: GetTodoQuery<TR>,

    /// TODO 一覧取得クエリ
    ///
    /// Note: 一覧はキャッシュしない（フィルタ条件が多様なため）
    pub list_todos: ListTodosQuery<TR>,

    /// バッチ操作サービス（トランザクション対応）
    ///
    /// 複数 TODO の一括作成や TODO + ファイル同時作成に使用
    pub batch_service: TransactionalTodoService,
}

// =============================================================================
// AppState 実装
// =============================================================================

impl<TW: TodoWriter, TR: TodoReader, C: TodoCacheOps, UR: UserReader, UW: UserWriter>
    AppState<TW, TR, C, UR, UW>
{
    /// 新しい AppState を作成する
    ///
    /// # Arguments
    ///
    /// * `todo_writer` - TodoWriter の実装（Arc でラップ）
    /// * `todo_reader` - TodoReader の実装（Arc でラップ）
    /// * `cache` - TodoCacheOps の実装（Arc でラップ）- Write-Through/無効化用
    /// * `user_reader` - UserReader の実装（Arc でラップ）
    /// * `user_writer` - UserWriter の実装（Arc でラップ）
    /// * `batch_service` - トランザクション対応バッチサービス
    /// * `jwt_secret` - JWT 署名用シークレット
    /// * `jwt_expiry_hours` - JWT 有効期間（時間）
    ///
    /// # 属性
    ///
    /// `#[allow(clippy::too_many_arguments)]`: 引数が多いことを許容
    /// DI コンテナとしての役割上、多くの依存を受け取る必要がある
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        todo_writer: Arc<TW>,                    // Arc: スレッド安全な共有参照
        todo_reader: Arc<TR>,                    // Arc: スレッド安全な共有参照
        cache: Arc<C>,                           // Arc: Commands 間でキャッシュを共有
        user_reader: Arc<UR>,                    // Arc: AuthService で使用
        user_writer: Arc<UW>,                    // Arc: AuthService で使用
        batch_service: TransactionalTodoService, // Clone 可能
        jwt_secret: String,                      // JWT 署名用シークレット
        jwt_expiry_hours: i64,                   // JWT 有効期間（時間）
    ) -> Self {
        Self {
            // AuthService: UserReader + UserWriter + JWT 設定
            auth_service: AuthService::new(user_reader, user_writer, jwt_secret, jwt_expiry_hours),

            // TODO Commands（キャッシュ操作を含む）
            // Arc::clone: 参照カウントを増やすだけ（安価な操作）
            create_todo: CreateTodoCommand::new(Arc::clone(&todo_writer), Some(Arc::clone(&cache))),
            update_todo: UpdateTodoCommand::new(Arc::clone(&todo_writer), Some(Arc::clone(&cache))),
            delete_todo: DeleteTodoCommand::new(todo_writer, Some(cache)),

            // TODO Queries
            get_todo: GetTodoQuery::new(Arc::clone(&todo_reader)),
            list_todos: ListTodosQuery::new(todo_reader),

            // バッチサービス
            batch_service,
        }
    }
}

// =============================================================================
// Clone トレイト実装
// =============================================================================

/// AppState の Clone 実装
///
/// axum の State エクストラクタは Clone を要求する。
/// 内部のユースケースは全て Clone 可能（Arc を使用しているため安価）。
impl<TW: TodoWriter, TR: TodoReader, C: TodoCacheOps, UR: UserReader, UW: UserWriter> Clone
    for AppState<TW, TR, C, UR, UW>
{
    /// AppState を複製する
    ///
    /// 内部のユースケースは全て Arc を使用しているため、
    /// 実際のデータはコピーされず、参照カウントが増えるだけ。
    fn clone(&self) -> Self {
        Self {
            auth_service: self.auth_service.clone(),
            create_todo: self.create_todo.clone(),
            update_todo: self.update_todo.clone(),
            delete_todo: self.delete_todo.clone(),
            get_todo: self.get_todo.clone(),
            list_todos: self.list_todos.clone(),
            batch_service: self.batch_service.clone(),
        }
    }
}
