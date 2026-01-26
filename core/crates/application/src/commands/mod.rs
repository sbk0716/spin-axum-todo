// =============================================================================
// application/src/commands/mod.rs: コマンド（状態変更操作）モジュール
// =============================================================================
// 軽量 CQRS パターン: Commands は状態を変更する操作を担当。
// Writer DB プールを使用し、データベースの書き込み操作を行う。
//
// CQRS（Command Query Responsibility Segregation）とは:
// - コマンド（書き込み）とクエリ（読み取り）を分離するパターン
// - このプロジェクトでは「軽量 CQRS」を採用
//   - 同一のデータモデルを使用（イベントソーシングは使用しない）
//   - Reader/Writer で DB プールを分離（レプリケーション対応可能）
//
// 各コマンドの責務:
// 1. リクエスト DTO の受け取り
// 2. バリデーション（ドメインロジックを呼び出し）
// 3. Writer リポジトリを使用した永続化
// 4. キャッシュ操作（Write-Through または Invalidation）
// =============================================================================

// -----------------------------------------------------------------------------
// サブモジュールの宣言
// -----------------------------------------------------------------------------

/// TODO 作成コマンド
mod create_todo;

/// ファイル削除コマンド
mod delete_file;

/// TODO 削除コマンド
mod delete_todo;

/// ファイルアップロードコマンド
mod upload_file;

/// TODO 更新コマンド
mod update_todo;

// -----------------------------------------------------------------------------
// 再エクスポート（Re-export）
// -----------------------------------------------------------------------------

/// CreateTodoCommand を公開
pub use create_todo::CreateTodoCommand;

/// DeleteFileCommand を公開
pub use delete_file::DeleteFileCommand;

/// DeleteTodoCommand を公開
pub use delete_todo::DeleteTodoCommand;

/// UploadFileCommand, UploadFileResult を公開
pub use upload_file::{UploadFileCommand, UploadFileResult};

/// UpdateTodoCommand を公開
pub use update_todo::UpdateTodoCommand;
