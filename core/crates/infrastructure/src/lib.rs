// =============================================================================
// infrastructure/src/lib.rs: インフラストラクチャ層のルートモジュール
// =============================================================================
// クリーンアーキテクチャの最外層（Frameworks & Drivers 層）。
// 外部システム（PostgreSQL、Redis）との連携を実装する。
//
// 依存関係:
// - Domain 層のトレイト（TodoReader, TodoWriter 等）を実装
// - Application 層への依存は禁止（依存性逆転の原則）
//
// 公開する主な要素:
// ┌─────────────────────────────────────────────────────────────┐
// │ DB 接続                                                     │
// │ - DbPools: Reader/Writer 分離対応の PostgreSQL 接続プール   │
// ├─────────────────────────────────────────────────────────────┤
// │ CQRS 実装（PostgreSQL）                                     │
// │ - PostgresTodoReader / PostgresTodoWriter: TODO             │
// │ - PostgresUserReader / PostgresUserWriter: ユーザー         │
// │ - PostgresFileReader / PostgresFileWriter: ファイル         │
// ├─────────────────────────────────────────────────────────────┤
// │ キャッシュ                                                   │
// │ - TodoCache: Redis キャッシュ操作                           │
// │ - CachedTodoReader: キャッシュ付き TodoReader（デコレータ）  │
// ├─────────────────────────────────────────────────────────────┤
// │ トランザクション                                             │
// │ - TransactionalTodoService: バッチ操作（複数 TODO 一括作成）│
// │ - FileInput: ファイル作成用入力データ                       │
// └─────────────────────────────────────────────────────────────┘
//
// 使用例（main.rs での DI）:
// ```rust,ignore
// let db_pools = DbPools::from_env().await?;
// let todo_writer = Arc::new(PostgresTodoWriter::new(db_pools.writer.clone()));
// let todo_reader = Arc::new(CachedTodoReader::new(
//     PostgresTodoReader::new(db_pools.reader.clone()),
//     Arc::clone(&cache),
// ));
// ```
// =============================================================================

// -----------------------------------------------------------------------------
// サブモジュール宣言
// -----------------------------------------------------------------------------

/// 永続化モジュール（PostgreSQL、Redis）
pub mod persistence;

/// リポジトリモジュール（デコレータ実装）
pub mod repositories;

/// サービスモジュール（トランザクション対応）
pub mod services;

// -----------------------------------------------------------------------------
// 再エクスポート（Re-export）
// -----------------------------------------------------------------------------
// クレート利用者が `infrastructure::PostgresTodoReader` のように
// 簡潔にアクセスできるようにする

// DB 接続プール
pub use persistence::db_pools::DbPools;

// PostgreSQL CQRS 実装: TODO
pub use persistence::postgres::PostgresTodoReader;
pub use persistence::postgres::PostgresTodoWriter;

// PostgreSQL CQRS 実装: User
pub use persistence::postgres::PostgresUserReader;
pub use persistence::postgres::PostgresUserWriter;

// PostgreSQL CQRS 実装: File
pub use persistence::postgres::PostgresFileReader;
pub use persistence::postgres::PostgresFileWriter;

// Redis キャッシュ
pub use persistence::redis::TodoCache;

// キャッシュ付きリポジトリ（デコレータ）
pub use repositories::CachedTodoReader;

// トランザクション対応サービス
pub use services::{FileInput, TransactionalTodoService};
