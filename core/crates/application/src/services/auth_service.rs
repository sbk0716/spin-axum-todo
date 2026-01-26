// =============================================================================
// application/src/services/auth_service.rs: 認証サービス
// =============================================================================
// ユーザー登録とログインを処理する。
// パスワードのハッシュ化と JWT の発行を行う。
//
// 統一 CQRS パターン:
// - UserReader: ログイン認証（find_by_email）
// - UserWriter: ユーザー登録（create）
//
// セキュリティ:
// - パスワード: bcrypt でハッシュ化（コスト係数: DEFAULT_COST = 12）
// - JWT: HS256 アルゴリズムで署名
// - 認証エラー: 詳細を漏らさない（"invalid credentials" のみ）
// =============================================================================

// -----------------------------------------------------------------------------
// 標準ライブラリのインポート
// -----------------------------------------------------------------------------

use std::sync::Arc; // スレッド安全な参照カウントスマートポインタ

// -----------------------------------------------------------------------------
// 外部クレートのインポート
// -----------------------------------------------------------------------------

// chrono: 日時操作
// Duration: 時間の長さを表す
// Utc: UTC タイムゾーン
use chrono::{Duration, Utc};

// domain クレートの型
use domain::{DomainError, User, UserReader, UserWriter};

// jsonwebtoken: JWT のエンコード/デコード
// encode: JWT トークン生成
// EncodingKey: 署名用の鍵
// Header: JWT ヘッダー（アルゴリズム指定）
use jsonwebtoken::{EncodingKey, Header, encode};

// serde: シリアライズ/デシリアライズ
use serde::{Deserialize, Serialize};

// tracing: 構造化ログ
use tracing::info;

// =============================================================================
// JWT クレーム
// =============================================================================

/// JWT クレーム（ペイロード）
///
/// JWT トークンに含まれる情報。
/// RFC 7519 の標準クレームに準拠。
///
/// # フィールド
///
/// - `sub` (Subject): ユーザー ID（UUID 文字列）
/// - `exp` (Expiration Time): 有効期限（Unix タイムスタンプ）
/// - `iat` (Issued At): 発行日時（Unix タイムスタンプ）
///
/// # セキュリティ
///
/// 機密情報（パスワード、メールアドレス）は含めない。
/// クレームはデコードすれば誰でも読める（Base64）。
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// ユーザー ID（UUID 文字列）
    /// JWT の "subject" クレーム
    pub sub: String,

    /// 有効期限（Unix タイムスタンプ）
    /// この時刻を過ぎるとトークンは無効
    pub exp: usize,

    /// 発行日時（Unix タイムスタンプ）
    /// トークンが生成された時刻
    pub iat: usize,
}

// =============================================================================
// 認証サービス
// =============================================================================

/// 認証サービス
///
/// ユーザー登録、ログイン、JWT 発行を担当する。
///
/// # ジェネリクス
///
/// - `UR: UserReader` - ユーザー読み取り（ログイン認証用）
/// - `UW: UserWriter` - ユーザー書き込み（ユーザー登録用）
///
/// CQRS パターンに従い、読み取りと書き込みを分離。
///
/// # 依存性注入
///
/// Arc でラップされたリポジトリを受け取る。
/// これにより:
/// - テスト時にモック実装を注入可能
/// - 複数リクエストでリポジトリを共有
pub struct AuthService<UR: UserReader, UW: UserWriter> {
    /// ユーザー読み取りリポジトリ（Queries 用）
    /// ログイン時のユーザー検索に使用
    user_reader: Arc<UR>,

    /// ユーザー書き込みリポジトリ（Commands 用）
    /// 登録時のユーザー作成に使用
    user_writer: Arc<UW>,

    /// JWT 署名用シークレット
    /// 環境変数から読み込む（本番環境では安全に管理）
    jwt_secret: String,

    /// JWT 有効期間（時間）
    /// 例: 24 = 24時間有効
    jwt_expiry_hours: i64,
}

// -----------------------------------------------------------------------------
// Clone トレイト実装
// -----------------------------------------------------------------------------

impl<UR: UserReader, UW: UserWriter> Clone for AuthService<UR, UW> {
    fn clone(&self) -> Self {
        Self {
            user_reader: Arc::clone(&self.user_reader),
            user_writer: Arc::clone(&self.user_writer),
            // String は Clone を実装しているので直接 clone()
            jwt_secret: self.jwt_secret.clone(),
            // i64 は Copy トレイトを実装しているのでそのままコピー
            jwt_expiry_hours: self.jwt_expiry_hours,
        }
    }
}

// -----------------------------------------------------------------------------
// AuthService の実装
// -----------------------------------------------------------------------------

impl<UR: UserReader, UW: UserWriter> AuthService<UR, UW> {
    /// 新しい AuthService を作成
    ///
    /// # Arguments
    /// * `user_reader` - ユーザー読み取りリポジトリ（Queries）
    /// * `user_writer` - ユーザー書き込みリポジトリ（Commands）
    /// * `jwt_secret` - JWT 署名用シークレット（本番は安全に管理）
    /// * `jwt_expiry_hours` - JWT 有効期間（時間）
    pub fn new(
        user_reader: Arc<UR>,
        user_writer: Arc<UW>,
        jwt_secret: String,
        jwt_expiry_hours: i64,
    ) -> Self {
        Self {
            user_reader,
            user_writer,
            jwt_secret,
            jwt_expiry_hours,
        }
    }

    /// ユーザー登録
    ///
    /// # Arguments
    /// * `email` - メールアドレス
    /// * `password` - パスワード（平文）
    /// * `display_name` - 表示名（任意）
    ///
    /// # Returns
    /// * `Ok(User)` - 作成されたユーザー
    /// * `Err(DomainError::Validation)` - バリデーションエラー
    /// * `Err(DomainError::Duplicate)` - メールアドレスが既に使用されている
    ///
    /// # 処理フロー
    ///
    /// 1. メールアドレスのバリデーション
    /// 2. パスワードのバリデーション
    /// 3. パスワードを bcrypt でハッシュ化
    /// 4. ユーザーエンティティ作成
    /// 5. データベースに保存
    pub async fn register(
        &self,
        email: &str,
        password: &str,
        display_name: Option<String>,
    ) -> Result<User, DomainError> {
        // 1. バリデーション
        // User::validate_email はメールアドレスを正規化（小文字化）して返す
        let email = User::validate_email(email)?;
        // パスワードの最小長などをチェック
        User::validate_password(password)?;

        // 2. パスワードハッシュ化（bcrypt）
        // DEFAULT_COST = 12: 適度なセキュリティと速度のバランス
        let password_hash = bcrypt::hash(password, bcrypt::DEFAULT_COST)
            .map_err(|e| DomainError::Validation(format!("password hash error: {}", e)))?;

        // 3. ユーザー作成（UserWriter を使用）
        let user = User::new(email.clone(), password_hash, display_name);
        let created_user = self.user_writer.create(&user).await?;

        // 4. ログ出力（構造化ログ）
        info!(user_id = %created_user.id, email = %email, "User registered");

        Ok(created_user)
    }

    /// ログイン
    ///
    /// # Arguments
    /// * `email` - メールアドレス
    /// * `password` - パスワード（平文）
    ///
    /// # Returns
    /// * `Ok(String)` - JWT トークン
    /// * `Err(DomainError::Authentication)` - 認証失敗
    ///
    /// # セキュリティ
    ///
    /// エラーメッセージは詳細を漏らさない（"invalid credentials" のみ）。
    /// これにより:
    /// - ユーザーの存在確認を防ぐ
    /// - パスワード推測攻撃の情報を与えない
    pub async fn login(&self, email: &str, password: &str) -> Result<String, DomainError> {
        // 1. メールアドレスでユーザー検索（UserReader を使用）
        // メールアドレスを正規化（小文字化、空白除去）
        let email = email.trim().to_lowercase();
        let user = self
            .user_reader
            .find_by_email(&email)
            .await?
            // ユーザーが見つからない場合も "invalid credentials" を返す
            // （ユーザーの存在確認を防ぐ）
            .ok_or_else(|| DomainError::Authentication("invalid credentials".into()))?;

        // 2. パスワード検証
        // bcrypt::verify は平文とハッシュを比較
        let is_valid = bcrypt::verify(password, &user.password_hash)
            .map_err(|e| DomainError::Authentication(format!("password verify error: {}", e)))?;

        // パスワードが一致しない場合も同じエラーメッセージ
        if !is_valid {
            return Err(DomainError::Authentication("invalid credentials".into()));
        }

        // 3. JWT 発行
        let token = self.generate_token(&user)?;

        // 4. ログ出力
        info!(user_id = %user.id, email = %email, "User logged in");

        Ok(token)
    }

    /// JWT トークン生成
    ///
    /// # Arguments
    /// * `user` - トークンを発行するユーザー
    ///
    /// # Returns
    /// * `Ok(String)` - エンコードされた JWT トークン
    /// * `Err(DomainError::Authentication)` - トークン生成エラー
    fn generate_token(&self, user: &User) -> Result<String, DomainError> {
        // 現在時刻（UTC）
        let now = Utc::now();
        // 有効期限 = 現在時刻 + 設定時間
        let exp = now + Duration::hours(self.jwt_expiry_hours);

        // クレーム（ペイロード）を作成
        let claims = Claims {
            sub: user.id.to_string(),      // ユーザー ID
            exp: exp.timestamp() as usize, // 有効期限（Unix タイムスタンプ）
            iat: now.timestamp() as usize, // 発行日時（Unix タイムスタンプ）
        };

        // JWT をエンコード
        // Header::default() = HS256 アルゴリズム
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .map_err(|e| DomainError::Authentication(format!("token generation error: {}", e)))
    }
}

// =============================================================================
// テスト
// =============================================================================

#[cfg(test)]
mod tests {
    // テストは統合テストで実施
    // モックリポジトリを使用した単体テストも追加可能
}
