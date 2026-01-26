// =============================================================================
// application/src/commands/update_todo.rs: TODO 更新コマンド
// =============================================================================
// 軽量 CQRS: 状態変更操作（Command）
// Writer DB プールを使用。
//
// CQRS ベストプラクティス:
// - 読み取り操作（find_by_id）を使用せず、単一の atomic UPDATE を実行
// - WHERE 句で id と user_id を指定し、認可と更新を同時に行う
// - 2回の DB ラウンドトリップを 1回に削減
//
// Write-Through キャッシュ:
// - 更新成功後、キャッシュを更新
// - キャッシュエラーは無視（メイン操作の成功を優先）
// =============================================================================

// -----------------------------------------------------------------------------
// 標準ライブラリのインポート
// -----------------------------------------------------------------------------

use std::sync::Arc; // スレッド安全な参照カウントスマートポインタ

// -----------------------------------------------------------------------------
// 外部クレートのインポート
// -----------------------------------------------------------------------------

use domain::{DomainError, Todo, TodoCacheOps, TodoWriter}; // ドメイン層の型
use tracing::{info, warn}; // 構造化ログ
use uuid::Uuid; // 一意識別子

// -----------------------------------------------------------------------------
// 同一クレート内のインポート
// -----------------------------------------------------------------------------

use crate::dto::UpdateTodoDto; // 更新リクエスト DTO

// =============================================================================
// TODO 更新コマンド構造体
// =============================================================================

/// TODO 更新コマンド
///
/// 単一の atomic UPDATE クエリで更新を実行する。
/// 読み取り操作を含まない純粋な CQRS Command。
///
/// # なぜ読み取りなしで更新するのか
///
/// 従来のパターン（2回の DB アクセス）:
/// 1. find_by_id() → 取得
/// 2. save() → 保存
///
/// 改善後のパターン（1回の DB アクセス）:
/// 1. UPDATE ... WHERE id = ? AND user_id = ?
///
/// メリット:
/// - パフォーマンス向上
/// - レースコンディションの回避
/// - atomic な操作を保証
pub struct UpdateTodoCommand<W: TodoWriter, C: TodoCacheOps> {
    /// 書き込みリポジトリ
    writer: Arc<W>,

    /// キャッシュ操作（オプショナル）
    cache: Option<Arc<C>>,
}

// -----------------------------------------------------------------------------
// Clone トレイト実装
// -----------------------------------------------------------------------------

impl<W: TodoWriter, C: TodoCacheOps> Clone for UpdateTodoCommand<W, C> {
    fn clone(&self) -> Self {
        Self {
            writer: Arc::clone(&self.writer),
            cache: self.cache.as_ref().map(Arc::clone),
        }
    }
}

// -----------------------------------------------------------------------------
// UpdateTodoCommand の実装
// -----------------------------------------------------------------------------

impl<W: TodoWriter, C: TodoCacheOps> UpdateTodoCommand<W, C> {
    /// 新しいコマンドを作成
    ///
    /// # Arguments
    /// * `writer` - TodoWriter の共有参照（Arc でラップ）
    /// * `cache` - オプションのキャッシュ（Write-Through 用）
    pub fn new(writer: Arc<W>, cache: Option<Arc<C>>) -> Self {
        Self { writer, cache }
    }

    /// TODO を更新する
    ///
    /// # Arguments
    /// * `id` - 更新する TODO の UUID
    /// * `user_id` - 所有者のユーザー ID（認可チェックに使用）
    /// * `dto` - 更新リクエスト DTO（全フィールドが Option）
    ///
    /// # Returns
    /// * `Ok(Todo)` - 更新された TODO
    /// * `Err(DomainError::NotFound)` - TODO が見つからないか、所有者でない
    /// * `Err(DomainError::Validation)` - タイトルが不正
    pub async fn execute(
        &self,
        id: Uuid,
        user_id: Uuid,
        dto: UpdateTodoDto,
    ) -> Result<Todo, DomainError> {
        // 1. タイトルのバリデーション（指定されている場合のみ）
        let title = match dto.title {
            Some(t) => Some(Todo::validate_title(&t)?),
            None => None, // 未指定は None のまま（変更なし）
        };

        // 2. description の変換
        // Option<String> → Option<Option<String>> に変換
        // - Some("value") → Some(Some("value")): 新しい値を設定
        // - None → None: 変更なし
        let description = dto.description.map(Some);

        // 3. 単一の atomic UPDATE クエリで更新
        // WHERE 句に user_id を含めて認可チェック
        let updated = self
            .writer
            .update_fields(id, user_id, title, description, dto.completed)
            .await?;

        // 4. Write-Through: キャッシュを更新（エラーは無視）
        if let Some(cache) = &self.cache {
            if let Err(e) = cache.set(&updated).await {
                warn!(todo_id = %updated.id, error = %e, "Failed to cache updated todo");
            }
        }

        // 5. ログ出力
        info!(todo_id = %updated.id, user_id = %updated.user_id, "Todo updated");

        Ok(updated)
    }
}
