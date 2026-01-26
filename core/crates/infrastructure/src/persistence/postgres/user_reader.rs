// =============================================================================
// infrastructure/src/persistence/postgres/user_reader.rs: ユーザー読み取り実装
// =============================================================================
// PostgreSQL を使用した UserReader の実装。
// ログイン認証や JWT 検証時のユーザー取得を担当。
// Reader DB プールと組み合わせることでレプリケーション対応可能。
//
// 使用場面:
// - ログイン時: find_by_email でユーザーを検索
// - JWT 検証時: find_by_id でユーザーを検索
// =============================================================================

// -----------------------------------------------------------------------------
// 外部クレートのインポート
// -----------------------------------------------------------------------------

// async_trait: async fn を含むトレイトを定義可能にする
use async_trait::async_trait;

// chrono: 日付・時刻ライブラリ
use chrono::{DateTime, Utc};

// domain: ドメイン層の型をインポート
use domain::{DomainError, User, UserReader};

// sqlx: PostgreSQL クライアントライブラリ
use sqlx::{FromRow, PgPool};

// tracing: 構造化ログライブラリ
use tracing::debug;

// uuid: 一意識別子ライブラリ
use uuid::Uuid;

// =============================================================================
// PostgresUserReader 構造体
// =============================================================================

/// PostgreSQL への接続を管理するユーザー読み取りリポジトリ
///
/// # 用途
///
/// - ログイン認証: メールアドレスでユーザーを検索
/// - JWT 検証: ユーザー ID でユーザーを検索
///
/// # CQRS パターン
///
/// Reader 実装として、参照操作のみを提供。
/// 書き込み操作は PostgresUserWriter が担当。
pub struct PostgresUserReader {
    /// PostgreSQL 接続プール（Reader 用推奨）
    pool: PgPool,
}

impl PostgresUserReader {
    /// 新しい PostgresUserReader を作成する
    ///
    /// # Arguments
    ///
    /// * `pool` - PostgreSQL 接続プール（Reader プール推奨）
    ///
    /// # Note
    ///
    /// 本番環境では Reader エンドポイント（読み取りレプリカ）を指すプールを
    /// 渡すことで、書き込み負荷を Writer に集中させられる。
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

// =============================================================================
// UserRow 構造体（内部用）
// =============================================================================

/// データベース行からのマッピング用構造体
///
/// SQLx の `query_as` で使用する内部構造体。
/// `#[derive(FromRow)]` により、SELECT 結果から自動マッピング。
///
/// # Security
///
/// password_hash を含むため、この構造体は pub でない。
/// 外部には domain::User として返す。
#[derive(FromRow)]
struct UserRow {
    /// ユーザーの一意識別子
    id: Uuid,
    /// メールアドレス（UNIQUE 制約）
    email: String,
    /// パスワードハッシュ（bcrypt）
    password_hash: String,
    /// 表示名（任意）
    display_name: Option<String>,
    /// 作成日時
    created_at: DateTime<Utc>,
    /// 更新日時
    updated_at: DateTime<Utc>,
}

/// UserRow から domain::User への変換
///
/// From トレイトを実装することで `.into()` が使用可能に。
impl From<UserRow> for User {
    fn from(row: UserRow) -> Self {
        // User::from_raw: バリデーションをスキップ
        // DB から取得したデータは信頼できる前提
        User::from_raw(
            row.id,            // UUID: 主キー
            row.email,         // String: メールアドレス
            row.password_hash, // String: bcrypt ハッシュ
            row.display_name,  // Option<String>: 表示名
            row.created_at,    // DateTime<Utc>: 作成日時
            row.updated_at,    // DateTime<Utc>: 更新日時
        )
    }
}

// =============================================================================
// UserReader トレイト実装
// =============================================================================

/// UserReader トレイトの PostgreSQL 実装
#[async_trait]
impl UserReader for PostgresUserReader {
    /// メールアドレスでユーザーを検索
    ///
    /// ログイン時の認証に使用。
    /// メールアドレスは UNIQUE 制約があるため、最大1件。
    ///
    /// # Arguments
    ///
    /// * `email` - 検索するメールアドレス
    ///
    /// # Returns
    ///
    /// * `Ok(Some(user))` - ユーザーが見つかった場合
    /// * `Ok(None)` - ユーザーが見つからない場合
    /// * `Err(DomainError)` - DB エラーの場合
    ///
    /// # Security
    ///
    /// 見つからない場合も同じ処理時間になるよう、
    /// パスワード検証は呼び出し側（AuthService）で行う。
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, DomainError> {
        // 構造化ログ: メールアドレスを記録（本番では注意）
        debug!(email = %email, "Finding user by email in PostgreSQL");

        // メールアドレスで検索（UNIQUE 制約あり）
        let row: Option<UserRow> = sqlx::query_as(
            r#"
            SELECT id, email, password_hash, display_name, created_at, updated_at
            FROM users
            WHERE email = $1
            "#,
        )
        .bind(email) // $1: メールアドレス（プレースホルダバインド）
        .fetch_optional(&self.pool) // 0件 or 1件を取得
        .await // 非同期実行
        .map_err(|e| DomainError::Repository(e.to_string()))?; // エラー変換

        // Option<UserRow> → Option<User> に変換
        Ok(row.map(Into::into))
    }

    /// ID でユーザーを検索
    ///
    /// JWT の sub クレームからユーザーを取得する際に使用。
    ///
    /// # Arguments
    ///
    /// * `id` - 検索するユーザー ID
    ///
    /// # Returns
    ///
    /// * `Ok(Some(user))` - ユーザーが見つかった場合
    /// * `Ok(None)` - ユーザーが見つからない場合（削除済みなど）
    /// * `Err(DomainError)` - DB エラーの場合
    ///
    /// # Note
    ///
    /// JWT は有効でもユーザーが削除されている可能性があるため、
    /// 認証ミドルウェアではこのメソッドでユーザー存在を確認する。
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError> {
        // 構造化ログ: ユーザー ID を記録
        debug!(user_id = %id, "Finding user by ID in PostgreSQL");

        // UUID で検索（PRIMARY KEY）
        let row: Option<UserRow> = sqlx::query_as(
            r#"
            SELECT id, email, password_hash, display_name, created_at, updated_at
            FROM users
            WHERE id = $1
            "#,
        )
        .bind(id) // $1: ユーザー ID（UUID）
        .fetch_optional(&self.pool) // 0件 or 1件を取得
        .await // 非同期実行
        .map_err(|e| DomainError::Repository(e.to_string()))?; // エラー変換

        // Option<UserRow> → Option<User> に変換
        Ok(row.map(Into::into))
    }
}
