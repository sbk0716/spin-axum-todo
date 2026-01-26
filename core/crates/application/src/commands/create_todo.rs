// =============================================================================
// application/src/commands/create_todo.rs: TODO 作成コマンド
// =============================================================================
// 軽量 CQRS: 状態変更操作（Command）
// Writer DB プールを使用。
//
// Write-Through キャッシュ:
// - 作成成功後、キャッシュに新しい TODO を保存
// - キャッシュエラーは無視（メイン操作の成功を優先）
//
// 処理フロー:
// 1. DTO からタイトルを取得してバリデーション
// 2. Todo エンティティを生成
// 3. Writer で DB に永続化
// 4. キャッシュに保存（Write-Through）
// 5. ログ出力して結果を返す
// =============================================================================

// -----------------------------------------------------------------------------
// 標準ライブラリのインポート
// -----------------------------------------------------------------------------

// Arc: スレッド安全な参照カウントスマートポインタ
// 複数のスレッドで Writer を共有するために使用
use std::sync::Arc;

// -----------------------------------------------------------------------------
// 外部クレートのインポート
// -----------------------------------------------------------------------------

// domain クレートからエンティティとトレイトをインポート
use domain::{DomainError, Todo, TodoCacheOps, TodoWriter};

// tracing: 構造化ログ出力
// info: 通常の情報ログ、warn: 警告ログ
use tracing::{info, warn};

// uuid: 一意識別子
use uuid::Uuid;

// -----------------------------------------------------------------------------
// 同一クレート内のインポート
// -----------------------------------------------------------------------------

// CreateTodoDto: 作成リクエスト DTO
use crate::dto::CreateTodoDto;

// =============================================================================
// TODO 作成コマンド構造体
// =============================================================================

/// TODO 作成コマンド
///
/// クリーンアーキテクチャでは、コマンドがアプリケーション固有の
/// ビジネスロジックをカプセル化する。
///
/// # ジェネリクス
///
/// - `W: TodoWriter` - 書き込みリポジトリの型
/// - `C: TodoCacheOps` - キャッシュ操作の型
///
/// トレイト境界を使うことで、任意の実装を受け入れる。
/// テスト時にはモック実装を注入できる（依存性注入）。
///
/// # Arc の役割
///
/// 複数のリクエストが同時に処理される Web サーバーでは、
/// Arc（Atomic Reference Counted）で Writer を共有する必要がある。
pub struct CreateTodoCommand<W: TodoWriter, C: TodoCacheOps> {
    /// 書き込みリポジトリ（Arc でラップして共有可能に）
    writer: Arc<W>,

    /// キャッシュ操作（オプショナル - キャッシュなしでも動作可能）
    cache: Option<Arc<C>>,
}

// -----------------------------------------------------------------------------
// Clone トレイト実装
// -----------------------------------------------------------------------------

/// Clone トレイトの手動実装
///
/// #[derive(Clone)] を使わない理由:
/// - derive(Clone) は W: Clone と C: Clone を要求する
/// - しかし Arc<W> は W: Clone がなくてもクローン可能
/// - 手動実装により、不要な制約を避けられる
impl<W: TodoWriter, C: TodoCacheOps> Clone for CreateTodoCommand<W, C> {
    fn clone(&self) -> Self {
        Self {
            // Arc::clone は参照カウントをインクリメントするだけ（効率的）
            writer: Arc::clone(&self.writer),
            // Option::as_ref() で参照を取得し、map で Some の場合のみクローン
            cache: self.cache.as_ref().map(Arc::clone),
        }
    }
}

// -----------------------------------------------------------------------------
// CreateTodoCommand の実装
// -----------------------------------------------------------------------------

impl<W: TodoWriter, C: TodoCacheOps> CreateTodoCommand<W, C> {
    /// 新しいコマンドを作成
    ///
    /// # Arguments
    /// * `writer` - TodoWriter の共有参照（Arc でラップ）
    /// * `cache` - オプションのキャッシュ（Write-Through 用）
    pub fn new(writer: Arc<W>, cache: Option<Arc<C>>) -> Self {
        Self { writer, cache }
    }

    /// TODO を作成する
    ///
    /// # Arguments
    /// * `user_id` - 所有者のユーザー ID（JWT から抽出）
    /// * `dto` - 作成リクエスト DTO
    ///
    /// # Returns
    /// * `Ok(Todo)` - 作成された TODO（ID、作成日時など含む）
    /// * `Err(DomainError::Validation)` - タイトルが空または長すぎる
    /// * `Err(DomainError::Repository)` - DB エラー
    pub async fn execute(&self, user_id: Uuid, dto: CreateTodoDto) -> Result<Todo, DomainError> {
        // 1. バリデーション（ドメインロジックを呼び出す）
        // Todo::validate_title はタイトルの長さと空白をチェック
        let title = Todo::validate_title(&dto.title)?;

        // 2. エンティティ作成（user_id を含む）
        // Todo::new は UUID を生成し、作成日時を設定
        let todo = Todo::new(user_id, title, dto.description);

        // 3. 永続化（Writer に委譲）
        // INSERT クエリを実行し、作成された TODO を返す
        let created = self.writer.create(&todo).await?;

        // 4. Write-Through: キャッシュに保存（エラーは無視）
        // キャッシュエラーはメイン操作に影響させない
        if let Some(cache) = &self.cache
            && let Err(e) = cache.set(&created).await
        {
            // 警告ログを出力して処理を続行
            warn!(todo_id = %created.id, error = %e, "Failed to cache created todo");
        }

        // 5. ログ出力（構造化ログ）
        info!(todo_id = %created.id, user_id = %created.user_id, title = %created.title, "Todo created");

        Ok(created)
    }
}
