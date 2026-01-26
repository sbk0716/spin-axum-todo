// =============================================================================
// infrastructure/src/persistence/postgres/todo_reader.rs
// =============================================================================
// PostgreSQL を使用した TodoReader の実装。
// 軽量 CQRS パターン: 参照操作（Queries）を担当。
// Reader DB プール（DATABASE_READER_URL）を使用する。
//
// 特徴:
// - SQLx の compile-time query チェック機能を活用
// - FromRow derive マクロで自動マッピング
// - async/await による非同期 I/O
// =============================================================================

// -----------------------------------------------------------------------------
// 外部クレートのインポート
// -----------------------------------------------------------------------------

// async_trait: async fn を含むトレイトを定義可能にする
// Rust の async fn in traits は不安定機能のため、このマクロが必要
use async_trait::async_trait;

// chrono: 日付・時刻ライブラリ
// DateTime<Utc>: UTC タイムゾーンの日時型
use chrono::{DateTime, Utc};

// domain: ドメイン層の型をインポート
// DomainError: エラー型、Todo: エンティティ、TodoFilter: 検索条件
// TodoReader: 読み取り操作を定義するトレイト
use domain::{DomainError, Todo, TodoFilter, TodoReader};

// sqlx: PostgreSQL クライアントライブラリ
// FromRow: クエリ結果から構造体への自動マッピング
// PgPool: PostgreSQL 接続プール
use sqlx::{FromRow, PgPool};

// tracing: 構造化ログライブラリ
use tracing::debug;

// uuid: 一意識別子ライブラリ
use uuid::Uuid;

// =============================================================================
// PostgresTodoReader 構造体
// =============================================================================

/// PostgreSQL からの読み取りを管理するリポジトリ
///
/// 軽量 CQRS パターンにおける Reader 実装。
/// Queries（find_by_id, find_all）で使用する。
///
/// # フィールド
///
/// - `pool`: PostgreSQL 接続プール（Reader 用）
///
/// # 使用例
///
/// ```rust,ignore
/// // Reader プールから作成
/// let reader = PostgresTodoReader::new(pools.reader.clone());
///
/// // ID で TODO を検索
/// let todo = reader.find_by_id(id, user_id).await?;
///
/// // フィルタ条件で一覧取得
/// let todos = reader.find_all(TodoFilter::new(user_id)).await?;
/// ```
pub struct PostgresTodoReader {
    /// PostgreSQL 接続プール
    /// Clone は安価（内部は Arc でラップ）
    pool: PgPool,
}

impl PostgresTodoReader {
    /// 新しい PostgresTodoReader を作成する
    ///
    /// # Arguments
    ///
    /// * `pool` - PostgreSQL 接続プール（Reader 用）
    ///
    /// # Note
    ///
    /// PgPool は内部的に Arc でラップされているため、
    /// clone() しても実際のコネクションはコピーされない。
    pub fn new(pool: PgPool) -> Self {
        Self { pool } // フィールド初期化の省略記法
    }
}

// =============================================================================
// TodoRow 構造体（内部用）
// =============================================================================

/// データベース行からのマッピング用構造体
///
/// SQLx の `query_as` で使用する内部構造体。
/// `#[derive(FromRow)]` により、SQL クエリ結果から自動的に構造体を生成。
///
/// # derive マクロの説明
///
/// - `FromRow`: SQLx が提供するマクロ。SELECT 結果の各カラムを
///   同名のフィールドに自動マッピングする。
///
/// # Note
///
/// この構造体は pub でないため、モジュール外からはアクセス不可。
/// 外部には domain::Todo として返す（From トレイト使用）。
#[derive(FromRow)]
struct TodoRow {
    /// TODO の一意識別子（PRIMARY KEY）
    id: Uuid,
    /// 所有者のユーザー ID（外部キー）
    user_id: Uuid,
    /// タイトル（必須、1〜100文字）
    title: String,
    /// 詳細説明（任意）
    /// Option<String>: NULL 許容カラム
    description: Option<String>,
    /// 完了フラグ
    completed: bool,
    /// 作成日時（UTC）
    created_at: DateTime<Utc>,
    /// 更新日時（UTC）
    updated_at: DateTime<Utc>,
}

// -----------------------------------------------------------------------------
// From トレイト実装: TodoRow → Todo 変換
// -----------------------------------------------------------------------------

/// TodoRow から domain::Todo への変換
///
/// `From` トレイトを実装することで、`.into()` メソッドが使用可能になる。
/// これにより `row.into()` で自動的に Todo に変換される。
impl From<TodoRow> for Todo {
    /// TodoRow を Todo に変換する
    ///
    /// # Arguments
    ///
    /// * `row` - データベースから取得した行データ
    ///
    /// # Returns
    ///
    /// domain::Todo エンティティ
    fn from(row: TodoRow) -> Self {
        // Todo::from_raw: バリデーションをスキップして直接作成
        // DB から取得したデータは既にバリデーション済みの前提
        Todo::from_raw(
            row.id,          // UUID: 主キー
            row.user_id,     // UUID: 所有者
            row.title,       // String: タイトル
            row.description, // Option<String>: 説明
            row.completed,   // bool: 完了フラグ
            row.created_at,  // DateTime<Utc>: 作成日時
            row.updated_at,  // DateTime<Utc>: 更新日時
        )
    }
}

// =============================================================================
// TodoReader トレイト実装
// =============================================================================

/// TodoReader トレイトの PostgreSQL 実装
///
/// `#[async_trait]` により、async fn を含むトレイトを実装可能。
/// これは Rust の言語制限を回避するためのマクロ。
#[async_trait]
impl TodoReader for PostgresTodoReader {
    /// ID とユーザー ID で TODO を検索する
    ///
    /// # Arguments
    ///
    /// * `id` - 検索する TODO の UUID
    /// * `user_id` - 所有者のユーザー ID（認可チェック用）
    ///
    /// # Returns
    ///
    /// * `Ok(Some(todo))` - 見つかった場合
    /// * `Ok(None)` - 見つからない場合
    /// * `Err(DomainError)` - DB エラーの場合
    ///
    /// # Security
    ///
    /// WHERE 句で user_id もチェックすることで、
    /// 他ユーザーの TODO にアクセスできないようにしている。
    async fn find_by_id(&self, id: Uuid, user_id: Uuid) -> Result<Option<Todo>, DomainError> {
        // 構造化ログ: todo_id と user_id をフィールドとして出力
        debug!(todo_id = %id, user_id = %user_id, "Finding todo by ID in PostgreSQL (Reader)");

        // sqlx::query_as: 結果を指定した型（TodoRow）にマッピング
        // r#"..."#: raw string literal（エスケープ不要）
        let row: Option<TodoRow> = sqlx::query_as(
            r#"
            SELECT id, user_id, title, description, completed, created_at, updated_at
            FROM todos
            WHERE id = $1 AND user_id = $2
            "#,
        )
        .bind(id) // $1 にバインド（SQL インジェクション対策）
        .bind(user_id) // $2 にバインド
        .fetch_optional(&self.pool) // 0件 or 1件を取得
        .await // 非同期実行を待機
        .map_err(|e| DomainError::Repository(e.to_string()))?; // エラー変換 + 伝播

        // Option::map + Into::into で TodoRow を Todo に変換
        // None の場合は None のまま
        Ok(row.map(Into::into))
    }

    /// フィルタ条件に基づいて TODO 一覧を取得する
    ///
    /// # Arguments
    ///
    /// * `filter` - 検索条件（user_id 必須、completed 任意）
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<Todo>)` - TODO のリスト（0件も可）
    /// * `Err(DomainError)` - DB エラーの場合
    ///
    /// # Note
    ///
    /// 結果は created_at DESC でソート（新しい順）。
    async fn find_all(&self, filter: TodoFilter) -> Result<Vec<Todo>, DomainError> {
        // ?filter: Debug トレイトでフィルタ内容を表示
        debug!(?filter, "Finding all todos in PostgreSQL (Reader)");

        // match 式: filter.completed の有無でクエリを分岐
        let rows: Vec<TodoRow> = match filter.completed {
            // Some(completed): completed フィルタが指定されている場合
            Some(completed) => sqlx::query_as(
                r#"
                SELECT id, user_id, title, description, completed, created_at, updated_at
                FROM todos
                WHERE user_id = $1 AND completed = $2
                ORDER BY created_at DESC
                "#,
            )
            .bind(filter.user_id) // $1: ユーザー ID
            .bind(completed) // $2: 完了フラグ
            .fetch_all(&self.pool) // 全件取得
            .await // 非同期実行
            .map_err(|e| DomainError::Repository(e.to_string()))?, // エラー変換

            // None: completed フィルタなし（全件取得）
            None => sqlx::query_as(
                r#"
                SELECT id, user_id, title, description, completed, created_at, updated_at
                FROM todos
                WHERE user_id = $1
                ORDER BY created_at DESC
                "#,
            )
            .bind(filter.user_id) // $1: ユーザー ID のみ
            .fetch_all(&self.pool) // 全件取得
            .await // 非同期実行
            .map_err(|e| DomainError::Repository(e.to_string()))?, // エラー変換
        };

        // Vec<TodoRow> → Vec<Todo> への変換
        // into_iter(): 所有権を移動するイテレータ
        // map(Into::into): 各要素を Into トレイトで変換
        // collect(): イテレータから Vec を構築
        Ok(rows.into_iter().map(Into::into).collect())
    }
}
