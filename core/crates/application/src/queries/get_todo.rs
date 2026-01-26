// =============================================================================
// application/src/queries/get_todo.rs: TODO 取得クエリ
// =============================================================================
// 軽量 CQRS: 参照操作（Query）
// Reader DB プールを使用。
//
// 処理フロー:
// 1. TodoReader::find_by_id() を呼び出し
// 2. Option<Todo> を Result<Todo, DomainError> に変換
//
// 認可:
// - find_by_id() に user_id を渡し、所有者のみ取得可能にする
// - WHERE id = ? AND user_id = ? でフィルタリング
// =============================================================================

// -----------------------------------------------------------------------------
// 標準ライブラリのインポート
// -----------------------------------------------------------------------------

use std::sync::Arc; // スレッド安全な参照カウントスマートポインタ

// -----------------------------------------------------------------------------
// 外部クレートのインポート
// -----------------------------------------------------------------------------

use domain::{DomainError, Todo, TodoReader}; // ドメイン層の型
use uuid::Uuid; // 一意識別子

// =============================================================================
// TODO 取得クエリ構造体
// =============================================================================

/// TODO 取得クエリ
///
/// 単一の TODO を ID で取得する。
/// 所有者のみ取得可能（user_id で認可チェック）。
///
/// # ジェネリクス
///
/// - `R: TodoReader` - 読み取りリポジトリの型
///
/// CachedTodoReader を注入すれば、キャッシュを透過的に使用できる。
pub struct GetTodoQuery<R: TodoReader> {
    /// 読み取りリポジトリ
    reader: Arc<R>,
}

// -----------------------------------------------------------------------------
// Clone トレイト実装
// -----------------------------------------------------------------------------

impl<R: TodoReader> Clone for GetTodoQuery<R> {
    fn clone(&self) -> Self {
        Self {
            reader: Arc::clone(&self.reader),
        }
    }
}

// -----------------------------------------------------------------------------
// GetTodoQuery の実装
// -----------------------------------------------------------------------------

impl<R: TodoReader> GetTodoQuery<R> {
    /// 新しいクエリを作成
    ///
    /// # Arguments
    /// * `reader` - TodoReader の共有参照（Arc でラップ）
    pub fn new(reader: Arc<R>) -> Self {
        Self { reader }
    }

    /// ID とユーザー ID で TODO を取得する
    ///
    /// # Arguments
    /// * `id` - 取得する TODO の UUID
    /// * `user_id` - 所有者のユーザー ID（認可チェックに使用）
    ///
    /// # Returns
    /// * `Ok(Todo)` - 見つかった TODO
    /// * `Err(DomainError::NotFound)` - 見つからないか、所有者でない場合
    ///
    /// # Option から Result への変換
    ///
    /// `find_by_id()` は `Option<Todo>` を返すが、
    /// このクエリは見つからない場合をエラーとして扱う。
    /// `ok_or()` で `None` を `DomainError::NotFound` に変換。
    pub async fn execute(&self, id: Uuid, user_id: Uuid) -> Result<Todo, DomainError> {
        // find_by_id() は Option<Todo> を返す
        // None の場合は NotFound エラーに変換
        self.reader
            .find_by_id(id, user_id)
            .await? // DB エラーがあれば伝播
            .ok_or(DomainError::NotFound) // None → NotFound
    }
}
