# API クレート

アプリケーションのエントリーポイント。Composition Root として全ての依存関係を組み立て。

## 概要

API クレートはバイナリクレートとして、アプリケーションの起動と依存関係の組み立てを担当します。
クリーンアーキテクチャにおける Composition Root（構成ルート）の役割を果たします。

## 責務

- **環境設定の読み込み**: Config 構造体で一括管理
- **インフラ層のインスタンス生成**: PostgreSQL、Redis、S3
- **依存性注入**: リポジトリの組み立て
- **サーバー起動**: axum HTTP サーバー（グレースフルシャットダウン対応）
- **マイグレーション管理**: DB スキーマ

## ディレクトリ構成

```
api/
├── Cargo.toml          # バイナリクレート設定
├── Dockerfile          # Docker イメージ定義
├── src/
│   ├── main.rs         # エントリーポイント
│   └── config.rs       # 環境設定（AppConfig）
└── migrations/         # DB マイグレーション
    ├── 20250125000000_create_todos.up.sql
    ├── 20250125000000_create_todos.down.sql
    ├── 20250126000000_add_user_id.up.sql
    ├── 20250126000000_add_user_id.down.sql
    ├── 20250127000000_create_users.up.sql
    ├── 20250127000000_create_users.down.sql
    ├── 20250128000000_create_files.up.sql
    └── 20250128000000_create_files.down.sql
```

## main.rs の構造

```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. 環境設定
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .json()
        .init();

    // 2. Config から設定を一括読み込み
    let config = AppConfig::from_env()?;

    // 3. インフラ層セットアップ
    let db_pools = DbPools::from_config(
        &config.database.writer_url,
        config.database.reader_url.as_deref(),
    ).await?;
    let redis_client = redis::Client::open(config.redis.url.as_str())?;
    let storage_service = S3StorageService::from_config(
        &config.s3.bucket,
        config.s3.endpoint_url.as_deref(),
    ).await?;

    // 4. リポジトリ組み立て（CQRS + キャッシュ）
    let todo_writer = Arc::new(PostgresTodoWriter::new(db_pools.writer.clone()));
    let cache = Arc::new(TodoCache::new(redis_client));
    let postgres_reader = PostgresTodoReader::new(db_pools.reader.clone());
    let todo_reader = Arc::new(CachedTodoReader::new(postgres_reader, cache.clone()));
    // ... 他のリポジトリ

    // 5. AppState 作成
    let state = Arc::new(AppState::new(...));

    // 6. ルーター構築 & サーバー起動（グレースフルシャットダウン対応）
    let app = create_router(state, config.edge_secret);
    let listener = TcpListener::bind(config.server.addr).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}
```

## 依存関係の組み立て

main.rs は Composition Root として、全ての依存関係を組み立てます。
インフラ層（DB、Redis、S3）からリポジトリを生成し、キャッシュデコレータを適用した後、
AppState に集約してルーターに渡します。

```
┌──────────────────────────────────────────────────────────────────┐
│                          main.rs                                 │
│                     (Composition Root)                           │
├──────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌────────────┐  ┌────────────┐  ┌────────────┐                  │
│  │  DbPools   │  │   Redis    │  │  S3Storage │                  │
│  │            │  │   Client   │  │   Service  │                  │
│  └─────┬──────┘  └─────┬──────┘  └─────┬──────┘                  │
│        │               │               │                         │
│        ▼               ▼               ▼                         │
│  ┌────────────┐  ┌────────────┐  ┌────────────┐                  │
│  │  Postgres  │  │  TodoCache │  │    File    │                  │
│  │  Reader/   │  │            │  │  Reader/   │                  │
│  │  Writer    │  │            │  │  Writer    │                  │
│  └─────┬──────┘  └─────┬──────┘  └─────┬──────┘                  │
│        │               │               │                         │
│        └───────┬───────┘               │                         │
│                ▼                       │                         │
│  ┌─────────────────────────┐           │                         │
│  │   CachedTodoReader      │           │                         │
│  │                         │           │                         │
│  └────────────┬────────────┘           │                         │
│               │                        │                         │
│               └────────────┬───────────┘                         │
│                            ▼                                     │
│  ┌───────────────────────────────────────────────────────────┐   │
│  │                        AppState                           │   │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │   │
│  │  │  Commands   │  │   Queries   │  │   Storage   │        │   │
│  │  └─────────────┘  └─────────────┘  └─────────────┘        │   │
│  └───────────────────────────────────────────────────────────┘   │
│                            │                                     │
│                            ▼                                     │
│  ┌───────────────────────────────────────────────────────────┐   │
│  │                      axum Router                          │   │
│  └───────────────────────────────────────────────────────────┘   │
│                                                                  │
└──────────────────────────────────────────────────────────────────┘
```

| コンポーネント         | 説明                                           |
| ---------------------- | ---------------------------------------------- |
| DbPools                | Writer/Reader 分離の DB 接続プール             |
| Postgres Reader/Writer | Todo/User/File の CQRS リポジトリ              |
| TodoCache              | Redis ベースのキャッシュ                       |
| CachedTodoReader       | デコレータパターンで DB + Cache を統合         |
| File Reader/Writer     | S3 ストレージと連携するファイルメタデータ管理  |
| AppState               | Commands/Queries/Storage を集約                |
| axum Router            | グレースフルシャットダウン対応の HTTP サーバー |

## 環境変数

| 変数                  | 説明                               | 必須 | デフォルト    |
| --------------------- | ---------------------------------- | ---- | ------------- |
| `APP_ADDR`            | リッスンアドレス                   | ○    | -             |
| `DATABASE_WRITER_URL` | 書き込み用 DB URL                  | ○    | -             |
| `DATABASE_READER_URL` | 読み取り用 DB URL                  | ×    | Writer と同じ |
| `REDIS_URL`           | Redis URL                          | ○    | -             |
| `JWT_SECRET`          | JWT 署名シークレット               | ×    | デフォルト値  |
| `JWT_EXPIRY_HOURS`    | JWT 有効期間                       | ×    | 24            |
| `S3_BUCKET`           | S3 バケット名                      | ×    | todo-files    |
| `S3_ENDPOINT_URL`     | S3 エンドポイント（LocalStack 用） | ×    | AWS 標準      |
| `EDGE_SECRET`         | Edge 検証シークレット              | ×    | 検証スキップ  |
| `RUST_LOG`            | ログレベル                         | ×    | info          |

## マイグレーション

### 実行

```bash
# sqlx-cli インストール
cargo install sqlx-cli --no-default-features --features postgres

# マイグレーション実行
cd api
sqlx migrate run

# ロールバック
sqlx migrate revert
```

### スキーマ

```sql
-- users テーブル
CREATE TABLE users (
    id UUID PRIMARY KEY,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    display_name TEXT,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL
);

-- todos テーブル
CREATE TABLE todos (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    description TEXT,
    completed BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL
);

-- files テーブル
CREATE TABLE files (
    id UUID PRIMARY KEY,
    todo_id UUID NOT NULL REFERENCES todos(id) ON DELETE CASCADE,
    filename TEXT NOT NULL,
    mime_type TEXT NOT NULL,
    size_bytes BIGINT NOT NULL,
    storage_path TEXT UNIQUE NOT NULL,
    created_at TIMESTAMPTZ NOT NULL
);
```

## Docker

### ビルド

```bash
# core ディレクトリで実行
docker build -f api/Dockerfile -t todo-api .
```

### 実行

```bash
docker run -p 3001:3001 \
  -e APP_ADDR=0.0.0.0:3001 \
  -e DATABASE_WRITER_URL=postgres://... \
  -e REDIS_URL=redis://... \
  todo-api
```

### Dockerfile

```dockerfile
FROM rust:1.84 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release -p api

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/api /usr/local/bin/
EXPOSE 3001
CMD ["api"]
```

## 依存クレート

```toml
[dependencies]
presentation = { path = "../crates/presentation" }
infrastructure = { path = "../crates/infrastructure" }
anyhow = "1.0"
dotenvy = "0.15"
redis = { version = "0.29", features = ["tokio-comp"] }
tokio = { version = "1.43", features = ["full"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }
```

## 起動

```bash
# 開発
cargo run -p api

# リリース
cargo run -p api --release

# 環境変数を指定
APP_ADDR=0.0.0.0:3001 \
DATABASE_WRITER_URL=postgres://app:app@localhost:5432/app \
REDIS_URL=redis://localhost:6379 \
cargo run -p api
```

## ログ

```bash
# 全体のログレベル
RUST_LOG=info cargo run -p api

# モジュール別
RUST_LOG=api=debug,infrastructure=info cargo run -p api

# SQL クエリを表示
RUST_LOG=sqlx=debug cargo run -p api
```
