//! # JWT 認証コンポーネント
//!
//! このコンポーネントは WIT インターフェース `demo:auth/authenticator` を実装し、
//! JWT (JSON Web Token) の検証機能を提供します。
//!
//! ## 責務
//! - JWT トークンの構造検証（ヘッダー、ペイロード、署名）
//! - HMAC-SHA256 署名の検証
//! - ユーザーIDの抽出
//!
//! ## 使用方法
//! gateway コンポーネントから WIT 経由で `verify_token` 関数が呼び出されます。

// =============================================================================
// 外部クレートのインポート
// =============================================================================

// Base64 エンコーディング/デコーディング用
// URL_SAFE_NO_PAD: URL セーフな Base64（パディングなし）を使用
// JWT は URL セーフな Base64 でエンコードされているため、この形式を使用
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};

// HMAC (Hash-based Message Authentication Code) 用
// Hmac: HMAC アルゴリズムの実装
// Mac: Message Authentication Code のトレイト（共通インターフェース）
use hmac::{Hmac, Mac};

// JSON デシリアライズ用
// Deserialize: JSON から Rust 構造体への変換を自動生成
use serde::Deserialize;

// SHA-256 ハッシュアルゴリズム
// HMAC-SHA256 の内部ハッシュ関数として使用
use sha2::Sha256;

// =============================================================================
// WIT バインディングの生成
// =============================================================================

// wit_bindgen マクロで WIT インターフェースから Rust コードを自動生成
// world: このコンポーネントが実装する world 名（auth-world）
// path: WIT ファイルが配置されているディレクトリへの相対パス
wit_bindgen::generate!({
    world: "auth-world",
    path: "../wit",
});

// wit-bindgen が生成した型をインポート
// Guest: WIT の export を実装するためのトレイト
// AuthResult: 認証結果を表す構造体（WIT の auth-result レコードに対応）
use exports::demo::auth::authenticator::{AuthResult, Guest};

// =============================================================================
// 定数定義
// =============================================================================

/// JWT 署名検証用の秘密鍵
///
/// 注意: これはデモ用のハードコードされた秘密鍵です。
/// 本番環境では以下の方法で安全に管理してください：
/// - 環境変数から取得
/// - シークレット管理サービス（HashiCorp Vault など）を使用
/// - Spin の変数機能を使用
const SECRET_KEY: &[u8] = b"super-secret-key";

// =============================================================================
// JWT 構造体定義
// =============================================================================

/// JWT ヘッダーを表す構造体
///
/// JWT のヘッダー部分には、トークンのタイプと署名アルゴリズムが含まれます。
/// 例: {"alg":"HS256","typ":"JWT"}
#[derive(Debug, Deserialize)]
struct JwtHeader {
    /// 署名アルゴリズム（例: "HS256", "RS256"）
    /// このデモでは HS256 のみサポート
    alg: String,

    /// トークンタイプ（通常は "JWT"）
    /// 検証には使用しないため、dead_code 警告を抑制
    #[allow(dead_code)]
    typ: Option<String>,
}

/// JWT ペイロード（クレーム）を表す構造体
///
/// JWT のペイロード部分には、トークンに関する情報（クレーム）が含まれます。
/// 標準クレーム（RFC 7519）の一部を定義しています。
#[derive(Debug, Deserialize)]
struct JwtPayload {
    /// Subject（サブジェクト）: トークンの主体を識別
    /// 通常はユーザーIDを格納
    sub: Option<String>,

    /// Expiration Time（有効期限）: Unix タイムスタンプ（秒）
    /// この時刻を過ぎるとトークンは無効
    exp: Option<u64>,

    /// Issued At（発行時刻）: Unix タイムスタンプ（秒）
    /// トークンがいつ発行されたかを示す
    /// 検証には使用しないため、dead_code 警告を抑制
    #[allow(dead_code)]
    iat: Option<u64>,
}

// =============================================================================
// WIT インターフェースの実装
// =============================================================================

/// 認証コンポーネントの実装構造体
///
/// WIT の Guest トレイトを実装することで、
/// 他のコンポーネントから呼び出し可能な関数を公開します。
struct AuthComponent;

/// Guest トレイトの実装
///
/// WIT で定義された authenticator インターフェースの関数を実装します。
impl Guest for AuthComponent {
    /// JWT トークンを検証し、認証結果を返す
    ///
    /// # 引数
    /// * `token` - 検証対象の JWT トークン文字列
    ///
    /// # 戻り値
    /// * `AuthResult` - 認証結果
    ///   - 成功時: authenticated=true, user_id=Some(ユーザーID)
    ///   - 失敗時: authenticated=false, error=Some(エラーメッセージ)
    fn verify_token(token: String) -> AuthResult {
        // verify_jwt 関数でトークンを検証
        match verify_jwt(&token) {
            // 検証成功: ユーザーIDを含む成功レスポンスを返す
            Ok(user_id) => AuthResult {
                authenticated: true,
                user_id: Some(user_id),
                error: None,
            },
            // 検証失敗: エラーメッセージを含む失敗レスポンスを返す
            Err(e) => AuthResult {
                authenticated: false,
                user_id: None,
                error: Some(e),
            },
        }
    }
}

// =============================================================================
// JWT 検証ロジック
// =============================================================================

/// JWT トークンを検証し、ユーザーIDを抽出する
///
/// JWT の構造: ヘッダー.ペイロード.署名（すべて Base64URL エンコード）
///
/// # 検証手順
/// 1. トークンが空でないことを確認
/// 2. ドットで3つの部分に分割できることを確認
/// 3. ヘッダーをデコードし、アルゴリズムが HS256 であることを確認
/// 4. HMAC-SHA256 で署名を検証
/// 5. ペイロードをデコードし、有効期限を確認
/// 6. ユーザーID（sub クレーム）を抽出
///
/// # 引数
/// * `token` - 検証対象の JWT トークン文字列
///
/// # 戻り値
/// * `Ok(String)` - 検証成功時、ユーザーIDを返す
/// * `Err(String)` - 検証失敗時、エラーメッセージを返す
fn verify_jwt(token: &str) -> Result<String, String> {
    // --------------------------------------------------------
    // Step 1: 空トークンのチェック
    // --------------------------------------------------------
    // Authorization ヘッダーがない場合、空文字列が渡される
    if token.is_empty() {
        return Err("Missing token".to_string());
    }

    // --------------------------------------------------------
    // Step 2: JWT フォーマットの検証
    // --------------------------------------------------------
    // JWT は必ず3つの部分（ヘッダー、ペイロード、署名）で構成される
    // 各部分はドット（.）で区切られている
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return Err("Invalid token format".to_string());
    }

    // 各部分を変数に格納
    let header_b64 = parts[0]; // Base64 エンコードされたヘッダー
    let payload_b64 = parts[1]; // Base64 エンコードされたペイロード
    let signature_b64 = parts[2]; // Base64 エンコードされた署名

    // --------------------------------------------------------
    // Step 3: ヘッダーのデコードと検証
    // --------------------------------------------------------
    // Base64 をデコードして JSON 文字列を取得
    let header_json = URL_SAFE_NO_PAD
        .decode(header_b64)
        .map_err(|_| "Invalid header encoding".to_string())?;

    // JSON をパースして JwtHeader 構造体に変換
    let header: JwtHeader =
        serde_json::from_slice(&header_json).map_err(|_| "Invalid header JSON".to_string())?;

    // アルゴリズムが HS256 であることを確認
    // セキュリティ上、サポートするアルゴリズムを明示的に制限
    if header.alg != "HS256" {
        return Err(format!("Unsupported algorithm: {}", header.alg));
    }

    // --------------------------------------------------------
    // Step 4: 署名の検証
    // --------------------------------------------------------
    // Base64 エンコードされた署名をデコード
    let signature = URL_SAFE_NO_PAD
        .decode(signature_b64)
        .map_err(|_| "Invalid signature encoding".to_string())?;

    // 署名対象のメッセージを構築（ヘッダー.ペイロード）
    // 署名はヘッダーとペイロードの Base64 文字列に対して行われる
    let message = format!("{}.{}", header_b64, payload_b64);

    // HMAC-SHA256 で署名を検証
    verify_signature(&message, &signature)?;

    // --------------------------------------------------------
    // Step 5: ペイロードのデコード
    // --------------------------------------------------------
    // Base64 をデコードして JSON 文字列を取得
    let payload_json = URL_SAFE_NO_PAD
        .decode(payload_b64)
        .map_err(|_| "Invalid payload encoding".to_string())?;

    // JSON をパースして JwtPayload 構造体に変換
    let payload: JwtPayload =
        serde_json::from_slice(&payload_json).map_err(|_| "Invalid payload JSON".to_string())?;

    // --------------------------------------------------------
    // Step 6: 有効期限のチェック（簡易実装）
    // --------------------------------------------------------
    // 注意: Wasm 環境では現在時刻の取得に制限があります。
    // 本番環境では WASI の clock_time_get を使用して正確に検証してください。
    // ここでは簡易的なチェックのみ実施。
    if let Some(exp) = payload.exp {
        // 2020年1月1日 00:00:00 UTC = 1577836800 秒
        // この値より小さい exp は明らかに期限切れ
        if exp < 1577836800 {
            return Err("Token expired".to_string());
        }
    }

    // --------------------------------------------------------
    // Step 7: ユーザーIDの抽出
    // --------------------------------------------------------
    // sub（Subject）クレームからユーザーIDを取得
    // sub がない場合はエラー
    payload
        .sub
        .ok_or_else(|| "Missing subject claim".to_string())
}

/// HMAC-SHA256 で署名を検証する
///
/// # 引数
/// * `message` - 署名対象のメッセージ（ヘッダー.ペイロード）
/// * `signature` - 検証する署名（バイト列）
///
/// # 戻り値
/// * `Ok(())` - 署名が正しい場合
/// * `Err(String)` - 署名が不正な場合
fn verify_signature(message: &str, signature: &[u8]) -> Result<(), String> {
    // HMAC-SHA256 の型エイリアスを定義
    type HmacSha256 = Hmac<Sha256>;

    // 秘密鍵で HMAC インスタンスを初期化
    // new_from_slice: 任意長のスライスから HMAC を作成
    let mut mac =
        HmacSha256::new_from_slice(SECRET_KEY).map_err(|_| "Invalid key length".to_string())?;

    // メッセージを HMAC に入力
    // update: データを追加（複数回呼び出し可能）
    mac.update(message.as_bytes());

    // 署名を検証
    // verify_slice: 期待される署名と比較し、一致しなければエラー
    // タイミング攻撃を防ぐため、定数時間で比較を行う
    mac.verify_slice(signature)
        .map_err(|_| "Invalid signature".to_string())
}

// =============================================================================
// コンポーネントのエクスポート
// =============================================================================

// AuthComponent を Wasm コンポーネントとしてエクスポート
// これにより、WIT で定義したインターフェースが外部から呼び出し可能になる
export!(AuthComponent);
