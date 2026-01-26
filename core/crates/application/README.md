# Application 層

クリーンアーキテクチャのユースケース層。ビジネスロジックのオーケストレーションを担当。

## 概要

Application 層は Domain 層のエンティティとリポジトリを組み合わせて、
具体的なユースケースを実現します。CQRS パターンに基づき Commands と Queries に分離。

## 責務

- **Commands**: 状態変更操作（作成、更新、削除）
- **Queries**: 参照操作（取得、一覧）
- **Services**: 認証などの横断的サービス
- **DTOs**: レイヤー間のデータ転送

## ディレクトリ構成

```
src/
├── lib.rs              # モジュールエクスポート
├── commands/
│   ├── mod.rs
│   ├── create_todo.rs  # TODO 作成コマンド
│   ├── update_todo.rs  # TODO 更新コマンド
│   └── delete_todo.rs  # TODO 削除コマンド
├── queries/
│   ├── mod.rs
│   ├── get_todo.rs     # TODO 取得クエリ
│   └── list_todos.rs   # TODO 一覧クエリ
├── services/
│   ├── mod.rs
│   └── auth_service.rs # 認証サービス
└── dto/
    ├── mod.rs
    ├── create_todo_dto.rs
    ├── update_todo_dto.rs
    ├── batch_dto.rs
    └── auth_dto.rs
```

## Commands（状態変更操作）

### CreateTodoCommand

```rust
pub struct CreateTodoCommand<W: TodoWriter, C: TodoCacheOps> {
    writer: Arc<W>,
    cache: Option<Arc<C>>,  // Write-Through キャッシュ
}

impl<W: TodoWriter, C: TodoCacheOps> CreateTodoCommand<W, C> {
    pub async fn execute(&self, user_id: Uuid, dto: CreateTodoDto) -> Result<Todo, DomainError> {
        // 1. バリデーション
        let title = Todo::validate_title(&dto.title)?;

        // 2. エンティティ作成
        let todo = Todo::new(user_id, title, dto.description);

        // 3. 永続化
        let created = self.writer.create(&todo).await?;

        // 4. キャッシュ保存（Write-Through）
        if let Some(cache) = &self.cache {
            cache.set(&created).await.ok();
        }

        Ok(created)
    }
}
```

### UpdateTodoCommand

```rust
pub async fn execute(&self, id: Uuid, user_id: Uuid, dto: UpdateTodoDto) -> Result<Todo, DomainError> {
    // 1. 既存 TODO を取得
    let mut todo = self.writer.find_by_id(id).await?
        .ok_or(DomainError::NotFound)?;

    // 2. 所有者チェック
    if todo.user_id != user_id {
        return Err(DomainError::NotFound);
    }

    // 3. 更新
    todo.update(dto.title, dto.description, dto.completed);

    // 4. 永続化
    let updated = self.writer.update(&todo).await?;

    // 5. キャッシュ無効化
    if let Some(cache) = &self.cache {
        cache.delete(id).await.ok();
    }

    Ok(updated)
}
```

### DeleteTodoCommand

```rust
pub async fn execute(&self, id: Uuid, user_id: Uuid) -> Result<(), DomainError> {
    // 1. 所有者チェック（find_by_id で確認後削除）
    // 2. 削除
    // 3. キャッシュ無効化
}
```

## Queries（参照操作）

### GetTodoQuery

```rust
pub struct GetTodoQuery<R: TodoReader> {
    reader: Arc<R>,
}

impl<R: TodoReader> GetTodoQuery<R> {
    pub async fn execute(&self, id: Uuid, user_id: Uuid) -> Result<Todo, DomainError> {
        let todo = self.reader.find_by_id(id).await?
            .ok_or(DomainError::NotFound)?;

        // 所有者チェック
        if todo.user_id != user_id {
            return Err(DomainError::NotFound);
        }

        Ok(todo)
    }
}
```

### ListTodosQuery

```rust
pub async fn execute(&self, filter: TodoFilter) -> Result<Vec<Todo>, DomainError> {
    self.reader.find_all(filter).await
}
```

## Services

### AuthService

```rust
pub struct AuthService<R: UserReader, W: UserWriter> {
    reader: Arc<R>,
    writer: Arc<W>,
    jwt_secret: String,
    jwt_expiry_hours: i64,
}

impl<R: UserReader, W: UserWriter> AuthService<R, W> {
    /// ユーザー登録
    pub async fn register(&self, email: &str, password: &str, display_name: Option<String>) -> Result<User, DomainError> {
        // 1. バリデーション
        User::validate_email(email)?;
        User::validate_password(password)?;

        // 2. パスワードハッシュ化（bcrypt）
        let hash = bcrypt::hash(password, bcrypt::DEFAULT_COST)?;

        // 3. ユーザー作成
        let user = User::new(email.to_string(), hash, display_name);
        self.writer.create(&user).await
    }

    /// ログイン
    pub async fn login(&self, email: &str, password: &str) -> Result<String, DomainError> {
        // 1. ユーザー検索
        let user = self.reader.find_by_email(email).await?
            .ok_or(DomainError::Authentication("Invalid credentials".into()))?;

        // 2. パスワード検証
        if !bcrypt::verify(password, &user.password_hash)? {
            return Err(DomainError::Authentication("Invalid credentials".into()));
        }

        // 3. JWT 生成
        self.generate_jwt(user.id)
    }
}
```

## DTOs

### CreateTodoDto

```rust
#[derive(Debug, Deserialize)]
pub struct CreateTodoDto {
    pub title: String,
    pub description: Option<String>,
}
```

### UpdateTodoDto

```rust
#[derive(Debug, Deserialize)]
pub struct UpdateTodoDto {
    pub title: Option<String>,
    pub description: Option<String>,
    pub completed: Option<bool>,
}
```

### 認証 DTO

```rust
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub token: String,
}
```

## キャッシュ戦略

| 操作 | Commands | キャッシュ動作 |
|------|----------|---------------|
| 作成 | CreateTodoCommand | Write-Through（保存） |
| 更新 | UpdateTodoCommand | Cache Invalidation（削除） |
| 削除 | DeleteTodoCommand | Cache Invalidation（削除） |

## 設計原則

### CQRS（Command Query Responsibility Segregation）

- **Commands**: TodoWriter + TodoCacheOps を使用
- **Queries**: TodoReader を使用
- 読み取りと書き込みの責務を分離

### 依存性注入

```rust
// ジェネリクスでリポジトリを注入
pub struct CreateTodoCommand<W: TodoWriter, C: TodoCacheOps> {
    writer: Arc<W>,
    cache: Option<Arc<C>>,
}
```

## 依存クレート

```toml
[dependencies]
domain = { path = "../domain" }
async-trait = "0.1"
bcrypt = "0.17"         # パスワードハッシュ
chrono = "0.4"
jsonwebtoken = "9.3"    # JWT
serde = "1.0"
tracing = "0.1"
uuid = "1.11"
```

## 使用例

```rust
use application::{CreateTodoCommand, CreateTodoDto};

// DI で注入されたリポジトリを使用
let command = CreateTodoCommand::new(writer, Some(cache));

let dto = CreateTodoDto {
    title: "Buy milk".to_string(),
    description: None,
};

let todo = command.execute(user_id, dto).await?;
```
