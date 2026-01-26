# Infrastructure 層

クリーンアーキテクチャの外部システム連携層。永続化とキャッシュの具象実装を提供。

## 概要

Infrastructure 層は Domain 層で定義されたリポジトリトレイトの具象実装を提供します。
PostgreSQL によるデータ永続化、Redis によるキャッシュ、S3/LocalStack によるファイルストレージを担当。

## 責務

- **PostgreSQL 実装**: TodoReader/Writer, UserReader/Writer, FileReader/Writer
- **Redis 実装**: TodoCacheOps
- **S3 実装**: S3StorageService（ファイルストレージ）
- **デコレータ**: CachedTodoReader（Cache-Aside パターン）
- **トランザクション**: TransactionalTodoService（バッチ操作）
- **DB 接続管理**: DbPools（Reader/Writer 分離）

## ディレクトリ構成

```
src/
├── lib.rs                  # モジュールエクスポート
├── persistence/
│   ├── mod.rs
│   ├── db_pools.rs         # Reader/Writer DB プール管理
│   ├── postgres/
│   │   ├── mod.rs
│   │   ├── todo_reader.rs  # PostgresTodoReader
│   │   ├── todo_writer.rs  # PostgresTodoWriter
│   │   ├── user_reader.rs  # PostgresUserReader
│   │   ├── user_writer.rs  # PostgresUserWriter
│   │   ├── file_reader.rs  # PostgresFileReader
│   │   └── file_writer.rs  # PostgresFileWriter
│   ├── redis/
│   │   ├── mod.rs
│   │   └── todo_cache.rs   # TodoCache
│   └── s3/
│       ├── mod.rs
│       └── s3_storage_service.rs  # S3StorageService
├── repositories/
│   ├── mod.rs
│   └── cached_todo_reader.rs  # CachedTodoReader
└── services/
    ├── mod.rs
    └── transactional_todo_service.rs
```

## PostgreSQL 実装

### DbPools（Reader/Writer 分離）

```rust
pub struct DbPools {
    pub writer: PgPool,  // プライマリ DB（書き込み）
    pub reader: PgPool,  // レプリカ DB（読み取り）
}

impl DbPools {
    /// 環境変数から接続プールを作成
    pub async fn from_env() -> Result<Self, anyhow::Error> {
        // DATABASE_WRITER_URL: 必須
        // DATABASE_READER_URL: オプション（未設定時は Writer と同じ）
    }
}
```

### PostgresTodoReader

```rust
#[async_trait]
impl TodoReader for PostgresTodoReader {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Todo>, DomainError> {
        let row: Option<TodoRow> = sqlx::query_as(
            "SELECT * FROM todos WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::Repository(e.to_string()))?;

        Ok(row.map(Into::into))
    }
}
```

### PostgresTodoWriter

```rust
#[async_trait]
impl TodoWriter for PostgresTodoWriter {
    async fn create(&self, todo: &Todo) -> Result<Todo, DomainError> {
        let row: TodoRow = sqlx::query_as(
            r#"
            INSERT INTO todos (id, user_id, title, description, completed, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#
        )
        // ... bind parameters
        .fetch_one(&self.pool)
        .await?;

        Ok(row.into())
    }
}
```

## Redis 実装

### TodoCache

```rust
pub struct TodoCache {
    client: redis::Client,
}

impl TodoCache {
    const TTL_SECONDS: u64 = 300;  // 5分
    const KEY_PREFIX: &'static str = "todo:";
}

#[async_trait]
impl TodoCacheOps for TodoCache {
    async fn get(&self, id: Uuid) -> Result<Option<Todo>, DomainError> {
        let key = format!("{}{}", Self::KEY_PREFIX, id);
        // GET → JSON デシリアライズ
    }

    async fn set(&self, todo: &Todo) -> Result<(), DomainError> {
        let key = format!("{}{}", Self::KEY_PREFIX, todo.id);
        // SETEX（TTL 付き）
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        let key = format!("{}{}", Self::KEY_PREFIX, id);
        // DEL
    }
}
```

## S3 実装

### S3StorageService

```rust
pub struct S3StorageService {
    client: Client,
    bucket: String,
}

impl S3StorageService {
    /// 環境変数から初期化（LocalStack 対応）
    pub async fn from_env() -> Result<Self, DomainError> {
        // S3_ENDPOINT_URL: LocalStack 用（オプション）
        // S3_BUCKET: バケット名（デフォルト: todo-files）
    }

    /// ファイルをアップロード
    pub async fn upload(
        &self,
        user_id: Uuid,
        filename: &str,
        content_type: &str,
        data: Vec<u8>,
    ) -> Result<String, DomainError> {
        // S3 キー: users/{user_id}/files/{uuid}/{filename}
    }

    /// ファイルをダウンロード
    pub async fn download(&self, storage_path: &str) -> Result<Vec<u8>, DomainError> {
        // GET Object
    }

    /// ファイルを削除
    pub async fn delete(&self, storage_path: &str) -> Result<(), DomainError> {
        // DELETE Object（冪等）
    }

    /// バケットが存在することを確認（起動時チェック用）
    pub async fn ensure_bucket_exists(&self) -> Result<(), DomainError> {
        // HEAD Bucket → なければ CREATE Bucket
    }
}
```

### LocalStack 対応

開発環境では LocalStack を使用して S3 をエミュレート:

```rust
let client = if let Some(ref endpoint) = endpoint_url {
    // LocalStack 用（カスタムエンドポイント + パススタイル強制）
    let s3_config = aws_sdk_s3::config::Builder::from(&sdk_config)
        .endpoint_url(endpoint)
        .force_path_style(true)  // LocalStack 互換性のため必要
        .build();
    Client::from_conf(s3_config)
} else {
    // 本番用（AWS 標準）
    Client::new(&sdk_config)
};
```

## デコレータ

### CachedTodoReader（Cache-Aside パターン）

```rust
pub struct CachedTodoReader<R: TodoReader> {
    inner: R,
    cache: Arc<dyn TodoCacheOps>,
}

#[async_trait]
impl<R: TodoReader> TodoReader for CachedTodoReader<R> {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Todo>, DomainError> {
        // 1. キャッシュを確認
        if let Some(todo) = self.cache.get(id).await.unwrap_or(None) {
            return Ok(Some(todo));
        }

        // 2. キャッシュミス → DB から取得
        let todo = self.inner.find_by_id(id).await?;

        // 3. キャッシュに保存
        if let Some(ref t) = todo {
            self.cache.set(t).await.ok();
        }

        Ok(todo)
    }
}
```

## トランザクションサービス

### TransactionalTodoService

```rust
pub struct TransactionalTodoService {
    pool: PgPool,
}

impl TransactionalTodoService {
    /// 複数 TODO を1トランザクションで作成
    pub async fn batch_create(
        &self,
        user_id: Uuid,
        todos: Vec<(String, Option<String>)>,
    ) -> Result<Vec<Todo>, DomainError> {
        let mut tx = self.pool.begin().await?;

        let mut created = Vec::new();
        for (title, description) in todos {
            let todo = sqlx::query_as(...)
                .fetch_one(&mut *tx)
                .await?;
            created.push(todo);
        }

        tx.commit().await?;
        Ok(created)
    }

    /// TODO + ファイルを1トランザクションで作成
    pub async fn create_with_files(
        &self,
        user_id: Uuid,
        title: String,
        description: Option<String>,
        files: Vec<FileInput>,
    ) -> Result<(Todo, Vec<File>), DomainError> {
        let mut tx = self.pool.begin().await?;

        // 1. TODO 作成
        let todo = sqlx::query_as(...).fetch_one(&mut *tx).await?;

        // 2. ファイルメタデータ作成
        let mut created_files = Vec::new();
        for file in files {
            let f = sqlx::query_as(...).fetch_one(&mut *tx).await?;
            created_files.push(f);
        }

        tx.commit().await?;
        Ok((todo, created_files))
    }
}
```

## CQRS + キャッシュ戦略

```
┌─────────────────────────────────────────────────────────────┐
│                      Application 層                         │
├─────────────────────────────────────────────────────────────┤
│  Commands (Writer + Cache)    │    Queries (Reader)         │
│  ┌─────────────────────────┐  │  ┌─────────────────────────┐│
│  │ CreateTodoCommand       │  │  │ GetTodoQuery           ││
│  │ UpdateTodoCommand       │  │  │ ListTodosQuery         ││
│  │ DeleteTodoCommand       │  │  │                         ││
│  └─────────┬───────────────┘  │  └─────────┬───────────────┘│
│            │                  │            │                │
├────────────┼──────────────────┼────────────┼────────────────┤
│            │   Infrastructure │            │                │
│            ▼                  │            ▼                │
│  ┌─────────────────────────┐  │  ┌─────────────────────────┐│
│  │ PostgresTodoWriter      │  │  │ CachedTodoReader       ││
│  │   + Write-Through       │  │  │   ┌─────────────────┐  ││
│  │   + Cache Invalidation  │  │  │   │ PostgresTodoReader│ ││
│  └─────────┬───────────────┘  │  │   └─────────────────┘  ││
│            │                  │  │   + Cache-Aside        ││
│            ▼                  │  └─────────┬───────────────┘│
│  ┌─────────────────────────┐  │            │                │
│  │ PostgreSQL (Writer Pool)│  │            ▼                │
│  └─────────────────────────┘  │  ┌─────────────────────────┐│
│                               │  │ PostgreSQL (Reader Pool)││
│  ┌─────────────────────────┐  │  └─────────────────────────┘│
│  │ TodoCache (Redis)       │◄─┼──────────────────────────────│
│  └─────────────────────────┘  │                             │
└───────────────────────────────┴─────────────────────────────┘
```

## 設計原則

### 依存性逆転の原則

- Infrastructure 層は Domain 層の**トレイトを実装**
- 依存の方向: `Infrastructure → Domain`
- API や Application 層は具象型を知らない

### トレイトオブジェクトの回避

```rust
// ✅ 推奨: ジェネリクス
pub struct CachedTodoReader<R: TodoReader> {
    inner: R,
}

// ❌ 非推奨: dyn Trait（ただしキャッシュは例外）
pub struct CachedTodoReader {
    inner: Box<dyn TodoReader>,  // 動的ディスパッチ
}
```

## 依存クレート

```toml
[dependencies]
domain = { path = "../domain" }
anyhow = "1.0"
async-trait = "0.1"
chrono = "0.4"
redis = { version = "0.29", features = ["tokio-comp"] }
serde = "1.0"
serde_json = "1.0"
sqlx = { version = "0.8", features = ["runtime-tokio", "postgres", "uuid", "chrono"] }
tracing = "0.1"
uuid = "1.11"

# AWS SDK for S3
aws-config = { version = "1.5", features = ["behavior-version-latest"] }
aws-sdk-s3 = "1.65"
```

## 使用例

```rust
use infrastructure::{
    DbPools, PostgresTodoReader, PostgresTodoWriter,
    TodoCache, CachedTodoReader, TransactionalTodoService,
    S3StorageService,
};

// DB プール作成
let pools = DbPools::from_env().await?;

// Writer（プライマリ DB）
let writer = PostgresTodoWriter::new(pools.writer.clone());

// Reader（レプリカ DB + キャッシュ）
let redis = redis::Client::open(redis_url)?;
let cache = Arc::new(TodoCache::new(redis));
let postgres_reader = PostgresTodoReader::new(pools.reader.clone());
let reader = CachedTodoReader::new(postgres_reader, cache.clone());

// バッチサービス
let batch = TransactionalTodoService::new(pools.writer.clone());

// S3 ストレージサービス
let storage = S3StorageService::from_env().await?;
storage.ensure_bucket_exists().await?;
```
