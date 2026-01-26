// =============================================================================
// domain/src/repositories/file_repository.rs: ファイルリポジトリトレイト
// =============================================================================
// ファイルメタデータの永続化の抽象インターフェース。
//
// ファイル管理の設計:
// - メタデータ（filename, mime_type, size 等）はデータベースに保存
// - 実際のファイルコンテンツはストレージ（S3 等）に保存
// - storage_path フィールドでストレージ上のファイルを参照
//
// 統一 CQRS パターン:
// - FileReader: 参照操作（Queries）- メタデータの取得
// - FileWriter: 状態変更操作（Commands）- メタデータの作成・削除
//
// CASCADE 削除:
// - TODO 削除時、関連する files も自動削除（データベース制約）
// - ストレージ上の実ファイルは別途削除が必要
// =============================================================================

// -----------------------------------------------------------------------------
// 外部クレートのインポート
// -----------------------------------------------------------------------------

// async_trait: トレイト内で async fn を使用可能にする
use async_trait::async_trait;

// uuid: 一意識別子
use uuid::Uuid;

// 同じクレート内のエンティティとエラー型
use crate::entities::File;
use crate::errors::DomainError;

// =============================================================================
// CQRS: Reader / Writer トレイト分離
// =============================================================================

/// ファイル読み取りトレイト（Queries 用）
///
/// ファイルメタデータの取得を担当。
/// Reader DB プールと組み合わせることでレプリケーション対応可能。
///
/// # アクセス制御
/// ファイルは TODO に紐付くため、アクセス制御は親の TODO を通じて行う。
/// つまり、ファイルへのアクセス権 = 親 TODO へのアクセス権。
///
/// # 実装例
/// - `PostgresFileReader`: PostgreSQL 実装（infrastructure 層）
#[async_trait]
pub trait FileReader: Send + Sync {
    /// ID でファイルを取得
    ///
    /// # Arguments
    /// * `id` - 取得する File の UUID
    ///
    /// # Returns
    /// * `Ok(Some(File))` - ファイルが見つかった場合
    /// * `Ok(None)` - ファイルが見つからない場合
    /// * `Err(DomainError::Repository)` - データベースエラー
    ///
    /// # Note
    /// このメソッドだけではアクセス制御ができない。
    /// 呼び出し側で親 TODO の所有者を確認すること。
    async fn find_by_id(&self, id: Uuid) -> Result<Option<File>, DomainError>;

    /// TODO ID でファイル一覧を取得
    ///
    /// 特定の TODO に紐付く全ファイルを取得する。
    ///
    /// # Arguments
    /// * `todo_id` - ファイルが紐付く TODO の UUID
    ///
    /// # Returns
    /// * `Ok(Vec<File>)` - ファイルのリスト（作成日時の昇順）
    /// * `Err(DomainError::Repository)` - データベースエラー
    ///
    /// # Note
    /// 0件の場合は空の Vec を返す（エラーではない）。
    async fn find_by_todo_id(&self, todo_id: Uuid) -> Result<Vec<File>, DomainError>;
}

/// ファイル書き込みトレイト（Commands 用）
///
/// ファイルメタデータの作成・削除を担当。
/// Writer DB プールと組み合わせることでレプリケーション対応可能。
///
/// # ストレージとの連携
/// このトレイトはメタデータ（データベース）のみを扱う。
/// ストレージ（S3 等）上の実ファイルの操作は呼び出し側の責任。
///
/// # 実装例
/// - `PostgresFileWriter`: PostgreSQL 実装（infrastructure 層）
#[async_trait]
pub trait FileWriter: Send + Sync {
    /// ファイルを作成
    ///
    /// # Arguments
    /// * `file` - 作成する File エンティティ
    ///   - filename: バリデーション済み（パストラバーサル防止）
    ///   - mime_type: バリデーション済み
    ///   - size_bytes: バリデーション済み（最大サイズチェック）
    ///   - storage_path: ストレージ上のパス
    ///
    /// # Returns
    /// * `Ok(File)` - 作成された File（DB から返された値を反映）
    /// * `Err(DomainError::Repository)` - データベースエラー
    ///
    /// # Note
    /// このメソッドを呼ぶ前に、ストレージにファイルをアップロードしておくこと。
    async fn create(&self, file: &File) -> Result<File, DomainError>;

    /// ファイルを削除
    ///
    /// # Arguments
    /// * `id` - 削除する File の UUID
    ///
    /// # Returns
    /// * `Ok(true)` - 削除成功
    /// * `Ok(false)` - 該当する File が存在しない
    /// * `Err(DomainError::Repository)` - データベースエラー
    ///
    /// # Important
    /// このメソッドはメタデータのみを削除する。
    /// ストレージ上の実ファイルは呼び出し側で別途削除すること。
    ///
    /// # Example
    /// ```rust,ignore
    /// // 削除の流れ（疑似コード）
    /// // 1. メタデータを取得（storage_path を取得するため）
    /// let file = reader.find_by_id(id).await?;
    ///
    /// // 2. メタデータを削除
    /// writer.delete(id).await?;
    ///
    /// // 3. ストレージからファイルを削除
    /// storage.delete(&file.storage_path).await?;
    /// ```
    async fn delete(&self, id: Uuid) -> Result<bool, DomainError>;

    /// TODO に紐付く全ファイルを削除
    ///
    /// # Arguments
    /// * `todo_id` - ファイルが紐付く TODO の UUID
    ///
    /// # Returns
    /// * `Ok(u64)` - 削除されたファイル数
    /// * `Err(DomainError::Repository)` - データベースエラー
    ///
    /// # Note
    /// 通常は CASCADE DELETE で自動削除されるため、
    /// 明示的に呼び出す必要はない。
    /// TODO 削除前にストレージパスを取得する必要がある場合に使用。
    async fn delete_by_todo_id(&self, todo_id: Uuid) -> Result<u64, DomainError>;
}
