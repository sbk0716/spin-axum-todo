// =============================================================================
// infrastructure/src/persistence/postgres/mod.rs: PostgreSQL リポジトリモジュール
// =============================================================================
// Domain 層で定義された Reader/Writer トレイトの PostgreSQL 実装を提供。
//
// 統一 CQRS パターン（Command Query Responsibility Segregation）:
// - Reader: 読み取り専用操作（find_by_id, find_all, find_by_email）
// - Writer: 書き込み操作（create, update, delete）
//
// Reader/Writer を分離するメリット:
// 1. Aurora などで Reader/Writer エンドポイントを分離可能
// 2. 読み取り負荷をレプリカに分散できる
// 3. 責務の明確化（単一責任の原則）
//
// 各エンティティの実装:
// - TODO:  PostgresTodoReader / PostgresTodoWriter
// - User:  PostgresUserReader / PostgresUserWriter
// - File:  PostgresFileReader / PostgresFileWriter
//
// 使用例:
// ```rust,ignore
// // main.rs での DI
// let todo_writer = Arc::new(PostgresTodoWriter::new(db_pools.writer.clone()));
// let todo_reader = Arc::new(PostgresTodoReader::new(db_pools.reader.clone()));
// ```
// =============================================================================

// -----------------------------------------------------------------------------
// サブモジュール宣言
// -----------------------------------------------------------------------------
// 各エンティティの Reader/Writer 実装を非公開モジュールとして宣言
// pub use で必要な型のみを公開する

mod file_reader; // FileReader トレイトの PostgreSQL 実装
mod file_writer; // FileWriter トレイトの PostgreSQL 実装
mod todo_reader; // TodoReader トレイトの PostgreSQL 実装
mod todo_writer; // TodoWriter トレイトの PostgreSQL 実装
mod user_reader; // UserReader トレイトの PostgreSQL 実装
mod user_writer; // UserWriter トレイトの PostgreSQL 実装

// -----------------------------------------------------------------------------
// 再エクスポート（Re-export）
// -----------------------------------------------------------------------------
// 外部クレートから `infrastructure::PostgresTodoReader` のようにアクセス可能にする

// CQRS: TODO Writer/Reader
// - PostgresTodoWriter: create, update, delete（Writer Pool 使用）
// - PostgresTodoReader: find_by_id, find_all（Reader Pool 使用）
pub use todo_reader::PostgresTodoReader;
pub use todo_writer::PostgresTodoWriter;

// CQRS: User Writer/Reader
// - PostgresUserWriter: create, update, delete（Writer Pool 使用）
// - PostgresUserReader: find_by_id, find_by_email（Reader Pool 使用）
pub use user_reader::PostgresUserReader;
pub use user_writer::PostgresUserWriter;

// CQRS: File Writer/Reader
// - PostgresFileWriter: create, delete（Writer Pool 使用）
// - PostgresFileReader: find_by_id, find_by_todo_id（Reader Pool 使用）
pub use file_reader::PostgresFileReader;
pub use file_writer::PostgresFileWriter;
