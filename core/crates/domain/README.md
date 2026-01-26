# Domain 層

クリーンアーキテクチャの最内層。ビジネスロジックの核心部分を担当。

## 概要

Domain 層は外部依存を持たない純粋なビジネスロジック層です。
エンティティ、リポジトリトレイト、ドメインエラーを定義します。

## 責務

- **エンティティ定義**: Todo, User, File のビジネスルール
- **リポジトリトレイト**: 永続化の抽象化（具象実装は Infrastructure 層）
- **ドメインエラー**: ビジネスルール違反を表現

## ディレクトリ構成

```
src/
├── lib.rs              # モジュールエクスポート
├── entities/
│   ├── mod.rs
│   ├── todo.rs         # Todo エンティティ
│   ├── user.rs         # User エンティティ
│   └── file.rs         # File エンティティ
├── repositories/
│   ├── mod.rs
│   ├── todo_repository.rs  # TodoReader / TodoWriter トレイト
│   ├── user_repository.rs  # UserReader / UserWriter トレイト
│   ├── file_repository.rs  # FileReader / FileWriter トレイト
│   └── todo_cache.rs       # TodoCacheOps トレイト
└── errors/
    ├── mod.rs
    └── domain_error.rs     # DomainError 列挙型
```

## エンティティ

### Todo

TODO アイテムを表現するエンティティ。

```rust
pub struct Todo {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

**バリデーションルール**:
- タイトル: 1-200文字、前後の空白はトリム

### User

認証用ユーザーエンティティ。

```rust
pub struct User {
    pub id: Uuid,
    pub email: String,
    #[serde(skip_serializing)]  // レスポンスに含めない
    pub password_hash: String,
    pub display_name: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

**バリデーションルール**:
- メールアドレス: `@` を含むこと
- パスワード: 8文字以上

### File

TODO に添付されるファイルのメタデータ。

```rust
pub struct File {
    pub id: Uuid,
    pub todo_id: Uuid,
    pub filename: String,
    pub mime_type: String,
    pub size_bytes: i64,
    pub storage_path: String,
    pub created_at: DateTime<Utc>,
}
```

**バリデーションルール**:
- ファイル名: 1-255文字、危険な文字を含まない
- MIME タイプ: `type/subtype` 形式
- サイズ: 0より大きい

## リポジトリトレイト（CQRS パターン）

### TodoReader / TodoWriter

```rust
#[async_trait]
pub trait TodoReader: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Todo>, DomainError>;
    async fn find_all(&self, filter: TodoFilter) -> Result<Vec<Todo>, DomainError>;
}

#[async_trait]
pub trait TodoWriter: Send + Sync {
    async fn create(&self, todo: &Todo) -> Result<Todo, DomainError>;
    async fn update(&self, todo: &Todo) -> Result<Todo, DomainError>;
    async fn delete(&self, id: Uuid) -> Result<bool, DomainError>;
}
```

### UserReader / UserWriter

```rust
#[async_trait]
pub trait UserReader: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, DomainError>;
}

#[async_trait]
pub trait UserWriter: Send + Sync {
    async fn create(&self, user: &User) -> Result<User, DomainError>;
    async fn update(&self, user: &User) -> Result<User, DomainError>;
    async fn delete(&self, id: Uuid) -> Result<bool, DomainError>;
}
```

### TodoCacheOps

```rust
#[async_trait]
pub trait TodoCacheOps: Send + Sync {
    async fn get(&self, id: Uuid) -> Result<Option<Todo>, DomainError>;
    async fn set(&self, todo: &Todo) -> Result<(), DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
}
```

## ドメインエラー

```rust
pub enum DomainError {
    Validation(String),      // バリデーション違反
    Authentication(String),  // 認証エラー
    NotFound,                // リソースが見つからない
    Duplicate(String),       // 重複エラー
    Repository(String),      // DB エラー
    Cache(String),           // キャッシュエラー
}
```

## 設計原則

### 依存性逆転の原則（DIP）

- Domain 層はリポジトリの**インターフェース（トレイト）のみ**を定義
- 具象実装（PostgreSQL、Redis）は Infrastructure 層が提供
- これにより Domain 層は外部依存から完全に独立

### 純粋なビジネスロジック

- フレームワーク依存なし（axum, sqlx を知らない）
- テストが容易（モック不要で単体テスト可能）
- 他のプロジェクトへの移植が容易

## 依存クレート

```toml
[dependencies]
async-trait = "0.1"    # async fn in traits
chrono = "0.4"         # 日時処理
serde = "1.0"          # シリアライズ
thiserror = "2.0"      # エラー型定義
uuid = "1.11"          # UUID 生成
```

## 使用例

```rust
use domain::{Todo, TodoReader, DomainError};

// エンティティの作成
let todo = Todo::new(user_id, "Buy milk".to_string(), None);

// バリデーション
let title = Todo::validate_title("  Buy milk  ")?;  // "Buy milk" を返す

// リポジトリは Infrastructure 層の実装を使用
// let todo = reader.find_by_id(id).await?;
```
