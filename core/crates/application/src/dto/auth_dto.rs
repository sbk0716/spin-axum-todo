// =============================================================================
// application/src/dto/auth_dto.rs: 認証関連の DTO
// =============================================================================
// ユーザー認証（登録、ログイン）に関するリクエスト/レスポンス DTO。
//
// セキュリティ注意:
// - パスワードは平文で受け取り、サービス層でハッシュ化
// - パスワードはレスポンスに含めない
// - JWT トークンのみをレスポンスで返す
// =============================================================================

// -----------------------------------------------------------------------------
// 外部クレートのインポート
// -----------------------------------------------------------------------------

// serde: シリアライズ/デシリアライズ
use serde::{Deserialize, Serialize};

// =============================================================================
// ユーザー登録リクエスト
// =============================================================================

/// ユーザー登録リクエスト
///
/// 新規ユーザーの登録に使用。
/// パスワードは平文で受け取り、AuthService でハッシュ化する。
///
/// # 例
///
/// ```json
/// {
///   "email": "user@example.com",
///   "password": "SecurePassword123",
///   "display_name": "John Doe"
/// }
/// ```
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    /// メールアドレス
    /// バリデーション: User::validate_email() で検証
    pub email: String,

    /// パスワード（平文）
    /// バリデーション: User::validate_password() で検証
    /// セキュリティ: サービス層で bcrypt ハッシュ化
    pub password: String,

    /// 表示名（任意）
    /// プロフィールに表示される名前
    pub display_name: Option<String>,
}

// =============================================================================
// ログインリクエスト
// =============================================================================

/// ログインリクエスト
///
/// 既存ユーザーのログインに使用。
/// 成功すると JWT トークンが発行される。
///
/// # 例
///
/// ```json
/// {
///   "email": "user@example.com",
///   "password": "SecurePassword123"
/// }
/// ```
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    /// メールアドレス
    /// 大文字小文字を区別しない（小文字に正規化される）
    pub email: String,

    /// パスワード（平文）
    /// bcrypt で保存されたハッシュと照合
    pub password: String,
}

// =============================================================================
// トークンレスポンス
// =============================================================================

/// 認証レスポンス（JWT トークン）
///
/// ログイン成功時に返される。
/// クライアントはこのトークンを Authorization ヘッダーに設定する。
///
/// # 例
///
/// ```json
/// {
///   "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
///   "token_type": "Bearer"
/// }
/// ```
///
/// # 使用方法
///
/// ```http
/// Authorization: Bearer <token>
/// ```
#[derive(Debug, Serialize)]
pub struct TokenResponse {
    /// JWT トークン
    /// クレーム: sub（ユーザーID）、exp（有効期限）、iat（発行日時）
    pub token: String,

    /// トークンタイプ（常に "Bearer"）
    /// OAuth 2.0 の Bearer Token 形式に準拠
    pub token_type: String,
}

impl TokenResponse {
    /// 新しい TokenResponse を作成
    ///
    /// # Arguments
    /// * `token` - JWT トークン文字列
    ///
    /// # Returns
    /// token_type が "Bearer" に設定された TokenResponse
    pub fn new(token: String) -> Self {
        Self {
            token,
            token_type: "Bearer".to_string(),
        }
    }
}

// =============================================================================
// ユーザーレスポンス
// =============================================================================

/// ユーザーレスポンス（登録成功時）
///
/// ユーザー登録成功時に返される。
/// パスワードハッシュは含まれない（セキュリティ）。
///
/// # 例
///
/// ```json
/// {
///   "id": "550e8400-e29b-41d4-a716-446655440000",
///   "email": "user@example.com",
///   "display_name": "John Doe",
///   "created_at": "2024-01-15T10:30:00Z"
/// }
/// ```
#[derive(Debug, Serialize)]
pub struct UserResponse {
    /// ユーザー ID（UUID 文字列）
    pub id: String,

    /// メールアドレス
    pub email: String,

    /// 表示名（任意）
    pub display_name: Option<String>,

    /// 作成日時（ISO 8601 形式）
    pub created_at: String,
}

// -----------------------------------------------------------------------------
// From トレイト実装
// -----------------------------------------------------------------------------

/// domain::User → UserResponse への変換
///
/// password_hash は含まれない（セキュリティ）。
/// created_at は RFC 3339 形式の文字列に変換。
impl From<domain::User> for UserResponse {
    fn from(user: domain::User) -> Self {
        Self {
            // UUID を文字列に変換
            id: user.id.to_string(),
            email: user.email,
            display_name: user.display_name,
            // DateTime<Utc> を RFC 3339 形式の文字列に変換
            // 例: "2024-01-15T10:30:00+00:00"
            created_at: user.created_at.to_rfc3339(),
        }
    }
}
