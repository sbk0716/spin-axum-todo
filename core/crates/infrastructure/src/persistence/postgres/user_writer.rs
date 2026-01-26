// =============================================================================
// infrastructure/src/persistence/postgres/user_writer.rs: ユーザー書き込み実装
// =============================================================================
// PostgreSQL を使用した UserWriter の実装。
// ユーザー登録、更新、削除を担当。
// Writer DB プールと組み合わせることでレプリケーション対応可能。
//
// 使用場面:
// - ユーザー登録: create で新規作成
// - プロフィール更新: update で表示名変更
// - アカウント削除: delete でユーザー削除（CASCADE で TODO も削除）
// =============================================================================

// -----------------------------------------------------------------------------
// 外部クレートのインポート
// -----------------------------------------------------------------------------

// async_trait: async fn を含むトレイトを定義可能にする
use async_trait::async_trait;

// chrono: 日付・時刻ライブラリ
use chrono::{DateTime, Utc};

// domain: ドメイン層の型をインポート
use domain::{DomainError, User, UserWriter};

// sqlx: PostgreSQL クライアントライブラリ
use sqlx::{FromRow, PgPool};

// tracing: 構造化ログライブラリ
use tracing::debug;

// uuid: 一意識別子ライブラリ
use uuid::Uuid;

// =============================================================================
// PostgresUserWriter 構造体
// =============================================================================

/// PostgreSQL への接続を管理するユーザー書き込みリポジトリ
///
/// # CQRS パターン
///
/// Writer 実装として、状態変更操作を提供。
/// 参照操作は PostgresUserReader が担当。
///
/// # トランザクション
///
/// 各メソッドは単一クエリなので、明示的なトランザクション不要。
/// 複数操作が必要な場合は TransactionalTodoService を参照。
pub struct PostgresUserWriter {
    /// PostgreSQL 接続プール（Writer 用）
    pool: PgPool,
}

impl PostgresUserWriter {
    /// 新しい PostgresUserWriter を作成する
    ///
    /// # Arguments
    ///
    /// * `pool` - PostgreSQL 接続プール（Writer プール）
    ///
    /// # Note
    ///
    /// 本番環境では Writer エンドポイント（プライマリ）を指すプールを渡す。
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

// =============================================================================
// UserRow 構造体（内部用）
// =============================================================================

/// データベース行からのマッピング用構造体
///
/// RETURNING 句の結果を受け取るために使用。
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
impl From<UserRow> for User {
    fn from(row: UserRow) -> Self {
        User::from_raw(
            row.id,
            row.email,
            row.password_hash,
            row.display_name,
            row.created_at,
            row.updated_at,
        )
    }
}

// =============================================================================
// UserWriter トレイト実装
// =============================================================================

/// UserWriter トレイトの PostgreSQL 実装
#[async_trait]
impl UserWriter for PostgresUserWriter {
    /// ユーザーを作成（登録）
    ///
    /// # Arguments
    ///
    /// * `user` - 作成するユーザーエンティティ
    ///
    /// # Returns
    ///
    /// * `Ok(User)` - 作成されたユーザー
    /// * `Err(DomainError::Duplicate)` - メールアドレスが既存の場合
    /// * `Err(DomainError::Repository)` - その他の DB エラー
    ///
    /// # Note
    ///
    /// password_hash は呼び出し側（AuthService）で bcrypt 化済みの前提。
    async fn create(&self, user: &User) -> Result<User, DomainError> {
        // 構造化ログ: メールアドレスを記録
        debug!(email = %user.email, "Creating user in PostgreSQL");

        // INSERT ... RETURNING で挿入と取得を同時に実行
        let row: UserRow = sqlx::query_as(
            r#"
            INSERT INTO users (id, email, password_hash, display_name, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, email, password_hash, display_name, created_at, updated_at
            "#,
        )
        .bind(user.id) // $1: 事前生成した UUID
        .bind(&user.email) // $2: メールアドレス（UNIQUE）
        .bind(&user.password_hash) // $3: bcrypt ハッシュ
        .bind(&user.display_name) // $4: 表示名（NULL 許容）
        .bind(user.created_at) // $5: 作成日時
        .bind(user.updated_at) // $6: 更新日時
        .fetch_one(&self.pool) // 1行取得
        .await
        .map_err(|e| {
            // UNIQUE 制約違反をチェック
            // PostgreSQL のエラーメッセージから判定
            if e.to_string().contains("duplicate key")
                || e.to_string().contains("unique constraint")
            {
                // メールアドレス重複エラー
                DomainError::Duplicate("email already exists".into())
            } else {
                // その他の DB エラー
                DomainError::Repository(e.to_string())
            }
        })?;

        // UserRow → User に変換
        Ok(row.into())
    }

    /// ユーザーを更新
    ///
    /// 現在は display_name のみ更新可能。
    /// email やパスワードの変更は別途エンドポイントが必要。
    ///
    /// # Arguments
    ///
    /// * `user` - 更新するユーザーエンティティ
    ///
    /// # Returns
    ///
    /// * `Ok(User)` - 更新後のユーザー
    /// * `Err(DomainError::Repository)` - DB エラー
    ///
    /// # Note
    ///
    /// 存在しない ID を指定した場合、fetch_one がエラーになる。
    /// 必要に応じて NotFound エラーに変換することも検討。
    async fn update(&self, user: &User) -> Result<User, DomainError> {
        // 構造化ログ: ユーザー ID を記録
        debug!(user_id = %user.id, "Updating user in PostgreSQL");

        // UPDATE ... RETURNING で更新と取得を同時に実行
        let row: UserRow = sqlx::query_as(
            r#"
            UPDATE users
            SET display_name = $2, updated_at = $3
            WHERE id = $1
            RETURNING id, email, password_hash, display_name, created_at, updated_at
            "#,
        )
        .bind(user.id) // $1: 更新対象の ID
        .bind(&user.display_name) // $2: 新しい表示名
        .bind(user.updated_at) // $3: 更新日時
        .fetch_one(&self.pool) // 1行取得（存在しない場合はエラー）
        .await
        .map_err(|e| DomainError::Repository(e.to_string()))?;

        // UserRow → User に変換
        Ok(row.into())
    }

    /// ユーザーを削除（CASCADE で todos も削除）
    ///
    /// # Arguments
    ///
    /// * `id` - 削除するユーザーの ID
    ///
    /// # Returns
    ///
    /// * `Ok(true)` - 削除成功（1行削除）
    /// * `Ok(false)` - 該当なし（0行削除）
    /// * `Err(DomainError)` - DB エラー
    ///
    /// # Warning
    ///
    /// CASCADE により、ユーザーに紐付く全 TODO も削除される。
    /// この操作は元に戻せないため、呼び出し側で確認を取ること。
    async fn delete(&self, id: Uuid) -> Result<bool, DomainError> {
        // 構造化ログ: ユーザー ID を記録
        debug!(user_id = %id, "Deleting user from PostgreSQL");

        // DELETE 文を実行
        let result = sqlx::query(
            r#"
            DELETE FROM users
            WHERE id = $1
            "#,
        )
        .bind(id) // $1: 削除対象の ID
        .execute(&self.pool) // 実行（影響行数のみ取得）
        .await
        .map_err(|e| DomainError::Repository(e.to_string()))?;

        // rows_affected(): 削除された行数
        // 1 以上なら成功、0 なら該当なし
        Ok(result.rows_affected() > 0)
    }
}
