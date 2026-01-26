// =============================================================================
// infrastructure/src/persistence/db_pools.rs: Reader/Writer DB 接続プール
// =============================================================================
// Aurora などのマネージド DB では Reader/Writer エンドポイントが分離される。
// ローカル開発では同一 DB を指定可能だが、環境変数で切り替え可能にしておく。
//
// 軽量 CQRS パターンにおける DB 分離:
// - Commands（状態変更）→ Writer Pool（プライマリ DB）
// - Queries（参照）→ Reader Pool（レプリカ DB）
//
// メリット:
// 1. 読み取り負荷をレプリカに分散（スケールアウト）
// 2. レプリケーションラグを許容できるクエリをレプリカに逃がす
// 3. プライマリの書き込み性能を維持
//
// 使用例:
// ```rust,ignore
// let pools = DbPools::from_env().await?;
// let writer = PostgresTodoWriter::new(pools.writer.clone());
// let reader = PostgresTodoReader::new(pools.reader.clone());
// ```
// =============================================================================

// -----------------------------------------------------------------------------
// 外部クレートのインポート
// -----------------------------------------------------------------------------

// sqlx: PostgreSQL クライアント
// PgPoolOptions: 接続プールの設定オプション
// PgPool: PostgreSQL 接続プール
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

// std::env: 環境変数の読み取り
use std::env;

// std::time::Duration: 時間の長さ（タイムアウト設定）
use std::time::Duration;

// tracing: 構造化ログ
use tracing::info;

// =============================================================================
// DbPools 構造体
// =============================================================================

/// Reader/Writer 分離対応の DB 接続プール
///
/// # フィールド
///
/// - `writer`: 書き込み用プール（プライマリ DB に接続）
/// - `reader`: 読み取り用プール（レプリカ DB に接続、または writer と共有）
///
/// # 使用例
///
/// ```rust,ignore
/// // 環境変数から自動設定
/// let pools = DbPools::from_env().await?;
///
/// // Commands では writer を使用
/// let todo_writer = PostgresTodoWriter::new(pools.writer.clone());
///
/// // Queries では reader を使用
/// let todo_reader = PostgresTodoReader::new(pools.reader.clone());
/// ```
///
/// # derive マクロ
///
/// - `Clone`: 複数のリポジトリで共有するために必要
#[derive(Clone)]
pub struct DbPools {
    /// 書き込み用プール（Commands で使用）
    ///
    /// プライマリ DB に接続。INSERT, UPDATE, DELETE を実行。
    pub writer: PgPool,

    /// 読み取り用プール（Queries で使用）
    ///
    /// レプリカ DB に接続（または writer と同じプール）。
    /// SELECT クエリを実行。
    pub reader: PgPool,
}

impl DbPools {
    /// 環境変数から接続プールを作成
    ///
    /// # 環境変数
    ///
    /// | 変数 | 説明 | 必須 |
    /// |------|------|------|
    /// | `DATABASE_WRITER_URL` | 書き込み用接続文字列 | 必須 |
    /// | `DATABASE_READER_URL` | 読み取り用接続文字列 | 任意（Writer にフォールバック） |
    ///
    /// # ローカル開発
    ///
    /// ```bash
    /// # 同じ値を設定すれば単一 DB として動作
    /// DATABASE_WRITER_URL=postgres://app:app@localhost:5432/app
    /// DATABASE_READER_URL=postgres://app:app@localhost:5432/app
    /// ```
    ///
    /// # 本番（Aurora）
    ///
    /// ```bash
    /// DATABASE_WRITER_URL=postgres://...cluster-xxx.rds.amazonaws.com/app
    /// DATABASE_READER_URL=postgres://...cluster-ro-xxx.rds.amazonaws.com/app
    /// ```
    ///
    /// # Errors
    ///
    /// - 環境変数が設定されていない場合: panic
    /// - DB 接続に失敗した場合: `sqlx::Error`
    pub async fn from_env() -> Result<Self, sqlx::Error> {
        // -------------------------------------------------------------------------
        // 環境変数の読み取り
        // -------------------------------------------------------------------------

        // Writer URL: 必須
        let writer_url = env::var("DATABASE_WRITER_URL").expect("DATABASE_WRITER_URL must be set");

        // Reader URL: DATABASE_READER_URL、なければ Writer と同じ
        // unwrap_or_else: Result が Err の場合にデフォルト値を使用
        let reader_url = env::var("DATABASE_READER_URL").unwrap_or_else(|_| writer_url.clone());

        // 同一 DB かどうかを判定（プール共有の最適化に使用）
        let is_same_db = writer_url == reader_url;

        // -------------------------------------------------------------------------
        // ログ出力（パスワードはマスク）
        // -------------------------------------------------------------------------
        info!(
            writer_url = %mask_password(&writer_url),  // %: Display フォーマット
            reader_url = %mask_password(&reader_url),
            same_db = is_same_db,
            "Initializing database pools"
        );

        // -------------------------------------------------------------------------
        // プールの作成
        // -------------------------------------------------------------------------

        // Writer プールを作成
        let writer = create_pool(&writer_url, "writer").await?;

        // Reader プールを作成（同一 DB の場合は Writer と共有）
        let reader = if is_same_db {
            // 同一 DB の場合はプールを共有（リソース節約）
            // clone(): Arc 内部の参照カウントをインクリメント
            writer.clone()
        } else {
            // 別 DB の場合は新しいプールを作成
            create_pool(&reader_url, "reader").await?
        };

        Ok(Self { writer, reader })
    }

    /// 単一プールから作成（テスト用）
    ///
    /// ローカル開発やテストで単一 DB を使用する場合に便利。
    /// Reader と Writer で同じプールを共有する。
    ///
    /// # Arguments
    ///
    /// * `pool` - 共有する PgPool
    ///
    /// # 使用例
    ///
    /// ```rust,ignore
    /// let pool = PgPool::connect("postgres://...").await?;
    /// let pools = DbPools::single(pool);
    /// ```
    pub fn single(pool: PgPool) -> Self {
        Self {
            writer: pool.clone(), // clone で参照を共有
            reader: pool,
        }
    }

    /// Writer プールの参照を取得
    ///
    /// # Returns
    ///
    /// 書き込み用プールへの参照
    pub fn writer(&self) -> &PgPool {
        &self.writer
    }

    /// Reader プールの参照を取得
    ///
    /// # Returns
    ///
    /// 読み取り用プールへの参照
    pub fn reader(&self) -> &PgPool {
        &self.reader
    }
}

// =============================================================================
// ヘルパー関数
// =============================================================================

/// 接続プールを作成する
///
/// # Arguments
///
/// * `url` - PostgreSQL 接続文字列
/// * `name` - プール名（ログ用）
///
/// # Pool 設定
///
/// | 設定 | 値 | 説明 |
/// |------|-------|------|
/// | `max_connections` | 10 | 最大接続数 |
/// | `min_connections` | 1 | 最小接続数（アイドル時） |
/// | `acquire_timeout` | 5秒 | 接続取得タイムアウト |
/// | `idle_timeout` | 300秒 | アイドル接続のタイムアウト |
///
/// # Errors
///
/// DB 接続に失敗した場合は `sqlx::Error` を返す
async fn create_pool(url: &str, name: &str) -> Result<PgPool, sqlx::Error> {
    // PgPoolOptions: 接続プールの設定ビルダー
    let pool = PgPoolOptions::new()
        .max_connections(10) // 最大 10 接続
        .min_connections(1) // 最小 1 接続
        .acquire_timeout(Duration::from_secs(5)) // 5 秒でタイムアウト
        .idle_timeout(Duration::from_secs(300)) // 5 分でアイドル接続を解放
        .connect(url) // 接続を開始
        .await?; // 非同期待機 + エラー伝播

    // 接続成功をログ出力
    info!(pool = name, max_connections = 10, "Database pool created");

    Ok(pool)
}

/// 接続文字列のパスワードをマスク（ログ出力用）
///
/// セキュリティのため、ログにパスワードを出力しないようにする。
///
/// # Arguments
///
/// * `url` - PostgreSQL 接続文字列
///
/// # Returns
///
/// パスワード部分を `***` に置換した文字列
///
/// # 例
///
/// ```text
/// postgres://user:password@host:port/db → postgres://user:***@host:port/db
/// ```
fn mask_password(url: &str) -> String {
    // @ の位置を探す（ユーザー情報とホストの区切り）
    if let Some(at_pos) = url.find('@') {
        // @ より前の部分で最後の : を探す（パスワードの開始位置）
        if let Some(colon_pos) = url[..at_pos].rfind(':') {
            // prefix: "postgres://user:" まで
            let prefix = &url[..=colon_pos];
            // suffix: "@host:port/db"
            let suffix = &url[at_pos..];
            // パスワード部分を *** に置換
            return format!("{}***{}", prefix, suffix);
        }
    }
    // パスワードが見つからない場合はそのまま返す
    url.to_string()
}

// =============================================================================
// テスト
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// mask_password のテスト
    #[test]
    fn test_mask_password() {
        // パスワードあり
        assert_eq!(
            mask_password("postgres://user:secret@localhost:5432/db"),
            "postgres://user:***@localhost:5432/db"
        );

        // パスワードなし
        assert_eq!(
            mask_password("postgres://localhost:5432/db"),
            "postgres://localhost:5432/db"
        );
    }
}
