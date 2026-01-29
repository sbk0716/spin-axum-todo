# axum State ドキュメント

axum の State を使ったデータ共有の仕組みを、段階的に理解するためのドキュメント集です。

---

## ドキュメント一覧

| ファイル | 内容 | 対象読者 |
|---------|------|---------|
| [01-introduction.md](./01-introduction.md) | State とは何か、なぜ必要か | 全員（最初に読む） |
| [02-lifecycle.md](./02-lifecycle.md) | シナリオベースで State のライフサイクルを理解 | 全員（2番目に読む） |
| [03-arc.md](./03-arc.md) | Arc（参照カウント）の詳細解説 | Rust 初心者、Arc を深く理解したい人 |
| [04-clone.md](./04-clone.md) | Clone トレイトの詳細解説 | Rust 初心者、Clone を深く理解したい人 |
| [05-mutex-and-design.md](./05-mutex-and-design.md) | Mutex と AppState の設計パターン | 実装者、設計判断が必要な人 |
| [06-quickref.md](./06-quickref.md) | 5W1H 形式のクイックリファレンス | 実装中に参照したい人 |
| [07-faq.md](./07-faq.md) | よくある疑問と回答 | 疑問がある人 |
| [08-generics-design.md](./08-generics-design.md) | ジェネリクス設計の解説 | 実装者、設計判断が必要な人 |

---

## 読む順番

### Rust 初心者の場合

```
01-introduction.md（State の概念を理解）
       ↓
02-lifecycle.md（具体的な動作を理解）
       ↓
03-arc.md（Arc を深掘り）
       ↓
04-clone.md（Clone を深掘り）
       ↓
05-mutex-and-design.md（設計パターン）
       ↓
08-generics-design.md（ジェネリクス設計）
```

### Rust 経験者の場合

```
01-introduction.md（axum 固有の概念を確認）
       ↓
02-lifecycle.md（axum での State の流れを確認）
       ↓
05-mutex-and-design.md（設計パターン）
       ↓
08-generics-design.md（ジェネリクス設計）
```

### 実装中にサッと参照したい場合

```
06-quickref.md（5W1H クイックリファレンス）
       ↓
07-faq.md（疑問があれば）
```

---

## 関連ファイル

このプロジェクトでの実装例：

- [api/src/main.rs](../../api/src/main.rs) - Arc<AppState> の実際の使用例
- [crates/presentation/src/state.rs](../../crates/presentation/src/state.rs) - AppState の定義
