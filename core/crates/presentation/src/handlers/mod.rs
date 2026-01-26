// =============================================================================
// presentation/src/handlers/mod.rs: ハンドラモジュール
// =============================================================================
// HTTP リクエストを処理するハンドラ関数を定義するモジュール群。
// 各ハンドラは対応する HTTP エンドポイントにマッピングされる。
//
// モジュール構成:
// - auth: 認証関連（登録、ログイン）
// - batch: バッチ操作（一括作成、TODO + ファイル同時作成）
// - file: ファイル操作（アップロード、ダウンロード、削除）
// - healthz: ヘルスチェック
// - todo: TODO CRUD 操作
//
// 設計原則:
// - ハンドラは「薄く」保つ（ロジックは Application 層に委譲）
// - 入力バリデーションは DTO または Domain で行う
// - エラーは ApiError に変換して統一的に処理
// =============================================================================

// AppState のジェネリックパラメータが多いため、型の複雑性警告を抑制
// これは Clean Architecture のトレイト境界による設計上の制約
#![allow(clippy::type_complexity)]

// -----------------------------------------------------------------------------
// サブモジュール宣言
// -----------------------------------------------------------------------------

// auth: 認証ハンドラ（register, login）
pub mod auth;

// batch: バッチ操作ハンドラ（batch_create_todos, create_todo_with_files）
pub mod batch;

// file: ファイル操作ハンドラ（upload_file, download_file, delete_file）
pub mod file;

// healthz: ヘルスチェックハンドラ
pub mod healthz;

// todo: TODO CRUD ハンドラ（list, create, get, update, delete）
pub mod todo;

// -----------------------------------------------------------------------------
// 再エクスポート
// -----------------------------------------------------------------------------
// pub use xxx::* で、handlers::register のように直接アクセス可能にする

// auth モジュールの全公開アイテムを再エクスポート
// これにより handlers::register, handlers::login でアクセス可能
pub use auth::*;

// batch モジュールの全公開アイテムを再エクスポート
// これにより handlers::batch_create_todos, handlers::create_todo_with_files でアクセス可能
pub use batch::*;

// file モジュールの全公開アイテムを再エクスポート
// これにより handlers::upload_file, handlers::download_file, handlers::delete_file でアクセス可能
pub use file::*;

// healthz 関数のみを再エクスポート
// 単一関数なので明示的に指定
pub use healthz::healthz;

// todo モジュールの全公開アイテムを再エクスポート
// これにより handlers::list_todos, handlers::create_todo などでアクセス可能
pub use todo::*;
