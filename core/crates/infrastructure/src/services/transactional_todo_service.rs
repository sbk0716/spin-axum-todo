// =============================================================================
// infrastructure/src/services/transactional_todo_service.rs
// =============================================================================
// トランザクションが必要なバッチ操作を提供するサービス。
// sqlx の RAII 自動ロールバック機能を活用し、
// 複数テーブルを跨ぐアトミック操作を実現する。
//
// 設計判断:
// - PgPool を直接使用してトランザクションを管理
// - Domain 層のトレイトを sqlx に依存させない
// - トランザクションが必要な操作のみこのサービスで提供
//
// トランザクションの RAII パターン:
// - begin() でトランザクション開始
// - commit() で明示的にコミット
// - commit() を呼ばずに Drop すると自動ロールバック
// =============================================================================

// -----------------------------------------------------------------------------
// 外部クレートのインポート
// -----------------------------------------------------------------------------

// chrono: 日付・時刻ライブラリ
use chrono::{DateTime, Utc};

// domain: ドメイン層の型をインポート
use domain::{DomainError, File, Todo};

// sqlx: PostgreSQL クライアントライブラリ
// FromRow: クエリ結果から構造体への自動マッピング
// PgPool: PostgreSQL 接続プール
use sqlx::{FromRow, PgPool};

// tracing: 構造化ログライブラリ
use tracing::{debug, info};

// uuid: 一意識別子ライブラリ
use uuid::Uuid;

// =============================================================================
// TransactionalTodoService 構造体
// =============================================================================

/// トランザクション対応 TODO サービス
///
/// バッチ作成や TODO + ファイル同時作成など、
/// 複数操作を1トランザクションで実行する必要がある場合に使用。
///
/// # いつ使うか
///
/// - 複数 TODO の一括作成（バッチ操作）
/// - TODO とファイルの同時作成
/// - 複数テーブルを跨ぐ操作
///
/// # いつ使わないか
///
/// - 単一 TODO の CRUD 操作（TodoWriter で十分）
/// - 読み取り専用操作（TodoReader を使用）
///
/// # derive マクロ
///
/// - `Clone`: サービスを複数箇所で共有可能に
///   PgPool は内部で Arc を使用しているため、Clone は安価
#[derive(Clone)]
pub struct TransactionalTodoService {
    /// PostgreSQL 接続プール（Writer 用推奨）
    pool: PgPool,
}

impl TransactionalTodoService {
    /// 新しい TransactionalTodoService を作成する
    ///
    /// # Arguments
    ///
    /// * `pool` - PostgreSQL 接続プール（Writer 用を推奨）
    ///
    /// # Note
    ///
    /// トランザクションを使用するため、Writer プールを指定すること。
    /// Reader プールでは書き込み操作が失敗する可能性がある。
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

// =============================================================================
// TodoRow 構造体（内部用）
// =============================================================================

/// データベース行からのマッピング用構造体（TODO）
///
/// RETURNING 句の結果を受け取るために使用。
#[derive(FromRow)]
struct TodoRow {
    /// TODO の一意識別子
    id: Uuid,
    /// 所有者のユーザー ID
    user_id: Uuid,
    /// タイトル
    title: String,
    /// 詳細説明
    description: Option<String>,
    /// 完了フラグ
    completed: bool,
    /// 作成日時
    created_at: DateTime<Utc>,
    /// 更新日時
    updated_at: DateTime<Utc>,
}

/// TodoRow から domain::Todo への変換
impl From<TodoRow> for Todo {
    fn from(row: TodoRow) -> Self {
        Todo::from_raw(
            row.id,
            row.user_id,
            row.title,
            row.description,
            row.completed,
            row.created_at,
            row.updated_at,
        )
    }
}

// =============================================================================
// FileRow 構造体（内部用）
// =============================================================================

/// データベース行からのマッピング用構造体（ファイル）
///
/// RETURNING 句の結果を受け取るために使用。
#[derive(FromRow)]
struct FileRow {
    /// ファイルの一意識別子
    id: Uuid,
    /// 紐付く TODO の ID
    todo_id: Uuid,
    /// ファイル名
    filename: String,
    /// MIME タイプ
    mime_type: String,
    /// ファイルサイズ（バイト）
    size_bytes: i64,
    /// ストレージ上のパス
    storage_path: String,
    /// 作成日時
    created_at: DateTime<Utc>,
}

/// FileRow から domain::File への変換
impl From<FileRow> for File {
    fn from(row: FileRow) -> Self {
        File::from_raw(
            row.id,
            row.todo_id,
            row.filename,
            row.mime_type,
            row.size_bytes,
            row.storage_path,
            row.created_at,
        )
    }
}

// =============================================================================
// FileInput 構造体
// =============================================================================

/// ファイル作成用の入力データ
///
/// ファイルアップロード後のメタデータを保持する。
///
/// # derive マクロ
///
/// - `Debug`: デバッグ出力用
/// - `Clone`: 複製可能に（複数箇所で使用する場合）
#[derive(Debug, Clone)]
pub struct FileInput {
    /// ファイル名（元のファイル名）
    pub filename: String,
    /// MIME タイプ（例: "image/png"）
    pub mime_type: String,
    /// ファイルサイズ（バイト）
    pub size_bytes: i64,
    /// ストレージ上のパス（S3 キーなど）
    pub storage_path: String,
}

// =============================================================================
// TransactionalTodoService メソッド実装
// =============================================================================

impl TransactionalTodoService {
    /// 複数 TODO を1トランザクションで作成
    ///
    /// バリデーション済みの TODO リストを受け取り、
    /// 全て成功するか、全て失敗（ロールバック）するかのどちらか。
    ///
    /// # Arguments
    ///
    /// * `user_id` - 所有者のユーザー ID
    /// * `todos` - (title, description) のリスト
    ///
    /// # Returns
    ///
    /// 作成された TODO のリスト
    ///
    /// # Errors
    ///
    /// いずれかの INSERT が失敗した場合、全てロールバックされる。
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let todos = vec![
    ///     ("Buy groceries".to_string(), Some("Milk, eggs".to_string())),
    ///     ("Clean room".to_string(), None),
    /// ];
    /// let created = service.batch_create(user_id, todos).await?;
    /// assert_eq!(created.len(), 2);
    /// ```
    pub async fn batch_create(
        &self,
        user_id: Uuid,
        todos: Vec<(String, Option<String>)>, // (title, description) のタプル
    ) -> Result<Vec<Todo>, DomainError> {
        // 構造化ログ: ユーザー ID と件数を記録
        debug!(user_id = %user_id, count = todos.len(), "Starting batch create transaction");

        // トランザクション開始
        // begin(): トランザクションを開始し、Transaction<Postgres> を返す
        // RAII: tx が Drop されるとき、commit() されていなければ自動ロールバック
        let mut tx = self
            .pool
            .begin() // BEGIN 文を実行
            .await // 非同期実行
            .map_err(|e| DomainError::Repository(e.to_string()))?; // エラー変換

        // 結果を格納する Vec を事前確保（パフォーマンス最適化）
        let mut created = Vec::with_capacity(todos.len());

        // 各 TODO を順番に作成
        for (title, description) in todos {
            // 新しい Todo エンティティを作成
            let todo = Todo::new(user_id, title, description);

            // INSERT 文を実行（トランザクション内）
            let row: TodoRow = sqlx::query_as(
                r#"
                INSERT INTO todos (id, user_id, title, description, completed, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                RETURNING id, user_id, title, description, completed, created_at, updated_at
                "#,
            )
            .bind(todo.id) // $1: UUID
            .bind(&todo.user_id) // $2: 所有者 ID
            .bind(&todo.title) // $3: タイトル
            .bind(&todo.description) // $4: 説明
            .bind(todo.completed) // $5: 完了フラグ
            .bind(todo.created_at) // $6: 作成日時
            .bind(todo.updated_at) // $7: 更新日時
            .fetch_one(&mut *tx) // トランザクション内で実行（&mut *tx でデリファレンス）
            .await // 非同期実行
            .map_err(|e| DomainError::Repository(e.to_string()))?; // エラー変換

            // 作成結果を追加
            created.push(row.into()); // TodoRow → Todo に変換
        }

        // コミット
        // 重要: commit() を呼ばないと、tx が Drop されるときに自動ロールバックされる
        tx.commit() // COMMIT 文を実行
            .await // 非同期実行
            .map_err(|e| DomainError::Repository(e.to_string()))?; // エラー変換

        // 成功ログ
        info!(user_id = %user_id, count = created.len(), "Batch create transaction committed");

        Ok(created)
    }

    /// TODO + ファイルを1トランザクションで作成
    ///
    /// TODO とその添付ファイルを同時に作成する。
    /// いずれかが失敗した場合、全てロールバックされる。
    ///
    /// # Arguments
    ///
    /// * `user_id` - 所有者のユーザー ID
    /// * `title` - TODO タイトル（バリデーション済み）
    /// * `description` - TODO 説明
    /// * `files` - ファイル入力データのリスト
    ///
    /// # Returns
    ///
    /// (作成された TODO, 作成されたファイルのリスト)
    ///
    /// # Important
    ///
    /// ファイル本体は事前にストレージにアップロード済みであること。
    /// トランザクション失敗時、ストレージのファイルは手動で削除が必要
    /// （補償トランザクション）。
    pub async fn create_with_files(
        &self,
        user_id: Uuid,
        title: String,
        description: Option<String>,
        files: Vec<FileInput>,
    ) -> Result<(Todo, Vec<File>), DomainError> {
        // 構造化ログ
        debug!(user_id = %user_id, file_count = files.len(), "Starting create with files transaction");

        // トランザクション開始
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| DomainError::Repository(e.to_string()))?;

        // 1. TODO 作成
        let todo = Todo::new(user_id, title, description);

        let todo_row: TodoRow = sqlx::query_as(
            r#"
            INSERT INTO todos (id, user_id, title, description, completed, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, user_id, title, description, completed, created_at, updated_at
            "#,
        )
        .bind(todo.id)
        .bind(&todo.user_id)
        .bind(&todo.title)
        .bind(&todo.description)
        .bind(todo.completed)
        .bind(todo.created_at)
        .bind(todo.updated_at)
        .fetch_one(&mut *tx) // トランザクション内で実行
        .await
        .map_err(|e| DomainError::Repository(e.to_string()))?;

        // TodoRow → Todo に変換
        let created_todo: Todo = todo_row.into();

        // 2. ファイル作成
        let mut created_files = Vec::with_capacity(files.len());

        for f in files {
            // File エンティティを作成（todo_id は作成した TODO の ID）
            let file = File::new(
                created_todo.id, // 親 TODO の ID
                f.filename,      // ファイル名
                f.mime_type,     // MIME タイプ
                f.size_bytes,    // サイズ
                f.storage_path,  // ストレージパス
            );

            // ファイルメタデータを INSERT
            let file_row: FileRow = sqlx::query_as(
                r#"
                INSERT INTO files (id, todo_id, filename, mime_type, size_bytes, storage_path, created_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                RETURNING id, todo_id, filename, mime_type, size_bytes, storage_path, created_at
                "#,
            )
            .bind(file.id)
            .bind(file.todo_id)
            .bind(&file.filename)
            .bind(&file.mime_type)
            .bind(file.size_bytes)
            .bind(&file.storage_path)
            .bind(file.created_at)
            .fetch_one(&mut *tx) // トランザクション内で実行
            .await
            .map_err(|e| DomainError::Repository(e.to_string()))?;

            created_files.push(file_row.into()); // FileRow → File に変換
        }

        // 3. コミット
        tx.commit()
            .await
            .map_err(|e| DomainError::Repository(e.to_string()))?;

        // 成功ログ
        info!(
            todo_id = %created_todo.id,
            file_count = created_files.len(),
            "Create with files transaction committed"
        );

        // タプルで返す: (TODO, ファイルリスト)
        Ok((created_todo, created_files))
    }
}
