// =============================================================================
// presentation/src/middleware/mod.rs: ミドルウェアモジュール
// =============================================================================
// HTTP リクエストの前処理を行うミドルウェア群を定義。
// Edge 層からのリクエスト検証と認証情報の抽出を担当。
//
// モジュール構成:
// - edge_verify: Edge 検証ミドルウェア（Defense in Depth）
// - user_context: UserContext エクストラクタ（認証情報）
//
// セキュリティ戦略:
// - Defense in Depth（多層防御）パターンを採用
// - Edge 層: JWT 検証、レート制限
// - Core 層: Edge 検証、所有者チェック
//
// リクエストフロー:
// ```
// Client → Edge Layer (JWT 検証) → Core Layer (Edge 検証) → Handler
//                                        ↓
//                               UserContext 抽出
// ```
// =============================================================================

// -----------------------------------------------------------------------------
// サブモジュール宣言
// -----------------------------------------------------------------------------

// edge_verify: Edge 検証ミドルウェア
// X-Edge-Verified ヘッダーでリクエスト元を検証
mod edge_verify;

// user_context: UserContext エクストラクタ
// X-User-Id ヘッダーから認証済みユーザー情報を抽出
mod user_context;

// -----------------------------------------------------------------------------
// 再エクスポート
// -----------------------------------------------------------------------------

// with_edge_verify: Router に Edge 検証を適用する関数
// 使用例: with_edge_verify(router, "secret".to_string())
pub use edge_verify::with_edge_verify;

// UserContext: 認証済みユーザー情報（ハンドラの引数として使用）
// 使用例: async fn handler(user: UserContext) -> impl IntoResponse
pub use user_context::UserContext;
