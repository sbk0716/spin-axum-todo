// =============================================================================
// domain/src/entities/user.rs: User エンティティ
// =============================================================================
// ローカル認証用のユーザーエンティティ。
// パスワードハッシュを含み、認証サービスで使用される。
//
// このエンティティが持つ責務:
// - ユーザーのデータ構造を定義
// - 認証に必要な情報（メール、パスワードハッシュ）を保持
// - バリデーションルール（メール形式、パスワード長）をカプセル化
//
// セキュリティ考慮:
// - password_hash は #[serde(skip_serializing)] で JSON 出力から除外
// - パスワードは平文で保存しない（bcrypt ハッシュのみ）
// =============================================================================

// -----------------------------------------------------------------------------
// 外部クレートのインポート
// -----------------------------------------------------------------------------

// chrono: 日時処理ライブラリ
use chrono::{DateTime, Utc};

// serde: シリアライズ/デシリアライズ
use serde::{Deserialize, Serialize};

// uuid: 一意識別子
use uuid::Uuid;

// 同じクレート内のエラー型
use crate::errors::DomainError;

// =============================================================================
// User 構造体の定義
// =============================================================================

/// ユーザーエンティティ
///
/// ローカル認証を使用するため、パスワードハッシュを保持する。
/// JWT の sub クレームには id（UUID）を使用する。
///
/// # データベーステーブルとの対応
///
/// | フィールド | カラム | 型 |
/// |-----------|--------|-----|
/// | id | id | UUID (PRIMARY KEY) |
/// | email | email | VARCHAR(255) UNIQUE NOT NULL |
/// | password_hash | password_hash | VARCHAR(255) NOT NULL |
/// | display_name | display_name | VARCHAR(255) |
/// | created_at | created_at | TIMESTAMPTZ |
/// | updated_at | updated_at | TIMESTAMPTZ |
// -----------------------------------------------------------------------------
// derive マクロの説明:
// - Debug: デバッグ出力用（{:?} フォーマット）
// - Clone: 値のコピーを作成可能
// - PartialEq: == 演算子で比較可能
// - Serialize/Deserialize: JSON 変換可能
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    /// 一意識別子（UUID v4）- JWT sub クレームとして使用
    ///
    /// JWT（JSON Web Token）の sub（subject）クレームに設定される。
    /// これにより、トークンからユーザーを特定できる。
    pub id: Uuid,

    /// メールアドレス（ログイン用、一意）
    ///
    /// ログイン時の識別子として使用。
    /// データベースで UNIQUE 制約が設定されている。
    pub email: String,

    /// パスワードハッシュ（bcrypt）
    ///
    /// 平文パスワードは保存せず、bcrypt でハッシュ化した値のみ保持。
    /// bcrypt は salt を内部に含むため、別途 salt を保存する必要がない。
    ///
    /// # Security
    /// `#[serde(skip_serializing)]` により、JSON シリアライズ時に出力されない。
    /// これにより、API レスポンスにパスワードハッシュが含まれることを防ぐ。
    #[serde(skip_serializing)]
    pub password_hash: String,

    /// 表示名（任意）
    ///
    /// UI に表示するためのユーザー名。
    /// 未設定の場合は None。
    pub display_name: Option<String>,

    /// 作成日時（UTC）
    pub created_at: DateTime<Utc>,

    /// 最終更新日時（UTC）
    pub updated_at: DateTime<Utc>,
}

// =============================================================================
// User のメソッド実装
// =============================================================================

impl User {
    /// 新しいユーザーを作成
    ///
    /// # Arguments
    /// * `email` - メールアドレス（バリデーション済み）
    /// * `password_hash` - bcrypt でハッシュ化済みのパスワード
    /// * `display_name` - 表示名（任意）
    ///
    /// # Returns
    /// 新しい User インスタンス
    ///
    /// # Note
    /// password_hash は既にハッシュ化済みであること。
    /// 平文パスワードをこのメソッドに渡さないこと。
    pub fn new(email: String, password_hash: String, display_name: Option<String>) -> Self {
        // 現在時刻を取得
        let now = Utc::now();

        // User インスタンスを作成
        Self {
            // 新しい UUID v4 を生成
            id: Uuid::new_v4(),

            // メールアドレスを設定
            email,

            // ハッシュ化済みパスワードを設定
            password_hash,

            // 表示名を設定（None でも可）
            display_name,

            // 作成日時と更新日時を現在時刻で初期化
            created_at: now,
            updated_at: now,
        }
    }

    /// データベースからの復元用コンストラクタ
    ///
    /// 既存のデータからエンティティを再構築する。
    /// インフラ層（PostgresUserReader 等）からの呼び出し専用。
    ///
    /// # Arguments
    /// * `id` - UUID
    /// * `email` - メールアドレス
    /// * `password_hash` - パスワードハッシュ
    /// * `display_name` - 表示名
    /// * `created_at` - 作成日時
    /// * `updated_at` - 最終更新日時
    pub fn from_raw(
        id: Uuid,
        email: String,
        password_hash: String,
        display_name: Option<String>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        // 引数をそのまま構造体に設定
        Self {
            id,
            email,
            password_hash,
            display_name,
            created_at,
            updated_at,
        }
    }

    /// メールアドレスのバリデーション
    ///
    /// # Arguments
    /// * `email` - 検証するメールアドレス
    ///
    /// # Returns
    /// * `Ok(String)` - トリム済みの小文字メールアドレス
    /// * `Err(DomainError::Validation)` - 無効なメールアドレス
    ///
    /// # Example
    /// ```
    /// use domain::User;
    ///
    /// // 大文字・前後空白は正規化される
    /// let email = User::validate_email("  Test@Example.COM  ").unwrap();
    /// assert_eq!(email, "test@example.com");
    /// ```
    ///
    /// # Note
    /// 現在は簡易的なバリデーション（@ と . の存在確認）のみ。
    /// 本番環境ではより厳密な検証（RFC 5322 準拠）が推奨される。
    pub fn validate_email(email: &str) -> Result<String, DomainError> {
        // trim(): 前後の空白を除去
        // to_lowercase(): 小文字に変換（メールアドレスは大文字小文字を区別しない）
        let email = email.trim().to_lowercase();

        // 空文字チェック
        if email.is_empty() {
            return Err(DomainError::Validation("email cannot be empty".into()));
        }

        // 簡易的なメールアドレスバリデーション
        // @ と . が含まれていることを確認
        // 本番環境ではより厳密な検証が必要（例: validator クレートの使用）
        if !email.contains('@') || !email.contains('.') {
            return Err(DomainError::Validation("invalid email format".into()));
        }

        // 正規化されたメールアドレスを返す
        Ok(email)
    }

    /// パスワードのバリデーション
    ///
    /// # Arguments
    /// * `password` - 検証するパスワード（平文）
    ///
    /// # Returns
    /// * `Ok(())` - パスワードが有効
    /// * `Err(DomainError::Validation)` - パスワードが無効
    ///
    /// # Current Rules
    /// - 最低 8 文字以上
    ///
    /// # Future Improvements
    /// - 大文字を含む
    /// - 小文字を含む
    /// - 数字を含む
    /// - 特殊文字を含む
    pub fn validate_password(password: &str) -> Result<(), DomainError> {
        // 最低文字数チェック
        if password.len() < 8 {
            return Err(DomainError::Validation(
                "password must be at least 8 characters".into(),
            ));
        }

        // バリデーション成功
        Ok(())
    }

    /// プロファイル更新
    ///
    /// # Arguments
    /// * `display_name` - 新しい表示名
    ///   * `Some(Some("名前"))`: 表示名を設定
    ///   * `Some(None)`: 表示名を削除
    ///   * `None`: 変更なし
    ///
    /// # Note
    /// 二重 Option により「変更なし」と「削除」を区別できる。
    pub fn update_profile(&mut self, display_name: Option<Option<String>>) {
        // 表示名の更新（二重 Option の処理）
        if let Some(name) = display_name {
            // name は Option<String>
            // Some("名前") または None が設定される
            self.display_name = name;
        }

        // 更新日時を現在時刻に設定
        self.updated_at = Utc::now();
    }
}

// =============================================================================
// ユニットテスト
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// 新規ユーザー作成のテスト
    #[test]
    fn test_new_user() {
        // ユーザーを新規作成
        let user = User::new(
            "test@example.com".to_string(), // メールアドレス
            "hashed_password".to_string(),  // ハッシュ化済みパスワード
            Some("Test User".to_string()),  // 表示名
        );

        // アサーション
        assert_eq!(user.email, "test@example.com");
        assert_eq!(user.password_hash, "hashed_password");
        assert_eq!(user.display_name, Some("Test User".to_string()));
    }

    /// メールバリデーション成功のテスト
    #[test]
    fn test_validate_email_success() {
        // 大文字・前後空白のあるメールアドレス
        let result = User::validate_email("  Test@Example.COM  ");

        // 小文字・トリム済みになっていること
        assert_eq!(result.unwrap(), "test@example.com");
    }

    /// メールバリデーション失敗のテスト（空文字）
    #[test]
    fn test_validate_email_empty() {
        // 空白のみ
        let result = User::validate_email("   ");

        // Validation エラーであること
        assert!(matches!(result, Err(DomainError::Validation(_))));
    }

    /// メールバリデーション失敗のテスト（無効な形式）
    #[test]
    fn test_validate_email_invalid() {
        // @ がないメールアドレス
        let result = User::validate_email("invalid-email");

        // Validation エラーであること
        assert!(matches!(result, Err(DomainError::Validation(_))));
    }

    /// パスワードバリデーション成功のテスト
    #[test]
    fn test_validate_password_success() {
        // 8文字以上のパスワード
        let result = User::validate_password("password123");

        // 成功すること
        assert!(result.is_ok());
    }

    /// パスワードバリデーション失敗のテスト（短すぎる）
    #[test]
    fn test_validate_password_too_short() {
        // 8文字未満のパスワード
        let result = User::validate_password("short");

        // Validation エラーであること
        assert!(matches!(result, Err(DomainError::Validation(_))));
    }
}
