// =============================================================================
// infrastructure/src/persistence/postgres/todo_writer.rs
// =============================================================================
// PostgreSQL を使用した TodoWriter の実装。
// 軽量 CQRS パターン: 状態変更操作（Commands）を担当。
// Writer DB プール（DATABASE_WRITER_URL）を使用する。
//
// 特徴:
// - RETURNING 句で INSERT/UPDATE 結果を即座に取得
// - 部分更新に COALESCE + CASE を活用
// - user_id による所有権チェックを SQL レベルで実行
// =============================================================================

// -----------------------------------------------------------------------------
// 外部クレートのインポート
// -----------------------------------------------------------------------------

// async_trait: async fn を含むトレイトを定義可能にする
use async_trait::async_trait;

// chrono: 日付・時刻ライブラリ
use chrono::{DateTime, Utc};

// domain: ドメイン層の型をインポート
// TodoWriter: 書き込み操作を定義するトレイト
use domain::{DomainError, Todo, TodoWriter};

// sqlx: PostgreSQL クライアントライブラリ
use sqlx::{FromRow, PgPool};

// tracing: 構造化ログライブラリ
use tracing::debug;

// uuid: 一意識別子ライブラリ
use uuid::Uuid;

// =============================================================================
// PostgresTodoWriter 構造体
// =============================================================================

/// PostgreSQL への書き込みを管理するリポジトリ
///
/// 軽量 CQRS パターンにおける Writer 実装。
/// Commands（create, update, delete）で使用する。
///
/// # キャッシュとの連携
///
/// このリポジトリ自体はキャッシュを意識しない。
/// キャッシュの更新/無効化は、呼び出し側（Command ハンドラ）で行う。
/// これにより、単一責任の原則を維持している。
pub struct PostgresTodoWriter {
    /// PostgreSQL 接続プール（Writer 用）
    pool: PgPool,
}

impl PostgresTodoWriter {
    /// 新しい PostgresTodoWriter を作成する
    ///
    /// # Arguments
    ///
    /// * `pool` - PostgreSQL 接続プール（Writer 用）
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

// =============================================================================
// TodoRow 構造体（内部用）
// =============================================================================

/// データベース行からのマッピング用構造体
///
/// RETURNING 句の結果を受け取るために使用。
/// TodoReader の TodoRow と同じ構造だが、
/// モジュールが異なるため別途定義している。
///
/// # Note
///
/// 将来的には共通モジュールに移動して DRY にすることも検討。
#[derive(FromRow)]
struct TodoRow {
    id: Uuid,
    user_id: Uuid,
    title: String,
    description: Option<String>,
    completed: bool,
    created_at: DateTime<Utc>,
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
// TodoWriter トレイト実装
// =============================================================================

/// TodoWriter トレイトの PostgreSQL 実装
#[async_trait]
impl TodoWriter for PostgresTodoWriter {
    /// 新しい TODO を作成する
    ///
    /// # Arguments
    ///
    /// * `todo` - 作成する TODO エンティティ
    ///
    /// # Returns
    ///
    /// * `Ok(Todo)` - 作成された TODO（DB が設定した値を含む）
    /// * `Err(DomainError)` - 作成失敗
    ///
    /// # Note
    ///
    /// RETURNING 句により、INSERT と SELECT を1回の往復で実行。
    /// これはパフォーマンス最適化であり、PostgreSQL 固有の機能。
    async fn create(&self, todo: &Todo) -> Result<Todo, DomainError> {
        debug!(todo_id = %todo.id, user_id = %todo.user_id, "Creating todo in PostgreSQL (Writer)");

        // INSERT ... RETURNING で挿入と取得を同時に実行
        let row: TodoRow = sqlx::query_as(
            r#"
            INSERT INTO todos (id, user_id, title, description, completed, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, user_id, title, description, completed, created_at, updated_at
            "#,
        )
        .bind(todo.id) // $1: 事前に生成した UUID
        .bind(todo.user_id) // $2: 所有者 ID
        .bind(&todo.title) // $3: タイトル
        .bind(&todo.description) // $4: 説明（NULL 許容）
        .bind(todo.completed) // $5: 完了フラグ（通常 false）
        .bind(todo.created_at) // $6: 作成日時
        .bind(todo.updated_at) // $7: 更新日時（作成時は created_at と同じ）
        .fetch_one(&self.pool) // 1行取得（RETURNING 句の結果）
        .await
        .map_err(|e| DomainError::Repository(e.to_string()))?;

        // TodoRow → Todo に変換して返す
        Ok(row.into())
    }

    /// TODO を部分更新する（単一の atomic UPDATE クエリ）
    ///
    /// # Arguments
    ///
    /// * `id` - 更新する TODO の ID
    /// * `user_id` - 所有者の ID（認可チェック用）
    /// * `title` - 新しいタイトル（None なら変更なし）
    /// * `description` - 新しい説明（None なら変更なし、Some(None) なら NULL に設定）
    /// * `completed` - 新しい完了状態（None なら変更なし）
    ///
    /// # Returns
    ///
    /// * `Ok(Todo)` - 更新後の TODO
    /// * `Err(DomainError::NotFound)` - 該当 TODO がない、または権限なし
    ///
    /// # SQL の説明
    ///
    /// COALESCE と CASE を使用して、指定されたフィールドのみを更新する。
    /// WHERE 句で user_id もチェックすることで、認可も同時に行う。
    ///
    /// ## description の特殊処理
    ///
    /// `description: Option<Option<String>>` は以下を区別:
    /// - `None` → フィールドを更新しない
    /// - `Some(None)` → NULL に設定する
    /// - `Some(Some("text"))` → "text" に設定する
    async fn update_fields(
        &self,
        id: Uuid,
        user_id: Uuid,
        title: Option<String>,
        description: Option<Option<String>>, // 二重 Option で「更新しない」と「NULL にする」を区別
        completed: Option<bool>,
    ) -> Result<Todo, DomainError> {
        debug!(todo_id = %id, user_id = %user_id, "Updating todo fields in PostgreSQL (Writer)");

        // 単一の atomic UPDATE クエリで部分更新を実行
        //
        // SQL の各部分の説明:
        // - COALESCE($3, title): $3 が NULL でなければ $3、NULL なら既存の title を使用
        // - CASE WHEN $4::boolean THEN $5::text ELSE description END:
        //   - $4 が true なら $5 の値を使用（NULL でも可）
        //   - $4 が false なら既存の description を維持
        // - COALESCE($6, completed): $6 が NULL でなければ $6、NULL なら既存の completed を使用
        let row: Option<TodoRow> = sqlx::query_as(
            r#"
            UPDATE todos
            SET title = COALESCE($3, title),
                description = CASE
                    WHEN $4::boolean THEN $5::text
                    ELSE description
                END,
                completed = COALESCE($6, completed),
                updated_at = NOW()
            WHERE id = $1 AND user_id = $2
            RETURNING id, user_id, title, description, completed, created_at, updated_at
            "#,
        )
        .bind(id) // $1: 更新対象の ID
        .bind(user_id) // $2: 所有者 ID（認可チェック）
        .bind(title) // $3: 新しいタイトル（Option<String> → NULL or 値）
        .bind(description.is_some()) // $4: description を更新するかどうか（bool）
        .bind(description.flatten()) // $5: 実際の description 値（Option::flatten で二重 Option を解除）
        .bind(completed) // $6: 新しい完了状態
        .fetch_optional(&self.pool) // 0件 or 1件を取得
        .await
        .map_err(|e| DomainError::Repository(e.to_string()))?;

        // Option<TodoRow> → Result<Todo, DomainError>
        // None の場合は NotFound エラー（該当なし or 権限なし）
        row.map(|r| r.into()).ok_or(DomainError::NotFound)
    }

    /// TODO を削除する（user_id による所有権チェック込み）
    ///
    /// # Arguments
    ///
    /// * `id` - 削除する TODO の ID
    /// * `user_id` - 所有者の ID（認可チェック用）
    ///
    /// # Returns
    ///
    /// * `Ok(true)` - 削除成功（1行削除）
    /// * `Ok(false)` - 該当なし（0行削除）
    /// * `Err(DomainError)` - DB エラー
    ///
    /// # Security
    ///
    /// WHERE 句で user_id もチェックすることで、
    /// 他ユーザーの TODO を削除できないようにしている。
    async fn delete(&self, id: Uuid, user_id: Uuid) -> Result<bool, DomainError> {
        debug!(todo_id = %id, user_id = %user_id, "Deleting todo from PostgreSQL (Writer)");

        // DELETE 文を実行（RETURNING は不要）
        let result = sqlx::query(
            r#"
            DELETE FROM todos
            WHERE id = $1 AND user_id = $2
            "#,
        )
        .bind(id) // $1: 削除対象の ID
        .bind(user_id) // $2: 所有者 ID（認可チェック）
        .execute(&self.pool) // 実行（結果の行数のみ取得）
        .await
        .map_err(|e| DomainError::Repository(e.to_string()))?;

        // rows_affected(): 影響を受けた行数
        // 1 以上なら削除成功、0 なら該当なし
        Ok(result.rows_affected() > 0)
    }
}
