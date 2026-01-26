// =============================================================================
// domain/src/repositories/todo_repository.rs: Todo リポジトリトレイト
// =============================================================================
// 永続化の抽象インターフェース（依存性逆転の原則）。
//
// クリーンアーキテクチャの依存性逆転の原則（DIP）:
// - 上位モジュール（ドメイン層）は下位モジュール（インフラ層）に依存しない
// - 両者は抽象（トレイト）に依存する
// - ドメイン層がトレイトを定義し、インフラ層がそれを実装
//
// 軽量 CQRS（Command Query Responsibility Segregation）パターン:
// - TodoWriter: 状態変更操作（Commands）- Writer DB プールを使用
// - TodoReader: 参照操作（Queries）- Reader DB プールを使用
// - Writer と Reader を分離することで、スケーラビリティが向上
// - AWS Aurora 等の読み取りレプリカを活用可能
//
// CQRS の利点:
// - 読み取りと書き込みを独立してスケーリング可能
// - 読み取りはキャッシュやレプリカで最適化
// - 書き込みはトランザクション整合性を保証
// =============================================================================

// -----------------------------------------------------------------------------
// 外部クレートのインポート
// -----------------------------------------------------------------------------

// async_trait: Rust の async fn をトレイト内で使用可能にする
// Rust 標準ではトレイト内の async fn は直接サポートされていないため必要
use async_trait::async_trait;

// uuid: 一意識別子
use uuid::Uuid;

// 同じクレート内のエンティティとエラー型
use crate::entities::Todo;
use crate::errors::DomainError;

// =============================================================================
// TodoFilter 構造体
// =============================================================================

/// TODO 一覧取得のフィルタ条件
///
/// ビルダーパターンで柔軟にフィルタ条件を設定できる。
///
/// # Example
/// ```
/// use uuid::Uuid;
/// use domain::TodoFilter;
///
/// let user_id = Uuid::new_v4();
///
/// // 基本的な使用（ユーザーの全 TODO）
/// let filter = TodoFilter::new(user_id);
///
/// // 完了済みのみ取得
/// let filter = TodoFilter::new(user_id).with_completed(Some(true));
///
/// // 未完了のみ取得
/// let filter = TodoFilter::new(user_id).with_completed(Some(false));
/// ```
// -----------------------------------------------------------------------------
// derive マクロ:
// - Debug: {:?} フォーマットでデバッグ出力
// - Clone: .clone() で値をコピー
// -----------------------------------------------------------------------------
#[derive(Debug, Clone)]
pub struct TodoFilter {
    /// 所有者のユーザー ID（必須、UUID）
    ///
    /// マルチテナント対応のため、常にユーザーでフィルタリングする。
    /// これにより、他のユーザーの TODO にアクセスできないようにする。
    pub user_id: Uuid,

    /// 完了状態でフィルタリング
    ///
    /// - `Some(true)`: 完了済みの TODO のみ取得
    /// - `Some(false)`: 未完了の TODO のみ取得
    /// - `None`: 全件取得（完了状態でフィルタリングしない）
    pub completed: Option<bool>,
}

impl TodoFilter {
    /// 新しいフィルタを作成
    ///
    /// # Arguments
    /// * `user_id` - 所有者のユーザー ID
    ///
    /// # Returns
    /// completed = None（フィルタなし）の TodoFilter
    pub fn new(user_id: Uuid) -> Self {
        Self {
            user_id,
            // デフォルトは None（全件取得）
            completed: None,
        }
    }

    /// 完了状態フィルタを設定（ビルダーパターン）
    ///
    /// # Arguments
    /// * `completed` - 完了状態フィルタ
    ///
    /// # Returns
    /// 自身を返す（メソッドチェーン可能）
    ///
    /// # Note
    /// `mut self` で所有権を取得し、変更後の self を返す。
    /// これによりビルダーパターンが実現できる。
    pub fn with_completed(mut self, completed: Option<bool>) -> Self {
        // 完了状態フィルタを設定
        self.completed = completed;

        // 自身を返す（メソッドチェーン可能）
        self
    }
}

// =============================================================================
// CQRS: Writer / Reader トレイト分離
// =============================================================================

/// TODO 書き込みトレイト（Commands 用）
///
/// 軽量 CQRS パターンにおける状態変更操作のインターフェース。
/// Writer DB プール（DATABASE_WRITER_URL）を使用する実装と組み合わせる。
///
/// # CQRS の原則
/// - このトレイトには読み取り操作（find_by_id 等）を含めない
/// - 更新は単一の atomic UPDATE クエリで実行（楽観的ロック不要）
/// - 更新結果は RETURNING 句で取得（追加の SELECT 不要）
///
/// # 実装例
/// - `PostgresTodoWriter`: PostgreSQL 実装（infrastructure 層）
// -----------------------------------------------------------------------------
// #[async_trait] マクロについて:
// Rust の標準では、トレイト内で async fn を定義できない。
// async_trait マクロがこの制限を回避し、非同期メソッドを定義可能にする。
//
// Send + Sync について:
// - Send: 値を別スレッドに安全に送信できる
// - Sync: 参照を複数スレッドで安全に共有できる
// - tokio の非同期タスクはスレッド間で移動する可能性があるため必要
// -----------------------------------------------------------------------------
#[async_trait]
pub trait TodoWriter: Send + Sync {
    /// TODO を作成
    ///
    /// # Arguments
    /// * `todo` - 作成する TODO エンティティ
    ///
    /// # Returns
    /// * `Ok(Todo)` - 作成された TODO（ID や日時が設定済み）
    /// * `Err(DomainError::Repository)` - データベースエラー
    async fn create(&self, todo: &Todo) -> Result<Todo, DomainError>;

    /// TODO を部分更新（単一の atomic UPDATE クエリ）
    ///
    /// CQRS の Writer として、更新前の読み取りを行わない。
    /// SQL の COALESCE と CASE WHEN を使用して、単一クエリで部分更新を実現。
    ///
    /// # Arguments
    /// * `id` - 更新する TODO の UUID
    /// * `user_id` - 所有者のユーザー ID（認可チェック用）
    /// * `title` - 新しいタイトル（None なら変更なし）
    /// * `description` - 新しい説明
    ///   * `Some(Some("value"))`: 説明を設定
    ///   * `Some(None)`: 説明を NULL に設定（削除）
    ///   * `None`: 変更なし
    /// * `completed` - 新しい完了状態（None なら変更なし）
    ///
    /// # Returns
    /// * `Ok(Todo)` - 更新された TODO
    /// * `Err(DomainError::NotFound)` - TODO が見つからないか、ユーザーが所有者でない
    /// * `Err(DomainError::Repository)` - データベースエラー
    ///
    /// # SQL Example
    /// ```sql
    /// UPDATE todos
    /// SET title = COALESCE($3, title),
    ///     description = CASE WHEN $4 THEN $5 ELSE description END,
    ///     completed = COALESCE($6, completed),
    ///     updated_at = NOW()
    /// WHERE id = $1 AND user_id = $2
    /// RETURNING *
    /// ```
    async fn update_fields(
        &self,
        id: Uuid,
        user_id: Uuid,
        title: Option<String>,
        description: Option<Option<String>>,
        completed: Option<bool>,
    ) -> Result<Todo, DomainError>;

    /// TODO を削除
    ///
    /// # Arguments
    /// * `id` - 削除する TODO の UUID
    /// * `user_id` - 所有者のユーザー ID（認可チェック用）
    ///
    /// # Returns
    /// * `Ok(true)` - 削除成功
    /// * `Ok(false)` - TODO が見つからなかった
    /// * `Err(DomainError::Repository)` - データベースエラー
    async fn delete(&self, id: Uuid, user_id: Uuid) -> Result<bool, DomainError>;
}

/// TODO 読み取りトレイト（Queries 用）
///
/// 軽量 CQRS パターンにおける参照操作のインターフェース。
/// Reader DB プール（DATABASE_READER_URL）を使用する実装と組み合わせる。
///
/// # 実装例
/// - `PostgresTodoReader`: PostgreSQL 実装（infrastructure 層）
/// - `CachedTodoReader`: キャッシュ付きデコレータ（infrastructure 層）
#[async_trait]
pub trait TodoReader: Send + Sync {
    /// ID とユーザー ID で TODO を取得
    ///
    /// # Arguments
    /// * `id` - 取得する TODO の UUID
    /// * `user_id` - 所有者のユーザー ID（認可チェック用）
    ///
    /// # Returns
    /// * `Ok(Some(Todo))` - TODO が見つかった
    /// * `Ok(None)` - TODO が見つからない、またはユーザーが所有者でない
    /// * `Err(DomainError::Repository)` - データベースエラー
    async fn find_by_id(&self, id: Uuid, user_id: Uuid) -> Result<Option<Todo>, DomainError>;

    /// TODO 一覧を取得
    ///
    /// # Arguments
    /// * `filter` - フィルタ条件（ユーザー ID、完了状態）
    ///
    /// # Returns
    /// * `Ok(Vec<Todo>)` - TODO のリスト（0件の場合は空の Vec）
    /// * `Err(DomainError::Repository)` - データベースエラー
    async fn find_all(&self, filter: TodoFilter) -> Result<Vec<Todo>, DomainError>;
}
