// =============================================================================
// domain/src/repositories/mod.rs: リポジトリモジュール
// =============================================================================
// クリーンアーキテクチャにおける「リポジトリパターン」を定義するモジュール。
//
// リポジトリパターンとは:
// - 永続化（データベース操作）の抽象インターフェース
// - ドメイン層がトレイト（インターフェース）を定義
// - インフラ層がトレイトを実装
// - これにより、ビジネスロジックが特定のデータベースに依存しなくなる
//
// 統一 CQRS（Command Query Responsibility Segregation）パターン:
// - Writer トレイト: 状態変更操作（Create, Update, Delete）
// - Reader トレイト: 参照操作（Read, List）
// - 全エンティティで一貫したパターンを適用
//
// このプロジェクトのリポジトリトレイト:
// - Todo: TodoWriter / TodoReader
// - User: UserWriter / UserReader
// - File: FileWriter / FileReader
// - Cache: TodoCacheOps（キャッシュ操作）
// - Storage: StorageOps（ファイルストレージ操作）
// =============================================================================

// -----------------------------------------------------------------------------
// サブモジュールの宣言
// -----------------------------------------------------------------------------

/// File エンティティのリポジトリトレイトを定義
mod file_repository;

/// ストレージ操作トレイトを定義
mod storage_repository;

/// TODO キャッシュ操作トレイトを定義
mod todo_cache;

/// Todo エンティティのリポジトリトレイトを定義
mod todo_repository;

/// User エンティティのリポジトリトレイトを定義
mod user_repository;

// -----------------------------------------------------------------------------
// 再エクスポート（Re-export）
// -----------------------------------------------------------------------------
// pub use により、サブモジュールの型を親モジュールから直接アクセス可能にする。
// domain::repositories::file_repository::FileReader
//   → domain::repositories::FileReader
//   → domain::FileReader（lib.rs で再エクスポート）
// -----------------------------------------------------------------------------

/// File の読み取り/書き込みトレイトを再エクスポート
pub use file_repository::{FileReader, FileWriter};

/// ストレージ操作トレイトを再エクスポート
pub use storage_repository::StorageOps;

/// TODO キャッシュ操作トレイトを再エクスポート
pub use todo_cache::TodoCacheOps;

/// Todo のフィルタ条件と読み取り/書き込みトレイトを再エクスポート
pub use todo_repository::{TodoFilter, TodoReader, TodoWriter};

/// User の読み取り/書き込みトレイトを再エクスポート
pub use user_repository::{UserReader, UserWriter};
