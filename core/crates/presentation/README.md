# Presentation 層

クリーンアーキテクチャの最外層。HTTP リクエスト/レスポンスの処理を担当。

## 概要

Presentation 層は axum フレームワークを使用して HTTP API を提供します。
ハンドラは「薄く」保ち、ビジネスロジックは Application 層に委譲します。

## 責務

- **HTTP ハンドラ**: リクエスト受信、レスポンス生成
- **ルーティング**: エンドポイントとハンドラの紐付け
- **ミドルウェア**: Edge 検証、ユーザーコンテキスト抽出
- **エラー変換**: DomainError → HTTP ステータスコード
- **状態管理**: AppState（DI コンテナ）

## ディレクトリ構成

```
src/
├── lib.rs              # モジュールエクスポート
├── error.rs            # ApiError（HTTP エラーレスポンス）
├── routes.rs           # ルーティング設定
├── state.rs            # AppState（ユースケース保持）
├── handlers/
│   ├── mod.rs
│   ├── healthz.rs      # ヘルスチェック
│   ├── auth.rs         # 認証（登録、ログイン）
│   ├── todo.rs         # TODO CRUD
│   └── batch.rs        # バッチ操作
└── middleware/
    ├── mod.rs
    ├── edge_verify.rs  # Edge 検証ミドルウェア
    └── user_context.rs # UserContext エクストラクタ
```

## ハンドラ

### TODO ハンドラ

```rust
/// TODO 作成
/// POST /api/todos
pub async fn create_todo<TW, TR, C, UR, UW>(
    user: UserContext,                              // X-User-Id から抽出
    State(state): State<Arc<AppState<TW, TR, C, UR, UW>>>,
    Json(req): Json<CreateTodoRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let dto = CreateTodoDto {
        title: req.title,
        description: req.description,
    };

    let todo = state.create_todo.execute(user.user_id, dto).await?;

    Ok((StatusCode::CREATED, Json(todo)))
}
```

### 認証ハンドラ

```rust
/// ログイン
/// POST /api/auth/login
pub async fn login<TW, TR, C, UR, UW>(
    State(state): State<Arc<AppState<TW, TR, C, UR, UW>>>,
    Json(req): Json<LoginRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let token = state.auth_service.login(&req.email, &req.password).await?;

    Ok((StatusCode::OK, Json(TokenResponse::new(token))))
}
```

## ルーティング

```rust
pub fn create_router<TW, TR, C, UR, UW>(
    state: Arc<AppState<TW, TR, C, UR, UW>>,
    edge_secret: Option<String>,
) -> Router {
    // 認証ルート（Edge 検証不要）
    let auth_routes = Router::new()
        .route("/register", post(register::<TW, TR, C, UR, UW>))
        .route("/login", post(login::<TW, TR, C, UR, UW>));

    // TODO ルート（Edge 検証必要）
    let todo_routes = Router::new()
        .route("/", get(list_todos).post(create_todo))
        .route("/{id}", get(get_todo).patch(update_todo).delete(delete_todo))
        .route("/batch", post(batch_create_todos))
        .route("/with-files", post(create_todo_with_files));

    // Edge 検証ミドルウェアを適用
    let todo_routes = if let Some(secret) = edge_secret {
        with_edge_verify(todo_routes, secret)
    } else {
        todo_routes  // 開発モード: 検証スキップ
    };

    Router::new()
        .route("/health", get(healthz))
        .nest("/api/auth", auth_routes)
        .nest("/api/todos", todo_routes)
        .with_state(state)
}
```

## ミドルウェア

### Edge 検証ミドルウェア

```rust
/// X-Edge-Verified ヘッダーを検証
async fn edge_verify(
    State(state): State<EdgeVerifyState>,
    request: Request<Body>,
    next: Next,
) -> Response {
    let edge_verified = request.headers().get("X-Edge-Verified");

    match edge_verified {
        Some(secret) if secret == state.secret => {
            next.run(request).await  // 検証成功
        }
        _ => {
            (StatusCode::FORBIDDEN, "Forbidden").into_response()
        }
    }
}
```

### UserContext エクストラクタ

```rust
#[derive(Debug, Clone)]
pub struct UserContext {
    pub user_id: Uuid,
    pub request_id: Option<String>,
}

impl<S: Send + Sync> FromRequestParts<S> for UserContext {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // X-User-Id ヘッダーから UUID を抽出
        let user_id = parts.headers
            .get("X-User-Id")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| Uuid::parse_str(s).ok())
            .ok_or((StatusCode::UNAUTHORIZED, "Missing X-User-Id"))?;

        Ok(UserContext { user_id, request_id: None })
    }
}
```

## エラー変換

```rust
pub enum ApiError {
    BadRequest(String),     // 400
    Unauthorized(String),   // 401
    NotFound,               // 404
    Conflict(String),       // 409
    Internal(String),       // 500
}

impl From<DomainError> for ApiError {
    fn from(err: DomainError) -> Self {
        match err {
            DomainError::Validation(msg) => ApiError::BadRequest(msg),
            DomainError::Authentication(msg) => ApiError::Unauthorized(msg),
            DomainError::NotFound => ApiError::NotFound,
            DomainError::Duplicate(msg) => ApiError::Conflict(msg),
            DomainError::Repository(msg) => ApiError::Internal(msg),
            DomainError::Cache(msg) => ApiError::Internal(msg),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            // ...
        };

        (status, Json(json!({"error": message}))).into_response()
    }
}
```

## AppState

```rust
pub struct AppState<TW, TR, C, UR, UW> {
    // 認証サービス
    pub auth_service: AuthService<UR, UW>,

    // TODO Commands
    pub create_todo: CreateTodoCommand<TW, C>,
    pub update_todo: UpdateTodoCommand<TW, C>,
    pub delete_todo: DeleteTodoCommand<TW, C>,

    // TODO Queries
    pub get_todo: GetTodoQuery<TR>,
    pub list_todos: ListTodosQuery<TR>,

    // バッチサービス
    pub batch_service: TransactionalTodoService,
}
```

## API エンドポイント

| メソッド | パス | 説明 | 認証 |
|---------|------|------|-----|
| GET | `/health` | ヘルスチェック | 不要 |
| POST | `/api/auth/register` | ユーザー登録 | 不要 |
| POST | `/api/auth/login` | ログイン | 不要 |
| GET | `/api/todos` | TODO 一覧 | 必要 |
| POST | `/api/todos` | TODO 作成 | 必要 |
| GET | `/api/todos/{id}` | TODO 取得 | 必要 |
| PATCH | `/api/todos/{id}` | TODO 更新 | 必要 |
| DELETE | `/api/todos/{id}` | TODO 削除 | 必要 |
| POST | `/api/todos/batch` | バッチ作成 | 必要 |
| POST | `/api/todos/with-files` | TODO+ファイル作成 | 必要 |

## セキュリティ

### Defense in Depth（多層防御）

```
Client → Edge Layer (JWT 検証) → Core Layer (Edge 検証) → Handler (所有者チェック)
```

1. **Edge 層**: JWT 署名検証
2. **Core 層 ミドルウェア**: X-Edge-Verified ヘッダー検証
3. **Core 層 ハンドラ**: user_id による所有権検証

## 依存クレート

```toml
[dependencies]
application = { path = "../application" }
domain = { path = "../domain" }
infrastructure = { path = "../infrastructure" }
axum = "0.8"
serde = "1.0"
serde_json = "1.0"
thiserror = "2.0"
tracing = "0.1"
uuid = "1.11"
```

## 使用例

```rust
use presentation::{create_router, AppState};

// AppState を作成（DI）
let state = Arc::new(AppState::new(
    todo_writer, todo_reader, cache,
    user_reader, user_writer,
    batch_service, jwt_secret, jwt_expiry_hours,
));

// ルーターを構築
let app = create_router(state, Some("edge-secret".to_string()));

// サーバー起動
let listener = TcpListener::bind("0.0.0.0:3001").await?;
axum::serve(listener, app).await?;
```
