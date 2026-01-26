// =============================================================================
// domain/src/entities/todo.rs: Todo エンティティ
// =============================================================================
// ビジネスドメインの中心となるデータ構造。
// バリデーションロジックもここに含める（ドメイン駆動設計の原則）。
//
// このエンティティが持つ責務:
// - TODO のデータ構造を定義
// - ビジネスルール（タイトルのバリデーション等）をカプセル化
// - データベースとは独立した純粋なドメインモデル
// =============================================================================

// -----------------------------------------------------------------------------
// 外部クレートのインポート
// -----------------------------------------------------------------------------
// use キーワードで外部クレートの型を現在のスコープに取り込む。
// これにより、完全修飾名（chrono::DateTime）を使わずに DateTime と書ける。
// -----------------------------------------------------------------------------

// chrono クレート: 日時を扱うための Rust の標準的なライブラリ
// DateTime<Utc>: UTC タイムゾーンを持つ日時型
// Utc: 協定世界時（Coordinated Universal Time）を表す型
use chrono::{DateTime, Utc};

// serde クレート: シリアライズ/デシリアライズのための derive マクロ
// Serialize: Rust の構造体を JSON 等に変換可能にする
// Deserialize: JSON 等から Rust の構造体に変換可能にする
use serde::{Deserialize, Serialize};

// uuid クレート: UUID（Universally Unique Identifier）を扱う
// UUID v4 はランダムに生成される一意識別子
use uuid::Uuid;

// 同じクレート内の errors モジュールから DomainError をインポート
// crate:: は現在のクレートのルートを指す（domain クレート）
use crate::errors::DomainError;

// =============================================================================
// Todo 構造体の定義
// =============================================================================

/// TODO エンティティ
///
/// クリーンアーキテクチャでは、エンティティはビジネスルールをカプセル化する。
/// 外部フレームワーク（axum、sqlx）に依存しない純粋な Rust 構造体。
///
/// # データベーステーブルとの対応
///
/// | フィールド | カラム | 型 |
/// |-----------|--------|-----|
/// | id | id | UUID (PRIMARY KEY) |
/// | user_id | user_id | UUID (FOREIGN KEY → users.id) |
/// | title | title | VARCHAR(255) NOT NULL |
/// | description | description | TEXT |
/// | completed | completed | BOOLEAN DEFAULT false |
/// | created_at | created_at | TIMESTAMPTZ |
/// | updated_at | updated_at | TIMESTAMPTZ |
// -----------------------------------------------------------------------------
// #[derive(...)] 属性マクロについて:
// Rust の derive マクロは、トレイトの実装を自動生成する。
// - Debug: {:?} フォーマットでデバッグ出力可能にする
// - Clone: .clone() メソッドで値のコピーを作成可能にする
// - PartialEq: == 演算子で比較可能にする
// - Serialize: serde で JSON 等にシリアライズ可能にする
// - Deserialize: serde で JSON 等からデシリアライズ可能にする
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Todo {
    /// 一意識別子（UUID v4）
    ///
    /// UUID v4 はランダムに生成される 128 ビットの識別子。
    /// 例: "550e8400-e29b-41d4-a716-446655440000"
    pub id: Uuid,

    /// 所有者のユーザー ID（users.id への外部キー）
    ///
    /// マルチテナント対応のため、全ての TODO はユーザーに紐づく。
    /// SQL クエリでは必ず user_id でフィルタリングする。
    pub user_id: Uuid,

    /// タイトル（必須、空文字不可）
    ///
    /// TODO の主要な識別名。一覧画面等で表示される。
    /// バリデーション: 空でないこと、トリム後に値があること
    pub title: String,

    /// 詳細説明（任意）
    ///
    /// Option<String> は Rust の nullable 表現。
    /// Some("説明") または None を取る。
    pub description: Option<String>,

    /// 完了状態
    ///
    /// true: 完了、false: 未完了
    /// デフォルトは false で新規作成される。
    pub completed: bool,

    /// 作成日時（UTC）
    ///
    /// TODO が作成された時刻。変更されない。
    pub created_at: DateTime<Utc>,

    /// 最終更新日時（UTC）
    ///
    /// TODO が最後に更新された時刻。
    /// update() メソッド呼び出し時に自動更新される。
    pub updated_at: DateTime<Utc>,
}

// =============================================================================
// Todo のメソッド実装
// =============================================================================
// impl ブロックで構造体にメソッドを追加する。
// Rust では関連関数（Self を返すコンストラクタ等）とメソッドを定義できる。
// =============================================================================

impl Todo {
    /// 新しい Todo を作成
    ///
    /// # Arguments
    /// * `user_id` - 所有者のユーザー ID（UUID）
    /// * `title` - タイトル（バリデーション済み）
    /// * `description` - 詳細説明（任意）
    ///
    /// # Returns
    /// 新しい Todo インスタンス（completed = false、日時は現在時刻）
    ///
    /// # Example
    /// ```
    /// use uuid::Uuid;
    /// use domain::Todo;
    ///
    /// let user_id = Uuid::new_v4();
    /// let todo = Todo::new(user_id, "買い物に行く".to_string(), None);
    /// assert!(!todo.completed); // 新規作成時は未完了
    /// ```
    pub fn new(user_id: Uuid, title: String, description: Option<String>) -> Self {
        // 現在時刻を UTC で取得
        // Utc::now() は chrono クレートが提供する関数
        let now = Utc::now();

        // Self は impl ブロック内で Todo 型を指す
        // 構造体のインスタンスを作成して返す
        Self {
            // UUID v4 をランダム生成
            // 衝突確率は実質的にゼロ（2^122 通り）
            id: Uuid::new_v4(),

            // 所有者の ID を設定
            user_id,

            // タイトルを設定（所有権を移動）
            title,

            // 説明を設定（Option<String> をそのまま）
            description,

            // 新規作成時は未完了
            completed: false,

            // 作成日時と更新日時を現在時刻で初期化
            created_at: now,
            updated_at: now,
        }
    }

    /// データベースからの復元用コンストラクタ
    ///
    /// バリデーションをスキップして、既存のデータからエンティティを再構築する。
    /// インフラ層からの呼び出し専用。
    ///
    /// # Arguments
    /// * `id` - UUID
    /// * `user_id` - 所有者のユーザー ID（UUID）
    /// * `title` - タイトル
    /// * `description` - 詳細説明
    /// * `completed` - 完了状態
    /// * `created_at` - 作成日時
    /// * `updated_at` - 最終更新日時
    ///
    /// # Note
    /// このメソッドは既にバリデーション済みのデータを復元するため、
    /// 新規データには new() を使用すること。
    pub fn from_raw(
        id: Uuid,
        user_id: Uuid,
        title: String,
        description: Option<String>,
        completed: bool,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        // 引数をそのまま構造体に設定
        // バリデーションは行わない（DB から取得したデータは信頼する）
        Self {
            id,
            user_id,
            title,
            description,
            completed,
            created_at,
            updated_at,
        }
    }

    /// タイトルのバリデーション
    ///
    /// # Arguments
    /// * `title` - 検証するタイトル
    ///
    /// # Returns
    /// * `Ok(String)` - トリム済みのタイトル
    /// * `Err(DomainError::Validation)` - タイトルが空の場合
    ///
    /// # Example
    /// ```
    /// use domain::Todo;
    ///
    /// // 前後の空白はトリムされる
    /// let title = Todo::validate_title("  買い物  ").unwrap();
    /// assert_eq!(title, "買い物");
    ///
    /// // 空文字はエラー
    /// let result = Todo::validate_title("   ");
    /// assert!(result.is_err());
    /// ```
    pub fn validate_title(title: &str) -> Result<String, DomainError> {
        // trim() で前後の空白を除去
        // &str から &str を返す（所有権は移動しない）
        let trimmed = title.trim();

        // トリム後に空文字かチェック
        if trimmed.is_empty() {
            // Err を返してエラーを伝播
            // into() で &str から String に変換
            return Err(DomainError::Validation("title cannot be empty".into()));
        }

        // to_string() で &str から String（所有権のある文字列）に変換
        Ok(trimmed.to_string())
    }

    /// Todo を更新
    ///
    /// 部分更新をサポート: None のフィールドは既存の値を維持。
    /// updated_at は自動的に現在時刻に更新される。
    ///
    /// # Arguments
    /// * `title` - 新しいタイトル（None なら変更なし）
    /// * `description` - 新しい説明（None なら変更なし）
    ///   * `Some(Some("説明"))`: 説明を設定
    ///   * `Some(None)`: 説明を削除（NULL に設定）
    ///   * `None`: 変更なし
    /// * `completed` - 新しい完了状態（None なら変更なし）
    ///
    /// # Note
    /// description は Option<Option<String>> という二重の Option になっている。
    /// これにより「変更なし」と「NULL に設定」を区別できる。
    pub fn update(
        &mut self, // &mut self: 自身を可変参照として受け取る（値を変更するため）
        title: Option<String>,
        description: Option<Option<String>>,
        completed: Option<bool>,
    ) {
        // if let Some(t) = title: Option が Some の場合のみ実行
        // パターンマッチングで値を取り出す
        if let Some(t) = title {
            // タイトルを更新
            self.title = t;
        }

        // 説明の更新（二重 Option の処理）
        if let Some(d) = description {
            // d は Option<String>
            // Some("説明") または None が設定される
            self.description = d;
        }

        // 完了状態の更新
        if let Some(c) = completed {
            self.completed = c;
        }

        // 更新日時を現在時刻に設定
        // 何かしらのフィールドが更新されたら updated_at も更新
        self.updated_at = Utc::now();
    }
}

// =============================================================================
// ユニットテスト
// =============================================================================
// #[cfg(test)] 属性により、cargo test 実行時のみコンパイルされる。
// mod tests で専用のテストモジュールを作成。
// =============================================================================

#[cfg(test)]
mod tests {
    // super::* で親モジュール（このファイル）の全ての公開アイテムをインポート
    use super::*;

    /// 新規 Todo 作成のテスト
    #[test] // #[test] 属性でテスト関数としてマーク
    fn test_new_todo() {
        // テスト用の UUID を生成
        let user_id = Uuid::new_v4();

        // Todo を新規作成
        let todo = Todo::new(
            user_id,
            "テスト".to_string(),     // タイトル
            Some("説明".to_string()), // 説明あり
        );

        // アサーション: 期待値と実際の値を比較
        assert_eq!(todo.user_id, user_id); // ユーザー ID が一致
        assert_eq!(todo.title, "テスト"); // タイトルが一致
        assert_eq!(todo.description, Some("説明".to_string())); // 説明が一致
        assert!(!todo.completed); // 未完了であること
    }

    /// タイトルバリデーション成功のテスト
    #[test]
    fn test_validate_title_success() {
        // 前後に空白があるタイトル
        let result = Todo::validate_title("  有効なタイトル  ");

        // unwrap() で Ok の値を取り出す（テストでは失敗時にパニックで良い）
        assert_eq!(result.unwrap(), "有効なタイトル"); // トリムされていること
    }

    /// タイトルバリデーション失敗のテスト（空文字）
    #[test]
    fn test_validate_title_empty() {
        // 空白のみのタイトル
        let result = Todo::validate_title("   ");

        // matches! マクロでパターンマッチング
        // Err(DomainError::Validation(_)) にマッチすることを確認
        assert!(matches!(result, Err(DomainError::Validation(_))));
    }

    /// 部分更新のテスト
    #[test]
    fn test_update_partial() {
        let user_id = Uuid::new_v4();

        // mut: 可変変数として宣言（後で update() で変更するため）
        let mut todo = Todo::new(user_id, "元のタイトル".to_string(), None);

        // 作成時の created_at を保存
        let original_created_at = todo.created_at;

        // 部分更新: タイトルと完了状態のみ変更、説明は変更なし（None）
        todo.update(Some("新しいタイトル".to_string()), None, Some(true));

        // タイトルが更新されていること
        assert_eq!(todo.title, "新しいタイトル");

        // 完了状態が更新されていること
        assert!(todo.completed);

        // created_at は変わらないこと（作成日時は不変）
        assert_eq!(todo.created_at, original_created_at);

        // updated_at は更新されていること
        assert!(todo.updated_at > original_created_at);
    }
}
