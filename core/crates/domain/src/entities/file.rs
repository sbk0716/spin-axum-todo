// =============================================================================
// domain/src/entities/file.rs: File エンティティ
// =============================================================================
// TODO に添付されるファイルのメタデータを表すエンティティ。
// 実際のファイルコンテンツはストレージ（S3 等）に保存され、
// このエンティティはメタデータのみを管理する。
//
// このエンティティが持つ責務:
// - ファイルメタデータのデータ構造を定義
// - セキュリティバリデーション（パストラバーサル防止等）
// - ファイルサイズ・MIME タイプの検証
//
// ストレージ設計:
// - ファイル実体は S3 等のオブジェクトストレージに保存
// - storage_path フィールドで実体への参照を保持
// - データベースにはメタデータのみを保存（軽量化）
// =============================================================================

// -----------------------------------------------------------------------------
// 外部クレートのインポート
// -----------------------------------------------------------------------------

// chrono: 日時処理
use chrono::{DateTime, Utc};

// serde: シリアライズ/デシリアライズ
use serde::{Deserialize, Serialize};

// uuid: 一意識別子
use uuid::Uuid;

// 同じクレート内のエラー型
use crate::errors::DomainError;

// =============================================================================
// 定数定義
// =============================================================================
// const キーワードでコンパイル時定数を定義。
// 大文字スネークケースが慣例（UPPER_SNAKE_CASE）。
// =============================================================================

/// ファイルサイズの最大値（100MB）
///
/// 100 * 1024 * 1024 = 104,857,600 バイト
/// i64 を使用するのは、データベース（PostgreSQL BIGINT）との互換性のため。
const MAX_FILE_SIZE_BYTES: i64 = 100 * 1024 * 1024;

/// ファイル名の最大長
///
/// 多くのファイルシステムの制限（255文字）に合わせている。
const MAX_FILENAME_LENGTH: usize = 255;

// =============================================================================
// File 構造体の定義
// =============================================================================

/// File エンティティ
///
/// TODO に添付されるファイルのメタデータ。
/// ストレージパスは一意制約があり、ファイルの実体への参照となる。
///
/// # データベーステーブルとの対応
///
/// | フィールド | カラム | 型 |
/// |-----------|--------|-----|
/// | id | id | UUID (PRIMARY KEY) |
/// | todo_id | todo_id | UUID (FOREIGN KEY → todos.id) |
/// | filename | filename | VARCHAR(255) NOT NULL |
/// | mime_type | mime_type | VARCHAR(255) NOT NULL |
/// | size_bytes | size_bytes | BIGINT NOT NULL |
/// | storage_path | storage_path | VARCHAR(1024) UNIQUE NOT NULL |
/// | created_at | created_at | TIMESTAMPTZ |
// -----------------------------------------------------------------------------
// derive マクロの説明:
// - Debug: デバッグ出力用
// - Clone: 値のコピーを作成可能
// - PartialEq: == 演算子で比較可能
// - Serialize/Deserialize: JSON 変換可能
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct File {
    /// 一意識別子（UUID v4）
    pub id: Uuid,

    /// 関連する TODO の ID（todos.id への外部キー）
    ///
    /// CASCADE DELETE により、TODO 削除時にファイルも削除される。
    pub todo_id: Uuid,

    /// 元のファイル名
    ///
    /// ユーザーがアップロードした時点のファイル名。
    /// ダウンロード時の Content-Disposition ヘッダーに使用。
    pub filename: String,

    /// MIME タイプ（例: "application/pdf", "image/png"）
    ///
    /// Content-Type ヘッダーやファイルの種類判定に使用。
    pub mime_type: String,

    /// ファイルサイズ（バイト）
    ///
    /// i64 を使用するのは PostgreSQL の BIGINT との互換性のため。
    pub size_bytes: i64,

    /// ストレージ内のパス（例: "/uploads/2025/01/abc123.pdf"）
    ///
    /// S3 等のオブジェクトストレージにおけるキー。
    /// データベースで UNIQUE 制約が設定されている。
    pub storage_path: String,

    /// 作成日時（UTC）
    ///
    /// ファイルがアップロードされた時刻。
    /// File は更新されない（イミュータブル）ため updated_at はない。
    pub created_at: DateTime<Utc>,
}

// =============================================================================
// File のメソッド実装
// =============================================================================

impl File {
    /// 新しい File を作成
    ///
    /// # Arguments
    /// * `todo_id` - 関連する TODO の ID
    /// * `filename` - 元のファイル名（バリデーション済み）
    /// * `mime_type` - MIME タイプ
    /// * `size_bytes` - ファイルサイズ（バリデーション済み）
    /// * `storage_path` - ストレージ内のパス
    ///
    /// # Returns
    /// 新しい File インスタンス
    ///
    /// # Note
    /// バリデーションは呼び出し側で事前に行うこと。
    /// このコンストラクタはバリデーション済みの値を受け取る想定。
    pub fn new(
        todo_id: Uuid,
        filename: String,
        mime_type: String,
        size_bytes: i64,
        storage_path: String,
    ) -> Self {
        Self {
            // 新しい UUID v4 を生成
            id: Uuid::new_v4(),

            // 関連する TODO の ID
            todo_id,

            // ファイル名
            filename,

            // MIME タイプ
            mime_type,

            // ファイルサイズ
            size_bytes,

            // ストレージパス
            storage_path,

            // 作成日時を現在時刻で設定
            created_at: Utc::now(),
        }
    }

    /// データベースからの復元用コンストラクタ
    ///
    /// バリデーションをスキップして、既存のデータからエンティティを再構築する。
    /// インフラ層からの呼び出し専用。
    ///
    /// # Arguments
    /// * `id` - UUID
    /// * `todo_id` - 関連する TODO の ID
    /// * `filename` - ファイル名
    /// * `mime_type` - MIME タイプ
    /// * `size_bytes` - ファイルサイズ
    /// * `storage_path` - ストレージパス
    /// * `created_at` - 作成日時
    pub fn from_raw(
        id: Uuid,
        todo_id: Uuid,
        filename: String,
        mime_type: String,
        size_bytes: i64,
        storage_path: String,
        created_at: DateTime<Utc>,
    ) -> Self {
        // 引数をそのまま構造体に設定
        Self {
            id,
            todo_id,
            filename,
            mime_type,
            size_bytes,
            storage_path,
            created_at,
        }
    }

    /// ファイル名のバリデーション
    ///
    /// セキュリティ上重要なバリデーション。
    /// パストラバーサル攻撃（../../../etc/passwd 等）を防ぐ。
    ///
    /// # Arguments
    /// * `filename` - 検証するファイル名
    ///
    /// # Returns
    /// * `Ok(String)` - トリム済みのファイル名
    /// * `Err(DomainError::Validation)` - ファイル名が無効な場合
    ///
    /// # Security Checks
    /// - 空文字でないこと
    /// - 最大長を超えないこと
    /// - パストラバーサル文字（..、/、\）を含まないこと
    pub fn validate_filename(filename: &str) -> Result<String, DomainError> {
        // 前後の空白を除去
        let trimmed = filename.trim();

        // 空文字チェック
        if trimmed.is_empty() {
            return Err(DomainError::Validation("filename cannot be empty".into()));
        }

        // 最大長チェック
        if trimmed.len() > MAX_FILENAME_LENGTH {
            return Err(DomainError::Validation(format!(
                "filename exceeds maximum length of {} characters",
                MAX_FILENAME_LENGTH
            )));
        }

        // パストラバーサル攻撃の防止
        // .. : 親ディレクトリへの移動
        // / : Unix 系のパス区切り
        // \ : Windows のパス区切り
        if trimmed.contains("..") || trimmed.contains('/') || trimmed.contains('\\') {
            return Err(DomainError::Validation(
                "filename contains invalid characters".into(),
            ));
        }

        // バリデーション済みのファイル名を返す
        Ok(trimmed.to_string())
    }

    /// ファイルサイズのバリデーション
    ///
    /// # Arguments
    /// * `size_bytes` - 検証するファイルサイズ（バイト）
    ///
    /// # Returns
    /// * `Ok(())` - サイズが有効な場合
    /// * `Err(DomainError::Validation)` - サイズが無効な場合
    ///
    /// # Rules
    /// - 負の値は不可
    /// - MAX_FILE_SIZE_BYTES（100MB）を超える場合は不可
    pub fn validate_size(size_bytes: i64) -> Result<(), DomainError> {
        // 負の値チェック
        if size_bytes < 0 {
            return Err(DomainError::Validation(
                "file size cannot be negative".into(),
            ));
        }

        // 最大サイズチェック
        if size_bytes > MAX_FILE_SIZE_BYTES {
            return Err(DomainError::Validation(format!(
                "file size exceeds maximum of {} bytes",
                MAX_FILE_SIZE_BYTES
            )));
        }

        // バリデーション成功
        Ok(())
    }

    /// MIME タイプのバリデーション
    ///
    /// # Arguments
    /// * `mime_type` - 検証する MIME タイプ
    ///
    /// # Returns
    /// * `Ok(String)` - 正規化された MIME タイプ（小文字）
    /// * `Err(DomainError::Validation)` - MIME タイプが無効な場合
    ///
    /// # Rules
    /// - 空でないこと
    /// - type/subtype 形式であること（/ を含む）
    ///
    /// # Example
    /// ```
    /// use domain::File;
    ///
    /// let mime = File::validate_mime_type("Application/PDF").unwrap();
    /// assert_eq!(mime, "application/pdf");
    /// ```
    pub fn validate_mime_type(mime_type: &str) -> Result<String, DomainError> {
        // 空白除去と小文字化
        let trimmed = mime_type.trim().to_lowercase();

        // 空文字チェック
        if trimmed.is_empty() {
            return Err(DomainError::Validation("mime_type cannot be empty".into()));
        }

        // 基本的な MIME タイプ形式チェック
        // 正しい形式: "type/subtype"（例: "application/pdf", "image/png"）
        if !trimmed.contains('/') {
            return Err(DomainError::Validation(
                "mime_type must be in format 'type/subtype'".into(),
            ));
        }

        // 正規化された MIME タイプを返す
        Ok(trimmed)
    }
}

// =============================================================================
// ユニットテスト
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// 新規ファイル作成のテスト
    #[test]
    fn test_new_file() {
        // テスト用の TODO ID
        let todo_id = Uuid::new_v4();

        // ファイルを新規作成
        let file = File::new(
            todo_id,
            "document.pdf".to_string(),                // ファイル名
            "application/pdf".to_string(),             // MIME タイプ
            12345,                                     // サイズ（バイト）
            "/uploads/2025/01/abc123.pdf".to_string(), // ストレージパス
        );

        // アサーション
        assert_eq!(file.todo_id, todo_id);
        assert_eq!(file.filename, "document.pdf");
        assert_eq!(file.mime_type, "application/pdf");
        assert_eq!(file.size_bytes, 12345);
        assert_eq!(file.storage_path, "/uploads/2025/01/abc123.pdf");
    }

    /// ファイル名バリデーション成功のテスト
    #[test]
    fn test_validate_filename_success() {
        // 前後に空白があるファイル名
        let result = File::validate_filename("  document.pdf  ");

        // トリムされていること
        assert_eq!(result.unwrap(), "document.pdf");
    }

    /// ファイル名バリデーション失敗のテスト（空文字）
    #[test]
    fn test_validate_filename_empty() {
        // 空白のみ
        let result = File::validate_filename("   ");

        // Validation エラーであること
        assert!(matches!(result, Err(DomainError::Validation(_))));
    }

    /// ファイル名バリデーション失敗のテスト（パストラバーサル）
    #[test]
    fn test_validate_filename_path_traversal() {
        // パストラバーサル攻撃を試みるファイル名
        let result = File::validate_filename("../../../etc/passwd");

        // Validation エラーであること
        assert!(matches!(result, Err(DomainError::Validation(_))));
    }

    /// ファイルサイズバリデーション成功のテスト
    #[test]
    fn test_validate_size_success() {
        // 有効なサイズ（1KB）
        let result = File::validate_size(1024);

        // 成功すること
        assert!(result.is_ok());
    }

    /// ファイルサイズバリデーション失敗のテスト（負の値）
    #[test]
    fn test_validate_size_negative() {
        // 負のサイズ
        let result = File::validate_size(-1);

        // Validation エラーであること
        assert!(matches!(result, Err(DomainError::Validation(_))));
    }

    /// ファイルサイズバリデーション失敗のテスト（最大値超過）
    #[test]
    fn test_validate_size_too_large() {
        // 最大値を超えるサイズ
        let result = File::validate_size(MAX_FILE_SIZE_BYTES + 1);

        // Validation エラーであること
        assert!(matches!(result, Err(DomainError::Validation(_))));
    }

    /// MIME タイプバリデーション成功のテスト
    #[test]
    fn test_validate_mime_type_success() {
        // 大文字・前後空白のある MIME タイプ
        let result = File::validate_mime_type("  Application/PDF  ");

        // 小文字・トリム済みになっていること
        assert_eq!(result.unwrap(), "application/pdf");
    }

    /// MIME タイプバリデーション失敗のテスト（無効な形式）
    #[test]
    fn test_validate_mime_type_invalid() {
        // / を含まない無効な MIME タイプ
        let result = File::validate_mime_type("invalid");

        // Validation エラーであること
        assert!(matches!(result, Err(DomainError::Validation(_))));
    }
}
