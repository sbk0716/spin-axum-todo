// =============================================================================
// api/src/config.rs: アプリケーション設定
// =============================================================================
// 全ての環境変数を一括で読み込み、型安全な設定構造体として提供する。
// Composition Root（main.rs）で使用し、各層に必要な設定を渡す。
//
// メリット:
// - 起動時に全ての設定エラーを検出
// - 環境変数の一覧がドキュメントとして機能
// - テスト時に Config をモック化可能
//
// Clone vs Copy:
// - 環境変数は String として読み込まれる
// - String はヒープメモリを所有するため Copy トレイトを実装できない
// - したがって Config 構造体は Clone のみ実装
// - 起動時に1回読み込むだけなので Clone のコストは無視できる
// =============================================================================

use std::net::SocketAddr;

// =============================================================================
// AppConfig: アプリケーション全体の設定
// =============================================================================

/// アプリケーション設定
///
/// 全ての環境変数を集約した設定構造体。
/// `from_env()` で環境変数から一括読み込みする。
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// サーバー設定
    pub server: ServerConfig,
    /// データベース設定
    pub database: DatabaseConfig,
    /// Redis 設定
    pub redis: RedisConfig,
    /// JWT 認証設定
    pub jwt: JwtConfig,
    /// S3 ストレージ設定
    pub s3: S3Config,
    /// Edge 検証シークレット（None の場合は検証スキップ）
    pub edge_secret: Option<String>,
}

// =============================================================================
// サブ設定構造体
// =============================================================================

/// サーバー設定
#[derive(Debug, Clone)]
pub struct ServerConfig {
    /// サーバーがリッスンするアドレス（例: 0.0.0.0:3000）
    pub addr: SocketAddr,
}

/// データベース設定（CQRS: Reader/Writer 分離）
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    /// 書き込み用 DB URL（プライマリ）
    pub writer_url: String,
    /// 読み取り用 DB URL（レプリカ、None の場合は writer_url を使用）
    pub reader_url: Option<String>,
}

/// Redis 設定
#[derive(Debug, Clone)]
pub struct RedisConfig {
    /// Redis 接続 URL（例: redis://localhost:6379）
    pub url: String,
}

/// JWT 認証設定
#[derive(Debug, Clone)]
pub struct JwtConfig {
    /// JWT 署名用シークレット
    pub secret: String,
    /// JWT 有効期間（時間）
    pub expiry_hours: i64,
}

/// S3 ストレージ設定
#[derive(Debug, Clone)]
pub struct S3Config {
    /// S3 バケット名
    pub bucket: String,
    /// カスタムエンドポイント URL（LocalStack 用、None の場合は AWS 標準）
    pub endpoint_url: Option<String>,
}

// =============================================================================
// AppConfig 実装
// =============================================================================

impl AppConfig {
    /// 環境変数から設定を読み込む
    ///
    /// # Environment Variables
    ///
    /// | 変数名 | 説明 | 必須 | デフォルト |
    /// |--------|------|:----:|-----------|
    /// | `APP_ADDR` | サーバーアドレス | ✓ | - |
    /// | `DATABASE_WRITER_URL` | 書き込み用 DB URL | ✓ | - |
    /// | `DATABASE_READER_URL` | 読み取り用 DB URL | - | writer と同じ |
    /// | `REDIS_URL` | Redis 接続 URL | ✓ | - |
    /// | `JWT_SECRET` | JWT シークレット | - | デフォルト値 |
    /// | `JWT_EXPIRY_HOURS` | JWT 有効期間 | - | 24 |
    /// | `S3_BUCKET` | S3 バケット名 | - | todo-files |
    /// | `S3_ENDPOINT_URL` | S3 エンドポイント | - | AWS 標準 |
    /// | `EDGE_SECRET` | Edge 検証シークレット | - | None（検証スキップ） |
    ///
    /// # Errors
    ///
    /// 必須の環境変数が設定されていない場合、または値のパースに失敗した場合
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            server: ServerConfig {
                addr: std::env::var("APP_ADDR")?
                    .parse()
                    .map_err(|e| anyhow::anyhow!("Invalid APP_ADDR: {}", e))?,
            },
            database: DatabaseConfig {
                writer_url: std::env::var("DATABASE_WRITER_URL")?,
                reader_url: std::env::var("DATABASE_READER_URL").ok(),
            },
            redis: RedisConfig {
                url: std::env::var("REDIS_URL")?,
            },
            jwt: JwtConfig {
                secret: std::env::var("JWT_SECRET")
                    .unwrap_or_else(|_| "default-secret-change-in-production".to_string()),
                expiry_hours: std::env::var("JWT_EXPIRY_HOURS")
                    .unwrap_or_else(|_| "24".to_string())
                    .parse()
                    .unwrap_or(24),
            },
            s3: S3Config {
                bucket: std::env::var("S3_BUCKET").unwrap_or_else(|_| "todo-files".to_string()),
                endpoint_url: std::env::var("S3_ENDPOINT_URL").ok(),
            },
            edge_secret: std::env::var("EDGE_SECRET").ok(),
        })
    }
}
