// =============================================================================
// application/src/commands/delete_file.rs: ファイル削除コマンド
// =============================================================================
// ファイルをストレージとデータベースから削除するユースケース。
//
// Clean Architecture:
// - Presentation 層から呼び出される
// - Domain 層の FileReader, FileWriter, TodoReader, StorageOps トレイトに依存
// - アクセス制御: TODO の所有者のみがファイルを削除可能
//
// 処理フロー:
// 1. ファイルメタデータを取得（FileReader 経由）
// 2. 親 TODO の所有者を確認（TodoReader 経由）
// 3. ストレージから削除（StorageOps 経由）
// 4. DB からメタデータを削除（FileWriter 経由）
// 5. ログ出力
// =============================================================================

// -----------------------------------------------------------------------------
// 標準ライブラリのインポート
// -----------------------------------------------------------------------------

use std::sync::Arc;

// -----------------------------------------------------------------------------
// 外部クレートのインポート
// -----------------------------------------------------------------------------

use domain::{DomainError, FileReader, FileWriter, StorageOps, TodoReader};
use tracing::info;
use uuid::Uuid;

// =============================================================================
// DeleteFileCommand 構造体
// =============================================================================

/// ファイル削除コマンド
///
/// # ジェネリクス
///
/// - `TR: TodoReader` - TODO 読み取りの型（所有者確認用）
/// - `S: StorageOps` - ストレージ操作の型
///
/// # トレイトオブジェクト
///
/// - `file_reader: Arc<dyn FileReader>` - AppState と同じくトレイトオブジェクトを使用
/// - `file_writer: Arc<dyn FileWriter>` - AppState と同じくトレイトオブジェクトを使用
///
/// # アクセス制御
///
/// ファイルの所有者（= 親 TODO の所有者）のみが削除可能。
/// これにより他ユーザーのファイルへの不正アクセスを防止。
///
/// # 削除順序
///
/// 1. ストレージから削除
/// 2. DB からメタデータを削除
///
/// この順序により、DB 削除失敗時にストレージにゴミが残る可能性があるが、
/// これは許容する（参照されないファイルはストレージコストのみ）。
/// 逆の順序だと、DB 削除後にストレージ削除失敗した場合、
/// 参照できないのにストレージにファイルが残る問題は同じだが、
/// DB レコードがないためリカバリが難しくなる。
pub struct DeleteFileCommand<TR: TodoReader, S: StorageOps> {
    /// ファイルメタデータ読み取り（トレイトオブジェクト）
    file_reader: Arc<dyn FileReader>,
    /// ファイルメタデータ書き込み（トレイトオブジェクト）
    file_writer: Arc<dyn FileWriter>,
    /// TODO 読み取り（所有者確認用）
    todo_reader: Arc<TR>,
    /// ストレージ操作
    storage: Arc<S>,
}

// -----------------------------------------------------------------------------
// Clone トレイト実装
// -----------------------------------------------------------------------------

impl<TR: TodoReader, S: StorageOps> Clone for DeleteFileCommand<TR, S> {
    fn clone(&self) -> Self {
        Self {
            file_reader: Arc::clone(&self.file_reader),
            file_writer: Arc::clone(&self.file_writer),
            todo_reader: Arc::clone(&self.todo_reader),
            storage: Arc::clone(&self.storage),
        }
    }
}

// -----------------------------------------------------------------------------
// DeleteFileCommand の実装
// -----------------------------------------------------------------------------

impl<TR: TodoReader, S: StorageOps> DeleteFileCommand<TR, S> {
    /// 新しいコマンドを作成
    ///
    /// # Arguments
    /// * `file_reader` - FileReader のトレイトオブジェクト
    /// * `file_writer` - FileWriter のトレイトオブジェクト
    /// * `todo_reader` - TodoReader の共有参照
    /// * `storage` - StorageOps の共有参照
    pub fn new(
        file_reader: Arc<dyn FileReader>,
        file_writer: Arc<dyn FileWriter>,
        todo_reader: Arc<TR>,
        storage: Arc<S>,
    ) -> Self {
        Self {
            file_reader,
            file_writer,
            todo_reader,
            storage,
        }
    }

    /// ファイルを削除する
    ///
    /// # Arguments
    /// * `file_id` - 削除するファイルの ID
    /// * `user_id` - リクエストしたユーザーの ID
    ///
    /// # Returns
    /// * `Ok(())` - 削除成功
    /// * `Err(DomainError::NotFound)` - ファイルまたは TODO が見つからない
    /// * `Err(DomainError::External)` - ストレージエラー
    /// * `Err(DomainError::Repository)` - DB エラー
    ///
    /// # アクセス制御
    /// user_id が親 TODO の所有者でない場合、NotFound を返す。
    pub async fn execute(&self, file_id: Uuid, user_id: Uuid) -> Result<(), DomainError> {
        // 1. ファイルメタデータを取得
        let file = self
            .file_reader
            .find_by_id(file_id)
            .await?
            .ok_or(DomainError::NotFound)?;

        // 2. 親 TODO を取得して所有者を確認
        // find_by_id(todo_id, user_id) は user_id が所有者でない場合 None を返す
        let _todo = self
            .todo_reader
            .find_by_id(file.todo_id, user_id)
            .await?
            .ok_or(DomainError::NotFound)?;

        // 3. ストレージから削除
        // S3 の DELETE は冪等（存在しなくても成功）
        self.storage.delete(&file.storage_path).await?;

        // 4. DB からメタデータを削除
        self.file_writer.delete(file_id).await?;

        // 5. ログ出力
        info!(
            file_id = %file_id,
            user_id = %user_id,
            storage_path = %file.storage_path,
            "File deleted successfully"
        );

        Ok(())
    }
}
