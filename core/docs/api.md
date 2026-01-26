# API リファレンス

## エンドポイント一覧

### 認証 API

| メソッド | パス                 | 説明                 | レスポンス |
| -------- | -------------------- | -------------------- | ---------- |
| POST     | `/api/auth/register` | ユーザー登録         | 201 / 409  |
| POST     | `/api/auth/login`    | ログイン（JWT 取得） | 200 / 401  |

### TODO API

| メソッド | パス                         | 説明                   | レスポンス |
| -------- | ---------------------------- | ---------------------- | ---------- |
| GET      | `/health`                    | ヘルスチェック         | 200        |
| GET      | `/api/todos`                 | TODO 一覧取得          | 200        |
| GET      | `/api/todos?completed=true`  | 完了済みのみ           | 200        |
| GET      | `/api/todos?completed=false` | 未完了のみ             | 200        |
| POST     | `/api/todos`                 | TODO 作成              | 201        |
| GET      | `/api/todos/{id}`            | TODO 取得              | 200 / 404  |
| PATCH    | `/api/todos/{id}`            | TODO 更新              | 200 / 404  |
| DELETE   | `/api/todos/{id}`            | TODO 削除              | 204 / 404  |
| POST     | `/api/todos/batch`           | バッチ TODO 作成       | 201 / 400  |
| POST     | `/api/todos/with-files`      | TODO + ファイル作成    | 201 / 400  |

> **Note**: TODO API は `X-User-Id` ヘッダーが必要です（Edge 層が JWT から抽出して付与）。

## 認証 API 詳細

### POST /api/auth/register

ユーザー登録。

**リクエスト:**

```json
{
  "email": "user@example.com",
  "password": "password123",
  "display_name": "User Name"  // 任意
}
```

**レスポンス (201 Created):**

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "email": "user@example.com",
  "display_name": "User Name",
  "created_at": "2026-01-26T00:00:00Z"
}
```

**エラー:**

| ステータス | 条件 |
| ---------- | ---- |
| 400 | バリデーションエラー（メール形式不正、パスワード8文字未満） |
| 409 | メールアドレス重複 |

### POST /api/auth/login

ログイン（JWT 取得）。

**リクエスト:**

```json
{
  "email": "user@example.com",
  "password": "password123"
}
```

**レスポンス (200 OK):**

```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}
```

**エラー:**

| ステータス | 条件 |
| ---------- | ---- |
| 401 | メールアドレスまたはパスワードが不正 |

## TODO API 詳細

### POST /api/todos

TODO 作成。

**リクエスト:**

```json
{
  "title": "買い物",
  "description": "牛乳とパン"  // 任意
}
```

**レスポンス (201 Created):**

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "user_id": "...",
  "title": "買い物",
  "description": "牛乳とパン",
  "completed": false,
  "created_at": "2026-01-26T00:00:00Z",
  "updated_at": "2026-01-26T00:00:00Z"
}
```

### PATCH /api/todos/{id}

TODO 更新（部分更新）。

**リクエスト:**

```json
{
  "title": "新しいタイトル",      // 任意
  "description": "新しい説明",    // 任意
  "completed": true               // 任意
}
```

> **Note**: 指定したフィールドのみ更新されます。

### POST /api/todos/batch

複数の TODO を1トランザクションで作成。いずれかが失敗した場合、全てロールバック。

**リクエスト:**

```json
{
  "todos": [
    {"title": "TODO 1", "description": "説明1"},
    {"title": "TODO 2"},
    {"title": "TODO 3", "description": "説明3"}
  ]
}
```

**レスポンス (201 Created):**

```json
[
  {
    "id": "...",
    "user_id": "...",
    "title": "TODO 1",
    "description": "説明1",
    "completed": false,
    "created_at": "...",
    "updated_at": "..."
  },
  // ...
]
```

**エラー:**

| ステータス | 条件 |
| ---------- | ---- |
| 400 | 空配列、タイトルバリデーションエラー |

### POST /api/todos/with-files

TODO とその添付ファイルを1トランザクションで作成。

**リクエスト:**

```json
{
  "title": "ファイル付き TODO",
  "description": "説明",
  "files": [
    {
      "filename": "document.pdf",
      "mime_type": "application/pdf",
      "size_bytes": 12345,
      "storage_path": "/uploads/2026/01/abc123.pdf"
    }
  ]
}
```

**レスポンス (201 Created):**

```json
{
  "todo": {
    "id": "...",
    "user_id": "...",
    "title": "ファイル付き TODO",
    "description": "説明",
    "completed": false,
    "created_at": "...",
    "updated_at": "..."
  },
  "files": [
    {
      "id": "...",
      "todo_id": "...",
      "filename": "document.pdf",
      "mime_type": "application/pdf",
      "size_bytes": 12345,
      "storage_path": "/uploads/2026/01/abc123.pdf",
      "created_at": "..."
    }
  ]
}
```

> **Note**: ファイル本体は事前にストレージにアップロード済みの前提。
> このエンドポイントはメタデータのみを DB に登録します。

## 使用例

### Edge 層経由（推奨）

```bash
# 1. ユーザー登録
curl -X POST http://localhost:3000/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{"email": "test@example.com", "password": "password123"}'

# 2. ログイン（JWT 取得）
TOKEN=$(curl -s -X POST http://localhost:3000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email": "test@example.com", "password": "password123"}' | jq -r '.token')

# 3. TODO 作成
curl -X POST http://localhost:3000/api/todos \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"title": "買い物", "description": "牛乳とパン"}'

# 4. TODO 一覧取得
curl -H "Authorization: Bearer $TOKEN" http://localhost:3000/api/todos

# 5. TODO 更新
curl -X PATCH http://localhost:3000/api/todos/{id} \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"completed": true}'

# 6. TODO 削除
curl -X DELETE http://localhost:3000/api/todos/{id} \
  -H "Authorization: Bearer $TOKEN"
```

### Core 層への直接アクセス（開発・デバッグ用）

```bash
# ヘルスチェック
curl http://127.0.0.1:3001/health
# {"status":"ok"}

# 認証 API（Edge 層経由でなくても動作）
curl -X POST http://127.0.0.1:3001/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{"email": "direct@example.com", "password": "password123"}'

# TODO API（X-User-Id と X-Edge-Verified ヘッダーが必要）
curl http://127.0.0.1:3001/api/todos \
  -H "X-User-Id: 550e8400-e29b-41d4-a716-446655440000" \
  -H "X-Edge-Verified: super-secret-edge-key"
```

## エラーレスポンス

### 形式

```json
{
  "error": "エラーメッセージ"
}
```

### ステータスコード一覧

| ステータス | 説明 | 原因 |
| ---------- | ---- | ---- |
| 400 | Bad Request | バリデーションエラー |
| 401 | Unauthorized | 認証失敗、Edge 検証失敗 |
| 404 | Not Found | リソースが存在しない、または所有権なし |
| 409 | Conflict | 重複エラー（メールアドレス等） |
| 500 | Internal Server Error | サーバー内部エラー |

> **Note**: 404 は「存在しない」と「所有権なし」を区別しません（セキュリティ上の理由）。
