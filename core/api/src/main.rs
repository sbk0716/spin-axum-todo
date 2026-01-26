// =============================================================================
// api/src/main.rs: アプリケーションエントリーポイント
// =============================================================================
// クリーンアーキテクチャにおける Composition Root（構成ルート）。
// 全ての依存関係をここで組み立て、サーバーを起動する。
//
// 主な役割:
// - 環境設定の読み込み（.env、環境変数）
// - インフラ層のインスタンス生成（PostgreSQL、Redis）
// - リポジトリの組み立て（デコレータパターンでキャッシュを追加）
// - プレゼンテーション層のルーター構築
// - HTTP サーバー起動
//
// 統一 CQRS パターン:
// - DbPools で Reader/Writer DB 接続を分離
// - TODO: TodoWriter（Commands）/ TodoReader（Queries）
// - User: UserWriter（Commands）/ UserReader（Queries）
// - TodoCache で読み取りキャッシュ + 書き込み時無効化
//
// 依存性注入（DI）:
// - 具象型はここでのみ使用
// - 他の層はトレイト経由でアクセス
// - テスト時はモック実装に差し替え可能
// =============================================================================

// -----------------------------------------------------------------------------
// 外部クレートのインポート
// -----------------------------------------------------------------------------

// std::net::SocketAddr: ソケットアドレス（IP + ポート）
// std::sync::Arc: スレッド安全な参照カウントポインタ
use std::{net::SocketAddr, sync::Arc};

// dotenvy: .env ファイルを環境変数に読み込む
// dotenv().ok() で失敗しても続行（本番環境では .env は無いことが多い）
use dotenvy::dotenv;

// tracing_subscriber: 構造化ログの購読者
// EnvFilter: RUST_LOG 環境変数でログレベルをフィルタリング
use tracing_subscriber::EnvFilter;

// infrastructure: Infrastructure 層の型
// - CachedTodoReader: キャッシュ付き TodoReader（デコレータ）
// - DbPools: Reader/Writer 分離された DB 接続プール
// - PostgresTodoReader/Writer: PostgreSQL 実装
// - PostgresUserReader/Writer: PostgreSQL 実装
// - PostgresFileReader/Writer: PostgreSQL 実装
// - TodoCache: Redis キャッシュ
// - S3StorageService: S3 ファイルストレージ
// - TransactionalTodoService: トランザクション対応バッチサービス
use infrastructure::{
    CachedTodoReader, DbPools, PostgresFileReader, PostgresFileWriter, PostgresTodoReader,
    PostgresTodoWriter, PostgresUserReader, PostgresUserWriter, S3StorageService, TodoCache,
    TransactionalTodoService,
};

// presentation: Presentation 層の型
// - create_router: axum Router を構築する関数
// - AppState: アプリケーション状態（ユースケースを保持）
use presentation::{create_router, AppState};

// =============================================================================
// main 関数
// =============================================================================

/// アプリケーションのメインエントリーポイント
///
/// # 属性マクロ
///
/// `#[tokio::main]`: 非同期 main 関数を可能にする。
/// 内部的には tokio ランタイムを起動し、async fn main を実行する。
///
/// ```rust,ignore
/// // 展開後のイメージ
/// fn main() {
///     tokio::runtime::Runtime::new()
///         .unwrap()
///         .block_on(async {
///             // async main の内容
///         })
/// }
/// ```
///
/// # Returns
///
/// * `anyhow::Result<()>` - 成功時は Ok(()), エラー時は anyhow::Error
///
/// # 起動手順
///
/// 1. .env ファイル読み込み
/// 2. ログ初期化
/// 3. 環境変数から設定読み込み
/// 4. PostgreSQL 接続プール作成
/// 5. Redis クライアント作成
/// 6. リポジトリ組み立て
/// 7. アプリケーション状態作成
/// 8. ルーター構築
/// 9. サーバー起動
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // =========================================================================
    // 設定の読み込み
    // =========================================================================

    // .env ファイルを読み込む（存在しなくても続行）
    // ok() で Result を Option に変換し、エラーを無視
    // 本番環境では .env は使用せず、環境変数を直接設定することが多い
    dotenv().ok();

    // -------------------------------------------------------------------------
    // ログ出力の初期化
    // -------------------------------------------------------------------------
    // tracing_subscriber: 構造化ログの購読者を設定
    // fmt(): フォーマット済み出力を使用
    // with_env_filter: RUST_LOG 環境変数でログレベルをフィルタリング
    //   例: RUST_LOG=info,sqlx=warn,tower_http=debug
    // init(): グローバルなデフォルト購読者として登録
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env()) // RUST_LOG でフィルタ
        .init(); // 購読者を登録

    // -------------------------------------------------------------------------
    // 環境変数から設定を読み込む
    // -------------------------------------------------------------------------

    // APP_ADDR: サーバーがリッスンするアドレス（例: 0.0.0.0:3000）
    // ? 演算子: エラー時は早期リターン（anyhow::Error に変換）
    // parse(): 文字列を SocketAddr にパース
    let addr: SocketAddr = std::env::var("APP_ADDR")? // 環境変数を取得
        .parse()?; // SocketAddr にパース

    // REDIS_URL: Redis 接続 URL（例: redis://localhost:6379）
    let redis_url = std::env::var("REDIS_URL")?;

    // EDGE_SECRET: Edge 検証用シークレット（オプショナル）
    // ok(): Result を Option に変換（エラー = None）
    // 設定されていない場合、Edge 検証はスキップされる（開発モード）
    let edge_secret = std::env::var("EDGE_SECRET").ok();

    // -------------------------------------------------------------------------
    // JWT 設定
    // -------------------------------------------------------------------------

    // JWT_SECRET: JWT 署名用シークレット
    // unwrap_or_else: 環境変数が無い場合のデフォルト値
    // 本番環境では必ず安全なシークレットを設定すること
    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "default-secret-change-in-production".to_string());

    // JWT_EXPIRY_HOURS: JWT 有効期間（時間）
    // デフォルト: 24時間
    // parse() でパース失敗した場合も 24 をデフォルトに
    let jwt_expiry_hours: i64 = std::env::var("JWT_EXPIRY_HOURS")
        .unwrap_or_else(|_| "24".to_string()) // 環境変数なしなら "24"
        .parse() // i64 にパース
        .unwrap_or(24); // パース失敗なら 24

    // =========================================================================
    // インフラ層のセットアップ（統一 CQRS: Reader/Writer 分離）
    // =========================================================================

    // -------------------------------------------------------------------------
    // PostgreSQL 接続プールを作成
    // -------------------------------------------------------------------------
    // DbPools::from_env(): 環境変数から接続設定を読み込む
    // - DATABASE_WRITER_URL: 書き込み用プライマリ DB（Commands）
    // - DATABASE_READER_URL: 読み取り用レプリカ DB（Queries）
    // - DATABASE_READER_URL が無い場合は Writer と同じ URL を使用
    // await?: 非同期待機 + エラー伝播
    let db_pools = DbPools::from_env().await?;

    // 接続成功ログ
    tracing::info!("Connected to PostgreSQL (Reader/Writer pools initialized)");

    // -------------------------------------------------------------------------
    // Redis クライアントを作成
    // -------------------------------------------------------------------------
    // redis::Client::open: Redis 接続クライアントを作成
    // 実際の接続は操作時に行われる（遅延接続）
    let redis_client = redis::Client::open(redis_url)?;

    // 接続成功ログ
    tracing::info!("Connected to Redis");

    // -------------------------------------------------------------------------
    // S3 ストレージサービスを作成
    // -------------------------------------------------------------------------
    // S3StorageService::from_env(): 環境変数から接続設定を読み込む
    // - S3_ENDPOINT_URL: カスタムエンドポイント（LocalStack 用）
    // - S3_BUCKET: バケット名（デフォルト: todo-files）
    // 環境変数が設定されていない場合は AWS 標準エンドポイントを使用
    let storage_service = S3StorageService::from_env().await?;

    // バケットの存在確認と作成（LocalStack 用）
    storage_service.ensure_bucket_exists().await?;

    // 接続成功ログ
    tracing::info!("Connected to S3 (bucket: {})", storage_service.bucket());

    // Arc でラップ（StorageOps トレイトを通じて共有）
    let storage = Arc::new(storage_service);

    // =========================================================================
    // リポジトリの組み立て（依存性注入 - 統一 CQRS + キャッシュ）
    // =========================================================================
    // ここで具象型を生成し、Arc でラップして共有可能にする
    // 他の層はトレイト経由でアクセスするため、具象型を知らない

    // -------------------------------------------------------------------------
    // TODO Writer を作成（Commands 用）
    // -------------------------------------------------------------------------
    // PostgresTodoWriter: PostgreSQL 実装
    // db_pools.writer.clone(): Writer プールを使用（プライマリ DB）
    // Arc::new: スレッド安全な参照カウントでラップ
    let todo_writer = Arc::new(PostgresTodoWriter::new(db_pools.writer.clone()));

    // -------------------------------------------------------------------------
    // キャッシュを作成
    // -------------------------------------------------------------------------
    // TodoCache: Redis を使用したキャッシュ実装
    // 読み取り時のキャッシュ + 書き込み時の無効化で使用
    // Arc::clone で Commands と Queries の両方で共有
    let cache = Arc::new(TodoCache::new(redis_client));

    // -------------------------------------------------------------------------
    // TODO Reader を作成（Queries 用）
    // -------------------------------------------------------------------------
    // デコレータパターン: PostgresReader → CachedReader でラップ
    // 1. PostgresTodoReader: DB から直接読み取り
    // 2. CachedTodoReader: キャッシュ層を追加（Cache-Aside パターン）
    let postgres_reader = PostgresTodoReader::new(db_pools.reader.clone()); // Reader プールを使用
    let todo_reader = Arc::new(CachedTodoReader::new(
        postgres_reader,    // 内部 Reader
        Arc::clone(&cache), // キャッシュを共有
    ));

    // -------------------------------------------------------------------------
    // User Reader を作成（Queries 用）
    // -------------------------------------------------------------------------
    // PostgresUserReader: PostgreSQL 実装
    // db_pools.reader.clone(): Reader プールを使用（レプリカ DB）
    let user_reader = Arc::new(PostgresUserReader::new(db_pools.reader.clone()));

    // -------------------------------------------------------------------------
    // User Writer を作成（Commands 用）
    // -------------------------------------------------------------------------
    // PostgresUserWriter: PostgreSQL 実装
    // db_pools.writer.clone(): Writer プールを使用（プライマリ DB）
    let user_writer = Arc::new(PostgresUserWriter::new(db_pools.writer.clone()));

    // -------------------------------------------------------------------------
    // File Reader を作成（Queries 用）
    // -------------------------------------------------------------------------
    // PostgresFileReader: PostgreSQL 実装
    // db_pools.reader.clone(): Reader プールを使用（レプリカ DB）
    let file_reader = Arc::new(PostgresFileReader::new(db_pools.reader.clone()));

    // -------------------------------------------------------------------------
    // File Writer を作成（Commands 用）
    // -------------------------------------------------------------------------
    // PostgresFileWriter: PostgreSQL 実装
    // db_pools.writer.clone(): Writer プールを使用（プライマリ DB）
    let file_writer = Arc::new(PostgresFileWriter::new(db_pools.writer.clone()));

    // -------------------------------------------------------------------------
    // バッチ操作サービスを作成
    // -------------------------------------------------------------------------
    // TransactionalTodoService: トランザクション対応のバッチ操作
    // - batch_create: 複数 TODO の一括作成
    // - create_with_files: TODO + ファイルの同時作成
    // Writer プールを使用（プライマリ DB 直接アクセス）
    let batch_service = TransactionalTodoService::new(db_pools.writer.clone());

    // =========================================================================
    // プレゼンテーション層のセットアップ
    // =========================================================================

    // -------------------------------------------------------------------------
    // アプリケーション状態を作成
    // -------------------------------------------------------------------------
    // AppState: ユースケース（Commands, Queries）を保持
    // Arc でラップして複数ハンドラ間で共有可能に
    //
    // 引数:
    // - todo_writer: TODO 書き込み（CreateTodoCommand 等で使用）
    // - todo_reader: TODO 読み取り（GetTodoQuery 等で使用、キャッシュ付き）
    // - cache: キャッシュ操作（Commands で Write-Through / 無効化）
    // - user_reader: ユーザー読み取り（AuthService のログインで使用）
    // - user_writer: ユーザー書き込み（AuthService の登録で使用）
    // - batch_service: バッチ操作サービス
    // - storage_service: S3 ストレージサービス
    // - file_reader: ファイル読み取り
    // - file_writer: ファイル書き込み
    // - jwt_secret: JWT 署名用シークレット
    // - jwt_expiry_hours: JWT 有効期間
    let state = Arc::new(AppState::new(
        todo_writer,      // Commands: TODO 作成/更新/削除
        todo_reader,      // Queries: TODO 取得/一覧
        cache,            // キャッシュ: Write-Through / 無効化
        user_reader,      // Queries: ユーザー検索（ログイン）
        user_writer,      // Commands: ユーザー作成（登録）
        batch_service,    // バッチ: 一括作成、TODO + ファイル作成
        storage,          // S3: ファイルアップロード/ダウンロード（Arc でラップ済み）
        file_reader,      // Queries: ファイル検索
        file_writer,      // Commands: ファイル作成/削除
        jwt_secret,       // JWT 署名用シークレット
        jwt_expiry_hours, // JWT 有効期間（時間）
    ));

    // -------------------------------------------------------------------------
    // ルーターを構築
    // -------------------------------------------------------------------------
    // create_router: axum Router を構築
    // - state: アプリケーション状態（ユースケース）
    // - edge_secret: Edge 検証用シークレット（None なら検証スキップ）
    let app = create_router(state, edge_secret);

    // =========================================================================
    // サーバー起動
    // =========================================================================

    // サーバー起動ログ
    tracing::info!("Listening on {}", addr);

    // TCP リスナーを作成
    // tokio::net::TcpListener: 非同期 TCP リスナー
    // bind(addr): 指定アドレスでリッスン開始
    let listener = tokio::net::TcpListener::bind(addr).await?;

    // axum サーバーを起動
    // axum::serve: axum 0.8 の新しいサーバー起動 API
    // listener: TCP リスナー
    // app: Router（リクエストハンドラ）
    // await?: サーバーが終了するまでブロック
    axum::serve(listener, app).await?;

    // サーバー終了（通常ここには到達しない）
    Ok(())
}
