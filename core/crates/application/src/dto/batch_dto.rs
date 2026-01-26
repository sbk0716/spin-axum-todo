// =============================================================================
// application/src/dto/batch_dto.rs: バッチ操作用 DTO
// =============================================================================
// バッチ作成や TODO + ファイル同時作成用のリクエスト/レスポンス DTO。
//
// バッチ操作のメリット:
// - 複数の TODO を 1 リクエストで作成（ネットワーク往復を削減）
// - TODO とファイルをトランザクション内で同時作成（整合性保証）
// =============================================================================

// -----------------------------------------------------------------------------
// 外部クレートのインポート
// -----------------------------------------------------------------------------

// chrono: 日時型
use chrono::{DateTime, Utc};

// serde: シリアライズ/デシリアライズ
// Deserialize: リクエスト DTO 用
// Serialize: レスポンス DTO 用
use serde::{Deserialize, Serialize};

// uuid: 一意識別子
use uuid::Uuid;

// -----------------------------------------------------------------------------
// 同一モジュール内のインポート
// -----------------------------------------------------------------------------

// CreateTodoDto: 単一 TODO 作成用 DTO を再利用
use super::CreateTodoDto;

// =============================================================================
// バッチ TODO 作成リクエスト
// =============================================================================

/// バッチ TODO 作成リクエスト
///
/// 複数の TODO を 1 リクエストで作成する。
///
/// # 例
///
/// ```json
/// {
///   "todos": [
///     { "title": "Task 1" },
///     { "title": "Task 2", "description": "Details" }
///   ]
/// }
/// ```
#[derive(Debug, Clone, Deserialize)]
pub struct BatchCreateTodosRequest {
    /// 作成する TODO のリスト
    /// 既存の CreateTodoDto を再利用
    pub todos: Vec<CreateTodoDto>,
}

// =============================================================================
// ファイルアップロード情報
// =============================================================================

/// ファイルアップロード情報
///
/// クライアントが事前にファイルをストレージにアップロードし、
/// そのメタデータをこの DTO で送信する。
///
/// # ファイルアップロードフロー
///
/// 1. クライアント: ファイルを S3/GCS などにアップロード
/// 2. クライアント: アップロード結果（パス等）を含む TODO 作成リクエスト送信
/// 3. サーバー: メタデータを files テーブルに保存
#[derive(Debug, Clone, Deserialize)]
pub struct FileUploadDto {
    /// 元のファイル名（ユーザーがアップロードした時の名前）
    pub filename: String,

    /// MIME タイプ（例: "application/pdf", "image/png"）
    /// Content-Type ヘッダーと同等の情報
    pub mime_type: String,

    /// ファイルサイズ（バイト）
    /// i64 を使用（PostgreSQL の BIGINT に対応）
    pub size_bytes: i64,

    /// ストレージ内のパス（アップロード済みファイルへの参照）
    /// 例: "uploads/2024/01/abc123.pdf"
    pub storage_path: String,
}

// =============================================================================
// TODO + ファイル同時作成リクエスト
// =============================================================================

/// TODO + ファイル同時作成リクエスト
///
/// TODO とその添付ファイルをトランザクション内で同時に作成する。
/// 一方が失敗した場合、両方ロールバックされる。
///
/// # 例
///
/// ```json
/// {
///   "title": "Submit report",
///   "description": "Q4 financial report",
///   "files": [
///     {
///       "filename": "report.pdf",
///       "mime_type": "application/pdf",
///       "size_bytes": 1024000,
///       "storage_path": "uploads/report.pdf"
///     }
///   ]
/// }
/// ```
#[derive(Debug, Clone, Deserialize)]
pub struct CreateTodoWithFilesRequest {
    /// TODO タイトル
    pub title: String,

    /// TODO 説明（任意）
    pub description: Option<String>,

    /// 添付ファイル情報のリスト
    /// 空のリストも可（ファイルなしの TODO 作成）
    pub files: Vec<FileUploadDto>,
}

// =============================================================================
// ファイルレスポンス
// =============================================================================

/// ファイルレスポンス
///
/// ファイルメタデータを API レスポンスとして返す。
///
/// # From トレイト
///
/// domain::File から自動変換可能（`From<domain::File>` 実装）。
#[derive(Debug, Clone, Serialize)]
pub struct FileResponse {
    /// ファイル ID（UUID）
    pub id: Uuid,

    /// 関連する TODO の ID
    pub todo_id: Uuid,

    /// 元のファイル名
    pub filename: String,

    /// MIME タイプ
    pub mime_type: String,

    /// ファイルサイズ（バイト）
    pub size_bytes: i64,

    /// ストレージ内のパス
    pub storage_path: String,

    /// 作成日時（UTC）
    pub created_at: DateTime<Utc>,
}

// -----------------------------------------------------------------------------
// From トレイト実装
// -----------------------------------------------------------------------------

/// domain::File → FileResponse への変換
///
/// From トレイトを実装することで、`.into()` で変換可能になる。
///
/// # 使用例
///
/// ```rust,ignore
/// let file: domain::File = ...;
/// let response: FileResponse = file.into();
/// ```
impl From<domain::File> for FileResponse {
    fn from(file: domain::File) -> Self {
        Self {
            id: file.id,
            todo_id: file.todo_id,
            filename: file.filename,
            mime_type: file.mime_type,
            size_bytes: file.size_bytes,
            storage_path: file.storage_path,
            created_at: file.created_at,
        }
    }
}

// =============================================================================
// TODO + ファイル作成レスポンス
// =============================================================================

/// TODO + ファイル作成レスポンス
///
/// トランザクションで作成された TODO とファイルを返す。
#[derive(Debug, Clone, Serialize)]
pub struct TodoWithFilesResponse {
    /// 作成された TODO
    /// domain::Todo は Serialize を derive しているので直接使用可能
    pub todo: domain::Todo,

    /// 作成されたファイルのリスト
    pub files: Vec<FileResponse>,
}
