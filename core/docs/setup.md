# セットアップ

> **推奨**: プロジェクトルートの `make setup` を使用してください。
> 以下は Core 層のみを手動でセットアップする場合の手順です。

## クイックスタート

```bash
# 1. PostgreSQL と Redis を起動
docker compose up -d

# 2. 環境変数を設定
cp ../example.env .env

# 3. マイグレーション実行
cargo install sqlx-cli --no-default-features --features postgres
cd api && sqlx migrate run && cd ..

# 4. サーバー起動
cargo run -p api
```

サーバーが `http://127.0.0.1:3001` で起動します。

## 環境変数

| 変数                  | 説明                                       | 必須 |
| --------------------- | ------------------------------------------ | ---- |
| `APP_ADDR`            | サーバーのリッスンアドレス                 | -    |
| `DATABASE_WRITER_URL` | PostgreSQL 書き込み用接続文字列            | 必須 |
| `DATABASE_READER_URL` | PostgreSQL 読み取り用接続文字列            | -    |
| `REDIS_URL`           | Redis 接続文字列                           | 必須 |
| `JWT_SECRET`          | JWT 署名用シークレット（Edge 層と同じ値）  | -    |
| `JWT_EXPIRY_HOURS`    | JWT 有効期間（時間）                       | -    |
| `EDGE_SECRET`         | Edge 検証用シークレット                    | -    |
| `RUST_LOG`            | ログレベル                                 | -    |

> **Note**: `DATABASE_READER_URL` が未設定の場合、`DATABASE_WRITER_URL` が使用されます。

### Reader/Writer DB 分離（Aurora 対応）

Aurora などのマネージド DB では Reader/Writer エンドポイントが分離されます。

```bash
# ローカル開発（同じ DB を使用）
DATABASE_WRITER_URL=postgres://app:app@localhost:5432/app
DATABASE_READER_URL=postgres://app:app@localhost:5432/app

# 本番（Aurora）
DATABASE_WRITER_URL=postgres://...cluster-xxx.rds.amazonaws.com/app
DATABASE_READER_URL=postgres://...cluster-ro-xxx.rds.amazonaws.com/app
```

## データベーススキーマ

### users テーブル

ローカル認証用のユーザー管理テーブル。

| カラム          | 型          | 説明                       |
| --------------- | ----------- | -------------------------- |
| `id`            | UUID        | 主キー（JWT sub クレーム） |
| `email`         | TEXT        | メールアドレス（UNIQUE）   |
| `password_hash` | TEXT        | bcrypt ハッシュ            |
| `display_name`  | TEXT        | 表示名（任意）             |
| `created_at`    | TIMESTAMPTZ | 作成日時                   |
| `updated_at`    | TIMESTAMPTZ | 更新日時                   |

### todos テーブル

TODO データを格納するテーブル。

| カラム        | 型          | 説明                 |
| ------------- | ----------- | -------------------- |
| `id`          | UUID        | 主キー               |
| `user_id`     | UUID        | 外部キー（users.id） |
| `title`       | TEXT        | タイトル             |
| `description` | TEXT        | 説明（任意）         |
| `completed`   | BOOLEAN     | 完了フラグ           |
| `created_at`  | TIMESTAMPTZ | 作成日時             |
| `updated_at`  | TIMESTAMPTZ | 更新日時             |

### files テーブル

TODO に添付されるファイルのメタデータを格納するテーブル。

| カラム         | 型          | 説明                     |
| -------------- | ----------- | ------------------------ |
| `id`           | UUID        | 主キー                   |
| `todo_id`      | UUID        | 外部キー（todos.id）     |
| `filename`     | TEXT        | 元のファイル名           |
| `mime_type`    | TEXT        | MIME タイプ              |
| `size_bytes`   | BIGINT      | ファイルサイズ（バイト） |
| `storage_path` | TEXT        | ストレージパス（UNIQUE） |
| `created_at`   | TIMESTAMPTZ | 作成日時                 |

### インデックスと制約

| 対象                 | 種類         | 説明                                                |
| -------------------- | ------------ | --------------------------------------------------- |
| `users.email`        | UNIQUE INDEX | メールアドレス検索用                                |
| `todos.user_id`      | INDEX        | ユーザー別 TODO 検索用                              |
| `todos.user_id`      | FOREIGN KEY  | `ON DELETE CASCADE`（ユーザー削除時に TODO も削除） |
| `files.todo_id`      | INDEX        | TODO 別ファイル検索用                               |
| `files.todo_id`      | FOREIGN KEY  | `ON DELETE CASCADE`（TODO 削除時にファイルも削除）  |
| `files.storage_path` | UNIQUE INDEX | ストレージパスの一意性保証                          |

### ER 図

```mermaid
erDiagram
    users ||--o{ todos : "has many"
    todos ||--o{ files : "has many"

    users {
        uuid id PK
        text email UK
        text password_hash
        text display_name
        timestamptz created_at
        timestamptz updated_at
    }

    todos {
        uuid id PK
        uuid user_id FK
        text title
        text description
        boolean completed
        timestamptz created_at
        timestamptz updated_at
    }

    files {
        uuid id PK
        uuid todo_id FK
        text filename
        text mime_type
        bigint size_bytes
        text storage_path UK
        timestamptz created_at
    }
```

## Docker

### イメージのビルド

```bash
# core ディレクトリで実行
docker build -f api/Dockerfile -t todo-api .
```

### 実行

```bash
docker run -p 3001:3001 \
  -e DATABASE_WRITER_URL=postgres://app:app@host.docker.internal:5432/app \
  -e REDIS_URL=redis://host.docker.internal:6379 \
  -e EDGE_SECRET=your-edge-secret \
  todo-api
```

### Docker Compose 構成

プロジェクトルートの `compose.yaml` で PostgreSQL と Redis を起動します。

```mermaid
graph TB
    subgraph compose["Docker Compose (プロジェクトルート)"]
        Edge["Edge Layer<br/>:3000"]
        Core["Core Layer<br/>:3001"]
        PG["postgres<br/>:5432"]
        Redis["redis<br/>:6379"]
    end

    Client((Client)) -->|HTTP| Edge
    Edge -->|X-User-Id, X-Edge-Verified| Core
    Core -->|SQL| PG
    Core -->|REDIS_URL| Redis
```

> **Note**: 開発環境では Edge 層と Core 層をローカルで直接起動し、
> PostgreSQL と Redis のみ Docker で起動することを推奨します。

## プロジェクト構成

```
core/
├── Cargo.toml              # ワークスペース定義
├── .env                    # 環境変数
├── docs/                   # ドキュメント
│   ├── architecture.md     # アーキテクチャ
│   ├── transaction.md      # トランザクション管理
│   ├── cache.md            # キャッシュ戦略
│   ├── security.md         # セキュリティ
│   ├── api.md              # API リファレンス
│   └── setup.md            # セットアップ（このファイル）
├── crates/
│   ├── domain/             # ドメイン層
│   │   └── src/
│   │       ├── entities/
│   │       │   ├── todo.rs       # Todo エンティティ
│   │       │   ├── user.rs       # User エンティティ
│   │       │   └── file.rs       # File エンティティ
│   │       ├── repositories/
│   │       │   ├── todo_repository.rs  # TodoReader/Writer トレイト
│   │       │   ├── user_repository.rs  # UserReader/Writer トレイト
│   │       │   ├── file_repository.rs  # FileReader/Writer トレイト
│   │       │   └── todo_cache.rs       # TodoCacheOps トレイト
│   │       └── errors/           # DomainError
│   │
│   ├── application/        # アプリケーション層
│   │   └── src/
│   │       ├── commands/         # CQRS Commands（Create, Update, Delete）
│   │       ├── queries/          # CQRS Queries（Get, List）
│   │       ├── services/
│   │       │   └── auth_service.rs  # 認証サービス（登録・ログイン）
│   │       └── dto/              # データ転送オブジェクト
│   │
│   ├── infrastructure/     # インフラ層
│   │   └── src/
│   │       ├── persistence/
│   │       │   ├── db_pools.rs      # Reader/Writer DB プール
│   │       │   ├── postgres/
│   │       │   │   ├── todo_reader.rs   # PostgresTodoReader
│   │       │   │   ├── todo_writer.rs   # PostgresTodoWriter
│   │       │   │   ├── user_reader.rs   # PostgresUserReader
│   │       │   │   ├── user_writer.rs   # PostgresUserWriter
│   │       │   │   ├── file_reader.rs   # PostgresFileReader
│   │       │   │   └── file_writer.rs   # PostgresFileWriter
│   │       │   └── redis/
│   │       │       └── todo_cache.rs    # TodoCache（Redis）
│   │       ├── repositories/
│   │       │   └── cached_todo_reader.rs  # CachedTodoReader
│   │       └── services/
│   │           └── transactional_todo_service.rs  # バッチ操作
│   │
│   └── presentation/       # プレゼンテーション層
│       └── src/
│           ├── handlers/
│           │   ├── todo.rs       # TODO ハンドラ
│           │   ├── auth.rs       # 認証ハンドラ
│           │   ├── batch.rs      # バッチハンドラ
│           │   └── healthz.rs    # ヘルスチェックハンドラ
│           ├── middleware/
│           │   ├── edge_verify.rs    # Edge 検証ミドルウェア
│           │   └── user_context.rs   # ユーザーコンテキスト抽出
│           ├── routes.rs   # ルーティング
│           ├── state.rs    # AppState（DI コンテナ）
│           └── error.rs    # ApiError
│
└── api/                    # エントリーポイント
    ├── Cargo.toml
    ├── src/main.rs         # DI 設定、サーバー起動
    ├── migrations/         # DB マイグレーション
    │   ├── 20250125000000_create_todos.up.sql
    │   ├── 20250126000000_add_user_id.up.sql
    │   └── 20250127000000_create_users.up.sql
    └── Dockerfile
```

## マイグレーション

### 実行

```bash
cd api
sqlx migrate run
```

### ロールバック

```bash
cd api
sqlx migrate revert
```

### 新規マイグレーション作成

```bash
cd api
sqlx migrate add <name>
```

## 開発コマンド

```bash
# ビルド
cargo build

# テスト
cargo test

# フォーマット
cargo fmt

# リント
cargo clippy

# ドキュメント生成
cargo doc --open

# 開発サーバー起動（ホットリロード）
cargo watch -x 'run -p api'
```
