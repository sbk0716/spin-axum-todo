// =============================================================================
// application/src/dto/mod.rs: DTO モジュール
// =============================================================================
// DTO（Data Transfer Object）は、レイヤー間でデータを転送するための構造体。
//
// DTO を使う理由:
// 1. API の契約とドメインモデルを分離
//    - ドメインモデルの変更が API に直接影響しない
//    - API のバージョニングが容易
// 2. バリデーションの境界を明確化
//    - DTO はリクエストデータをそのまま表現
//    - バリデーションはユースケース層で実行
// 3. シリアライズ/デシリアライズの制御
//    - 各 DTO に必要な derive だけを追加
//    - 例: リクエストは Deserialize、レスポンスは Serialize
//
// DTO の種類:
// - Request DTO: HTTP リクエストボディからデシリアライズ
// - Response DTO: HTTP レスポンスボディにシリアライズ
// =============================================================================

// -----------------------------------------------------------------------------
// サブモジュールの宣言
// -----------------------------------------------------------------------------

/// 認証関連の DTO（登録、ログイン、トークン）
mod auth_dto;

/// バッチ操作関連の DTO（一括作成、TODO + ファイル）
mod batch_dto;

/// TODO 作成リクエスト DTO
mod create_todo_dto;

/// TODO 更新リクエスト DTO
mod update_todo_dto;

// -----------------------------------------------------------------------------
// 再エクスポート（Re-export）
// -----------------------------------------------------------------------------

/// 認証 DTO を公開
/// - LoginRequest: ログインリクエスト
/// - RegisterRequest: ユーザー登録リクエスト
/// - TokenResponse: JWT トークンレスポンス
/// - UserResponse: ユーザー情報レスポンス
pub use auth_dto::{LoginRequest, RegisterRequest, TokenResponse, UserResponse};

/// バッチ DTO を公開
/// - BatchCreateTodosRequest: TODO 一括作成リクエスト
/// - CreateTodoWithFilesRequest: TODO + ファイル作成リクエスト
/// - FileResponse: ファイル情報レスポンス
/// - FileUploadDto: ファイルアップロード情報
/// - TodoWithFilesResponse: TODO + ファイルレスポンス
pub use batch_dto::{
    BatchCreateTodosRequest, CreateTodoWithFilesRequest, FileResponse, FileUploadDto,
    TodoWithFilesResponse,
};

/// TODO 作成 DTO を公開
pub use create_todo_dto::CreateTodoDto;

/// TODO 更新 DTO を公開
pub use update_todo_dto::UpdateTodoDto;
