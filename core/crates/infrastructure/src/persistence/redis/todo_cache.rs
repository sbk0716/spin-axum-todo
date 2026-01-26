// =============================================================================
// infrastructure/src/persistence/redis/todo_cache.rs
// =============================================================================
// Redis を使用した TODO キャッシュの実装。
// キャッシュの有効期限は 5 分（300 秒）に設定。
//
// TodoCacheOps トレイトを実装し、Commands からのキャッシュ操作を可能にする。
//
// キャッシュの共有:
// - CachedTodoReader（Queries）: get メソッドを使用
// - Commands（CreateTodo など）: set/delete メソッドを使用
// - 両方で Arc<TodoCache> を共有して一貫性を保つ
// =============================================================================

// -----------------------------------------------------------------------------
// 外部クレートのインポート
// -----------------------------------------------------------------------------

// async_trait: async fn を含むトレイトを定義可能にする
use async_trait::async_trait;

// domain: ドメイン層の型をインポート
// TodoCacheOps: キャッシュ操作のトレイト（ドメイン層で定義）
use domain::{DomainError, Todo, TodoCacheOps};

// redis: Redis クライアントライブラリ
// AsyncCommands: 非同期コマンドのトレイト（get, set_ex, del など）
use redis::AsyncCommands;

// tracing: 構造化ログライブラリ
use tracing::debug;

// uuid: 一意識別子ライブラリ
use uuid::Uuid;

// =============================================================================
// 定数
// =============================================================================

/// キャッシュの有効期限（秒）
///
/// 5分（300秒）に設定。
/// - 短すぎる: キャッシュヒット率が下がる
/// - 長すぎる: データの一貫性が低下
/// - 5分は一般的なセッション中の操作に適切
const CACHE_TTL_SECONDS: u64 = 300;

// =============================================================================
// TodoCache 構造体
// =============================================================================

/// Redis を使用した TODO キャッシュ
///
/// # 責務
///
/// - TODO の取得（キャッシュヒット/ミス判定）
/// - TODO の保存（TTL 付き）
/// - TODO の削除（キャッシュ無効化）
///
/// # キーフォーマット
///
/// `todo:{uuid}` の形式でキーを生成。
/// 例: `todo:550e8400-e29b-41d4-a716-446655440000`
///
/// # シリアライズ
///
/// serde_json を使用して Todo を JSON 文字列として保存。
pub struct TodoCache {
    /// Redis クライアント
    /// 接続プールは内部で管理される
    client: redis::Client,
}

impl TodoCache {
    /// 新しい TodoCache を作成する
    ///
    /// # Arguments
    ///
    /// * `client` - Redis クライアント
    ///
    /// # Note
    ///
    /// クライアントは `redis::Client::open("redis://localhost:6379")` で作成。
    /// 接続は実際の操作時に確立される（遅延接続）。
    pub fn new(client: redis::Client) -> Self {
        Self { client }
    }

    /// キャッシュキーを生成する
    ///
    /// # Arguments
    ///
    /// * `id` - TODO の UUID
    ///
    /// # Returns
    ///
    /// `todo:{uuid}` 形式のキー文字列
    fn cache_key(id: Uuid) -> String {
        format!("todo:{}", id) // format! マクロで文字列を組み立て
    }

    /// キャッシュから TODO を取得する（CachedTodoReader で使用）
    ///
    /// # Arguments
    ///
    /// * `id` - 取得する TODO の UUID
    ///
    /// # Returns
    ///
    /// * `Ok(Some(Todo))` - キャッシュヒット
    /// * `Ok(None)` - キャッシュミス
    /// * `Err(DomainError::Cache)` - Redis エラー
    ///
    /// # Note
    ///
    /// このメソッドは pub であり、CachedTodoReader から呼び出される。
    /// TodoCacheOps トレイトには含まれない（読み取り専用操作のため）。
    pub async fn get(&self, id: Uuid) -> Result<Option<Todo>, DomainError> {
        // 構造化ログ: TODO ID を記録
        debug!(todo_id = %id, "Getting todo from Redis cache");

        // 非同期マルチプレクス接続を取得
        // マルチプレクス: 単一の TCP 接続で複数のコマンドを処理
        let mut conn = self
            .client
            .get_multiplexed_async_connection() // 非同期接続を取得
            .await // 接続完了を待機
            .map_err(|e| DomainError::Cache(e.to_string()))?; // エラー変換

        // キャッシュキーを生成
        let key = Self::cache_key(id);

        // GET コマンドを実行
        // Option<String>: キーが存在すれば Some、なければ None
        let value: Option<String> = conn
            .get(&key) // Redis GET コマンド
            .await // 非同期実行
            .map_err(|e| DomainError::Cache(e.to_string()))?; // エラー変換

        // match 式で Option を処理
        match value {
            // キャッシュヒット: JSON をデシリアライズ
            Some(json) => {
                // serde_json::from_str: JSON 文字列を Todo に変換
                let todo: Todo =
                    serde_json::from_str(&json).map_err(|e| DomainError::Cache(e.to_string()))?;
                debug!(todo_id = %id, "Cache hit");
                Ok(Some(todo))
            }
            // キャッシュミス: None を返す
            None => {
                debug!(todo_id = %id, "Cache miss");
                Ok(None)
            }
        }
    }
}

// =============================================================================
// TodoCacheOps トレイト実装
// =============================================================================

/// TodoCacheOps トレイトの実装
///
/// Commands からキャッシュ操作を行うためのインターフェース。
/// ドメイン層で定義されたトレイトをインフラ層で実装している
/// （依存性逆転の原則）。
#[async_trait]
impl TodoCacheOps for TodoCache {
    /// TODO をキャッシュに保存する
    ///
    /// # Arguments
    ///
    /// * `todo` - 保存する TODO エンティティ
    ///
    /// # Returns
    ///
    /// * `Ok(())` - 保存成功
    /// * `Err(DomainError::Cache)` - Redis エラー
    ///
    /// # Note
    ///
    /// TTL（有効期限）付きで保存するため、自動的に期限切れになる。
    /// 明示的な削除が不要な場面では、TTL による自動失効を活用。
    async fn set(&self, todo: &Todo) -> Result<(), DomainError> {
        // 構造化ログ: TODO ID を記録
        debug!(todo_id = %todo.id, "Caching todo in Redis");

        // 非同期マルチプレクス接続を取得
        let mut conn = self
            .client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| DomainError::Cache(e.to_string()))?;

        // キャッシュキーを生成
        let key = Self::cache_key(todo.id);

        // Todo を JSON 文字列にシリアライズ
        // serde_json::to_string: 構造体を JSON に変換
        let value = serde_json::to_string(todo).map_err(|e| DomainError::Cache(e.to_string()))?;

        // SETEX コマンドを実行（SET with EXpire）
        // set_ex: キーに値を設定し、TTL を指定
        // let _: () は結果を破棄することを明示
        let _: () = conn
            .set_ex(&key, value, CACHE_TTL_SECONDS) // SETEX key value seconds
            .await // 非同期実行
            .map_err(|e| DomainError::Cache(e.to_string()))?; // エラー変換

        Ok(())
    }

    /// キャッシュから TODO を削除する
    ///
    /// # Arguments
    ///
    /// * `id` - 削除する TODO の UUID
    ///
    /// # Returns
    ///
    /// * `Ok(())` - 削除成功（キーが存在しなくても成功）
    /// * `Err(DomainError::Cache)` - Redis エラー
    ///
    /// # Note
    ///
    /// TODO の更新/削除時にキャッシュを無効化するために使用。
    /// キーが存在しなくてもエラーにならない（冪等性）。
    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        // 構造化ログ: TODO ID を記録
        debug!(todo_id = %id, "Deleting todo from Redis cache");

        // 非同期マルチプレクス接続を取得
        let mut conn = self
            .client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| DomainError::Cache(e.to_string()))?;

        // キャッシュキーを生成
        let key = Self::cache_key(id);

        // DEL コマンドを実行
        // del: キーを削除（存在しなくてもエラーにならない）
        let _: () = conn
            .del(&key) // Redis DEL コマンド
            .await // 非同期実行
            .map_err(|e| DomainError::Cache(e.to_string()))?; // エラー変換

        Ok(())
    }
}
