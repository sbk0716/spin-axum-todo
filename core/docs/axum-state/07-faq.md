# よくある疑問と回答（FAQ）

State に関するよくある疑問をまとめました。

---

## このドキュメントのコード例について

FAQ では質問の内容に応じて2種類のコード例を使い分けています：

| 質問タイプ | 使用するコード例 |
|-----------|----------------|
| 概念説明（Arc、Mutex、パフォーマンス） | 最小サンプル（Counter App） |
| 実践パターン（テスト、設計） | TODO アプリ |

---

## 目次

1. [Arc について](#arc-について)
2. [Mutex について](#mutex-について)
3. [パフォーマンスについて](#パフォーマンスについて)
4. [設計について](#設計について)
5. [ジェネリクスについて](#ジェネリクスについて)
6. [テストについて](#テストについて)

---

## Arc と axum 推奨パターンについて

### Q1: axum が State をどう処理しているのか？

**A: 各リクエストで State をクローンして、ハンドラに渡している。**

axum 公式ドキュメント:
> "Your top level state needs to derive Clone"

axum メンテナー mladedav（GitHub Discussion #3223）:
> "When you extract the state, axum will clone it and pass it to your handler.
>  You can use Arc to make the clone cheap if your state is large or expensive to clone."

```rust
// axum 内部の動作イメージ（axum 推奨パターン）
loop {
    let request = accept_request().await;
    let state_clone = state.clone();  // ← 毎リクエストで AppState を clone
    tokio::spawn(async move {
        let response = handler(State(state_clone), request).await;
        send_response(response).await;
    });
}
```

### Q2: AppState の clone() は遅くないのか？

**A: AppState の全フィールドは内部で Arc を使用しているため、実質的に O(1) で高速。**

```rust
// AppState のフィールド（全て Arc を内部に持つ）
pub struct AppState<TW, TR, C, UR, UW, S> {
    pub auth_service: AuthService<UR, UW>,      // 内部で Arc 使用
    pub create_todo: CreateTodoCommand<TW, C>,  // 内部で Arc 使用
    // ...
}

// clone() は各フィールドの Arc::clone() を呼ぶだけ
// → 参照カウント +1 のみ
// → 約 5 ナノ秒/フィールド
```

### Q3: with_state() は何をしているのか？

**A: State を Router に登録し、各ハンドラーに自動で渡す仕組みを設定。**

```rust
let app = Router::new()
    .route("/todos", get(list_todos))
    .with_state(state);  // ← State を登録（axum 推奨: Clone 可能な AppState）

// 登録後、ハンドラーは State を引数で受け取れる
async fn list_todos(
    State(state): State<AppState<TW, TR, C, UR, UW, S>>,  // ← 自動で注入される
) { ... }
```

---

## Mutex について

### Q4: Mutex はいつ使うのか？

**A: State のフィールドを変更する必要がある場合。**

```rust
struct AppState {
    // 読み取り専用 → Mutex 不要
    config: AppConfig,

    // 変更する → Mutex 必要
    request_count: Mutex<u64>,
}

async fn increment_count(State(state): State<Arc<AppState>>) {
    let mut count = state.request_count.lock().unwrap();
    *count += 1;
}
```

ただし、DB 接続プール（PgPool）や Redis クライアントは**内部でスレッドセーフ**なので Mutex 不要。

### Q5: 複数のハンドラーが同時に db_pool を使っても大丈夫？

**A: PgPool は内部でスレッドセーフに設計されているから大丈夫。**

```rust
// PgPool の内部構造（イメージ）
struct PgPool {
    connections: Mutex<Vec<Connection>>,  // ← 内部で Mutex を持っている
    // ...
}
```

DB 接続プールは「複数のスレッドから同時に使われること」を前提に設計されている。
だから AppState 全体を Mutex で包む必要はない。

### Q6: AppState のフィールドを変更したい場合は？

**A: そのフィールドだけを Mutex で包む。**

```rust
struct AppState {
    // 読み取り専用 → そのまま
    db_pool: PgPool,
    jwt_secret: String,

    // 変更したい → Mutex で包む
    request_count: Mutex<u64>,
}

// 使い方
async fn some_handler(State(state): State<Arc<AppState>>) {
    // 変更するフィールドだけロック
    let mut count = state.request_count.lock().unwrap();
    *count += 1;
}
```

---

## パフォーマンスについて

### Q7: Arc::clone() のコストは？

**A: 約5ナノ秒。HTTP リクエスト処理（数ミリ秒〜数百ミリ秒）と比べると無視できる。**

| 操作 | 時間 |
|------|------|
| Arc::clone() | 約 5 ナノ秒 |
| HTTP パース | 約 1 マイクロ秒 |
| DB クエリ | 約 1-100 ミリ秒 |
| HTTP レスポンス | 約 1 マイクロ秒 |

Arc::clone() のコストは全体の **0.0001% 以下**。

### Q8: Arc と Rc どちらを使うべき？

**A: Web サーバーでは必ず Arc を使う。**

- Rc: シングルスレッド用（スレッドセーフではない）
- Arc: マルチスレッド用（アトミック操作でスレッドセーフ）

axum / tokio は複数のスレッドでリクエストを処理するため、Arc が必須。

---

## 設計について

### Q9: 複数の State を使いたい場合は？

**A: 1つの AppState に全てまとめるのが推奨。**

```rust
// ❌ 非推奨：複数の State
.with_state(state1)
.with_state(state2)  // エラーになる

// ✅ 推奨：1つにまとめる
struct AppState {
    db: DbState,
    cache: CacheState,
    auth: AuthState,
}
```

### Q10: 構造体全体を Mutex で包むのはダメ？

**A: パフォーマンスが大幅に悪化するので避けるべき。**

```rust
// ❌ 非推奨
Arc<Mutex<AppState>>
// → 全フィールドがロックされる
// → 読み取り専用フィールドにアクセスするだけでも待たされる

// ✅ 推奨
Arc<AppState>  // AppState 内で必要なフィールドだけ Mutex
```

詳細は [05-mutex-and-design.md](./05-mutex-and-design.md) を参照。

---

## ジェネリクスについて

### Q11: なぜ AppState はジェネリクスを使用しているのか？

**A: axum 公式がジェネリクスをトレイトオブジェクトより推奨しているため。**

axum のドキュメントでは以下のように記載されています：

> "The documentation emphasizes **generics over trait objects**."

ジェネリクスを使用することで：
- **コンパイル時型安全性**: 型エラーをコンパイル時に検出
- **ゼロコスト抽象化**: 動的ディスパッチのオーバーヘッドがない
- **テスト容易性**: モック実装への差し替えが容易

### Q12: ターボフィッシュ構文 `::<TW, TR, ...>` とは何か？

**A: ジェネリック関数の型パラメータを明示的に指定する構文。**

```rust
// routes.rs でのターボフィッシュ使用例
.route("/", get(list_todos::<TW, TR, C, UR, UW, S>))
//                        ^^^^^^^^^^^^^^^^^^^^^^^^^^ ターボフィッシュ
```

Rust コンパイラが型を推論できない場合に必要になります。魚のように見えることからこの名前が付きました。

### Q13: なぜ AppState は Clone を手動実装しているのか？

**A: axum 公式推奨パターンを採用しているため、Clone 実装は必須。**

axum 公式ドキュメント:
> "Your top level state needs to derive Clone"

```rust
// state.rs での手動 Clone 実装
impl<TW, TR, C, UR, UW, S> Clone for AppState<TW, TR, C, UR, UW, S>
where
    TW: TodoWriter,
    TR: TodoReader,
    // ...
{
    fn clone(&self) -> Self {
        Self {
            auth_service: self.auth_service.clone(),
            create_todo: self.create_todo.clone(),
            // ...
        }
    }
}
```

**なぜ `#[derive(Clone)]` ではなく手動実装？**

ジェネリクスを使用しているため、`#[derive(Clone)]` は `TW: Clone` などの不要な制約を追加してしまいます。
手動実装により、正確なトレイト境界（`TW: TodoWriter` など）のみを要求できます。

### Q14: トレイトオブジェクトに変更すべきか？

**A: 現状は変更しない方が良い。**

| 観点 | 現状（ジェネリクス） | トレイトオブジェクト |
|------|----------------------|----------------------|
| axum 推奨 | ✅ 準拠 | ❌ 言及なし |
| 型安全性 | ✅ コンパイル時 | △ 一部ランタイム |
| 可読性 | △ ターボフィッシュが冗長 | ✅ シンプル |
| 変更コスト | - | ~620行 |

現状は axum 公式推奨に沿っており、動作している本番前コードを大きく変更するリスクを避けるべきです。

詳細は [08-generics-design.md](./08-generics-design.md) を参照してください。

---

## テストについて

### Q15: テスト時はどうする？

**A: モック実装を注入する。**

```rust
#[tokio::test]
async fn test_list_todos() {
    // モックリポジトリを作成
    let mock_reader = Arc::new(MockTodoReader::new());
    mock_reader.expect_find_all().returning(|| Ok(vec![]));

    // モック State を作成（axum 推奨: Arc なしで直接作成）
    let state = AppState::new(
        Arc::new(MockTodoWriter::new()),
        mock_reader,
        // ...
    );

    // ハンドラーをテスト
    let response = list_todos(State(state)).await;
    assert!(response.is_ok());
}
```

State パターンの大きなメリットの1つが、テスト時にモックを注入できること。
本番コードを変更せずに、テスト用の実装に差し替えられる。

---

## よくあるミス

| ミス | 問題 | 正しい方法 |
|------|------|-----------|
| `Arc<Mutex<AppState>>` | 全フィールドがロックされる | フィールド単位で Mutex |
| ハンドラーで `state.clone()` | 不要なクローン | 参照を渡す（`&dyn Trait`） |
| Use Case で `State<...>` を受け取る | axum 依存が漏れる | トレイト参照を受け取る |
| PgPool を Mutex で包む | 二重ロック、不要なオーバーヘッド | そのまま使う（内部でスレッドセーフ） |
| 毎リクエストで DB 接続を新規作成 | 接続コスト高、接続数爆発 | State で接続プールを共有 |

---

## 関連ドキュメント

- [01-introduction.md](./01-introduction.md) - State の基本概念
- [02-lifecycle.md](./02-lifecycle.md) - ライフサイクルの詳細
- [03-arc.md](./03-arc.md) - Arc の詳細解説
- [05-mutex-and-design.md](./05-mutex-and-design.md) - Mutex と設計パターン
- [08-generics-design.md](./08-generics-design.md) - ジェネリクス設計の詳細
