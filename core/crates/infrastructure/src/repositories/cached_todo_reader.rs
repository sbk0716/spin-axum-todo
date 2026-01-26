// =============================================================================
// infrastructure/src/repositories/cached_todo_reader.rs
// =============================================================================
// TodoReader のキャッシュ付きデコレータ実装。
// Redis キャッシュを使用して読み取りパフォーマンスを向上させる。
//
// デコレータパターン:
// - TodoReader トレイトを実装しつつ、内部に別の TodoReader を保持
// - キャッシュ機能を透過的に追加
// - 呼び出し側はキャッシュの存在を意識しない
//
// 軽量 CQRS パターン:
// - Queries（参照操作）で使用
// - Reader Pool への負荷を軽減
//
// キャッシュの共有:
// - Commands（Write-Through / 無効化）と共有するため Arc<TodoCache> を使用
// - 書き込み時にキャッシュを更新/無効化することで整合性を維持
//
// キャッシュ戦略（Cache-Aside パターン）:
// 1. find_by_id 時にキャッシュを確認
// 2. ヒット → キャッシュから返却
// 3. ミス → DB から取得 → キャッシュに保存 → 返却
// =============================================================================

// -----------------------------------------------------------------------------
// 外部クレートのインポート
// -----------------------------------------------------------------------------

// std::sync::Arc: スレッド安全な参照カウントポインタ
// キャッシュを Commands と共有するために使用
use std::sync::Arc;

// async_trait: トレイトで async fn を使用可能にするマクロ
use async_trait::async_trait;

// domain: ドメイン層の型とトレイト
// DomainError: ドメインエラー型
// Todo: TODO エンティティ
// TodoCacheOps: キャッシュ操作トレイト
// TodoFilter: 一覧取得のフィルタ
// TodoReader: 読み取りトレイト
use domain::{DomainError, Todo, TodoCacheOps, TodoFilter, TodoReader};

// tracing: 構造化ログ
use tracing::debug;

// uuid: 一意識別子
use uuid::Uuid;

// crate: 自クレート内のモジュール
use crate::persistence::redis::TodoCache;

// =============================================================================
// CachedTodoReader 構造体
// =============================================================================

/// キャッシュ付き TodoReader（デコレータパターン）
///
/// 内部の TodoReader をラップし、find_by_id 時にキャッシュを確認する。
/// キャッシュミス時は Reader から取得し、結果をキャッシュに保存する。
///
/// # 型パラメータ
///
/// - `R`: 内部の TodoReader 実装（PostgresTodoReader など）
///
/// # 注意
///
/// - find_all（一覧取得）はキャッシュしない（フィルタ条件が多様なため）
/// - キャッシュの有効期限は TodoCache で管理（デフォルト 5 分）
/// - キャッシュは Commands と共有される（Arc<TodoCache>）
///
/// # 使用例
///
/// ```rust,ignore
/// let postgres_reader = PostgresTodoReader::new(pool);
/// let cache = Arc::new(TodoCache::new(redis_client));
/// let reader = CachedTodoReader::new(postgres_reader, Arc::clone(&cache));
///
/// // キャッシュを確認し、なければ DB から取得
/// let todo = reader.find_by_id(id, user_id).await?;
/// ```
pub struct CachedTodoReader<R: TodoReader> {
    /// 内部の TodoReader 実装
    ///
    /// キャッシュミス時にこの Reader から取得する
    reader: R,

    /// Redis キャッシュ
    ///
    /// Arc でラップして Commands と共有する
    cache: Arc<TodoCache>,
}

impl<R: TodoReader> CachedTodoReader<R> {
    /// 新しい CachedTodoReader を作成する
    ///
    /// # Arguments
    ///
    /// * `reader` - 内部の TodoReader 実装（PostgresTodoReader など）
    /// * `cache` - Redis キャッシュ（Arc でラップ、Commands と共有）
    ///
    /// # Returns
    ///
    /// 新しい CachedTodoReader インスタンス
    pub fn new(reader: R, cache: Arc<TodoCache>) -> Self {
        Self { reader, cache }
    }
}

// =============================================================================
// TodoReader トレイト実装
// =============================================================================

/// TodoReader トレイトの実装
///
/// # async_trait マクロ
///
/// Rust のトレイトでは async fn を直接定義できないため、
/// async_trait マクロを使用して非同期メソッドを実装する。
#[async_trait]
impl<R: TodoReader> TodoReader for CachedTodoReader<R> {
    /// ID とユーザー ID で TODO を取得する（キャッシュ対応）
    ///
    /// # 処理フロー
    ///
    /// 1. キャッシュを確認
    /// 2. キャッシュヒット時:
    ///    - user_id が一致すれば返却
    ///    - 一致しなければ None（セキュリティ対策）
    /// 3. キャッシュミス時:
    ///    - Reader から取得
    ///    - キャッシュに保存（ベストエフォート）
    ///    - 結果を返却
    ///
    /// # Arguments
    ///
    /// * `id` - TODO の ID
    /// * `user_id` - ユーザー ID（所有権チェック用）
    ///
    /// # Returns
    ///
    /// - `Ok(Some(todo))`: TODO が見つかった
    /// - `Ok(None)`: TODO が見つからない、または所有者が異なる
    /// - `Err(DomainError)`: エラーが発生
    async fn find_by_id(&self, id: Uuid, user_id: Uuid) -> Result<Option<Todo>, DomainError> {
        // ---------------------------------------------------------------------
        // キャッシュを確認
        // ---------------------------------------------------------------------
        match self.cache.get(id).await {
            // キャッシュヒット
            Ok(Some(todo)) => {
                // user_id が一致するか確認（セキュリティチェック）
                if todo.user_id == user_id {
                    // 所有者が一致 → キャッシュから返却
                    debug!(todo_id = %id, "Cache hit for todo");
                    return Ok(Some(todo));
                }
                // user_id が一致しない → None を返す（他ユーザーの TODO は見せない）
                debug!(todo_id = %id, "Cache hit but user_id mismatch");
                return Ok(None);
            }

            // キャッシュミス
            Ok(None) => {
                debug!(todo_id = %id, "Cache miss for todo");
                // Reader から取得する（後続の処理へ）
            }

            // キャッシュエラー
            Err(e) => {
                // キャッシュエラーは警告のみでスキップ（Reader にフォールバック）
                // キャッシュは可用性向上のためのものなので、エラー時は DB から取得
                tracing::warn!(todo_id = %id, error = %e, "Cache error, falling back to reader");
            }
        }

        // ---------------------------------------------------------------------
        // Reader から取得
        // ---------------------------------------------------------------------
        let todo = self.reader.find_by_id(id, user_id).await?;

        // ---------------------------------------------------------------------
        // 取得できた場合はキャッシュに保存
        // ---------------------------------------------------------------------
        if let Some(ref t) = todo {
            // キャッシュに保存（ベストエフォート）
            if let Err(e) = self.cache.set(t).await {
                // キャッシュ保存エラーは警告のみ（処理は続行）
                // DB からの取得は成功しているので、キャッシュ保存失敗は許容
                tracing::warn!(todo_id = %id, error = %e, "Failed to cache todo");
            }
        }

        Ok(todo)
    }

    /// TODO 一覧を取得する（キャッシュなし）
    ///
    /// 一覧取得はフィルタ条件が多様なため、キャッシュしない。
    /// Reader から直接取得する。
    ///
    /// # なぜキャッシュしないのか
    ///
    /// - フィルタ条件（completed, 日付範囲など）の組み合わせが多い
    /// - キャッシュキーの設計が複雑になる
    /// - キャッシュ無効化のタイミングが難しい
    /// - 個別エンティティのキャッシュで十分な効果が得られる
    ///
    /// # Arguments
    ///
    /// * `filter` - フィルタ条件（user_id, completed など）
    ///
    /// # Returns
    ///
    /// フィルタ条件に一致する TODO のリスト
    async fn find_all(&self, filter: TodoFilter) -> Result<Vec<Todo>, DomainError> {
        // 一覧取得はキャッシュせず、直接 Reader から取得
        self.reader.find_all(filter).await
    }
}
