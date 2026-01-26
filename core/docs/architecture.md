# アーキテクチャ

本プロジェクトはクリーンアーキテクチャに基づき、4つの層に分離されています。
外部システム（Client、PostgreSQL、Redis）との通信は最外層で行い、ビジネスロジックは内側の層で保護されています。

## 全体構成

点線の矢印は「トレイトへの依存/実装」を、実線の矢印は「直接的な呼び出し」を表しています。

```mermaid
flowchart TB
    subgraph External["外部システム"]
        Client[("Client")]
        PG[("PostgreSQL")]
        RD[("Redis")]
    end

    subgraph API["api クレート"]
        Main["main.rs<br/>DI 設定・サーバー起動"]
    end

    subgraph Presentation["presentation 層"]
        Router["Router<br/>ルーティング"]
        Handler["Handlers<br/>HTTP ハンドラ"]
        ApiError["ApiError<br/>エラー変換"]
    end

    subgraph Application["application 層"]
        UC["Use Cases<br/>ビジネスロジック"]
        Auth["AuthService<br/>認証サービス"]
        DTO["DTOs<br/>データ転送"]
    end

    subgraph Domain["domain 層"]
        Entity["Todo/User/File Entity<br/>エンティティ"]
        Repo["TodoReader/Writer<br/>UserReader/Writer<br/>FileReader/Writer<br/>CQRS トレイト定義"]
        DomainErr["DomainError<br/>エラー型"]
    end

    subgraph Infrastructure["infrastructure 層"]
        PgRepo["PostgresTodoReader/Writer<br/>PostgresUserReader/Writer<br/>PostgresFileReader/Writer<br/>DB 実装"]
        Cache["TodoCache<br/>Redis 実装"]
        CachedRepo["CachedTodoReader<br/>デコレータ"]
        TxService["TransactionalTodoService<br/>バッチ操作"]
    end

    Client <-->|HTTP Request/Response| Router
    Router -->|dispatch| Handler
    Handler -->|call| UC
    Handler -->|convert| ApiError
    UC -->|use| DTO
    UC -->|use| Entity
    UC -.->|depends on| Repo
    CachedRepo -.->|implements| Repo
    PgRepo -.->|implements| Repo
    CachedRepo -->|delegates| PgRepo
    CachedRepo -->|uses| Cache
    PgRepo <-->|SQL Query| PG
    Cache <-->|GET/SET| RD
    Main -->|creates| CachedRepo
    Main -->|starts| Router
```

## クレート依存関係

Cargo ワークスペースは5つのクレートで構成されています。
`api` がエントリーポイント（バイナリ）で、残りの4つはライブラリクレートです。

```mermaid
graph LR
    subgraph Crates["Cargo Workspace"]
        direction LR
        api["api<br/>(binary)"]
        presentation["presentation<br/>(lib)"]
        application["application<br/>(lib)"]
        domain["domain<br/>(lib)"]
        infrastructure["infrastructure<br/>(lib)"]
    end

    api -->|uses| presentation
    api -->|uses| infrastructure
    presentation -->|uses| application
    application -->|uses| domain
    infrastructure -->|implements| domain
```

**依存関係の方向**: `api → presentation → application → domain ← infrastructure`

> **依存性逆転の原則**: infrastructure は domain のトレイトを実装するため、矢印が domain に向かう

## CQRS パターン（Reader/Writer 分離）

本プロジェクトでは軽量 CQRS（Command Query Responsibility Segregation）パターンを採用しています。
各エンティティのリポジトリを **Reader**（読み取り）と **Writer**（書き込み）に分離することで、
責務の明確化とスケーラビリティを実現しています。

```mermaid
flowchart LR
    subgraph Application["Application 層"]
        Query["Queries<br/>GetTodo, ListTodos"]
        Command["Commands<br/>Create, Update, Delete"]
    end

    subgraph Domain["Domain 層（トレイト定義）"]
        Reader["TodoReader<br/>UserReader<br/>FileReader"]
        Writer["TodoWriter<br/>UserWriter<br/>FileWriter"]
    end

    subgraph Infrastructure["Infrastructure 層（実装）"]
        PgReader["PostgresXxxReader<br/>（Reader Pool）"]
        PgWriter["PostgresXxxWriter<br/>（Writer Pool）"]
        CachedReader["CachedTodoReader<br/>（キャッシュ付き）"]
    end

    Query -->|参照| Reader
    Command -->|変更| Writer
    Reader -.->|実装| PgReader
    Reader -.->|実装| CachedReader
    Writer -.->|実装| PgWriter
    CachedReader -->|委譲| PgReader
```

| 役割 | トレイト | 操作 | DB プール |
| ---- | -------- | ---- | --------- |
| **Reader** | `TodoReader`, `UserReader`, `FileReader` | `find_by_id`, `find_all`, `find_by_email` | Reader Pool（レプリカ） |
| **Writer** | `TodoWriter`, `UserWriter`, `FileWriter` | `create`, `update`, `delete` | Writer Pool（プライマリ） |

> **メリット**: Aurora などで Reader/Writer エンドポイントが分離されている場合、
> 読み取り負荷をレプリカに分散できます。

## リクエストフロー

### TODO 取得（キャッシュあり）

`GET /todos/{id}` のリクエストフローを示します。
CachedTodoReader はまず Redis キャッシュを確認し、ヒットすればそのまま返却します。
キャッシュミスの場合は PostgreSQL から取得し、次回アクセスのためにキャッシュに保存します。

```mermaid
sequenceDiagram
    autonumber
    participant C as Client
    participant H as Handler
    participant Q as GetTodoQuery
    participant CR as CachedTodoReader
    participant Cache as TodoCache (Redis)
    participant DB as PostgresTodoReader/Writer

    C->>H: GET /todos/{id}
    H->>Q: execute(id)
    Q->>CR: find_by_id(id)

    CR->>Cache: get(id)

    alt キャッシュヒット
        Cache-->>CR: Some(Todo)
        CR-->>Q: Ok(Some(Todo))
    else キャッシュミス
        Cache-->>CR: None
        CR->>DB: find_by_id(id)
        DB-->>CR: Ok(Some(Todo))
        CR->>Cache: set(todo)
        Cache-->>CR: Ok(())
        CR-->>Q: Ok(Some(Todo))
    end

    Q-->>H: Ok(Todo)
    H-->>C: 200 OK + JSON
```

### TODO 作成

`POST /todos` のリクエストフローを示します。
CreateTodoCommand はまずドメイン層でタイトルのバリデーションを行い、失敗した場合は早期リターンします。
バリデーション成功後、エンティティを生成して TodoWriter で永続化し、TodoCache に Write-Through で保存します。

```mermaid
sequenceDiagram
    autonumber
    participant C as Client
    participant H as Handler
    participant Cmd as CreateTodoCommand
    participant E as Todo Entity
    participant W as PostgresTodoWriter
    participant Cache as TodoCache

    C->>H: POST /todos {title, description}
    H->>Cmd: execute(user_id, CreateTodoDto)

    Cmd->>E: validate_title(title)
    alt バリデーション失敗
        E-->>Cmd: Err(Validation)
        Cmd-->>H: Err(DomainError)
        H-->>C: 400 Bad Request
    else バリデーション成功
        E-->>Cmd: Ok(trimmed_title)
        Cmd->>E: new(user_id, title, description)
        E-->>Cmd: Todo
    end

    Cmd->>W: create(todo)
    W-->>Cmd: Ok(Todo)
    Cmd->>Cache: set(todo)
    Note over Cache: Write-Through
    Cache-->>Cmd: Ok(()) or warn

    Cmd-->>H: Ok(Todo)
    H-->>C: 201 Created + JSON
```

## クリーンアーキテクチャの利点

```mermaid
mindmap
  root((Clean<br/>Architecture))
    テスト容易性
      ドメイン層は外部依存なし
      モックリポジトリで単体テスト可能
    柔軟性
      DB変更がドメイン層に影響しない
      キャッシュ実装の差し替えが容易
    関心の分離
      各層の責務が明確
      コードの見通しが良い
    依存性逆転
      インフラ層がドメイン層に依存
      トレイトによる抽象化
```

1. **テスト容易性**: ドメイン層は外部依存がなく、単体テストが容易
2. **柔軟性**: DB や キャッシュの実装を変更しても、ドメイン層に影響なし
3. **関心の分離**: 各層の責務が明確で、コードの見通しが良い
4. **依存性逆転**: インフラ層がドメイン層に依存（トレイト経由）

## エラーハンドリング

ドメイン層で発生したエラー（DomainError）は、プレゼンテーション層で HTTP ステータスコードに変換されます。

```mermaid
flowchart LR
    subgraph Domain["Domain Layer"]
        DE[DomainError]
        V[Validation]
        A[Authentication]
        D[Duplicate]
        NF[NotFound]
        R[Repository]
        C[Cache]
    end

    subgraph Presentation["Presentation Layer"]
        AE[ApiError]
        BR[BadRequest<br/>400]
        UA[Unauthorized<br/>401]
        CF[Conflict<br/>409]
        NFR[NotFound<br/>404]
        ISE[Internal<br/>500]
    end

    DE -->|変換| AE
    V -->|maps to| BR
    A -->|maps to| UA
    D -->|maps to| CF
    NF -->|maps to| NFR
    R -->|maps to| ISE
    C -->|maps to| ISE
```

| DomainError      | ApiError       | HTTP Status |
| ---------------- | -------------- | ----------- |
| `Validation`     | `BadRequest`   | 400         |
| `Authentication` | `Unauthorized` | 401         |
| `Duplicate`      | `Conflict`     | 409         |
| `NotFound`       | `NotFound`     | 404         |
| `Repository`     | `Internal`     | 500         |
| `Cache`          | `Internal`     | 500         |
