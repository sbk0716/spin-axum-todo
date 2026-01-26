// =============================================================================
// application/src/commands/delete_todo.rs: TODO 削除コマンド
// =============================================================================
// 軽量 CQRS: 状態変更操作（Command）
// Writer DB プールを使用。
//
// キャッシュ無効化:
// - 削除成功後、キャッシュから該当エントリを削除
// - キャッシュエラーは無視（メイン操作の成功を優先）
//
// CASCADE 削除:
// - DB の外部キー制約により、関連する File も自動削除される
// =============================================================================

// -----------------------------------------------------------------------------
// 標準ライブラリのインポート
// -----------------------------------------------------------------------------

use std::sync::Arc; // スレッド安全な参照カウントスマートポインタ

// -----------------------------------------------------------------------------
// 外部クレートのインポート
// -----------------------------------------------------------------------------

use domain::{DomainError, TodoCacheOps, TodoWriter}; // ドメイン層の型
use tracing::{info, warn}; // 構造化ログ
use uuid::Uuid; // 一意識別子

// =============================================================================
// TODO 削除コマンド構造体
// =============================================================================

/// TODO 削除コマンド
///
/// # 認可
/// WHERE 句に `user_id = ?` を含め、所有者のみ削除可能にする。
///
/// # CASCADE 削除
/// DB スキーマで files.todo_id に ON DELETE CASCADE を設定済み。
/// TODO 削除時、関連ファイルメタデータも自動削除される。
pub struct DeleteTodoCommand<W: TodoWriter, C: TodoCacheOps> {
    /// 書き込みリポジトリ
    writer: Arc<W>,

    /// キャッシュ操作（オプショナル）
    cache: Option<Arc<C>>,
}

// -----------------------------------------------------------------------------
// Clone トレイト実装
// -----------------------------------------------------------------------------

impl<W: TodoWriter, C: TodoCacheOps> Clone for DeleteTodoCommand<W, C> {
    fn clone(&self) -> Self {
        Self {
            writer: Arc::clone(&self.writer),
            cache: self.cache.as_ref().map(Arc::clone),
        }
    }
}

// -----------------------------------------------------------------------------
// DeleteTodoCommand の実装
// -----------------------------------------------------------------------------

impl<W: TodoWriter, C: TodoCacheOps> DeleteTodoCommand<W, C> {
    /// 新しいコマンドを作成
    ///
    /// # Arguments
    /// * `writer` - TodoWriter の共有参照（Arc でラップ）
    /// * `cache` - オプションのキャッシュ（無効化用）
    pub fn new(writer: Arc<W>, cache: Option<Arc<C>>) -> Self {
        Self { writer, cache }
    }

    /// TODO を削除する
    ///
    /// # Arguments
    /// * `id` - 削除する TODO の UUID
    /// * `user_id` - 所有者のユーザー ID（認可チェックに使用）
    ///
    /// # Returns
    /// * `Ok(())` - 削除成功
    /// * `Err(DomainError::NotFound)` - TODO が見つからないか、所有者でない
    /// * `Err(DomainError::Repository)` - DB エラー
    pub async fn execute(&self, id: Uuid, user_id: Uuid) -> Result<(), DomainError> {
        // 1. DB から削除（WHERE id = ? AND user_id = ?）
        let deleted = self.writer.delete(id, user_id).await?;

        // 2. 削除結果を確認
        // deleted = false: TODO が存在しない or 所有者でない
        if !deleted {
            return Err(DomainError::NotFound);
        }

        // 3. キャッシュ無効化（エラーは無視）
        if let Some(cache) = &self.cache
            && let Err(e) = cache.delete(id).await
        {
            warn!(todo_id = %id, error = %e, "Failed to invalidate cache for deleted todo");
        }

        // 4. ログ出力
        info!(todo_id = %id, user_id = %user_id, "Todo deleted");

        // 成功を表す () を返す
        Ok(())
    }
}
