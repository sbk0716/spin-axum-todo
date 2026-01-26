// =============================================================================
// domain/src/repositories/user_repository.rs: ユーザーリポジトリトレイト
// =============================================================================
// ユーザーの永続化に関する抽象インターフェース。
// ローカル認証のため、メールアドレスでの検索が主要なユースケース。
//
// 統一 CQRS パターン:
// - UserReader: 参照操作（Queries）- 認証時のユーザー検索
// - UserWriter: 状態変更操作（Commands）- ユーザー登録・更新・削除
//
// 認証フロー:
// 1. ログイン: find_by_email() でユーザー取得 → パスワード検証
// 2. JWT 検証: find_by_id() でユーザー取得 → 存在確認
// 3. ユーザー登録: create() で新規ユーザー作成
// =============================================================================

// -----------------------------------------------------------------------------
// 外部クレートのインポート
// -----------------------------------------------------------------------------

// async_trait: トレイト内で async fn を使用可能にする
use async_trait::async_trait;

// uuid: 一意識別子
use uuid::Uuid;

// 同じクレート内のエンティティとエラー型
use crate::entities::User;
use crate::errors::DomainError;

// =============================================================================
// CQRS: Reader / Writer トレイト分離
// =============================================================================

/// ユーザー読み取りトレイト（Queries 用）
///
/// ログイン認証や JWT 検証時のユーザー取得に使用。
/// Reader DB プールと組み合わせることでレプリケーション対応可能。
///
/// # 実装例
/// - `PostgresUserReader`: PostgreSQL 実装（infrastructure 層）
///
/// # 認証における役割
/// - `find_by_email()`: ログイン時のユーザー検索
/// - `find_by_id()`: JWT トークンからのユーザー検証
#[async_trait]
pub trait UserReader: Send + Sync {
    /// メールアドレスでユーザーを検索
    ///
    /// ログイン時の認証に使用。メールアドレスは正規化（小文字化）済み。
    ///
    /// # Arguments
    /// * `email` - 検索するメールアドレス（小文字）
    ///
    /// # Returns
    /// * `Ok(Some(User))` - ユーザーが見つかった場合
    /// * `Ok(None)` - ユーザーが見つからない場合
    /// * `Err(DomainError::Repository)` - データベースエラー
    ///
    /// # Security Note
    /// 返される User にはパスワードハッシュが含まれる。
    /// API レスポンスに含める場合は、`#[serde(skip_serializing)]` により
    /// password_hash は除外される。
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, DomainError>;

    /// ID でユーザーを検索
    ///
    /// JWT の sub クレームからユーザーを取得する際に使用。
    ///
    /// # Arguments
    /// * `id` - ユーザーの UUID（JWT の sub クレーム）
    ///
    /// # Returns
    /// * `Ok(Some(User))` - ユーザーが見つかった場合
    /// * `Ok(None)` - ユーザーが見つからない場合
    /// * `Err(DomainError::Repository)` - データベースエラー
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError>;
}

/// ユーザー書き込みトレイト（Commands 用）
///
/// ユーザー登録、更新、削除を担当。
/// Writer DB プールと組み合わせることでレプリケーション対応可能。
///
/// # 実装例
/// - `PostgresUserWriter`: PostgreSQL 実装（infrastructure 層）
///
/// # CASCADE 削除
/// ユーザー削除時、関連する todos も CASCADE 削除される。
/// データベースの外部キー制約で設定されている。
#[async_trait]
pub trait UserWriter: Send + Sync {
    /// ユーザーを作成（登録）
    ///
    /// # Arguments
    /// * `user` - 作成するユーザーエンティティ
    ///   - email: バリデーション済み、正規化済み
    ///   - password_hash: bcrypt でハッシュ化済み
    ///
    /// # Returns
    /// * `Ok(User)` - 作成されたユーザー
    /// * `Err(DomainError::Duplicate)` - メールアドレスが既に存在する場合
    /// * `Err(DomainError::Repository)` - データベースエラー
    ///
    /// # Example
    /// ```rust,ignore
    /// // 認証サービスでの使用例（疑似コード）
    /// let hashed = bcrypt::hash(password, DEFAULT_COST)?;
    /// let user = User::new(email, hashed, display_name);
    /// let created = writer.create(&user).await?;
    /// ```
    async fn create(&self, user: &User) -> Result<User, DomainError>;

    /// ユーザーを更新
    ///
    /// # Arguments
    /// * `user` - 更新するユーザーエンティティ
    ///
    /// # Returns
    /// * `Ok(User)` - 更新されたユーザー
    /// * `Err(DomainError::NotFound)` - ユーザーが見つからない場合
    /// * `Err(DomainError::Repository)` - データベースエラー
    async fn update(&self, user: &User) -> Result<User, DomainError>;

    /// ユーザーを削除
    ///
    /// # Arguments
    /// * `id` - 削除するユーザーの UUID
    ///
    /// # Returns
    /// * `Ok(true)` - 削除成功
    /// * `Ok(false)` - ユーザーが存在しない
    /// * `Err(DomainError::Repository)` - データベースエラー
    ///
    /// # CASCADE 削除
    /// データベースの外部キー制約により、関連する todos も自動削除される。
    async fn delete(&self, id: Uuid) -> Result<bool, DomainError>;
}
