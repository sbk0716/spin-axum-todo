// =============================================================================
// api/src/main.rs: アプリケーションエントリーポイント
// =============================================================================
// クリーンアーキテクチャにおける Composition Root（構成ルート）。
// 全ての依存関係をここで組み立て、サーバーを起動する。
//
// 主な役割:
// - 環境設定の読み込み（AppConfig 経由で一括管理）
// - インフラ層のインスタンス生成（PostgreSQL、Redis、S3）
// - リポジトリの組み立て（デコレータパターンでキャッシュを追加）
// - プレゼンテーション層のルーター構築
// - HTTP サーバー起動（グレースフルシャットダウン対応）
//
// CQRS パターン（Reader/Writer 分離）:
// - DbPools: Writer（プライマリ）/ Reader（レプリカ）接続プール
// - Todo: TodoWriter（Commands）/ CachedTodoReader（Queries + Redis キャッシュ）
// - User: UserWriter（Commands）/ UserReader（Queries）
// - File: FileWriter（Commands）/ FileReader（Queries）
//
// 依存性注入（DI）:
// - 具象型（Postgres*, S3*）はここでのみ使用
// - 他の層はトレイト経由でアクセス
// - テスト時はモック実装に差し替え可能
// =============================================================================

// -----------------------------------------------------------------------------
// モジュール宣言
// -----------------------------------------------------------------------------

mod config;

// -----------------------------------------------------------------------------
// 外部クレートのインポート
// -----------------------------------------------------------------------------

use std::sync::Arc;

use dotenvy::dotenv;
use tracing_subscriber::EnvFilter;

use infrastructure::{
    CachedTodoReader, DbPools, PostgresFileReader, PostgresFileWriter, PostgresTodoReader,
    PostgresTodoWriter, PostgresUserReader, PostgresUserWriter, S3StorageService, TodoCache,
    TransactionalTodoService,
};
use presentation::{create_router, AppState};

use crate::config::AppConfig;

// =============================================================================
// main 関数
// =============================================================================

/// アプリケーションのメインエントリーポイント
///
/// # 起動手順
///
/// 1. .env ファイル読み込み（ローカル開発用）
/// 2. ログ初期化（JSON 形式）
/// 3. Config から設定を一括読み込み
/// 4. PostgreSQL 接続プール作成（CQRS: Reader/Writer 分離）
/// 5. Redis クライアント作成
/// 6. S3 ストレージサービス作成
/// 7. リポジトリ組み立て（DI）
/// 8. アプリケーション状態作成
/// 9. ルーター構築
/// 10. サーバー起動（グレースフルシャットダウン対応）
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // =========================================================================
    // 設定の読み込み
    // =========================================================================

    // .env ファイルを読み込む（存在しなくても続行）
    // 本番環境では .env は使用せず、環境変数を直接設定する
    dotenv().ok();

    // -------------------------------------------------------------------------
    // ログ出力の初期化
    // -------------------------------------------------------------------------
    // JSON 形式でログ出力（CloudWatch/Cloud Logging 連携向け）
    // from_default_env() は RUST_LOG 環境変数からログレベルを読み込む
    // 例: RUST_LOG=info, RUST_LOG=api=debug,sqlx=warn
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .json()
        .init();

    // -------------------------------------------------------------------------
    // Config から設定を一括読み込み
    // -------------------------------------------------------------------------
    let config = AppConfig::from_env()?;
    tracing::info!("Configuration loaded");

    // =========================================================================
    // インフラ層のセットアップ（統一 CQRS: Reader/Writer 分離）
    // =========================================================================

    // -------------------------------------------------------------------------
    // PostgreSQL 接続プールを作成
    // -------------------------------------------------------------------------
    let db_pools = DbPools::from_config(
        &config.database.writer_url,
        config.database.reader_url.as_deref(),
    )
    .await?;
    tracing::info!("Connected to PostgreSQL (Reader/Writer pools initialized)");

    // -------------------------------------------------------------------------
    // Redis クライアントを作成
    // -------------------------------------------------------------------------
    let redis_client = redis::Client::open(config.redis.url.as_str())?;
    tracing::info!("Connected to Redis");

    // -------------------------------------------------------------------------
    // S3 ストレージサービスを作成
    // -------------------------------------------------------------------------
    // bucket: &String → &str に自動変換（Deref coercion）
    // endpoint_url: Option<String> → Option<&str> に変換（as_deref() が必要）
    let storage_service =
        S3StorageService::from_config(&config.s3.bucket, config.s3.endpoint_url.as_deref()).await?;

    // バケットの存在確認と作成（LocalStack 用）
    storage_service.ensure_bucket_exists().await?;
    tracing::info!("Connected to S3 (bucket: {})", storage_service.bucket());

    // Arc でラップ（StorageOps トレイトを通じて共有）
    let storage = Arc::new(storage_service);

    // =========================================================================
    // リポジトリの組み立て（依存性注入 - 統一 CQRS + キャッシュ）
    // =========================================================================

    // TODO Writer（Commands 用）
    let todo_writer = Arc::new(PostgresTodoWriter::new(db_pools.writer.clone()));

    // キャッシュ（Redis）
    let cache = Arc::new(TodoCache::new(redis_client));

    // TODO Reader（Queries 用、キャッシュ付きデコレータ）
    let postgres_reader = PostgresTodoReader::new(db_pools.reader.clone());
    let todo_reader = Arc::new(CachedTodoReader::new(postgres_reader, Arc::clone(&cache)));

    // User Reader / Writer
    let user_reader = Arc::new(PostgresUserReader::new(db_pools.reader.clone()));
    let user_writer = Arc::new(PostgresUserWriter::new(db_pools.writer.clone()));

    // File Reader / Writer
    let file_reader = Arc::new(PostgresFileReader::new(db_pools.reader.clone()));
    let file_writer = Arc::new(PostgresFileWriter::new(db_pools.writer.clone()));

    // バッチ操作サービス（トランザクション対応）
    let batch_service = TransactionalTodoService::new(db_pools.writer.clone());

    // =========================================================================
    // プレゼンテーション層のセットアップ
    // =========================================================================

    // -------------------------------------------------------------------------
    // アプリケーション状態を作成（axum 推奨: Clone 可能な AppState）
    // -------------------------------------------------------------------------
    // axum 公式ドキュメント:
    // > "Your top level state needs to derive Clone"
    //
    // axum メンテナー mladedav (GitHub Discussion #3223):
    // > "When you extract the state, axum will clone it and pass it to your handler."
    //
    // 動作イメージ:
    //
    //   AppState (Clone 実装)
    //        │
    //        ├─ state.clone() ──→ handler_1
    //        ├─ state.clone() ──→ handler_2
    //        └─ state.clone() ──→ handler_3
    //
    // AppState の各フィールドは Arc<T> を持つため、clone() は低コスト。
    // Arc::clone() はポインタのコピー + 参照カウント増加のみで、
    // 内部の大きなデータ（DB プール等）はコピーされない。
    // -------------------------------------------------------------------------
    let state = AppState::new(
        todo_writer,
        todo_reader,
        cache,
        user_reader,
        user_writer,
        batch_service,
        storage,
        file_reader,
        file_writer,
        config.jwt.secret.clone(),
        config.jwt.expiry_hours,
    );

    // ルーターを構築
    let app = create_router(state, config.edge_secret.clone());

    // =========================================================================
    // サーバー起動
    // =========================================================================

    tracing::info!("Listening on {}", config.server.addr);

    let listener = tokio::net::TcpListener::bind(config.server.addr).await?;

    // グレースフルシャットダウン対応でサーバー起動
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    tracing::info!("Server shutdown complete");
    Ok(())
}

// =============================================================================
// shutdown_signal 関数
// =============================================================================

/// シャットダウンシグナルを待機する
///
/// マネージドサービス（ECS、Cloud Run、AKS 等）では、
/// デプロイ時やスケールイン時に SIGTERM が送信される。
/// このシグナルを受けて、処理中のリクエストを完了させてから終了する。
///
/// # 対応シグナル
///
/// - Ctrl+C (SIGINT): ローカル開発時の停止
/// - SIGTERM: コンテナオーケストレーターからの停止要求
async fn shutdown_signal() {
    use tokio::signal;

    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("Shutdown signal received, starting graceful shutdown");
}
