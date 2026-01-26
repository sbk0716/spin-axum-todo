#![feature(prelude_import)]
//! # ゲートウェイコンポーネント
//!
//! このコンポーネントは HTTP リクエストのエントリーポイントとして機能し、
//! 以下の責務を担います：
//!
//! ## 責務
//! 1. HTTP リクエストの受信
//! 2. auth コンポーネントを呼び出して JWT 認証を実行
//! 3. 認証成功時、コア層（axum）へリクエストをプロキシ
//! 4. 認証失敗時、401 Unauthorized レスポンスを返却
//!
//! ## アーキテクチャ
//! ```text
//! クライアント → [gateway] → [auth] (WIT)
//!                    ↓
//!              [コア層 axum]
//! ```
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2021::*;
use serde::Serialize;
use uuid::Uuid;
use spin_sdk::http::{IntoResponse, Method, Request, Response};
use spin_sdk::http_component;
#[allow(dead_code, clippy::all)]
pub mod demo {
    pub mod auth {
        /// =============================================================================
        /// authenticator インターフェース
        /// =============================================================================
        ///
        /// JWT 認証機能を提供するインターフェースです。
        /// auth コンポーネントがこのインターフェースをエクスポート（実装）し、
        /// gateway コンポーネントがインポート（使用）します。
        /// 認証機能を提供するインターフェース
        #[allow(dead_code, async_fn_in_trait, unused_imports, clippy::all)]
        pub mod authenticator {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            /// -------------------------------------------------------------------------
            /// レコード型の定義
            /// -------------------------------------------------------------------------
            ///
            /// record は Rust の struct、TypeScript の interface に相当します。
            /// フィールドはケバブケース（kebab-case）で記述し、
            /// 生成されるコードではスネークケース（snake_case）に変換されます。
            /// JWT 検証の結果を表すレコード型
            ///
            /// 認証の成功/失敗を示し、成功時はユーザーID、失敗時はエラーメッセージを含みます。
            pub struct AuthResult {
                /// 認証が成功したかどうか
                /// - true: JWT が有効で認証成功
                /// - false: JWT が無効または期限切れで認証失敗
                pub authenticated: bool,
                /// 認証成功時のユーザーID
                /// JWT の sub (Subject) クレームから抽出した値
                /// 認証失敗時は None (null)
                pub user_id: Option<_rt::String>,
                /// 認証失敗時のエラーメッセージ
                /// 例: "Missing token", "Invalid signature", "Token expired"
                /// 認証成功時は None (null)
                pub error: Option<_rt::String>,
            }
            #[automatically_derived]
            impl ::core::clone::Clone for AuthResult {
                #[inline]
                fn clone(&self) -> AuthResult {
                    AuthResult {
                        authenticated: ::core::clone::Clone::clone(&self.authenticated),
                        user_id: ::core::clone::Clone::clone(&self.user_id),
                        error: ::core::clone::Clone::clone(&self.error),
                    }
                }
            }
            impl ::core::fmt::Debug for AuthResult {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("AuthResult")
                        .field("authenticated", &self.authenticated)
                        .field("user-id", &self.user_id)
                        .field("error", &self.error)
                        .finish()
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// -------------------------------------------------------------------------
            /// 関数の定義
            /// -------------------------------------------------------------------------
            ///
            /// func キーワードで関数を定義します。
            /// 引数と戻り値の型を指定し、実装は各コンポーネントで行います。
            /// JWT トークンを検証する
            ///
            /// Authorization ヘッダーから取得した Bearer トークンを検証し、
            /// 認証結果を返します。
            ///
            /// # 引数
            /// * `token` - 検証対象の JWT トークン文字列
            ///   - 空文字列の場合、"Missing token" エラー
            ///   - 不正な形式の場合、"Invalid token format" エラー
            ///
            /// # 戻り値
            /// * `auth-result` - 認証結果
            ///   - 成功時: authenticated=true, user-id=Some(ユーザーID)
            ///   - 失敗時: authenticated=false, error=Some(エラーメッセージ)
            ///
            /// # 検証内容
            /// 1. トークンが空でないこと
            /// 2. JWT フォーマット（header.payload.signature）であること
            /// 3. アルゴリズムが HS256 であること
            /// 4. HMAC-SHA256 署名が正しいこと
            /// 5. sub クレームが存在すること
            #[allow(async_fn_in_trait)]
            pub fn verify_token(token: &str) -> AuthResult {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea(
                        [::core::mem::MaybeUninit<
                            u8,
                        >; 7 * ::core::mem::size_of::<*const u8>()],
                    );
                    let mut ret_area = RetArea(
                        [::core::mem::MaybeUninit::uninit(); 7
                            * ::core::mem::size_of::<*const u8>()],
                    );
                    let vec0 = token;
                    let ptr0 = vec0.as_ptr().cast::<u8>();
                    let len0 = vec0.len();
                    let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[link(wasm_import_module = "demo:auth/authenticator")]
                    unsafe extern "C" {
                        #[link_name = "verify-token"]
                        fn wit_import2(_: *mut u8, _: usize, _: *mut u8);
                    }
                    wit_import2(ptr0.cast_mut(), len0, ptr1);
                    let l3 = i32::from(*ptr1.add(0).cast::<u8>());
                    let l4 = i32::from(
                        *ptr1.add(::core::mem::size_of::<*const u8>()).cast::<u8>(),
                    );
                    let l8 = i32::from(
                        *ptr1.add(4 * ::core::mem::size_of::<*const u8>()).cast::<u8>(),
                    );
                    let result12 = AuthResult {
                        authenticated: _rt::bool_lift(l3 as u8),
                        user_id: match l4 {
                            0 => None,
                            1 => {
                                let e = {
                                    let l5 = *ptr1
                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                        .cast::<*mut u8>();
                                    let l6 = *ptr1
                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                        .cast::<usize>();
                                    let len7 = l6;
                                    let bytes7 = _rt::Vec::from_raw_parts(
                                        l5.cast(),
                                        len7,
                                        len7,
                                    );
                                    _rt::string_lift(bytes7)
                                };
                                Some(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        },
                        error: match l8 {
                            0 => None,
                            1 => {
                                let e = {
                                    let l9 = *ptr1
                                        .add(5 * ::core::mem::size_of::<*const u8>())
                                        .cast::<*mut u8>();
                                    let l10 = *ptr1
                                        .add(6 * ::core::mem::size_of::<*const u8>())
                                        .cast::<usize>();
                                    let len11 = l10;
                                    let bytes11 = _rt::Vec::from_raw_parts(
                                        l9.cast(),
                                        len11,
                                        len11,
                                    );
                                    _rt::string_lift(bytes11)
                                };
                                Some(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        },
                    };
                    result12
                }
            }
        }
    }
}
mod _rt {
    #![allow(dead_code, unused_imports, clippy::all)]
    pub use alloc_crate::string::String;
    pub unsafe fn bool_lift(val: u8) -> bool {
        if true {
            match val {
                0 => false,
                1 => true,
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("invalid bool discriminant"),
                    );
                }
            }
        } else {
            val != 0
        }
    }
    pub use alloc_crate::vec::Vec;
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if true {
            String::from_utf8(bytes).unwrap()
        } else {
            unsafe { String::from_utf8_unchecked(bytes) }
        }
    }
    pub unsafe fn invalid_enum_discriminant<T>() -> T {
        if true {
            {
                ::core::panicking::panic_fmt(format_args!("invalid enum discriminant"));
            }
        } else {
            unsafe { core::hint::unreachable_unchecked() }
        }
    }
    extern crate alloc as alloc_crate;
}
#[unsafe(
    link_section = "component-type:wit-bindgen:0.51.0:demo:auth:gateway-world:encoded world"
)]
#[doc(hidden)]
#[allow(clippy::octal_escapes)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 280] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\x94\x01\x01A\x02\x01\
A\x02\x01B\x05\x01ks\x01r\x03\x0dauthenticated\x7f\x07user-id\0\x05error\0\x04\0\
\x0bauth-result\x03\0\x01\x01@\x01\x05tokens\0\x02\x04\0\x0cverify-token\x01\x03\
\x03\0\x17demo:auth/authenticator\x05\0\x04\0\x17demo:auth/gateway-world\x04\0\x0b\
\x13\x01\0\x0dgateway-world\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dw\
it-component\x070.244.0\x10wit-bindgen-rust\x060.51.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen::rt::maybe_link_cabi_realloc();
}
const _: &[u8] = b"// =============================================================================\n// WIT (WebAssembly Interface Types) \xe3\x82\xa4\xe3\x83\xb3\xe3\x82\xbf\xe3\x83\xbc\xe3\x83\x95\xe3\x82\xa7\xe3\x83\xbc\xe3\x82\xb9\xe5\xae\x9a\xe7\xbe\xa9\n// =============================================================================\n//\n// \xe3\x81\x93\xe3\x81\xae\xe3\x83\x95\xe3\x82\xa1\xe3\x82\xa4\xe3\x83\xab\xe3\x81\xaf Wasm \xe3\x82\xb3\xe3\x83\xb3\xe3\x83\x9d\xe3\x83\xbc\xe3\x83\x8d\xe3\x83\xb3\xe3\x83\x88\xe9\x96\x93\xe3\x81\xae\xe3\x82\xa4\xe3\x83\xb3\xe3\x82\xbf\xe3\x83\xbc\xe3\x83\x95\xe3\x82\xa7\xe3\x83\xbc\xe3\x82\xb9\xe3\x82\x92\xe5\xae\x9a\xe7\xbe\xa9\xe3\x81\x97\xe3\x81\xbe\xe3\x81\x99\xe3\x80\x82\n// WIT \xe3\x81\xaf Wasm \xe3\x82\xb3\xe3\x83\xb3\xe3\x83\x9d\xe3\x83\xbc\xe3\x83\x8d\xe3\x83\xb3\xe3\x83\x88\xe3\x83\xa2\xe3\x83\x87\xe3\x83\xab\xe3\x81\xae\xe4\xb8\x80\xe9\x83\xa8\xe3\x81\xa7\xe3\x80\x81\xe5\x9e\x8b\xe5\xae\x89\xe5\x85\xa8\xe3\x81\xaa\xe3\x82\xb3\xe3\x83\xb3\xe3\x83\x9d\xe3\x83\xbc\xe3\x83\x8d\xe3\x83\xb3\xe3\x83\x88\xe9\x96\x93\xe9\x80\x9a\xe4\xbf\xa1\xe3\x82\x92\xe5\x8f\xaf\xe8\x83\xbd\xe3\x81\xab\xe3\x81\x97\xe3\x81\xbe\xe3\x81\x99\xe3\x80\x82\n//\n// \xe5\x8f\x82\xe7\x85\xa7:\n// - WIT \xe4\xbb\x95\xe6\xa7\x98: https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md\n// - \xe3\x82\xb3\xe3\x83\xb3\xe3\x83\x9d\xe3\x83\xbc\xe3\x83\x8d\xe3\x83\xb3\xe3\x83\x88\xe3\x83\xa2\xe3\x83\x87\xe3\x83\xab: https://component-model.bytecodealliance.org/\n//\n// =============================================================================\n\n// -----------------------------------------------------------------------------\n// \xe3\x83\x91\xe3\x83\x83\xe3\x82\xb1\xe3\x83\xbc\xe3\x82\xb8\xe5\xae\xa3\xe8\xa8\x80\n// -----------------------------------------------------------------------------\n// \xe3\x83\x91\xe3\x83\x83\xe3\x82\xb1\xe3\x83\xbc\xe3\x82\xb8\xe5\x90\x8d\xe3\x81\xaf \"namespace:package\" \xe3\x81\xae\xe5\xbd\xa2\xe5\xbc\x8f\xe3\x81\xa7\xe6\x8c\x87\xe5\xae\x9a\n// - namespace: \xe7\xb5\x84\xe7\xb9\x94\xe3\x82\x84\xe3\x83\x97\xe3\x83\xad\xe3\x82\xb8\xe3\x82\xa7\xe3\x82\xaf\xe3\x83\x88\xe3\x82\x92\xe8\xad\x98\xe5\x88\xa5\xef\xbc\x88\xe4\xbe\x8b: wasi, demo\xef\xbc\x89\n// - package: \xe3\x83\x91\xe3\x83\x83\xe3\x82\xb1\xe3\x83\xbc\xe3\x82\xb8\xe5\x90\x8d\xef\xbc\x88\xe4\xbe\x8b: auth, http\xef\xbc\x89\npackage demo:auth;\n\n// =============================================================================\n// authenticator \xe3\x82\xa4\xe3\x83\xb3\xe3\x82\xbf\xe3\x83\xbc\xe3\x83\x95\xe3\x82\xa7\xe3\x83\xbc\xe3\x82\xb9\n// =============================================================================\n//\n// JWT \xe8\xaa\x8d\xe8\xa8\xbc\xe6\xa9\x9f\xe8\x83\xbd\xe3\x82\x92\xe6\x8f\x90\xe4\xbe\x9b\xe3\x81\x99\xe3\x82\x8b\xe3\x82\xa4\xe3\x83\xb3\xe3\x82\xbf\xe3\x83\xbc\xe3\x83\x95\xe3\x82\xa7\xe3\x83\xbc\xe3\x82\xb9\xe3\x81\xa7\xe3\x81\x99\xe3\x80\x82\n// auth \xe3\x82\xb3\xe3\x83\xb3\xe3\x83\x9d\xe3\x83\xbc\xe3\x83\x8d\xe3\x83\xb3\xe3\x83\x88\xe3\x81\x8c\xe3\x81\x93\xe3\x81\xae\xe3\x82\xa4\xe3\x83\xb3\xe3\x82\xbf\xe3\x83\xbc\xe3\x83\x95\xe3\x82\xa7\xe3\x83\xbc\xe3\x82\xb9\xe3\x82\x92\xe3\x82\xa8\xe3\x82\xaf\xe3\x82\xb9\xe3\x83\x9d\xe3\x83\xbc\xe3\x83\x88\xef\xbc\x88\xe5\xae\x9f\xe8\xa3\x85\xef\xbc\x89\xe3\x81\x97\xe3\x80\x81\n// gateway \xe3\x82\xb3\xe3\x83\xb3\xe3\x83\x9d\xe3\x83\xbc\xe3\x83\x8d\xe3\x83\xb3\xe3\x83\x88\xe3\x81\x8c\xe3\x82\xa4\xe3\x83\xb3\xe3\x83\x9d\xe3\x83\xbc\xe3\x83\x88\xef\xbc\x88\xe4\xbd\xbf\xe7\x94\xa8\xef\xbc\x89\xe3\x81\x97\xe3\x81\xbe\xe3\x81\x99\xe3\x80\x82\n\n/// \xe8\xaa\x8d\xe8\xa8\xbc\xe6\xa9\x9f\xe8\x83\xbd\xe3\x82\x92\xe6\x8f\x90\xe4\xbe\x9b\xe3\x81\x99\xe3\x82\x8b\xe3\x82\xa4\xe3\x83\xb3\xe3\x82\xbf\xe3\x83\xbc\xe3\x83\x95\xe3\x82\xa7\xe3\x83\xbc\xe3\x82\xb9\ninterface authenticator {\n    // -------------------------------------------------------------------------\n    // \xe3\x83\xac\xe3\x82\xb3\xe3\x83\xbc\xe3\x83\x89\xe5\x9e\x8b\xe3\x81\xae\xe5\xae\x9a\xe7\xbe\xa9\n    // -------------------------------------------------------------------------\n    //\n    // record \xe3\x81\xaf Rust \xe3\x81\xae struct\xe3\x80\x81TypeScript \xe3\x81\xae interface \xe3\x81\xab\xe7\x9b\xb8\xe5\xbd\x93\xe3\x81\x97\xe3\x81\xbe\xe3\x81\x99\xe3\x80\x82\n    // \xe3\x83\x95\xe3\x82\xa3\xe3\x83\xbc\xe3\x83\xab\xe3\x83\x89\xe3\x81\xaf\xe3\x82\xb1\xe3\x83\x90\xe3\x83\x96\xe3\x82\xb1\xe3\x83\xbc\xe3\x82\xb9\xef\xbc\x88kebab-case\xef\xbc\x89\xe3\x81\xa7\xe8\xa8\x98\xe8\xbf\xb0\xe3\x81\x97\xe3\x80\x81\n    // \xe7\x94\x9f\xe6\x88\x90\xe3\x81\x95\xe3\x82\x8c\xe3\x82\x8b\xe3\x82\xb3\xe3\x83\xbc\xe3\x83\x89\xe3\x81\xa7\xe3\x81\xaf\xe3\x82\xb9\xe3\x83\x8d\xe3\x83\xbc\xe3\x82\xaf\xe3\x82\xb1\xe3\x83\xbc\xe3\x82\xb9\xef\xbc\x88snake_case\xef\xbc\x89\xe3\x81\xab\xe5\xa4\x89\xe6\x8f\x9b\xe3\x81\x95\xe3\x82\x8c\xe3\x81\xbe\xe3\x81\x99\xe3\x80\x82\n\n    /// JWT \xe6\xa4\x9c\xe8\xa8\xbc\xe3\x81\xae\xe7\xb5\x90\xe6\x9e\x9c\xe3\x82\x92\xe8\xa1\xa8\xe3\x81\x99\xe3\x83\xac\xe3\x82\xb3\xe3\x83\xbc\xe3\x83\x89\xe5\x9e\x8b\n    ///\n    /// \xe8\xaa\x8d\xe8\xa8\xbc\xe3\x81\xae\xe6\x88\x90\xe5\x8a\x9f/\xe5\xa4\xb1\xe6\x95\x97\xe3\x82\x92\xe7\xa4\xba\xe3\x81\x97\xe3\x80\x81\xe6\x88\x90\xe5\x8a\x9f\xe6\x99\x82\xe3\x81\xaf\xe3\x83\xa6\xe3\x83\xbc\xe3\x82\xb6\xe3\x83\xbcID\xe3\x80\x81\xe5\xa4\xb1\xe6\x95\x97\xe6\x99\x82\xe3\x81\xaf\xe3\x82\xa8\xe3\x83\xa9\xe3\x83\xbc\xe3\x83\xa1\xe3\x83\x83\xe3\x82\xbb\xe3\x83\xbc\xe3\x82\xb8\xe3\x82\x92\xe5\x90\xab\xe3\x81\xbf\xe3\x81\xbe\xe3\x81\x99\xe3\x80\x82\n    record auth-result {\n        /// \xe8\xaa\x8d\xe8\xa8\xbc\xe3\x81\x8c\xe6\x88\x90\xe5\x8a\x9f\xe3\x81\x97\xe3\x81\x9f\xe3\x81\x8b\xe3\x81\xa9\xe3\x81\x86\xe3\x81\x8b\n        /// - true: JWT \xe3\x81\x8c\xe6\x9c\x89\xe5\x8a\xb9\xe3\x81\xa7\xe8\xaa\x8d\xe8\xa8\xbc\xe6\x88\x90\xe5\x8a\x9f\n        /// - false: JWT \xe3\x81\x8c\xe7\x84\xa1\xe5\x8a\xb9\xe3\x81\xbe\xe3\x81\x9f\xe3\x81\xaf\xe6\x9c\x9f\xe9\x99\x90\xe5\x88\x87\xe3\x82\x8c\xe3\x81\xa7\xe8\xaa\x8d\xe8\xa8\xbc\xe5\xa4\xb1\xe6\x95\x97\n        authenticated: bool,\n\n        /// \xe8\xaa\x8d\xe8\xa8\xbc\xe6\x88\x90\xe5\x8a\x9f\xe6\x99\x82\xe3\x81\xae\xe3\x83\xa6\xe3\x83\xbc\xe3\x82\xb6\xe3\x83\xbcID\n        /// JWT \xe3\x81\xae sub (Subject) \xe3\x82\xaf\xe3\x83\xac\xe3\x83\xbc\xe3\x83\xa0\xe3\x81\x8b\xe3\x82\x89\xe6\x8a\xbd\xe5\x87\xba\xe3\x81\x97\xe3\x81\x9f\xe5\x80\xa4\n        /// \xe8\xaa\x8d\xe8\xa8\xbc\xe5\xa4\xb1\xe6\x95\x97\xe6\x99\x82\xe3\x81\xaf None (null)\n        user-id: option<string>,\n\n        /// \xe8\xaa\x8d\xe8\xa8\xbc\xe5\xa4\xb1\xe6\x95\x97\xe6\x99\x82\xe3\x81\xae\xe3\x82\xa8\xe3\x83\xa9\xe3\x83\xbc\xe3\x83\xa1\xe3\x83\x83\xe3\x82\xbb\xe3\x83\xbc\xe3\x82\xb8\n        /// \xe4\xbe\x8b: \"Missing token\", \"Invalid signature\", \"Token expired\"\n        /// \xe8\xaa\x8d\xe8\xa8\xbc\xe6\x88\x90\xe5\x8a\x9f\xe6\x99\x82\xe3\x81\xaf None (null)\n        error: option<string>,\n    }\n\n    // -------------------------------------------------------------------------\n    // \xe9\x96\xa2\xe6\x95\xb0\xe3\x81\xae\xe5\xae\x9a\xe7\xbe\xa9\n    // -------------------------------------------------------------------------\n    //\n    // func \xe3\x82\xad\xe3\x83\xbc\xe3\x83\xaf\xe3\x83\xbc\xe3\x83\x89\xe3\x81\xa7\xe9\x96\xa2\xe6\x95\xb0\xe3\x82\x92\xe5\xae\x9a\xe7\xbe\xa9\xe3\x81\x97\xe3\x81\xbe\xe3\x81\x99\xe3\x80\x82\n    // \xe5\xbc\x95\xe6\x95\xb0\xe3\x81\xa8\xe6\x88\xbb\xe3\x82\x8a\xe5\x80\xa4\xe3\x81\xae\xe5\x9e\x8b\xe3\x82\x92\xe6\x8c\x87\xe5\xae\x9a\xe3\x81\x97\xe3\x80\x81\xe5\xae\x9f\xe8\xa3\x85\xe3\x81\xaf\xe5\x90\x84\xe3\x82\xb3\xe3\x83\xb3\xe3\x83\x9d\xe3\x83\xbc\xe3\x83\x8d\xe3\x83\xb3\xe3\x83\x88\xe3\x81\xa7\xe8\xa1\x8c\xe3\x81\x84\xe3\x81\xbe\xe3\x81\x99\xe3\x80\x82\n\n    /// JWT \xe3\x83\x88\xe3\x83\xbc\xe3\x82\xaf\xe3\x83\xb3\xe3\x82\x92\xe6\xa4\x9c\xe8\xa8\xbc\xe3\x81\x99\xe3\x82\x8b\n    ///\n    /// Authorization \xe3\x83\x98\xe3\x83\x83\xe3\x83\x80\xe3\x83\xbc\xe3\x81\x8b\xe3\x82\x89\xe5\x8f\x96\xe5\xbe\x97\xe3\x81\x97\xe3\x81\x9f Bearer \xe3\x83\x88\xe3\x83\xbc\xe3\x82\xaf\xe3\x83\xb3\xe3\x82\x92\xe6\xa4\x9c\xe8\xa8\xbc\xe3\x81\x97\xe3\x80\x81\n    /// \xe8\xaa\x8d\xe8\xa8\xbc\xe7\xb5\x90\xe6\x9e\x9c\xe3\x82\x92\xe8\xbf\x94\xe3\x81\x97\xe3\x81\xbe\xe3\x81\x99\xe3\x80\x82\n    ///\n    /// # \xe5\xbc\x95\xe6\x95\xb0\n    /// * `token` - \xe6\xa4\x9c\xe8\xa8\xbc\xe5\xaf\xbe\xe8\xb1\xa1\xe3\x81\xae JWT \xe3\x83\x88\xe3\x83\xbc\xe3\x82\xaf\xe3\x83\xb3\xe6\x96\x87\xe5\xad\x97\xe5\x88\x97\n    ///   - \xe7\xa9\xba\xe6\x96\x87\xe5\xad\x97\xe5\x88\x97\xe3\x81\xae\xe5\xa0\xb4\xe5\x90\x88\xe3\x80\x81\"Missing token\" \xe3\x82\xa8\xe3\x83\xa9\xe3\x83\xbc\n    ///   - \xe4\xb8\x8d\xe6\xad\xa3\xe3\x81\xaa\xe5\xbd\xa2\xe5\xbc\x8f\xe3\x81\xae\xe5\xa0\xb4\xe5\x90\x88\xe3\x80\x81\"Invalid token format\" \xe3\x82\xa8\xe3\x83\xa9\xe3\x83\xbc\n    ///\n    /// # \xe6\x88\xbb\xe3\x82\x8a\xe5\x80\xa4\n    /// * `auth-result` - \xe8\xaa\x8d\xe8\xa8\xbc\xe7\xb5\x90\xe6\x9e\x9c\n    ///   - \xe6\x88\x90\xe5\x8a\x9f\xe6\x99\x82: authenticated=true, user-id=Some(\xe3\x83\xa6\xe3\x83\xbc\xe3\x82\xb6\xe3\x83\xbcID)\n    ///   - \xe5\xa4\xb1\xe6\x95\x97\xe6\x99\x82: authenticated=false, error=Some(\xe3\x82\xa8\xe3\x83\xa9\xe3\x83\xbc\xe3\x83\xa1\xe3\x83\x83\xe3\x82\xbb\xe3\x83\xbc\xe3\x82\xb8)\n    ///\n    /// # \xe6\xa4\x9c\xe8\xa8\xbc\xe5\x86\x85\xe5\xae\xb9\n    /// 1. \xe3\x83\x88\xe3\x83\xbc\xe3\x82\xaf\xe3\x83\xb3\xe3\x81\x8c\xe7\xa9\xba\xe3\x81\xa7\xe3\x81\xaa\xe3\x81\x84\xe3\x81\x93\xe3\x81\xa8\n    /// 2. JWT \xe3\x83\x95\xe3\x82\xa9\xe3\x83\xbc\xe3\x83\x9e\xe3\x83\x83\xe3\x83\x88\xef\xbc\x88header.payload.signature\xef\xbc\x89\xe3\x81\xa7\xe3\x81\x82\xe3\x82\x8b\xe3\x81\x93\xe3\x81\xa8\n    /// 3. \xe3\x82\xa2\xe3\x83\xab\xe3\x82\xb4\xe3\x83\xaa\xe3\x82\xba\xe3\x83\xa0\xe3\x81\x8c HS256 \xe3\x81\xa7\xe3\x81\x82\xe3\x82\x8b\xe3\x81\x93\xe3\x81\xa8\n    /// 4. HMAC-SHA256 \xe7\xbd\xb2\xe5\x90\x8d\xe3\x81\x8c\xe6\xad\xa3\xe3\x81\x97\xe3\x81\x84\xe3\x81\x93\xe3\x81\xa8\n    /// 5. sub \xe3\x82\xaf\xe3\x83\xac\xe3\x83\xbc\xe3\x83\xa0\xe3\x81\x8c\xe5\xad\x98\xe5\x9c\xa8\xe3\x81\x99\xe3\x82\x8b\xe3\x81\x93\xe3\x81\xa8\n    verify-token: func(token: string) -> auth-result;\n}\n\n// =============================================================================\n// World \xe5\xae\x9a\xe7\xbe\xa9\n// =============================================================================\n//\n// World \xe3\x81\xaf\xe3\x82\xb3\xe3\x83\xb3\xe3\x83\x9d\xe3\x83\xbc\xe3\x83\x8d\xe3\x83\xb3\xe3\x83\x88\xe3\x81\xae\xe3\x80\x8c\xe9\xa1\x94\xe3\x80\x8d\xe3\x82\x92\xe5\xae\x9a\xe7\xbe\xa9\xe3\x81\x97\xe3\x81\xbe\xe3\x81\x99\xe3\x80\x82\n// - export: \xe3\x82\xb3\xe3\x83\xb3\xe3\x83\x9d\xe3\x83\xbc\xe3\x83\x8d\xe3\x83\xb3\xe3\x83\x88\xe3\x81\x8c\xe5\xa4\x96\xe9\x83\xa8\xe3\x81\xab\xe6\x8f\x90\xe4\xbe\x9b\xe3\x81\x99\xe3\x82\x8b\xe3\x82\xa4\xe3\x83\xb3\xe3\x82\xbf\xe3\x83\xbc\xe3\x83\x95\xe3\x82\xa7\xe3\x83\xbc\xe3\x82\xb9\n// - import: \xe3\x82\xb3\xe3\x83\xb3\xe3\x83\x9d\xe3\x83\xbc\xe3\x83\x8d\xe3\x83\xb3\xe3\x83\x88\xe3\x81\x8c\xe5\xa4\x96\xe9\x83\xa8\xe3\x81\x8b\xe3\x82\x89\xe4\xbd\xbf\xe7\x94\xa8\xe3\x81\x99\xe3\x82\x8b\xe3\x82\xa4\xe3\x83\xb3\xe3\x82\xbf\xe3\x83\xbc\xe3\x83\x95\xe3\x82\xa7\xe3\x83\xbc\xe3\x82\xb9\n//\n// \xe5\x90\x84 Wasm \xe3\x82\xb3\xe3\x83\xb3\xe3\x83\x9d\xe3\x83\xbc\xe3\x83\x8d\xe3\x83\xb3\xe3\x83\x88\xe3\x81\xaf 1 \xe3\x81\xa4\xe3\x81\xae world \xe3\x82\x92\xe5\xae\x9f\xe8\xa3\x85\xe3\x81\x97\xe3\x81\xbe\xe3\x81\x99\xe3\x80\x82\n\n/// auth \xe3\x82\xb3\xe3\x83\xb3\xe3\x83\x9d\xe3\x83\xbc\xe3\x83\x8d\xe3\x83\xb3\xe3\x83\x88\xe3\x81\x8c\xe5\xae\x9f\xe8\xa3\x85\xe3\x81\x99\xe3\x82\x8b world\n///\n/// \xe3\x81\x93\xe3\x81\xae\xe3\x82\xb3\xe3\x83\xb3\xe3\x83\x9d\xe3\x83\xbc\xe3\x83\x8d\xe3\x83\xb3\xe3\x83\x88\xe3\x81\xaf authenticator \xe3\x82\xa4\xe3\x83\xb3\xe3\x82\xbf\xe3\x83\xbc\xe3\x83\x95\xe3\x82\xa7\xe3\x83\xbc\xe3\x82\xb9\xe3\x82\x92\xe3\x82\xa8\xe3\x82\xaf\xe3\x82\xb9\xe3\x83\x9d\xe3\x83\xbc\xe3\x83\x88\xe3\x81\x97\xe3\x80\x81\n/// JWT \xe6\xa4\x9c\xe8\xa8\xbc\xe6\xa9\x9f\xe8\x83\xbd\xe3\x82\x92\xe5\xa4\x96\xe9\x83\xa8\xe3\x81\xab\xe6\x8f\x90\xe4\xbe\x9b\xe3\x81\x97\xe3\x81\xbe\xe3\x81\x99\xe3\x80\x82\n///\n/// \xe4\xbd\xbf\xe7\x94\xa8\xe4\xbe\x8b:\n/// - gateway \xe3\x82\xb3\xe3\x83\xb3\xe3\x83\x9d\xe3\x83\xbc\xe3\x83\x8d\xe3\x83\xb3\xe3\x83\x88\xe3\x81\x8b\xe3\x82\x89 verify_token \xe9\x96\xa2\xe6\x95\xb0\xe3\x81\x8c\xe5\x91\xbc\xe3\x81\xb3\xe5\x87\xba\xe3\x81\x95\xe3\x82\x8c\xe3\x82\x8b\nworld auth-world {\n    // authenticator \xe3\x82\xa4\xe3\x83\xb3\xe3\x82\xbf\xe3\x83\xbc\xe3\x83\x95\xe3\x82\xa7\xe3\x83\xbc\xe3\x82\xb9\xe3\x82\x92\xe5\xa4\x96\xe9\x83\xa8\xe3\x81\xab\xe5\x85\xac\xe9\x96\x8b\n    // \xe3\x81\x93\xe3\x82\x8c\xe3\x81\xab\xe3\x82\x88\xe3\x82\x8a\xe3\x80\x81\xe4\xbb\x96\xe3\x81\xae\xe3\x82\xb3\xe3\x83\xb3\xe3\x83\x9d\xe3\x83\xbc\xe3\x83\x8d\xe3\x83\xb3\xe3\x83\x88\xe3\x81\x8c\xe3\x81\x93\xe3\x81\xae\xe3\x82\xa4\xe3\x83\xb3\xe3\x82\xbf\xe3\x83\xbc\xe3\x83\x95\xe3\x82\xa7\xe3\x83\xbc\xe3\x82\xb9\xe3\x82\x92\xe4\xbd\xbf\xe7\x94\xa8\xe5\x8f\xaf\xe8\x83\xbd\n    export authenticator;\n}\n\n/// gateway \xe3\x82\xb3\xe3\x83\xb3\xe3\x83\x9d\xe3\x83\xbc\xe3\x83\x8d\xe3\x83\xb3\xe3\x83\x88\xe3\x81\x8c\xe5\xae\x9f\xe8\xa3\x85\xe3\x81\x99\xe3\x82\x8b world\n///\n/// \xe3\x81\x93\xe3\x81\xae\xe3\x82\xb3\xe3\x83\xb3\xe3\x83\x9d\xe3\x83\xbc\xe3\x83\x8d\xe3\x83\xb3\xe3\x83\x88\xe3\x81\xaf authenticator \xe3\x82\xa4\xe3\x83\xb3\xe3\x82\xbf\xe3\x83\xbc\xe3\x83\x95\xe3\x82\xa7\xe3\x83\xbc\xe3\x82\xb9\xe3\x82\x92\xe3\x82\xa4\xe3\x83\xb3\xe3\x83\x9d\xe3\x83\xbc\xe3\x83\x88\xe3\x81\x97\xe3\x80\x81\n/// auth \xe3\x82\xb3\xe3\x83\xb3\xe3\x83\x9d\xe3\x83\xbc\xe3\x83\x8d\xe3\x83\xb3\xe3\x83\x88\xe3\x81\xae\xe6\xa9\x9f\xe8\x83\xbd\xe3\x82\x92\xe5\x88\xa9\xe7\x94\xa8\xe3\x81\x97\xe3\x81\xbe\xe3\x81\x99\xe3\x80\x82\n///\n/// \xe4\xbd\xbf\xe7\x94\xa8\xe4\xbe\x8b:\n/// - HTTP \xe3\x83\xaa\xe3\x82\xaf\xe3\x82\xa8\xe3\x82\xb9\xe3\x83\x88\xe3\x82\x92\xe5\x8f\x97\xe4\xbf\xa1\n/// - auth \xe3\x82\xb3\xe3\x83\xb3\xe3\x83\x9d\xe3\x83\xbc\xe3\x83\x8d\xe3\x83\xb3\xe3\x83\x88\xe3\x81\xae verify_token \xe3\x82\x92\xe5\x91\xbc\xe3\x81\xb3\xe5\x87\xba\xe3\x81\x97\xe3\x81\xa6\xe8\xaa\x8d\xe8\xa8\xbc\n/// - \xe8\xaa\x8d\xe8\xa8\xbc\xe6\x88\x90\xe5\x8a\x9f\xe6\x99\x82\xe3\x80\x81\xe3\x82\xb3\xe3\x82\xa2\xe5\xb1\xa4\xe3\x81\xab\xe3\x83\x97\xe3\x83\xad\xe3\x82\xad\xe3\x82\xb7\nworld gateway-world {\n    // authenticator \xe3\x82\xa4\xe3\x83\xb3\xe3\x82\xbf\xe3\x83\xbc\xe3\x83\x95\xe3\x82\xa7\xe3\x83\xbc\xe3\x82\xb9\xe3\x82\x92\xe4\xbd\xbf\xe7\x94\xa8\n    // Spin \xe3\x81\xae\xe3\x82\xb3\xe3\x83\xb3\xe3\x83\x9d\xe3\x83\xbc\xe3\x83\x8d\xe3\x83\xb3\xe3\x83\x88\xe5\x90\x88\xe6\x88\x90\xe3\x81\xab\xe3\x82\x88\xe3\x82\x8a\xe3\x80\x81auth \xe3\x82\xb3\xe3\x83\xb3\xe3\x83\x9d\xe3\x83\xbc\xe3\x83\x8d\xe3\x83\xb3\xe3\x83\x88\xe3\x81\xab\xe6\x8e\xa5\xe7\xb6\x9a\xe3\x81\x95\xe3\x82\x8c\xe3\x82\x8b\n    import authenticator;\n}\n";
use demo::auth::authenticator::{verify_token, AuthResult};
/// コア層（axum サーバー）の URL
///
/// エッジ層で認証が成功した後、このURLにリクエストをプロキシします。
/// 本番環境では環境変数や Spin の変数機能で設定することを推奨。
const CORE_URL: &str = "http://localhost:3001";
/// Edge 検証用シークレット
///
/// Core 層がリクエストが正当な Edge 層から来たことを検証するために使用。
/// 本番環境では必ず環境変数から取得すること。
const EDGE_SECRET: &str = "super-secret-edge-key";
/// 認証不要のパブリックパス
///
/// これらのパスは JWT 認証なしでコア層にプロキシされる。
const PUBLIC_PATHS: &[&str] = &["/api/auth/register", "/api/auth/login"];
/// エラーレスポンスのボディを表す構造体
///
/// 認証失敗やエラー時に返却する JSON レスポンスの形式を定義。
/// 例: {"error": "Missing token"}
struct ErrorResponse {
    /// エラーメッセージ
    error: String,
}
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths,
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for ErrorResponse {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private228::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "ErrorResponse",
                false as usize + 1,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "error",
                &self.error,
            )?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
/// HTTP リクエストを処理するメインハンドラー
///
/// Spin ランタイムから HTTP リクエストを受け取り、適切なレスポンスを返します。
///
/// # 処理フロー
/// 1. リクエストのパスをログ出力
/// 2. /api/* パスの場合、JWT 認証を実行
///    - 認証成功: コア層にプロキシ
///    - 認証失敗: 401 レスポンスを返却
/// 3. /api/* 以外のパスは 401 を返却
///
/// # 引数
/// * `req` - Spin SDK の Request 構造体
///
/// # 戻り値
/// * `impl IntoResponse` - HTTP レスポンスに変換可能な型
async fn handle_request(req: Request) -> impl IntoResponse {
    let path = req.path();
    let method = req.method().to_string();
    {
        ::std::io::_print(format_args!("[Gateway] {0} {1}\n", method, path));
    };
    if path == "/health" {
        return proxy_health_check().await;
    }
    if is_public_path(path) {
        {
            ::std::io::_print(
                format_args!("[Gateway] Public path, bypassing auth: {0}\n", path),
            );
        };
        return proxy_to_core_public(&req).await;
    }
    if path.starts_with("/api/") {
        let token = extract_bearer_token(&req);
        let auth_result: AuthResult = verify_token(&token);
        if !auth_result.authenticated {
            let error_msg = auth_result
                .error
                .unwrap_or_else(|| "Unauthorized".to_string());
            {
                ::std::io::_print(
                    format_args!("[Gateway] Auth failed: {0}\n", error_msg),
                );
            };
            let body = serde_json::to_string(&ErrorResponse { error: error_msg })
                .unwrap();
            return Response::builder()
                .status(401)
                .header("Content-Type", "application/json")
                .body(body)
                .build();
        }
        let user_id = auth_result.user_id.unwrap_or_else(|| "unknown".to_string());
        {
            ::std::io::_print(
                format_args!("[Gateway] Auth success: user_id={0}\n", user_id),
            );
        };
        return proxy_to_core(&req, &user_id).await;
    }
    let body = serde_json::to_string(
            &ErrorResponse {
                error: "Unauthorized: Only /api/* paths are allowed".to_string(),
            },
        )
        .unwrap();
    Response::builder()
        .status(401)
        .header("Content-Type", "application/json")
        .body(body)
        .build()
}
mod __spin_wasi_http {
    mod preamble {
        #![allow(missing_docs)]
        #[allow(dead_code, clippy::all)]
        pub mod wasi {
            pub mod clocks {
                /// WASI Monotonic Clock is a clock API intended to let users measure elapsed
                /// time.
                ///
                /// It is intended to be portable at least between Unix-family platforms and
                /// Windows.
                ///
                /// A monotonic clock is a clock which has an unspecified initial value, and
                /// successive reads of the clock will produce non-decreasing values.
                ///
                /// It is intended for measuring elapsed time.
                #[allow(dead_code, async_fn_in_trait, unused_imports, clippy::all)]
                pub mod monotonic_clock {
                    #[used]
                    #[doc(hidden)]
                    static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
                    use super::super::super::_rt;
                    pub type Pollable = super::super::super::wasi::io::poll::Pollable;
                    /// An instant in time, in nanoseconds. An instant is relative to an
                    /// unspecified initial value, and can only be compared to instances from
                    /// the same monotonic-clock.
                    pub type Instant = u64;
                    /// A duration of time, in nanoseconds.
                    pub type Duration = u64;
                    #[allow(unused_unsafe, clippy::all)]
                    /// Read the current value of the clock.
                    ///
                    /// The clock is monotonic, therefore calling this function repeatedly will
                    /// produce a sequence of non-decreasing values.
                    #[allow(async_fn_in_trait)]
                    pub fn now() -> Instant {
                        unsafe {
                            #[link(
                                wasm_import_module = "wasi:clocks/monotonic-clock@0.2.0"
                            )]
                            unsafe extern "C" {
                                #[link_name = "now"]
                                fn wit_import0() -> i64;
                            }
                            let ret = wit_import0();
                            ret as u64
                        }
                    }
                    #[allow(unused_unsafe, clippy::all)]
                    /// Query the resolution of the clock. Returns the duration of time
                    /// corresponding to a clock tick.
                    #[allow(async_fn_in_trait)]
                    pub fn resolution() -> Duration {
                        unsafe {
                            #[link(
                                wasm_import_module = "wasi:clocks/monotonic-clock@0.2.0"
                            )]
                            unsafe extern "C" {
                                #[link_name = "resolution"]
                                fn wit_import0() -> i64;
                            }
                            let ret = wit_import0();
                            ret as u64
                        }
                    }
                    #[allow(unused_unsafe, clippy::all)]
                    /// Create a `pollable` which will resolve once the specified instant
                    /// occured.
                    #[allow(async_fn_in_trait)]
                    pub fn subscribe_instant(when: Instant) -> Pollable {
                        unsafe {
                            #[link(
                                wasm_import_module = "wasi:clocks/monotonic-clock@0.2.0"
                            )]
                            unsafe extern "C" {
                                #[link_name = "subscribe-instant"]
                                fn wit_import0(_: i64) -> i32;
                            }
                            let ret = wit_import0(_rt::as_i64(when));
                            super::super::super::wasi::io::poll::Pollable::from_handle(
                                ret as u32,
                            )
                        }
                    }
                    #[allow(unused_unsafe, clippy::all)]
                    /// Create a `pollable` which will resolve once the given duration has
                    /// elapsed, starting at the time at which this function was called.
                    /// occured.
                    #[allow(async_fn_in_trait)]
                    pub fn subscribe_duration(when: Duration) -> Pollable {
                        unsafe {
                            #[link(
                                wasm_import_module = "wasi:clocks/monotonic-clock@0.2.0"
                            )]
                            unsafe extern "C" {
                                #[link_name = "subscribe-duration"]
                                fn wit_import0(_: i64) -> i32;
                            }
                            let ret = wit_import0(_rt::as_i64(when));
                            super::super::super::wasi::io::poll::Pollable::from_handle(
                                ret as u32,
                            )
                        }
                    }
                }
            }
            pub mod http {
                /// This interface defines all of the types and methods for implementing
                /// HTTP Requests and Responses, both incoming and outgoing, as well as
                /// their headers, trailers, and bodies.
                #[allow(dead_code, async_fn_in_trait, unused_imports, clippy::all)]
                pub mod types {
                    #[used]
                    #[doc(hidden)]
                    static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
                    use super::super::super::_rt;
                    pub type Duration = super::super::super::wasi::clocks::monotonic_clock::Duration;
                    pub type InputStream = super::super::super::wasi::io::streams::InputStream;
                    pub type OutputStream = super::super::super::wasi::io::streams::OutputStream;
                    pub type IoError = super::super::super::wasi::io::error::Error;
                    pub type Pollable = super::super::super::wasi::io::poll::Pollable;
                    /// This type corresponds to HTTP standard Methods.
                    pub enum Method {
                        Get,
                        Head,
                        Post,
                        Put,
                        Delete,
                        Connect,
                        Options,
                        Trace,
                        Patch,
                        Other(_rt::String),
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for Method {
                        #[inline]
                        fn clone(&self) -> Method {
                            match self {
                                Method::Get => Method::Get,
                                Method::Head => Method::Head,
                                Method::Post => Method::Post,
                                Method::Put => Method::Put,
                                Method::Delete => Method::Delete,
                                Method::Connect => Method::Connect,
                                Method::Options => Method::Options,
                                Method::Trace => Method::Trace,
                                Method::Patch => Method::Patch,
                                Method::Other(__self_0) => {
                                    Method::Other(::core::clone::Clone::clone(__self_0))
                                }
                            }
                        }
                    }
                    impl ::core::fmt::Debug for Method {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter<'_>,
                        ) -> ::core::fmt::Result {
                            match self {
                                Method::Get => f.debug_tuple("Method::Get").finish(),
                                Method::Head => f.debug_tuple("Method::Head").finish(),
                                Method::Post => f.debug_tuple("Method::Post").finish(),
                                Method::Put => f.debug_tuple("Method::Put").finish(),
                                Method::Delete => f.debug_tuple("Method::Delete").finish(),
                                Method::Connect => f.debug_tuple("Method::Connect").finish(),
                                Method::Options => f.debug_tuple("Method::Options").finish(),
                                Method::Trace => f.debug_tuple("Method::Trace").finish(),
                                Method::Patch => f.debug_tuple("Method::Patch").finish(),
                                Method::Other(e) => {
                                    f.debug_tuple("Method::Other").field(e).finish()
                                }
                            }
                        }
                    }
                    /// This type corresponds to HTTP standard Related Schemes.
                    pub enum Scheme {
                        Http,
                        Https,
                        Other(_rt::String),
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for Scheme {
                        #[inline]
                        fn clone(&self) -> Scheme {
                            match self {
                                Scheme::Http => Scheme::Http,
                                Scheme::Https => Scheme::Https,
                                Scheme::Other(__self_0) => {
                                    Scheme::Other(::core::clone::Clone::clone(__self_0))
                                }
                            }
                        }
                    }
                    impl ::core::fmt::Debug for Scheme {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter<'_>,
                        ) -> ::core::fmt::Result {
                            match self {
                                Scheme::Http => f.debug_tuple("Scheme::Http").finish(),
                                Scheme::Https => f.debug_tuple("Scheme::Https").finish(),
                                Scheme::Other(e) => {
                                    f.debug_tuple("Scheme::Other").field(e).finish()
                                }
                            }
                        }
                    }
                    /// Defines the case payload type for `DNS-error` above:
                    pub struct DnsErrorPayload {
                        pub rcode: Option<_rt::String>,
                        pub info_code: Option<u16>,
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for DnsErrorPayload {
                        #[inline]
                        fn clone(&self) -> DnsErrorPayload {
                            DnsErrorPayload {
                                rcode: ::core::clone::Clone::clone(&self.rcode),
                                info_code: ::core::clone::Clone::clone(&self.info_code),
                            }
                        }
                    }
                    impl ::core::fmt::Debug for DnsErrorPayload {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter<'_>,
                        ) -> ::core::fmt::Result {
                            f.debug_struct("DnsErrorPayload")
                                .field("rcode", &self.rcode)
                                .field("info-code", &self.info_code)
                                .finish()
                        }
                    }
                    /// Defines the case payload type for `TLS-alert-received` above:
                    pub struct TlsAlertReceivedPayload {
                        pub alert_id: Option<u8>,
                        pub alert_message: Option<_rt::String>,
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for TlsAlertReceivedPayload {
                        #[inline]
                        fn clone(&self) -> TlsAlertReceivedPayload {
                            TlsAlertReceivedPayload {
                                alert_id: ::core::clone::Clone::clone(&self.alert_id),
                                alert_message: ::core::clone::Clone::clone(
                                    &self.alert_message,
                                ),
                            }
                        }
                    }
                    impl ::core::fmt::Debug for TlsAlertReceivedPayload {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter<'_>,
                        ) -> ::core::fmt::Result {
                            f.debug_struct("TlsAlertReceivedPayload")
                                .field("alert-id", &self.alert_id)
                                .field("alert-message", &self.alert_message)
                                .finish()
                        }
                    }
                    /// Defines the case payload type for `HTTP-response-{header,trailer}-size` above:
                    pub struct FieldSizePayload {
                        pub field_name: Option<_rt::String>,
                        pub field_size: Option<u32>,
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for FieldSizePayload {
                        #[inline]
                        fn clone(&self) -> FieldSizePayload {
                            FieldSizePayload {
                                field_name: ::core::clone::Clone::clone(&self.field_name),
                                field_size: ::core::clone::Clone::clone(&self.field_size),
                            }
                        }
                    }
                    impl ::core::fmt::Debug for FieldSizePayload {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter<'_>,
                        ) -> ::core::fmt::Result {
                            f.debug_struct("FieldSizePayload")
                                .field("field-name", &self.field_name)
                                .field("field-size", &self.field_size)
                                .finish()
                        }
                    }
                    /// These cases are inspired by the IANA HTTP Proxy Error Types:
                    ///   https://www.iana.org/assignments/http-proxy-status/http-proxy-status.xhtml#table-http-proxy-error-types
                    pub enum ErrorCode {
                        DnsTimeout,
                        DnsError(DnsErrorPayload),
                        DestinationNotFound,
                        DestinationUnavailable,
                        DestinationIpProhibited,
                        DestinationIpUnroutable,
                        ConnectionRefused,
                        ConnectionTerminated,
                        ConnectionTimeout,
                        ConnectionReadTimeout,
                        ConnectionWriteTimeout,
                        ConnectionLimitReached,
                        TlsProtocolError,
                        TlsCertificateError,
                        TlsAlertReceived(TlsAlertReceivedPayload),
                        HttpRequestDenied,
                        HttpRequestLengthRequired,
                        HttpRequestBodySize(Option<u64>),
                        HttpRequestMethodInvalid,
                        HttpRequestUriInvalid,
                        HttpRequestUriTooLong,
                        HttpRequestHeaderSectionSize(Option<u32>),
                        HttpRequestHeaderSize(Option<FieldSizePayload>),
                        HttpRequestTrailerSectionSize(Option<u32>),
                        HttpRequestTrailerSize(FieldSizePayload),
                        HttpResponseIncomplete,
                        HttpResponseHeaderSectionSize(Option<u32>),
                        HttpResponseHeaderSize(FieldSizePayload),
                        HttpResponseBodySize(Option<u64>),
                        HttpResponseTrailerSectionSize(Option<u32>),
                        HttpResponseTrailerSize(FieldSizePayload),
                        HttpResponseTransferCoding(Option<_rt::String>),
                        HttpResponseContentCoding(Option<_rt::String>),
                        HttpResponseTimeout,
                        HttpUpgradeFailed,
                        HttpProtocolError,
                        LoopDetected,
                        ConfigurationError,
                        /// This is a catch-all error for anything that doesn't fit cleanly into a
                        /// more specific case. It also includes an optional string for an
                        /// unstructured description of the error. Users should not depend on the
                        /// string for diagnosing errors, as it's not required to be consistent
                        /// between implementations.
                        InternalError(Option<_rt::String>),
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for ErrorCode {
                        #[inline]
                        fn clone(&self) -> ErrorCode {
                            match self {
                                ErrorCode::DnsTimeout => ErrorCode::DnsTimeout,
                                ErrorCode::DnsError(__self_0) => {
                                    ErrorCode::DnsError(::core::clone::Clone::clone(__self_0))
                                }
                                ErrorCode::DestinationNotFound => {
                                    ErrorCode::DestinationNotFound
                                }
                                ErrorCode::DestinationUnavailable => {
                                    ErrorCode::DestinationUnavailable
                                }
                                ErrorCode::DestinationIpProhibited => {
                                    ErrorCode::DestinationIpProhibited
                                }
                                ErrorCode::DestinationIpUnroutable => {
                                    ErrorCode::DestinationIpUnroutable
                                }
                                ErrorCode::ConnectionRefused => ErrorCode::ConnectionRefused,
                                ErrorCode::ConnectionTerminated => {
                                    ErrorCode::ConnectionTerminated
                                }
                                ErrorCode::ConnectionTimeout => ErrorCode::ConnectionTimeout,
                                ErrorCode::ConnectionReadTimeout => {
                                    ErrorCode::ConnectionReadTimeout
                                }
                                ErrorCode::ConnectionWriteTimeout => {
                                    ErrorCode::ConnectionWriteTimeout
                                }
                                ErrorCode::ConnectionLimitReached => {
                                    ErrorCode::ConnectionLimitReached
                                }
                                ErrorCode::TlsProtocolError => ErrorCode::TlsProtocolError,
                                ErrorCode::TlsCertificateError => {
                                    ErrorCode::TlsCertificateError
                                }
                                ErrorCode::TlsAlertReceived(__self_0) => {
                                    ErrorCode::TlsAlertReceived(
                                        ::core::clone::Clone::clone(__self_0),
                                    )
                                }
                                ErrorCode::HttpRequestDenied => ErrorCode::HttpRequestDenied,
                                ErrorCode::HttpRequestLengthRequired => {
                                    ErrorCode::HttpRequestLengthRequired
                                }
                                ErrorCode::HttpRequestBodySize(__self_0) => {
                                    ErrorCode::HttpRequestBodySize(
                                        ::core::clone::Clone::clone(__self_0),
                                    )
                                }
                                ErrorCode::HttpRequestMethodInvalid => {
                                    ErrorCode::HttpRequestMethodInvalid
                                }
                                ErrorCode::HttpRequestUriInvalid => {
                                    ErrorCode::HttpRequestUriInvalid
                                }
                                ErrorCode::HttpRequestUriTooLong => {
                                    ErrorCode::HttpRequestUriTooLong
                                }
                                ErrorCode::HttpRequestHeaderSectionSize(__self_0) => {
                                    ErrorCode::HttpRequestHeaderSectionSize(
                                        ::core::clone::Clone::clone(__self_0),
                                    )
                                }
                                ErrorCode::HttpRequestHeaderSize(__self_0) => {
                                    ErrorCode::HttpRequestHeaderSize(
                                        ::core::clone::Clone::clone(__self_0),
                                    )
                                }
                                ErrorCode::HttpRequestTrailerSectionSize(__self_0) => {
                                    ErrorCode::HttpRequestTrailerSectionSize(
                                        ::core::clone::Clone::clone(__self_0),
                                    )
                                }
                                ErrorCode::HttpRequestTrailerSize(__self_0) => {
                                    ErrorCode::HttpRequestTrailerSize(
                                        ::core::clone::Clone::clone(__self_0),
                                    )
                                }
                                ErrorCode::HttpResponseIncomplete => {
                                    ErrorCode::HttpResponseIncomplete
                                }
                                ErrorCode::HttpResponseHeaderSectionSize(__self_0) => {
                                    ErrorCode::HttpResponseHeaderSectionSize(
                                        ::core::clone::Clone::clone(__self_0),
                                    )
                                }
                                ErrorCode::HttpResponseHeaderSize(__self_0) => {
                                    ErrorCode::HttpResponseHeaderSize(
                                        ::core::clone::Clone::clone(__self_0),
                                    )
                                }
                                ErrorCode::HttpResponseBodySize(__self_0) => {
                                    ErrorCode::HttpResponseBodySize(
                                        ::core::clone::Clone::clone(__self_0),
                                    )
                                }
                                ErrorCode::HttpResponseTrailerSectionSize(__self_0) => {
                                    ErrorCode::HttpResponseTrailerSectionSize(
                                        ::core::clone::Clone::clone(__self_0),
                                    )
                                }
                                ErrorCode::HttpResponseTrailerSize(__self_0) => {
                                    ErrorCode::HttpResponseTrailerSize(
                                        ::core::clone::Clone::clone(__self_0),
                                    )
                                }
                                ErrorCode::HttpResponseTransferCoding(__self_0) => {
                                    ErrorCode::HttpResponseTransferCoding(
                                        ::core::clone::Clone::clone(__self_0),
                                    )
                                }
                                ErrorCode::HttpResponseContentCoding(__self_0) => {
                                    ErrorCode::HttpResponseContentCoding(
                                        ::core::clone::Clone::clone(__self_0),
                                    )
                                }
                                ErrorCode::HttpResponseTimeout => {
                                    ErrorCode::HttpResponseTimeout
                                }
                                ErrorCode::HttpUpgradeFailed => ErrorCode::HttpUpgradeFailed,
                                ErrorCode::HttpProtocolError => ErrorCode::HttpProtocolError,
                                ErrorCode::LoopDetected => ErrorCode::LoopDetected,
                                ErrorCode::ConfigurationError => {
                                    ErrorCode::ConfigurationError
                                }
                                ErrorCode::InternalError(__self_0) => {
                                    ErrorCode::InternalError(
                                        ::core::clone::Clone::clone(__self_0),
                                    )
                                }
                            }
                        }
                    }
                    impl ::core::fmt::Debug for ErrorCode {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter<'_>,
                        ) -> ::core::fmt::Result {
                            match self {
                                ErrorCode::DnsTimeout => {
                                    f.debug_tuple("ErrorCode::DnsTimeout").finish()
                                }
                                ErrorCode::DnsError(e) => {
                                    f.debug_tuple("ErrorCode::DnsError").field(e).finish()
                                }
                                ErrorCode::DestinationNotFound => {
                                    f.debug_tuple("ErrorCode::DestinationNotFound").finish()
                                }
                                ErrorCode::DestinationUnavailable => {
                                    f.debug_tuple("ErrorCode::DestinationUnavailable").finish()
                                }
                                ErrorCode::DestinationIpProhibited => {
                                    f.debug_tuple("ErrorCode::DestinationIpProhibited").finish()
                                }
                                ErrorCode::DestinationIpUnroutable => {
                                    f.debug_tuple("ErrorCode::DestinationIpUnroutable").finish()
                                }
                                ErrorCode::ConnectionRefused => {
                                    f.debug_tuple("ErrorCode::ConnectionRefused").finish()
                                }
                                ErrorCode::ConnectionTerminated => {
                                    f.debug_tuple("ErrorCode::ConnectionTerminated").finish()
                                }
                                ErrorCode::ConnectionTimeout => {
                                    f.debug_tuple("ErrorCode::ConnectionTimeout").finish()
                                }
                                ErrorCode::ConnectionReadTimeout => {
                                    f.debug_tuple("ErrorCode::ConnectionReadTimeout").finish()
                                }
                                ErrorCode::ConnectionWriteTimeout => {
                                    f.debug_tuple("ErrorCode::ConnectionWriteTimeout").finish()
                                }
                                ErrorCode::ConnectionLimitReached => {
                                    f.debug_tuple("ErrorCode::ConnectionLimitReached").finish()
                                }
                                ErrorCode::TlsProtocolError => {
                                    f.debug_tuple("ErrorCode::TlsProtocolError").finish()
                                }
                                ErrorCode::TlsCertificateError => {
                                    f.debug_tuple("ErrorCode::TlsCertificateError").finish()
                                }
                                ErrorCode::TlsAlertReceived(e) => {
                                    f.debug_tuple("ErrorCode::TlsAlertReceived")
                                        .field(e)
                                        .finish()
                                }
                                ErrorCode::HttpRequestDenied => {
                                    f.debug_tuple("ErrorCode::HttpRequestDenied").finish()
                                }
                                ErrorCode::HttpRequestLengthRequired => {
                                    f.debug_tuple("ErrorCode::HttpRequestLengthRequired")
                                        .finish()
                                }
                                ErrorCode::HttpRequestBodySize(e) => {
                                    f.debug_tuple("ErrorCode::HttpRequestBodySize")
                                        .field(e)
                                        .finish()
                                }
                                ErrorCode::HttpRequestMethodInvalid => {
                                    f.debug_tuple("ErrorCode::HttpRequestMethodInvalid")
                                        .finish()
                                }
                                ErrorCode::HttpRequestUriInvalid => {
                                    f.debug_tuple("ErrorCode::HttpRequestUriInvalid").finish()
                                }
                                ErrorCode::HttpRequestUriTooLong => {
                                    f.debug_tuple("ErrorCode::HttpRequestUriTooLong").finish()
                                }
                                ErrorCode::HttpRequestHeaderSectionSize(e) => {
                                    f.debug_tuple("ErrorCode::HttpRequestHeaderSectionSize")
                                        .field(e)
                                        .finish()
                                }
                                ErrorCode::HttpRequestHeaderSize(e) => {
                                    f.debug_tuple("ErrorCode::HttpRequestHeaderSize")
                                        .field(e)
                                        .finish()
                                }
                                ErrorCode::HttpRequestTrailerSectionSize(e) => {
                                    f.debug_tuple("ErrorCode::HttpRequestTrailerSectionSize")
                                        .field(e)
                                        .finish()
                                }
                                ErrorCode::HttpRequestTrailerSize(e) => {
                                    f.debug_tuple("ErrorCode::HttpRequestTrailerSize")
                                        .field(e)
                                        .finish()
                                }
                                ErrorCode::HttpResponseIncomplete => {
                                    f.debug_tuple("ErrorCode::HttpResponseIncomplete").finish()
                                }
                                ErrorCode::HttpResponseHeaderSectionSize(e) => {
                                    f.debug_tuple("ErrorCode::HttpResponseHeaderSectionSize")
                                        .field(e)
                                        .finish()
                                }
                                ErrorCode::HttpResponseHeaderSize(e) => {
                                    f.debug_tuple("ErrorCode::HttpResponseHeaderSize")
                                        .field(e)
                                        .finish()
                                }
                                ErrorCode::HttpResponseBodySize(e) => {
                                    f.debug_tuple("ErrorCode::HttpResponseBodySize")
                                        .field(e)
                                        .finish()
                                }
                                ErrorCode::HttpResponseTrailerSectionSize(e) => {
                                    f.debug_tuple("ErrorCode::HttpResponseTrailerSectionSize")
                                        .field(e)
                                        .finish()
                                }
                                ErrorCode::HttpResponseTrailerSize(e) => {
                                    f.debug_tuple("ErrorCode::HttpResponseTrailerSize")
                                        .field(e)
                                        .finish()
                                }
                                ErrorCode::HttpResponseTransferCoding(e) => {
                                    f.debug_tuple("ErrorCode::HttpResponseTransferCoding")
                                        .field(e)
                                        .finish()
                                }
                                ErrorCode::HttpResponseContentCoding(e) => {
                                    f.debug_tuple("ErrorCode::HttpResponseContentCoding")
                                        .field(e)
                                        .finish()
                                }
                                ErrorCode::HttpResponseTimeout => {
                                    f.debug_tuple("ErrorCode::HttpResponseTimeout").finish()
                                }
                                ErrorCode::HttpUpgradeFailed => {
                                    f.debug_tuple("ErrorCode::HttpUpgradeFailed").finish()
                                }
                                ErrorCode::HttpProtocolError => {
                                    f.debug_tuple("ErrorCode::HttpProtocolError").finish()
                                }
                                ErrorCode::LoopDetected => {
                                    f.debug_tuple("ErrorCode::LoopDetected").finish()
                                }
                                ErrorCode::ConfigurationError => {
                                    f.debug_tuple("ErrorCode::ConfigurationError").finish()
                                }
                                ErrorCode::InternalError(e) => {
                                    f.debug_tuple("ErrorCode::InternalError").field(e).finish()
                                }
                            }
                        }
                    }
                    impl ::core::fmt::Display for ErrorCode {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter<'_>,
                        ) -> ::core::fmt::Result {
                            f.write_fmt(format_args!("{0:?}", self))
                        }
                    }
                    impl std::error::Error for ErrorCode {}
                    /// This type enumerates the different kinds of errors that may occur when
                    /// setting or appending to a `fields` resource.
                    pub enum HeaderError {
                        /// This error indicates that a `field-key` or `field-value` was
                        /// syntactically invalid when used with an operation that sets headers in a
                        /// `fields`.
                        InvalidSyntax,
                        /// This error indicates that a forbidden `field-key` was used when trying
                        /// to set a header in a `fields`.
                        Forbidden,
                        /// This error indicates that the operation on the `fields` was not
                        /// permitted because the fields are immutable.
                        Immutable,
                    }
                    #[automatically_derived]
                    #[doc(hidden)]
                    unsafe impl ::core::clone::TrivialClone for HeaderError {}
                    #[automatically_derived]
                    impl ::core::clone::Clone for HeaderError {
                        #[inline]
                        fn clone(&self) -> HeaderError {
                            *self
                        }
                    }
                    #[automatically_derived]
                    impl ::core::marker::Copy for HeaderError {}
                    impl ::core::fmt::Debug for HeaderError {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter<'_>,
                        ) -> ::core::fmt::Result {
                            match self {
                                HeaderError::InvalidSyntax => {
                                    f.debug_tuple("HeaderError::InvalidSyntax").finish()
                                }
                                HeaderError::Forbidden => {
                                    f.debug_tuple("HeaderError::Forbidden").finish()
                                }
                                HeaderError::Immutable => {
                                    f.debug_tuple("HeaderError::Immutable").finish()
                                }
                            }
                        }
                    }
                    impl ::core::fmt::Display for HeaderError {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter<'_>,
                        ) -> ::core::fmt::Result {
                            f.write_fmt(format_args!("{0:?}", self))
                        }
                    }
                    impl std::error::Error for HeaderError {}
                    /// Field keys are always strings.
                    pub type FieldKey = _rt::String;
                    /// Field values should always be ASCII strings. However, in
                    /// reality, HTTP implementations often have to interpret malformed values,
                    /// so they are provided as a list of bytes.
                    pub type FieldValue = _rt::Vec<u8>;
                    /// This following block defines the `fields` resource which corresponds to
                    /// HTTP standard Fields. Fields are a common representation used for both
                    /// Headers and Trailers.
                    ///
                    /// A `fields` may be mutable or immutable. A `fields` created using the
                    /// constructor, `from-list`, or `clone` will be mutable, but a `fields`
                    /// resource given by other means (including, but not limited to,
                    /// `incoming-request.headers`, `outgoing-request.headers`) might be be
                    /// immutable. In an immutable fields, the `set`, `append`, and `delete`
                    /// operations will fail with `header-error.immutable`.
                    #[repr(transparent)]
                    pub struct Fields {
                        handle: _rt::Resource<Fields>,
                    }
                    #[automatically_derived]
                    impl ::core::fmt::Debug for Fields {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_struct_field1_finish(
                                f,
                                "Fields",
                                "handle",
                                &&self.handle,
                            )
                        }
                    }
                    impl Fields {
                        #[doc(hidden)]
                        pub unsafe fn from_handle(handle: u32) -> Self {
                            Self {
                                handle: unsafe { _rt::Resource::from_handle(handle) },
                            }
                        }
                        #[doc(hidden)]
                        pub fn take_handle(&self) -> u32 {
                            _rt::Resource::take_handle(&self.handle)
                        }
                        #[doc(hidden)]
                        pub fn handle(&self) -> u32 {
                            _rt::Resource::handle(&self.handle)
                        }
                    }
                    unsafe impl _rt::WasmResource for Fields {
                        #[inline]
                        unsafe fn drop(_handle: u32) {
                            #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                            unsafe extern "C" {
                                #[link_name = "[resource-drop]fields"]
                                fn drop(_: i32);
                            }
                            unsafe {
                                drop(_handle as i32);
                            }
                        }
                    }
                    /// Headers is an alias for Fields.
                    pub type Headers = Fields;
                    /// Trailers is an alias for Fields.
                    pub type Trailers = Fields;
                    /// Represents an incoming HTTP Request.
                    #[repr(transparent)]
                    pub struct IncomingRequest {
                        handle: _rt::Resource<IncomingRequest>,
                    }
                    #[automatically_derived]
                    impl ::core::fmt::Debug for IncomingRequest {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_struct_field1_finish(
                                f,
                                "IncomingRequest",
                                "handle",
                                &&self.handle,
                            )
                        }
                    }
                    impl IncomingRequest {
                        #[doc(hidden)]
                        pub unsafe fn from_handle(handle: u32) -> Self {
                            Self {
                                handle: unsafe { _rt::Resource::from_handle(handle) },
                            }
                        }
                        #[doc(hidden)]
                        pub fn take_handle(&self) -> u32 {
                            _rt::Resource::take_handle(&self.handle)
                        }
                        #[doc(hidden)]
                        pub fn handle(&self) -> u32 {
                            _rt::Resource::handle(&self.handle)
                        }
                    }
                    unsafe impl _rt::WasmResource for IncomingRequest {
                        #[inline]
                        unsafe fn drop(_handle: u32) {
                            #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                            unsafe extern "C" {
                                #[link_name = "[resource-drop]incoming-request"]
                                fn drop(_: i32);
                            }
                            unsafe {
                                drop(_handle as i32);
                            }
                        }
                    }
                    /// Represents an outgoing HTTP Request.
                    #[repr(transparent)]
                    pub struct OutgoingRequest {
                        handle: _rt::Resource<OutgoingRequest>,
                    }
                    #[automatically_derived]
                    impl ::core::fmt::Debug for OutgoingRequest {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_struct_field1_finish(
                                f,
                                "OutgoingRequest",
                                "handle",
                                &&self.handle,
                            )
                        }
                    }
                    impl OutgoingRequest {
                        #[doc(hidden)]
                        pub unsafe fn from_handle(handle: u32) -> Self {
                            Self {
                                handle: unsafe { _rt::Resource::from_handle(handle) },
                            }
                        }
                        #[doc(hidden)]
                        pub fn take_handle(&self) -> u32 {
                            _rt::Resource::take_handle(&self.handle)
                        }
                        #[doc(hidden)]
                        pub fn handle(&self) -> u32 {
                            _rt::Resource::handle(&self.handle)
                        }
                    }
                    unsafe impl _rt::WasmResource for OutgoingRequest {
                        #[inline]
                        unsafe fn drop(_handle: u32) {
                            #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                            unsafe extern "C" {
                                #[link_name = "[resource-drop]outgoing-request"]
                                fn drop(_: i32);
                            }
                            unsafe {
                                drop(_handle as i32);
                            }
                        }
                    }
                    /// Parameters for making an HTTP Request. Each of these parameters is
                    /// currently an optional timeout applicable to the transport layer of the
                    /// HTTP protocol.
                    ///
                    /// These timeouts are separate from any the user may use to bound a
                    /// blocking call to `wasi:io/poll.poll`.
                    #[repr(transparent)]
                    pub struct RequestOptions {
                        handle: _rt::Resource<RequestOptions>,
                    }
                    #[automatically_derived]
                    impl ::core::fmt::Debug for RequestOptions {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_struct_field1_finish(
                                f,
                                "RequestOptions",
                                "handle",
                                &&self.handle,
                            )
                        }
                    }
                    impl RequestOptions {
                        #[doc(hidden)]
                        pub unsafe fn from_handle(handle: u32) -> Self {
                            Self {
                                handle: unsafe { _rt::Resource::from_handle(handle) },
                            }
                        }
                        #[doc(hidden)]
                        pub fn take_handle(&self) -> u32 {
                            _rt::Resource::take_handle(&self.handle)
                        }
                        #[doc(hidden)]
                        pub fn handle(&self) -> u32 {
                            _rt::Resource::handle(&self.handle)
                        }
                    }
                    unsafe impl _rt::WasmResource for RequestOptions {
                        #[inline]
                        unsafe fn drop(_handle: u32) {
                            #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                            unsafe extern "C" {
                                #[link_name = "[resource-drop]request-options"]
                                fn drop(_: i32);
                            }
                            unsafe {
                                drop(_handle as i32);
                            }
                        }
                    }
                    /// Represents the ability to send an HTTP Response.
                    ///
                    /// This resource is used by the `wasi:http/incoming-handler` interface to
                    /// allow a Response to be sent corresponding to the Request provided as the
                    /// other argument to `incoming-handler.handle`.
                    #[repr(transparent)]
                    pub struct ResponseOutparam {
                        handle: _rt::Resource<ResponseOutparam>,
                    }
                    #[automatically_derived]
                    impl ::core::fmt::Debug for ResponseOutparam {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_struct_field1_finish(
                                f,
                                "ResponseOutparam",
                                "handle",
                                &&self.handle,
                            )
                        }
                    }
                    impl ResponseOutparam {
                        #[doc(hidden)]
                        pub unsafe fn from_handle(handle: u32) -> Self {
                            Self {
                                handle: unsafe { _rt::Resource::from_handle(handle) },
                            }
                        }
                        #[doc(hidden)]
                        pub fn take_handle(&self) -> u32 {
                            _rt::Resource::take_handle(&self.handle)
                        }
                        #[doc(hidden)]
                        pub fn handle(&self) -> u32 {
                            _rt::Resource::handle(&self.handle)
                        }
                    }
                    unsafe impl _rt::WasmResource for ResponseOutparam {
                        #[inline]
                        unsafe fn drop(_handle: u32) {
                            #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                            unsafe extern "C" {
                                #[link_name = "[resource-drop]response-outparam"]
                                fn drop(_: i32);
                            }
                            unsafe {
                                drop(_handle as i32);
                            }
                        }
                    }
                    /// This type corresponds to the HTTP standard Status Code.
                    pub type StatusCode = u16;
                    /// Represents an incoming HTTP Response.
                    #[repr(transparent)]
                    pub struct IncomingResponse {
                        handle: _rt::Resource<IncomingResponse>,
                    }
                    #[automatically_derived]
                    impl ::core::fmt::Debug for IncomingResponse {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_struct_field1_finish(
                                f,
                                "IncomingResponse",
                                "handle",
                                &&self.handle,
                            )
                        }
                    }
                    impl IncomingResponse {
                        #[doc(hidden)]
                        pub unsafe fn from_handle(handle: u32) -> Self {
                            Self {
                                handle: unsafe { _rt::Resource::from_handle(handle) },
                            }
                        }
                        #[doc(hidden)]
                        pub fn take_handle(&self) -> u32 {
                            _rt::Resource::take_handle(&self.handle)
                        }
                        #[doc(hidden)]
                        pub fn handle(&self) -> u32 {
                            _rt::Resource::handle(&self.handle)
                        }
                    }
                    unsafe impl _rt::WasmResource for IncomingResponse {
                        #[inline]
                        unsafe fn drop(_handle: u32) {
                            #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                            unsafe extern "C" {
                                #[link_name = "[resource-drop]incoming-response"]
                                fn drop(_: i32);
                            }
                            unsafe {
                                drop(_handle as i32);
                            }
                        }
                    }
                    /// Represents an incoming HTTP Request or Response's Body.
                    ///
                    /// A body has both its contents - a stream of bytes - and a (possibly
                    /// empty) set of trailers, indicating that the full contents of the
                    /// body have been received. This resource represents the contents as
                    /// an `input-stream` and the delivery of trailers as a `future-trailers`,
                    /// and ensures that the user of this interface may only be consuming either
                    /// the body contents or waiting on trailers at any given time.
                    #[repr(transparent)]
                    pub struct IncomingBody {
                        handle: _rt::Resource<IncomingBody>,
                    }
                    #[automatically_derived]
                    impl ::core::fmt::Debug for IncomingBody {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_struct_field1_finish(
                                f,
                                "IncomingBody",
                                "handle",
                                &&self.handle,
                            )
                        }
                    }
                    impl IncomingBody {
                        #[doc(hidden)]
                        pub unsafe fn from_handle(handle: u32) -> Self {
                            Self {
                                handle: unsafe { _rt::Resource::from_handle(handle) },
                            }
                        }
                        #[doc(hidden)]
                        pub fn take_handle(&self) -> u32 {
                            _rt::Resource::take_handle(&self.handle)
                        }
                        #[doc(hidden)]
                        pub fn handle(&self) -> u32 {
                            _rt::Resource::handle(&self.handle)
                        }
                    }
                    unsafe impl _rt::WasmResource for IncomingBody {
                        #[inline]
                        unsafe fn drop(_handle: u32) {
                            #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                            unsafe extern "C" {
                                #[link_name = "[resource-drop]incoming-body"]
                                fn drop(_: i32);
                            }
                            unsafe {
                                drop(_handle as i32);
                            }
                        }
                    }
                    /// Represents a future which may eventaully return trailers, or an error.
                    ///
                    /// In the case that the incoming HTTP Request or Response did not have any
                    /// trailers, this future will resolve to the empty set of trailers once the
                    /// complete Request or Response body has been received.
                    #[repr(transparent)]
                    pub struct FutureTrailers {
                        handle: _rt::Resource<FutureTrailers>,
                    }
                    #[automatically_derived]
                    impl ::core::fmt::Debug for FutureTrailers {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_struct_field1_finish(
                                f,
                                "FutureTrailers",
                                "handle",
                                &&self.handle,
                            )
                        }
                    }
                    impl FutureTrailers {
                        #[doc(hidden)]
                        pub unsafe fn from_handle(handle: u32) -> Self {
                            Self {
                                handle: unsafe { _rt::Resource::from_handle(handle) },
                            }
                        }
                        #[doc(hidden)]
                        pub fn take_handle(&self) -> u32 {
                            _rt::Resource::take_handle(&self.handle)
                        }
                        #[doc(hidden)]
                        pub fn handle(&self) -> u32 {
                            _rt::Resource::handle(&self.handle)
                        }
                    }
                    unsafe impl _rt::WasmResource for FutureTrailers {
                        #[inline]
                        unsafe fn drop(_handle: u32) {
                            #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                            unsafe extern "C" {
                                #[link_name = "[resource-drop]future-trailers"]
                                fn drop(_: i32);
                            }
                            unsafe {
                                drop(_handle as i32);
                            }
                        }
                    }
                    /// Represents an outgoing HTTP Response.
                    #[repr(transparent)]
                    pub struct OutgoingResponse {
                        handle: _rt::Resource<OutgoingResponse>,
                    }
                    #[automatically_derived]
                    impl ::core::fmt::Debug for OutgoingResponse {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_struct_field1_finish(
                                f,
                                "OutgoingResponse",
                                "handle",
                                &&self.handle,
                            )
                        }
                    }
                    impl OutgoingResponse {
                        #[doc(hidden)]
                        pub unsafe fn from_handle(handle: u32) -> Self {
                            Self {
                                handle: unsafe { _rt::Resource::from_handle(handle) },
                            }
                        }
                        #[doc(hidden)]
                        pub fn take_handle(&self) -> u32 {
                            _rt::Resource::take_handle(&self.handle)
                        }
                        #[doc(hidden)]
                        pub fn handle(&self) -> u32 {
                            _rt::Resource::handle(&self.handle)
                        }
                    }
                    unsafe impl _rt::WasmResource for OutgoingResponse {
                        #[inline]
                        unsafe fn drop(_handle: u32) {
                            #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                            unsafe extern "C" {
                                #[link_name = "[resource-drop]outgoing-response"]
                                fn drop(_: i32);
                            }
                            unsafe {
                                drop(_handle as i32);
                            }
                        }
                    }
                    /// Represents an outgoing HTTP Request or Response's Body.
                    ///
                    /// A body has both its contents - a stream of bytes - and a (possibly
                    /// empty) set of trailers, inducating the full contents of the body
                    /// have been sent. This resource represents the contents as an
                    /// `output-stream` child resource, and the completion of the body (with
                    /// optional trailers) with a static function that consumes the
                    /// `outgoing-body` resource, and ensures that the user of this interface
                    /// may not write to the body contents after the body has been finished.
                    ///
                    /// If the user code drops this resource, as opposed to calling the static
                    /// method `finish`, the implementation should treat the body as incomplete,
                    /// and that an error has occured. The implementation should propogate this
                    /// error to the HTTP protocol by whatever means it has available,
                    /// including: corrupting the body on the wire, aborting the associated
                    /// Request, or sending a late status code for the Response.
                    #[repr(transparent)]
                    pub struct OutgoingBody {
                        handle: _rt::Resource<OutgoingBody>,
                    }
                    #[automatically_derived]
                    impl ::core::fmt::Debug for OutgoingBody {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_struct_field1_finish(
                                f,
                                "OutgoingBody",
                                "handle",
                                &&self.handle,
                            )
                        }
                    }
                    impl OutgoingBody {
                        #[doc(hidden)]
                        pub unsafe fn from_handle(handle: u32) -> Self {
                            Self {
                                handle: unsafe { _rt::Resource::from_handle(handle) },
                            }
                        }
                        #[doc(hidden)]
                        pub fn take_handle(&self) -> u32 {
                            _rt::Resource::take_handle(&self.handle)
                        }
                        #[doc(hidden)]
                        pub fn handle(&self) -> u32 {
                            _rt::Resource::handle(&self.handle)
                        }
                    }
                    unsafe impl _rt::WasmResource for OutgoingBody {
                        #[inline]
                        unsafe fn drop(_handle: u32) {
                            #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                            unsafe extern "C" {
                                #[link_name = "[resource-drop]outgoing-body"]
                                fn drop(_: i32);
                            }
                            unsafe {
                                drop(_handle as i32);
                            }
                        }
                    }
                    /// Represents a future which may eventaully return an incoming HTTP
                    /// Response, or an error.
                    ///
                    /// This resource is returned by the `wasi:http/outgoing-handler` interface to
                    /// provide the HTTP Response corresponding to the sent Request.
                    #[repr(transparent)]
                    pub struct FutureIncomingResponse {
                        handle: _rt::Resource<FutureIncomingResponse>,
                    }
                    #[automatically_derived]
                    impl ::core::fmt::Debug for FutureIncomingResponse {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_struct_field1_finish(
                                f,
                                "FutureIncomingResponse",
                                "handle",
                                &&self.handle,
                            )
                        }
                    }
                    impl FutureIncomingResponse {
                        #[doc(hidden)]
                        pub unsafe fn from_handle(handle: u32) -> Self {
                            Self {
                                handle: unsafe { _rt::Resource::from_handle(handle) },
                            }
                        }
                        #[doc(hidden)]
                        pub fn take_handle(&self) -> u32 {
                            _rt::Resource::take_handle(&self.handle)
                        }
                        #[doc(hidden)]
                        pub fn handle(&self) -> u32 {
                            _rt::Resource::handle(&self.handle)
                        }
                    }
                    unsafe impl _rt::WasmResource for FutureIncomingResponse {
                        #[inline]
                        unsafe fn drop(_handle: u32) {
                            #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                            unsafe extern "C" {
                                #[link_name = "[resource-drop]future-incoming-response"]
                                fn drop(_: i32);
                            }
                            unsafe {
                                drop(_handle as i32);
                            }
                        }
                    }
                    #[allow(unused_unsafe, clippy::all)]
                    /// Attempts to extract a http-related `error` from the wasi:io `error`
                    /// provided.
                    ///
                    /// Stream operations which return
                    /// `wasi:io/stream/stream-error::last-operation-failed` have a payload of
                    /// type `wasi:io/error/error` with more information about the operation
                    /// that failed. This payload can be passed through to this function to see
                    /// if there's http-related information about the error to return.
                    ///
                    /// Note that this function is fallible because not all io-errors are
                    /// http-related errors.
                    #[allow(async_fn_in_trait)]
                    pub fn http_error_code(err: &IoError) -> Option<ErrorCode> {
                        unsafe {
                            #[repr(align(8))]
                            struct RetArea(
                                [::core::mem::MaybeUninit<
                                    u8,
                                >; 24 + 4 * ::core::mem::size_of::<*const u8>()],
                            );
                            let mut ret_area = RetArea(
                                [::core::mem::MaybeUninit::uninit(); 24
                                    + 4 * ::core::mem::size_of::<*const u8>()],
                            );
                            let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                            #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                            unsafe extern "C" {
                                #[link_name = "http-error-code"]
                                fn wit_import1(_: i32, _: *mut u8);
                            }
                            wit_import1((err).handle() as i32, ptr0);
                            let l2 = i32::from(*ptr0.add(0).cast::<u8>());
                            let result66 = match l2 {
                                0 => None,
                                1 => {
                                    let e = {
                                        let l3 = i32::from(*ptr0.add(8).cast::<u8>());
                                        let v65 = match l3 {
                                            0 => ErrorCode::DnsTimeout,
                                            1 => {
                                                let e65 = {
                                                    let l4 = i32::from(*ptr0.add(16).cast::<u8>());
                                                    let l8 = i32::from(
                                                        *ptr0
                                                            .add(16 + 3 * ::core::mem::size_of::<*const u8>())
                                                            .cast::<u8>(),
                                                    );
                                                    DnsErrorPayload {
                                                        rcode: match l4 {
                                                            0 => None,
                                                            1 => {
                                                                let e = {
                                                                    let l5 = *ptr0
                                                                        .add(16 + 1 * ::core::mem::size_of::<*const u8>())
                                                                        .cast::<*mut u8>();
                                                                    let l6 = *ptr0
                                                                        .add(16 + 2 * ::core::mem::size_of::<*const u8>())
                                                                        .cast::<usize>();
                                                                    let len7 = l6;
                                                                    let bytes7 = _rt::Vec::from_raw_parts(
                                                                        l5.cast(),
                                                                        len7,
                                                                        len7,
                                                                    );
                                                                    _rt::string_lift(bytes7)
                                                                };
                                                                Some(e)
                                                            }
                                                            _ => _rt::invalid_enum_discriminant(),
                                                        },
                                                        info_code: match l8 {
                                                            0 => None,
                                                            1 => {
                                                                let e = {
                                                                    let l9 = i32::from(
                                                                        *ptr0
                                                                            .add(18 + 3 * ::core::mem::size_of::<*const u8>())
                                                                            .cast::<u16>(),
                                                                    );
                                                                    l9 as u16
                                                                };
                                                                Some(e)
                                                            }
                                                            _ => _rt::invalid_enum_discriminant(),
                                                        },
                                                    }
                                                };
                                                ErrorCode::DnsError(e65)
                                            }
                                            2 => ErrorCode::DestinationNotFound,
                                            3 => ErrorCode::DestinationUnavailable,
                                            4 => ErrorCode::DestinationIpProhibited,
                                            5 => ErrorCode::DestinationIpUnroutable,
                                            6 => ErrorCode::ConnectionRefused,
                                            7 => ErrorCode::ConnectionTerminated,
                                            8 => ErrorCode::ConnectionTimeout,
                                            9 => ErrorCode::ConnectionReadTimeout,
                                            10 => ErrorCode::ConnectionWriteTimeout,
                                            11 => ErrorCode::ConnectionLimitReached,
                                            12 => ErrorCode::TlsProtocolError,
                                            13 => ErrorCode::TlsCertificateError,
                                            14 => {
                                                let e65 = {
                                                    let l10 = i32::from(*ptr0.add(16).cast::<u8>());
                                                    let l12 = i32::from(
                                                        *ptr0
                                                            .add(16 + 1 * ::core::mem::size_of::<*const u8>())
                                                            .cast::<u8>(),
                                                    );
                                                    TlsAlertReceivedPayload {
                                                        alert_id: match l10 {
                                                            0 => None,
                                                            1 => {
                                                                let e = {
                                                                    let l11 = i32::from(*ptr0.add(17).cast::<u8>());
                                                                    l11 as u8
                                                                };
                                                                Some(e)
                                                            }
                                                            _ => _rt::invalid_enum_discriminant(),
                                                        },
                                                        alert_message: match l12 {
                                                            0 => None,
                                                            1 => {
                                                                let e = {
                                                                    let l13 = *ptr0
                                                                        .add(16 + 2 * ::core::mem::size_of::<*const u8>())
                                                                        .cast::<*mut u8>();
                                                                    let l14 = *ptr0
                                                                        .add(16 + 3 * ::core::mem::size_of::<*const u8>())
                                                                        .cast::<usize>();
                                                                    let len15 = l14;
                                                                    let bytes15 = _rt::Vec::from_raw_parts(
                                                                        l13.cast(),
                                                                        len15,
                                                                        len15,
                                                                    );
                                                                    _rt::string_lift(bytes15)
                                                                };
                                                                Some(e)
                                                            }
                                                            _ => _rt::invalid_enum_discriminant(),
                                                        },
                                                    }
                                                };
                                                ErrorCode::TlsAlertReceived(e65)
                                            }
                                            15 => ErrorCode::HttpRequestDenied,
                                            16 => ErrorCode::HttpRequestLengthRequired,
                                            17 => {
                                                let e65 = {
                                                    let l16 = i32::from(*ptr0.add(16).cast::<u8>());
                                                    match l16 {
                                                        0 => None,
                                                        1 => {
                                                            let e = {
                                                                let l17 = *ptr0.add(24).cast::<i64>();
                                                                l17 as u64
                                                            };
                                                            Some(e)
                                                        }
                                                        _ => _rt::invalid_enum_discriminant(),
                                                    }
                                                };
                                                ErrorCode::HttpRequestBodySize(e65)
                                            }
                                            18 => ErrorCode::HttpRequestMethodInvalid,
                                            19 => ErrorCode::HttpRequestUriInvalid,
                                            20 => ErrorCode::HttpRequestUriTooLong,
                                            21 => {
                                                let e65 = {
                                                    let l18 = i32::from(*ptr0.add(16).cast::<u8>());
                                                    match l18 {
                                                        0 => None,
                                                        1 => {
                                                            let e = {
                                                                let l19 = *ptr0.add(20).cast::<i32>();
                                                                l19 as u32
                                                            };
                                                            Some(e)
                                                        }
                                                        _ => _rt::invalid_enum_discriminant(),
                                                    }
                                                };
                                                ErrorCode::HttpRequestHeaderSectionSize(e65)
                                            }
                                            22 => {
                                                let e65 = {
                                                    let l20 = i32::from(*ptr0.add(16).cast::<u8>());
                                                    match l20 {
                                                        0 => None,
                                                        1 => {
                                                            let e = {
                                                                let l21 = i32::from(
                                                                    *ptr0
                                                                        .add(16 + 1 * ::core::mem::size_of::<*const u8>())
                                                                        .cast::<u8>(),
                                                                );
                                                                let l25 = i32::from(
                                                                    *ptr0
                                                                        .add(16 + 4 * ::core::mem::size_of::<*const u8>())
                                                                        .cast::<u8>(),
                                                                );
                                                                FieldSizePayload {
                                                                    field_name: match l21 {
                                                                        0 => None,
                                                                        1 => {
                                                                            let e = {
                                                                                let l22 = *ptr0
                                                                                    .add(16 + 2 * ::core::mem::size_of::<*const u8>())
                                                                                    .cast::<*mut u8>();
                                                                                let l23 = *ptr0
                                                                                    .add(16 + 3 * ::core::mem::size_of::<*const u8>())
                                                                                    .cast::<usize>();
                                                                                let len24 = l23;
                                                                                let bytes24 = _rt::Vec::from_raw_parts(
                                                                                    l22.cast(),
                                                                                    len24,
                                                                                    len24,
                                                                                );
                                                                                _rt::string_lift(bytes24)
                                                                            };
                                                                            Some(e)
                                                                        }
                                                                        _ => _rt::invalid_enum_discriminant(),
                                                                    },
                                                                    field_size: match l25 {
                                                                        0 => None,
                                                                        1 => {
                                                                            let e = {
                                                                                let l26 = *ptr0
                                                                                    .add(20 + 4 * ::core::mem::size_of::<*const u8>())
                                                                                    .cast::<i32>();
                                                                                l26 as u32
                                                                            };
                                                                            Some(e)
                                                                        }
                                                                        _ => _rt::invalid_enum_discriminant(),
                                                                    },
                                                                }
                                                            };
                                                            Some(e)
                                                        }
                                                        _ => _rt::invalid_enum_discriminant(),
                                                    }
                                                };
                                                ErrorCode::HttpRequestHeaderSize(e65)
                                            }
                                            23 => {
                                                let e65 = {
                                                    let l27 = i32::from(*ptr0.add(16).cast::<u8>());
                                                    match l27 {
                                                        0 => None,
                                                        1 => {
                                                            let e = {
                                                                let l28 = *ptr0.add(20).cast::<i32>();
                                                                l28 as u32
                                                            };
                                                            Some(e)
                                                        }
                                                        _ => _rt::invalid_enum_discriminant(),
                                                    }
                                                };
                                                ErrorCode::HttpRequestTrailerSectionSize(e65)
                                            }
                                            24 => {
                                                let e65 = {
                                                    let l29 = i32::from(*ptr0.add(16).cast::<u8>());
                                                    let l33 = i32::from(
                                                        *ptr0
                                                            .add(16 + 3 * ::core::mem::size_of::<*const u8>())
                                                            .cast::<u8>(),
                                                    );
                                                    FieldSizePayload {
                                                        field_name: match l29 {
                                                            0 => None,
                                                            1 => {
                                                                let e = {
                                                                    let l30 = *ptr0
                                                                        .add(16 + 1 * ::core::mem::size_of::<*const u8>())
                                                                        .cast::<*mut u8>();
                                                                    let l31 = *ptr0
                                                                        .add(16 + 2 * ::core::mem::size_of::<*const u8>())
                                                                        .cast::<usize>();
                                                                    let len32 = l31;
                                                                    let bytes32 = _rt::Vec::from_raw_parts(
                                                                        l30.cast(),
                                                                        len32,
                                                                        len32,
                                                                    );
                                                                    _rt::string_lift(bytes32)
                                                                };
                                                                Some(e)
                                                            }
                                                            _ => _rt::invalid_enum_discriminant(),
                                                        },
                                                        field_size: match l33 {
                                                            0 => None,
                                                            1 => {
                                                                let e = {
                                                                    let l34 = *ptr0
                                                                        .add(20 + 3 * ::core::mem::size_of::<*const u8>())
                                                                        .cast::<i32>();
                                                                    l34 as u32
                                                                };
                                                                Some(e)
                                                            }
                                                            _ => _rt::invalid_enum_discriminant(),
                                                        },
                                                    }
                                                };
                                                ErrorCode::HttpRequestTrailerSize(e65)
                                            }
                                            25 => ErrorCode::HttpResponseIncomplete,
                                            26 => {
                                                let e65 = {
                                                    let l35 = i32::from(*ptr0.add(16).cast::<u8>());
                                                    match l35 {
                                                        0 => None,
                                                        1 => {
                                                            let e = {
                                                                let l36 = *ptr0.add(20).cast::<i32>();
                                                                l36 as u32
                                                            };
                                                            Some(e)
                                                        }
                                                        _ => _rt::invalid_enum_discriminant(),
                                                    }
                                                };
                                                ErrorCode::HttpResponseHeaderSectionSize(e65)
                                            }
                                            27 => {
                                                let e65 = {
                                                    let l37 = i32::from(*ptr0.add(16).cast::<u8>());
                                                    let l41 = i32::from(
                                                        *ptr0
                                                            .add(16 + 3 * ::core::mem::size_of::<*const u8>())
                                                            .cast::<u8>(),
                                                    );
                                                    FieldSizePayload {
                                                        field_name: match l37 {
                                                            0 => None,
                                                            1 => {
                                                                let e = {
                                                                    let l38 = *ptr0
                                                                        .add(16 + 1 * ::core::mem::size_of::<*const u8>())
                                                                        .cast::<*mut u8>();
                                                                    let l39 = *ptr0
                                                                        .add(16 + 2 * ::core::mem::size_of::<*const u8>())
                                                                        .cast::<usize>();
                                                                    let len40 = l39;
                                                                    let bytes40 = _rt::Vec::from_raw_parts(
                                                                        l38.cast(),
                                                                        len40,
                                                                        len40,
                                                                    );
                                                                    _rt::string_lift(bytes40)
                                                                };
                                                                Some(e)
                                                            }
                                                            _ => _rt::invalid_enum_discriminant(),
                                                        },
                                                        field_size: match l41 {
                                                            0 => None,
                                                            1 => {
                                                                let e = {
                                                                    let l42 = *ptr0
                                                                        .add(20 + 3 * ::core::mem::size_of::<*const u8>())
                                                                        .cast::<i32>();
                                                                    l42 as u32
                                                                };
                                                                Some(e)
                                                            }
                                                            _ => _rt::invalid_enum_discriminant(),
                                                        },
                                                    }
                                                };
                                                ErrorCode::HttpResponseHeaderSize(e65)
                                            }
                                            28 => {
                                                let e65 = {
                                                    let l43 = i32::from(*ptr0.add(16).cast::<u8>());
                                                    match l43 {
                                                        0 => None,
                                                        1 => {
                                                            let e = {
                                                                let l44 = *ptr0.add(24).cast::<i64>();
                                                                l44 as u64
                                                            };
                                                            Some(e)
                                                        }
                                                        _ => _rt::invalid_enum_discriminant(),
                                                    }
                                                };
                                                ErrorCode::HttpResponseBodySize(e65)
                                            }
                                            29 => {
                                                let e65 = {
                                                    let l45 = i32::from(*ptr0.add(16).cast::<u8>());
                                                    match l45 {
                                                        0 => None,
                                                        1 => {
                                                            let e = {
                                                                let l46 = *ptr0.add(20).cast::<i32>();
                                                                l46 as u32
                                                            };
                                                            Some(e)
                                                        }
                                                        _ => _rt::invalid_enum_discriminant(),
                                                    }
                                                };
                                                ErrorCode::HttpResponseTrailerSectionSize(e65)
                                            }
                                            30 => {
                                                let e65 = {
                                                    let l47 = i32::from(*ptr0.add(16).cast::<u8>());
                                                    let l51 = i32::from(
                                                        *ptr0
                                                            .add(16 + 3 * ::core::mem::size_of::<*const u8>())
                                                            .cast::<u8>(),
                                                    );
                                                    FieldSizePayload {
                                                        field_name: match l47 {
                                                            0 => None,
                                                            1 => {
                                                                let e = {
                                                                    let l48 = *ptr0
                                                                        .add(16 + 1 * ::core::mem::size_of::<*const u8>())
                                                                        .cast::<*mut u8>();
                                                                    let l49 = *ptr0
                                                                        .add(16 + 2 * ::core::mem::size_of::<*const u8>())
                                                                        .cast::<usize>();
                                                                    let len50 = l49;
                                                                    let bytes50 = _rt::Vec::from_raw_parts(
                                                                        l48.cast(),
                                                                        len50,
                                                                        len50,
                                                                    );
                                                                    _rt::string_lift(bytes50)
                                                                };
                                                                Some(e)
                                                            }
                                                            _ => _rt::invalid_enum_discriminant(),
                                                        },
                                                        field_size: match l51 {
                                                            0 => None,
                                                            1 => {
                                                                let e = {
                                                                    let l52 = *ptr0
                                                                        .add(20 + 3 * ::core::mem::size_of::<*const u8>())
                                                                        .cast::<i32>();
                                                                    l52 as u32
                                                                };
                                                                Some(e)
                                                            }
                                                            _ => _rt::invalid_enum_discriminant(),
                                                        },
                                                    }
                                                };
                                                ErrorCode::HttpResponseTrailerSize(e65)
                                            }
                                            31 => {
                                                let e65 = {
                                                    let l53 = i32::from(*ptr0.add(16).cast::<u8>());
                                                    match l53 {
                                                        0 => None,
                                                        1 => {
                                                            let e = {
                                                                let l54 = *ptr0
                                                                    .add(16 + 1 * ::core::mem::size_of::<*const u8>())
                                                                    .cast::<*mut u8>();
                                                                let l55 = *ptr0
                                                                    .add(16 + 2 * ::core::mem::size_of::<*const u8>())
                                                                    .cast::<usize>();
                                                                let len56 = l55;
                                                                let bytes56 = _rt::Vec::from_raw_parts(
                                                                    l54.cast(),
                                                                    len56,
                                                                    len56,
                                                                );
                                                                _rt::string_lift(bytes56)
                                                            };
                                                            Some(e)
                                                        }
                                                        _ => _rt::invalid_enum_discriminant(),
                                                    }
                                                };
                                                ErrorCode::HttpResponseTransferCoding(e65)
                                            }
                                            32 => {
                                                let e65 = {
                                                    let l57 = i32::from(*ptr0.add(16).cast::<u8>());
                                                    match l57 {
                                                        0 => None,
                                                        1 => {
                                                            let e = {
                                                                let l58 = *ptr0
                                                                    .add(16 + 1 * ::core::mem::size_of::<*const u8>())
                                                                    .cast::<*mut u8>();
                                                                let l59 = *ptr0
                                                                    .add(16 + 2 * ::core::mem::size_of::<*const u8>())
                                                                    .cast::<usize>();
                                                                let len60 = l59;
                                                                let bytes60 = _rt::Vec::from_raw_parts(
                                                                    l58.cast(),
                                                                    len60,
                                                                    len60,
                                                                );
                                                                _rt::string_lift(bytes60)
                                                            };
                                                            Some(e)
                                                        }
                                                        _ => _rt::invalid_enum_discriminant(),
                                                    }
                                                };
                                                ErrorCode::HttpResponseContentCoding(e65)
                                            }
                                            33 => ErrorCode::HttpResponseTimeout,
                                            34 => ErrorCode::HttpUpgradeFailed,
                                            35 => ErrorCode::HttpProtocolError,
                                            36 => ErrorCode::LoopDetected,
                                            37 => ErrorCode::ConfigurationError,
                                            n => {
                                                if true {
                                                    match (&n, &38) {
                                                        (left_val, right_val) => {
                                                            if !(*left_val == *right_val) {
                                                                let kind = ::core::panicking::AssertKind::Eq;
                                                                ::core::panicking::assert_failed(
                                                                    kind,
                                                                    &*left_val,
                                                                    &*right_val,
                                                                    ::core::option::Option::Some(
                                                                        format_args!("invalid enum discriminant"),
                                                                    ),
                                                                );
                                                            }
                                                        }
                                                    };
                                                }
                                                let e65 = {
                                                    let l61 = i32::from(*ptr0.add(16).cast::<u8>());
                                                    match l61 {
                                                        0 => None,
                                                        1 => {
                                                            let e = {
                                                                let l62 = *ptr0
                                                                    .add(16 + 1 * ::core::mem::size_of::<*const u8>())
                                                                    .cast::<*mut u8>();
                                                                let l63 = *ptr0
                                                                    .add(16 + 2 * ::core::mem::size_of::<*const u8>())
                                                                    .cast::<usize>();
                                                                let len64 = l63;
                                                                let bytes64 = _rt::Vec::from_raw_parts(
                                                                    l62.cast(),
                                                                    len64,
                                                                    len64,
                                                                );
                                                                _rt::string_lift(bytes64)
                                                            };
                                                            Some(e)
                                                        }
                                                        _ => _rt::invalid_enum_discriminant(),
                                                    }
                                                };
                                                ErrorCode::InternalError(e65)
                                            }
                                        };
                                        v65
                                    };
                                    Some(e)
                                }
                                _ => _rt::invalid_enum_discriminant(),
                            };
                            result66
                        }
                    }
                    impl Fields {
                        #[allow(unused_unsafe, clippy::all)]
                        /// Construct an empty HTTP Fields.
                        ///
                        /// The resulting `fields` is mutable.
                        #[allow(async_fn_in_trait)]
                        pub fn new() -> Self {
                            unsafe {
                                #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[constructor]fields"]
                                    fn wit_import0() -> i32;
                                }
                                let ret = wit_import0();
                                Fields::from_handle(ret as u32)
                            }
                        }
                    }
                    impl Fields {
                        #[allow(unused_unsafe, clippy::all)]
                        /// Construct an HTTP Fields.
                        ///
                        /// The resulting `fields` is mutable.
                        ///
                        /// The list represents each key-value pair in the Fields. Keys
                        /// which have multiple values are represented by multiple entries in this
                        /// list with the same key.
                        ///
                        /// The tuple is a pair of the field key, represented as a string, and
                        /// Value, represented as a list of bytes. In a valid Fields, all keys
                        /// and values are valid UTF-8 strings. However, values are not always
                        /// well-formed, so they are represented as a raw list of bytes.
                        ///
                        /// An error result will be returned if any header or value was
                        /// syntactically invalid, or if a header was forbidden.
                        #[allow(async_fn_in_trait)]
                        pub fn from_list(
                            entries: &[(FieldKey, FieldValue)],
                        ) -> Result<Fields, HeaderError> {
                            unsafe {
                                #[repr(align(4))]
                                struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                                let mut ret_area = RetArea(
                                    [::core::mem::MaybeUninit::uninit(); 8],
                                );
                                let vec3 = entries;
                                let len3 = vec3.len();
                                let layout3 = _rt::alloc::Layout::from_size_align(
                                        vec3.len() * (4 * ::core::mem::size_of::<*const u8>()),
                                        ::core::mem::size_of::<*const u8>(),
                                    )
                                    .unwrap();
                                let (result3, _cleanup3) = ::spin_sdk::wit_bindgen::rt::Cleanup::new(
                                    layout3,
                                );
                                for (i, e) in vec3.into_iter().enumerate() {
                                    let base = result3
                                        .add(i * (4 * ::core::mem::size_of::<*const u8>()));
                                    {
                                        let (t0_0, t0_1) = e;
                                        let vec1 = t0_0;
                                        let ptr1 = vec1.as_ptr().cast::<u8>();
                                        let len1 = vec1.len();
                                        *base
                                            .add(::core::mem::size_of::<*const u8>())
                                            .cast::<usize>() = len1;
                                        *base.add(0).cast::<*mut u8>() = ptr1.cast_mut();
                                        let vec2 = t0_1;
                                        let ptr2 = vec2.as_ptr().cast::<u8>();
                                        let len2 = vec2.len();
                                        *base
                                            .add(3 * ::core::mem::size_of::<*const u8>())
                                            .cast::<usize>() = len2;
                                        *base
                                            .add(2 * ::core::mem::size_of::<*const u8>())
                                            .cast::<*mut u8>() = ptr2.cast_mut();
                                    }
                                }
                                let ptr4 = ret_area.0.as_mut_ptr().cast::<u8>();
                                #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[static]fields.from-list"]
                                    fn wit_import5(_: *mut u8, _: usize, _: *mut u8);
                                }
                                wit_import5(result3, len3, ptr4);
                                let l6 = i32::from(*ptr4.add(0).cast::<u8>());
                                let result10 = match l6 {
                                    0 => {
                                        let e = {
                                            let l7 = *ptr4.add(4).cast::<i32>();
                                            Fields::from_handle(l7 as u32)
                                        };
                                        Ok(e)
                                    }
                                    1 => {
                                        let e = {
                                            let l8 = i32::from(*ptr4.add(4).cast::<u8>());
                                            let v9 = match l8 {
                                                0 => HeaderError::InvalidSyntax,
                                                1 => HeaderError::Forbidden,
                                                n => {
                                                    if true {
                                                        match (&n, &2) {
                                                            (left_val, right_val) => {
                                                                if !(*left_val == *right_val) {
                                                                    let kind = ::core::panicking::AssertKind::Eq;
                                                                    ::core::panicking::assert_failed(
                                                                        kind,
                                                                        &*left_val,
                                                                        &*right_val,
                                                                        ::core::option::Option::Some(
                                                                            format_args!("invalid enum discriminant"),
                                                                        ),
                                                                    );
                                                                }
                                                            }
                                                        };
                                                    }
                                                    HeaderError::Immutable
                                                }
                                            };
                                            v9
                                        };
                                        Err(e)
                                    }
                                    _ => _rt::invalid_enum_discriminant(),
                                };
                                result10
                            }
                        }
                    }
                    impl Fields {
                        #[allow(unused_unsafe, clippy::all)]
                        /// Get all of the values corresponding to a key. If the key is not present
                        /// in this `fields`, an empty list is returned. However, if the key is
                        /// present but empty, this is represented by a list with one or more
                        /// empty field-values present.
                        #[allow(async_fn_in_trait)]
                        pub fn get(&self, name: &str) -> _rt::Vec<FieldValue> {
                            unsafe {
                                #[repr(align(4))]
                                struct RetArea(
                                    [::core::mem::MaybeUninit<
                                        u8,
                                    >; 2 * ::core::mem::size_of::<*const u8>()],
                                );
                                let mut ret_area = RetArea(
                                    [::core::mem::MaybeUninit::uninit(); 2
                                        * ::core::mem::size_of::<*const u8>()],
                                );
                                let vec0 = name;
                                let ptr0 = vec0.as_ptr().cast::<u8>();
                                let len0 = vec0.len();
                                let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                                #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[method]fields.get"]
                                    fn wit_import2(_: i32, _: *mut u8, _: usize, _: *mut u8);
                                }
                                wit_import2(
                                    (self).handle() as i32,
                                    ptr0.cast_mut(),
                                    len0,
                                    ptr1,
                                );
                                let l3 = *ptr1.add(0).cast::<*mut u8>();
                                let l4 = *ptr1
                                    .add(::core::mem::size_of::<*const u8>())
                                    .cast::<usize>();
                                let base8 = l3;
                                let len8 = l4;
                                let mut result8 = _rt::Vec::with_capacity(len8);
                                for i in 0..len8 {
                                    let base = base8
                                        .add(i * (2 * ::core::mem::size_of::<*const u8>()));
                                    let e8 = {
                                        let l5 = *base.add(0).cast::<*mut u8>();
                                        let l6 = *base
                                            .add(::core::mem::size_of::<*const u8>())
                                            .cast::<usize>();
                                        let len7 = l6;
                                        _rt::Vec::from_raw_parts(l5.cast(), len7, len7)
                                    };
                                    result8.push(e8);
                                }
                                _rt::cabi_dealloc(
                                    base8,
                                    len8 * (2 * ::core::mem::size_of::<*const u8>()),
                                    ::core::mem::size_of::<*const u8>(),
                                );
                                let result9 = result8;
                                result9
                            }
                        }
                    }
                    impl Fields {
                        #[allow(unused_unsafe, clippy::all)]
                        /// Returns `true` when the key is present in this `fields`. If the key is
                        /// syntactically invalid, `false` is returned.
                        #[allow(async_fn_in_trait)]
                        pub fn has(&self, name: &str) -> bool {
                            unsafe {
                                let vec0 = name;
                                let ptr0 = vec0.as_ptr().cast::<u8>();
                                let len0 = vec0.len();
                                #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[method]fields.has"]
                                    fn wit_import1(_: i32, _: *mut u8, _: usize) -> i32;
                                }
                                let ret = wit_import1(
                                    (self).handle() as i32,
                                    ptr0.cast_mut(),
                                    len0,
                                );
                                _rt::bool_lift(ret as u8)
                            }
                        }
                    }
                    impl Fields {
                        #[allow(unused_unsafe, clippy::all)]
                        /// Set all of the values for a key. Clears any existing values for that
                        /// key, if they have been set.
                        ///
                        /// Fails with `header-error.immutable` if the `fields` are immutable.
                        #[allow(async_fn_in_trait)]
                        pub fn set(
                            &self,
                            name: &str,
                            value: &[FieldValue],
                        ) -> Result<(), HeaderError> {
                            unsafe {
                                #[repr(align(1))]
                                struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                                let mut ret_area = RetArea(
                                    [::core::mem::MaybeUninit::uninit(); 2],
                                );
                                let vec0 = name;
                                let ptr0 = vec0.as_ptr().cast::<u8>();
                                let len0 = vec0.len();
                                let vec2 = value;
                                let len2 = vec2.len();
                                let layout2 = _rt::alloc::Layout::from_size_align(
                                        vec2.len() * (2 * ::core::mem::size_of::<*const u8>()),
                                        ::core::mem::size_of::<*const u8>(),
                                    )
                                    .unwrap();
                                let (result2, _cleanup2) = ::spin_sdk::wit_bindgen::rt::Cleanup::new(
                                    layout2,
                                );
                                for (i, e) in vec2.into_iter().enumerate() {
                                    let base = result2
                                        .add(i * (2 * ::core::mem::size_of::<*const u8>()));
                                    {
                                        let vec1 = e;
                                        let ptr1 = vec1.as_ptr().cast::<u8>();
                                        let len1 = vec1.len();
                                        *base
                                            .add(::core::mem::size_of::<*const u8>())
                                            .cast::<usize>() = len1;
                                        *base.add(0).cast::<*mut u8>() = ptr1.cast_mut();
                                    }
                                }
                                let ptr3 = ret_area.0.as_mut_ptr().cast::<u8>();
                                #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[method]fields.set"]
                                    fn wit_import4(
                                        _: i32,
                                        _: *mut u8,
                                        _: usize,
                                        _: *mut u8,
                                        _: usize,
                                        _: *mut u8,
                                    );
                                }
                                wit_import4(
                                    (self).handle() as i32,
                                    ptr0.cast_mut(),
                                    len0,
                                    result2,
                                    len2,
                                    ptr3,
                                );
                                let l5 = i32::from(*ptr3.add(0).cast::<u8>());
                                let result8 = match l5 {
                                    0 => {
                                        let e = ();
                                        Ok(e)
                                    }
                                    1 => {
                                        let e = {
                                            let l6 = i32::from(*ptr3.add(1).cast::<u8>());
                                            let v7 = match l6 {
                                                0 => HeaderError::InvalidSyntax,
                                                1 => HeaderError::Forbidden,
                                                n => {
                                                    if true {
                                                        match (&n, &2) {
                                                            (left_val, right_val) => {
                                                                if !(*left_val == *right_val) {
                                                                    let kind = ::core::panicking::AssertKind::Eq;
                                                                    ::core::panicking::assert_failed(
                                                                        kind,
                                                                        &*left_val,
                                                                        &*right_val,
                                                                        ::core::option::Option::Some(
                                                                            format_args!("invalid enum discriminant"),
                                                                        ),
                                                                    );
                                                                }
                                                            }
                                                        };
                                                    }
                                                    HeaderError::Immutable
                                                }
                                            };
                                            v7
                                        };
                                        Err(e)
                                    }
                                    _ => _rt::invalid_enum_discriminant(),
                                };
                                result8
                            }
                        }
                    }
                    impl Fields {
                        #[allow(unused_unsafe, clippy::all)]
                        /// Delete all values for a key. Does nothing if no values for the key
                        /// exist.
                        ///
                        /// Fails with `header-error.immutable` if the `fields` are immutable.
                        #[allow(async_fn_in_trait)]
                        pub fn delete(&self, name: &str) -> Result<(), HeaderError> {
                            unsafe {
                                #[repr(align(1))]
                                struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                                let mut ret_area = RetArea(
                                    [::core::mem::MaybeUninit::uninit(); 2],
                                );
                                let vec0 = name;
                                let ptr0 = vec0.as_ptr().cast::<u8>();
                                let len0 = vec0.len();
                                let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                                #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[method]fields.delete"]
                                    fn wit_import2(_: i32, _: *mut u8, _: usize, _: *mut u8);
                                }
                                wit_import2(
                                    (self).handle() as i32,
                                    ptr0.cast_mut(),
                                    len0,
                                    ptr1,
                                );
                                let l3 = i32::from(*ptr1.add(0).cast::<u8>());
                                let result6 = match l3 {
                                    0 => {
                                        let e = ();
                                        Ok(e)
                                    }
                                    1 => {
                                        let e = {
                                            let l4 = i32::from(*ptr1.add(1).cast::<u8>());
                                            let v5 = match l4 {
                                                0 => HeaderError::InvalidSyntax,
                                                1 => HeaderError::Forbidden,
                                                n => {
                                                    if true {
                                                        match (&n, &2) {
                                                            (left_val, right_val) => {
                                                                if !(*left_val == *right_val) {
                                                                    let kind = ::core::panicking::AssertKind::Eq;
                                                                    ::core::panicking::assert_failed(
                                                                        kind,
                                                                        &*left_val,
                                                                        &*right_val,
                                                                        ::core::option::Option::Some(
                                                                            format_args!("invalid enum discriminant"),
                                                                        ),
                                                                    );
                                                                }
                                                            }
                                                        };
                                                    }
                                                    HeaderError::Immutable
                                                }
                                            };
                                            v5
                                        };
                                        Err(e)
                                    }
                                    _ => _rt::invalid_enum_discriminant(),
                                };
                                result6
                            }
                        }
                    }
                    impl Fields {
                        #[allow(unused_unsafe, clippy::all)]
                        /// Append a value for a key. Does not change or delete any existing
                        /// values for that key.
                        ///
                        /// Fails with `header-error.immutable` if the `fields` are immutable.
                        #[allow(async_fn_in_trait)]
                        pub fn append(
                            &self,
                            name: &str,
                            value: &[u8],
                        ) -> Result<(), HeaderError> {
                            unsafe {
                                #[repr(align(1))]
                                struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                                let mut ret_area = RetArea(
                                    [::core::mem::MaybeUninit::uninit(); 2],
                                );
                                let vec0 = name;
                                let ptr0 = vec0.as_ptr().cast::<u8>();
                                let len0 = vec0.len();
                                let vec1 = value;
                                let ptr1 = vec1.as_ptr().cast::<u8>();
                                let len1 = vec1.len();
                                let ptr2 = ret_area.0.as_mut_ptr().cast::<u8>();
                                #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[method]fields.append"]
                                    fn wit_import3(
                                        _: i32,
                                        _: *mut u8,
                                        _: usize,
                                        _: *mut u8,
                                        _: usize,
                                        _: *mut u8,
                                    );
                                }
                                wit_import3(
                                    (self).handle() as i32,
                                    ptr0.cast_mut(),
                                    len0,
                                    ptr1.cast_mut(),
                                    len1,
                                    ptr2,
                                );
                                let l4 = i32::from(*ptr2.add(0).cast::<u8>());
                                let result7 = match l4 {
                                    0 => {
                                        let e = ();
                                        Ok(e)
                                    }
                                    1 => {
                                        let e = {
                                            let l5 = i32::from(*ptr2.add(1).cast::<u8>());
                                            let v6 = match l5 {
                                                0 => HeaderError::InvalidSyntax,
                                                1 => HeaderError::Forbidden,
                                                n => {
                                                    if true {
                                                        match (&n, &2) {
                                                            (left_val, right_val) => {
                                                                if !(*left_val == *right_val) {
                                                                    let kind = ::core::panicking::AssertKind::Eq;
                                                                    ::core::panicking::assert_failed(
                                                                        kind,
                                                                        &*left_val,
                                                                        &*right_val,
                                                                        ::core::option::Option::Some(
                                                                            format_args!("invalid enum discriminant"),
                                                                        ),
                                                                    );
                                                                }
                                                            }
                                                        };
                                                    }
                                                    HeaderError::Immutable
                                                }
                                            };
                                            v6
                                        };
                                        Err(e)
                                    }
                                    _ => _rt::invalid_enum_discriminant(),
                                };
                                result7
                            }
                        }
                    }
                    impl Fields {
                        #[allow(unused_unsafe, clippy::all)]
                        /// Retrieve the full set of keys and values in the Fields. Like the
                        /// constructor, the list represents each key-value pair.
                        ///
                        /// The outer list represents each key-value pair in the Fields. Keys
                        /// which have multiple values are represented by multiple entries in this
                        /// list with the same key.
                        #[allow(async_fn_in_trait)]
                        pub fn entries(&self) -> _rt::Vec<(FieldKey, FieldValue)> {
                            unsafe {
                                #[repr(align(4))]
                                struct RetArea(
                                    [::core::mem::MaybeUninit<
                                        u8,
                                    >; 2 * ::core::mem::size_of::<*const u8>()],
                                );
                                let mut ret_area = RetArea(
                                    [::core::mem::MaybeUninit::uninit(); 2
                                        * ::core::mem::size_of::<*const u8>()],
                                );
                                let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                                #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[method]fields.entries"]
                                    fn wit_import1(_: i32, _: *mut u8);
                                }
                                wit_import1((self).handle() as i32, ptr0);
                                let l2 = *ptr0.add(0).cast::<*mut u8>();
                                let l3 = *ptr0
                                    .add(::core::mem::size_of::<*const u8>())
                                    .cast::<usize>();
                                let base10 = l2;
                                let len10 = l3;
                                let mut result10 = _rt::Vec::with_capacity(len10);
                                for i in 0..len10 {
                                    let base = base10
                                        .add(i * (4 * ::core::mem::size_of::<*const u8>()));
                                    let e10 = {
                                        let l4 = *base.add(0).cast::<*mut u8>();
                                        let l5 = *base
                                            .add(::core::mem::size_of::<*const u8>())
                                            .cast::<usize>();
                                        let len6 = l5;
                                        let bytes6 = _rt::Vec::from_raw_parts(
                                            l4.cast(),
                                            len6,
                                            len6,
                                        );
                                        let l7 = *base
                                            .add(2 * ::core::mem::size_of::<*const u8>())
                                            .cast::<*mut u8>();
                                        let l8 = *base
                                            .add(3 * ::core::mem::size_of::<*const u8>())
                                            .cast::<usize>();
                                        let len9 = l8;
                                        (
                                            _rt::string_lift(bytes6),
                                            _rt::Vec::from_raw_parts(l7.cast(), len9, len9),
                                        )
                                    };
                                    result10.push(e10);
                                }
                                _rt::cabi_dealloc(
                                    base10,
                                    len10 * (4 * ::core::mem::size_of::<*const u8>()),
                                    ::core::mem::size_of::<*const u8>(),
                                );
                                let result11 = result10;
                                result11
                            }
                        }
                    }
                    impl Fields {
                        #[allow(unused_unsafe, clippy::all)]
                        /// Make a deep copy of the Fields. Equivelant in behavior to calling the
                        /// `fields` constructor on the return value of `entries`. The resulting
                        /// `fields` is mutable.
                        #[allow(async_fn_in_trait)]
                        pub fn clone(&self) -> Fields {
                            unsafe {
                                #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[method]fields.clone"]
                                    fn wit_import0(_: i32) -> i32;
                                }
                                let ret = wit_import0((self).handle() as i32);
                                Fields::from_handle(ret as u32)
                            }
                        }
                    }
                    impl IncomingRequest {
                        #[allow(unused_unsafe, clippy::all)]
                        /// Returns the method of the incoming request.
                        #[allow(async_fn_in_trait)]
                        pub fn method(&self) -> Method {
                            unsafe {
                                #[repr(align(4))]
                                struct RetArea(
                                    [::core::mem::MaybeUninit<
                                        u8,
                                    >; 3 * ::core::mem::size_of::<*const u8>()],
                                );
                                let mut ret_area = RetArea(
                                    [::core::mem::MaybeUninit::uninit(); 3
                                        * ::core::mem::size_of::<*const u8>()],
                                );
                                let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                                #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[method]incoming-request.method"]
                                    fn wit_import1(_: i32, _: *mut u8);
                                }
                                wit_import1((self).handle() as i32, ptr0);
                                let l2 = i32::from(*ptr0.add(0).cast::<u8>());
                                let v6 = match l2 {
                                    0 => Method::Get,
                                    1 => Method::Head,
                                    2 => Method::Post,
                                    3 => Method::Put,
                                    4 => Method::Delete,
                                    5 => Method::Connect,
                                    6 => Method::Options,
                                    7 => Method::Trace,
                                    8 => Method::Patch,
                                    n => {
                                        if true {
                                            match (&n, &9) {
                                                (left_val, right_val) => {
                                                    if !(*left_val == *right_val) {
                                                        let kind = ::core::panicking::AssertKind::Eq;
                                                        ::core::panicking::assert_failed(
                                                            kind,
                                                            &*left_val,
                                                            &*right_val,
                                                            ::core::option::Option::Some(
                                                                format_args!("invalid enum discriminant"),
                                                            ),
                                                        );
                                                    }
                                                }
                                            };
                                        }
                                        let e6 = {
                                            let l3 = *ptr0
                                                .add(::core::mem::size_of::<*const u8>())
                                                .cast::<*mut u8>();
                                            let l4 = *ptr0
                                                .add(2 * ::core::mem::size_of::<*const u8>())
                                                .cast::<usize>();
                                            let len5 = l4;
                                            let bytes5 = _rt::Vec::from_raw_parts(
                                                l3.cast(),
                                                len5,
                                                len5,
                                            );
                                            _rt::string_lift(bytes5)
                                        };
                                        Method::Other(e6)
                                    }
                                };
                                let result7 = v6;
                                result7
                            }
                        }
                    }
                    impl IncomingRequest {
                        #[allow(unused_unsafe, clippy::all)]
                        /// Returns the path with query parameters from the request, as a string.
                        #[allow(async_fn_in_trait)]
                        pub fn path_with_query(&self) -> Option<_rt::String> {
                            unsafe {
                                #[repr(align(4))]
                                struct RetArea(
                                    [::core::mem::MaybeUninit<
                                        u8,
                                    >; 3 * ::core::mem::size_of::<*const u8>()],
                                );
                                let mut ret_area = RetArea(
                                    [::core::mem::MaybeUninit::uninit(); 3
                                        * ::core::mem::size_of::<*const u8>()],
                                );
                                let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                                #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[method]incoming-request.path-with-query"]
                                    fn wit_import1(_: i32, _: *mut u8);
                                }
                                wit_import1((self).handle() as i32, ptr0);
                                let l2 = i32::from(*ptr0.add(0).cast::<u8>());
                                let result6 = match l2 {
                                    0 => None,
                                    1 => {
                                        let e = {
                                            let l3 = *ptr0
                                                .add(::core::mem::size_of::<*const u8>())
                                                .cast::<*mut u8>();
                                            let l4 = *ptr0
                                                .add(2 * ::core::mem::size_of::<*const u8>())
                                                .cast::<usize>();
                                            let len5 = l4;
                                            let bytes5 = _rt::Vec::from_raw_parts(
                                                l3.cast(),
                                                len5,
                                                len5,
                                            );
                                            _rt::string_lift(bytes5)
                                        };
                                        Some(e)
                                    }
                                    _ => _rt::invalid_enum_discriminant(),
                                };
                                result6
                            }
                        }
                    }
                    impl IncomingRequest {
                        #[allow(unused_unsafe, clippy::all)]
                        /// Returns the protocol scheme from the request.
                        #[allow(async_fn_in_trait)]
                        pub fn scheme(&self) -> Option<Scheme> {
                            unsafe {
                                #[repr(align(4))]
                                struct RetArea(
                                    [::core::mem::MaybeUninit<
                                        u8,
                                    >; 4 * ::core::mem::size_of::<*const u8>()],
                                );
                                let mut ret_area = RetArea(
                                    [::core::mem::MaybeUninit::uninit(); 4
                                        * ::core::mem::size_of::<*const u8>()],
                                );
                                let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                                #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[method]incoming-request.scheme"]
                                    fn wit_import1(_: i32, _: *mut u8);
                                }
                                wit_import1((self).handle() as i32, ptr0);
                                let l2 = i32::from(*ptr0.add(0).cast::<u8>());
                                let result8 = match l2 {
                                    0 => None,
                                    1 => {
                                        let e = {
                                            let l3 = i32::from(
                                                *ptr0.add(::core::mem::size_of::<*const u8>()).cast::<u8>(),
                                            );
                                            let v7 = match l3 {
                                                0 => Scheme::Http,
                                                1 => Scheme::Https,
                                                n => {
                                                    if true {
                                                        match (&n, &2) {
                                                            (left_val, right_val) => {
                                                                if !(*left_val == *right_val) {
                                                                    let kind = ::core::panicking::AssertKind::Eq;
                                                                    ::core::panicking::assert_failed(
                                                                        kind,
                                                                        &*left_val,
                                                                        &*right_val,
                                                                        ::core::option::Option::Some(
                                                                            format_args!("invalid enum discriminant"),
                                                                        ),
                                                                    );
                                                                }
                                                            }
                                                        };
                                                    }
                                                    let e7 = {
                                                        let l4 = *ptr0
                                                            .add(2 * ::core::mem::size_of::<*const u8>())
                                                            .cast::<*mut u8>();
                                                        let l5 = *ptr0
                                                            .add(3 * ::core::mem::size_of::<*const u8>())
                                                            .cast::<usize>();
                                                        let len6 = l5;
                                                        let bytes6 = _rt::Vec::from_raw_parts(
                                                            l4.cast(),
                                                            len6,
                                                            len6,
                                                        );
                                                        _rt::string_lift(bytes6)
                                                    };
                                                    Scheme::Other(e7)
                                                }
                                            };
                                            v7
                                        };
                                        Some(e)
                                    }
                                    _ => _rt::invalid_enum_discriminant(),
                                };
                                result8
                            }
                        }
                    }
                    impl IncomingRequest {
                        #[allow(unused_unsafe, clippy::all)]
                        /// Returns the authority from the request, if it was present.
                        #[allow(async_fn_in_trait)]
                        pub fn authority(&self) -> Option<_rt::String> {
                            unsafe {
                                #[repr(align(4))]
                                struct RetArea(
                                    [::core::mem::MaybeUninit<
                                        u8,
                                    >; 3 * ::core::mem::size_of::<*const u8>()],
                                );
                                let mut ret_area = RetArea(
                                    [::core::mem::MaybeUninit::uninit(); 3
                                        * ::core::mem::size_of::<*const u8>()],
                                );
                                let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                                #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[method]incoming-request.authority"]
                                    fn wit_import1(_: i32, _: *mut u8);
                                }
                                wit_import1((self).handle() as i32, ptr0);
                                let l2 = i32::from(*ptr0.add(0).cast::<u8>());
                                let result6 = match l2 {
                                    0 => None,
                                    1 => {
                                        let e = {
                                            let l3 = *ptr0
                                                .add(::core::mem::size_of::<*const u8>())
                                                .cast::<*mut u8>();
                                            let l4 = *ptr0
                                                .add(2 * ::core::mem::size_of::<*const u8>())
                                                .cast::<usize>();
                                            let len5 = l4;
                                            let bytes5 = _rt::Vec::from_raw_parts(
                                                l3.cast(),
                                                len5,
                                                len5,
                                            );
                                            _rt::string_lift(bytes5)
                                        };
                                        Some(e)
                                    }
                                    _ => _rt::invalid_enum_discriminant(),
                                };
                                result6
                            }
                        }
                    }
                    impl IncomingRequest {
                        #[allow(unused_unsafe, clippy::all)]
                        /// Get the `headers` associated with the request.
                        ///
                        /// The returned `headers` resource is immutable: `set`, `append`, and
                        /// `delete` operations will fail with `header-error.immutable`.
                        ///
                        /// The `headers` returned are a child resource: it must be dropped before
                        /// the parent `incoming-request` is dropped. Dropping this
                        /// `incoming-request` before all children are dropped will trap.
                        #[allow(async_fn_in_trait)]
                        pub fn headers(&self) -> Headers {
                            unsafe {
                                #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[method]incoming-request.headers"]
                                    fn wit_import0(_: i32) -> i32;
                                }
                                let ret = wit_import0((self).handle() as i32);
                                Fields::from_handle(ret as u32)
                            }
                        }
                    }
                    impl IncomingRequest {
                        #[allow(unused_unsafe, clippy::all)]
                        /// Gives the `incoming-body` associated with this request. Will only
                        /// return success at most once, and subsequent calls will return error.
                        #[allow(async_fn_in_trait)]
                        pub fn consume(&self) -> Result<IncomingBody, ()> {
                            unsafe {
                                #[repr(align(4))]
                                struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                                let mut ret_area = RetArea(
                                    [::core::mem::MaybeUninit::uninit(); 8],
                                );
                                let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                                #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[method]incoming-request.consume"]
                                    fn wit_import1(_: i32, _: *mut u8);
                                }
                                wit_import1((self).handle() as i32, ptr0);
                                let l2 = i32::from(*ptr0.add(0).cast::<u8>());
                                let result4 = match l2 {
                                    0 => {
                                        let e = {
                                            let l3 = *ptr0.add(4).cast::<i32>();
                                            IncomingBody::from_handle(l3 as u32)
                                        };
                                        Ok(e)
                                    }
                                    1 => {
                                        let e = ();
                                        Err(e)
                                    }
                                    _ => _rt::invalid_enum_discriminant(),
                                };
                                result4
                            }
                        }
                    }
                    impl OutgoingRequest {
                        #[allow(unused_unsafe, clippy::all)]
                        /// Construct a new `outgoing-request` with a default `method` of `GET`, and
                        /// `none` values for `path-with-query`, `scheme`, and `authority`.
                        ///
                        /// * `headers` is the HTTP Headers for the Request.
                        ///
                        /// It is possible to construct, or manipulate with the accessor functions
                        /// below, an `outgoing-request` with an invalid combination of `scheme`
                        /// and `authority`, or `headers` which are not permitted to be sent.
                        /// It is the obligation of the `outgoing-handler.handle` implementation
                        /// to reject invalid constructions of `outgoing-request`.
                        #[allow(async_fn_in_trait)]
                        pub fn new(headers: Headers) -> Self {
                            unsafe {
                                #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[constructor]outgoing-request"]
                                    fn wit_import0(_: i32) -> i32;
                                }
                                let ret = wit_import0((&headers).take_handle() as i32);
                                OutgoingRequest::from_handle(ret as u32)
                            }
                        }
                    }
                    impl OutgoingRequest {
                        #[allow(unused_unsafe, clippy::all)]
                        /// Returns the resource corresponding to the outgoing Body for this
                        /// Request.
                        ///
                        /// Returns success on the first call: the `outgoing-body` resource for
                        /// this `outgoing-request` can be retrieved at most once. Subsequent
                        /// calls will return error.
                        #[allow(async_fn_in_trait)]
                        pub fn body(&self) -> Result<OutgoingBody, ()> {
                            unsafe {
                                #[repr(align(4))]
                                struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                                let mut ret_area = RetArea(
                                    [::core::mem::MaybeUninit::uninit(); 8],
                                );
                                let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                                #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[method]outgoing-request.body"]
                                    fn wit_import1(_: i32, _: *mut u8);
                                }
                                wit_import1((self).handle() as i32, ptr0);
                                let l2 = i32::from(*ptr0.add(0).cast::<u8>());
                                let result4 = match l2 {
                                    0 => {
                                        let e = {
                                            let l3 = *ptr0.add(4).cast::<i32>();
                                            OutgoingBody::from_handle(l3 as u32)
                                        };
                                        Ok(e)
                                    }
                                    1 => {
                                        let e = ();
                                        Err(e)
                                    }
                                    _ => _rt::invalid_enum_discriminant(),
                                };
                                result4
                            }
                        }
                    }
                    impl OutgoingRequest {
                        #[allow(unused_unsafe, clippy::all)]
                        /// Get the Method for the Request.
                        #[allow(async_fn_in_trait)]
                        pub fn method(&self) -> Method {
                            unsafe {
                                #[repr(align(4))]
                                struct RetArea(
                                    [::core::mem::MaybeUninit<
                                        u8,
                                    >; 3 * ::core::mem::size_of::<*const u8>()],
                                );
                                let mut ret_area = RetArea(
                                    [::core::mem::MaybeUninit::uninit(); 3
                                        * ::core::mem::size_of::<*const u8>()],
                                );
                                let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                                #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[method]outgoing-request.method"]
                                    fn wit_import1(_: i32, _: *mut u8);
                                }
                                wit_import1((self).handle() as i32, ptr0);
                                let l2 = i32::from(*ptr0.add(0).cast::<u8>());
                                let v6 = match l2 {
                                    0 => Method::Get,
                                    1 => Method::Head,
                                    2 => Method::Post,
                                    3 => Method::Put,
                                    4 => Method::Delete,
                                    5 => Method::Connect,
                                    6 => Method::Options,
                                    7 => Method::Trace,
                                    8 => Method::Patch,
                                    n => {
                                        if true {
                                            match (&n, &9) {
                                                (left_val, right_val) => {
                                                    if !(*left_val == *right_val) {
                                                        let kind = ::core::panicking::AssertKind::Eq;
                                                        ::core::panicking::assert_failed(
                                                            kind,
                                                            &*left_val,
                                                            &*right_val,
                                                            ::core::option::Option::Some(
                                                                format_args!("invalid enum discriminant"),
                                                            ),
                                                        );
                                                    }
                                                }
                                            };
                                        }
                                        let e6 = {
                                            let l3 = *ptr0
                                                .add(::core::mem::size_of::<*const u8>())
                                                .cast::<*mut u8>();
                                            let l4 = *ptr0
                                                .add(2 * ::core::mem::size_of::<*const u8>())
                                                .cast::<usize>();
                                            let len5 = l4;
                                            let bytes5 = _rt::Vec::from_raw_parts(
                                                l3.cast(),
                                                len5,
                                                len5,
                                            );
                                            _rt::string_lift(bytes5)
                                        };
                                        Method::Other(e6)
                                    }
                                };
                                let result7 = v6;
                                result7
                            }
                        }
                    }
                    impl OutgoingRequest {
                        #[allow(unused_unsafe, clippy::all)]
                        /// Set the Method for the Request. Fails if the string present in a
                        /// `method.other` argument is not a syntactically valid method.
                        #[allow(async_fn_in_trait)]
                        pub fn set_method(&self, method: &Method) -> Result<(), ()> {
                            unsafe {
                                let (result1_0, result1_1, result1_2) = match method {
                                    Method::Get => (0i32, ::core::ptr::null_mut(), 0usize),
                                    Method::Head => (1i32, ::core::ptr::null_mut(), 0usize),
                                    Method::Post => (2i32, ::core::ptr::null_mut(), 0usize),
                                    Method::Put => (3i32, ::core::ptr::null_mut(), 0usize),
                                    Method::Delete => (4i32, ::core::ptr::null_mut(), 0usize),
                                    Method::Connect => (5i32, ::core::ptr::null_mut(), 0usize),
                                    Method::Options => (6i32, ::core::ptr::null_mut(), 0usize),
                                    Method::Trace => (7i32, ::core::ptr::null_mut(), 0usize),
                                    Method::Patch => (8i32, ::core::ptr::null_mut(), 0usize),
                                    Method::Other(e) => {
                                        let vec0 = e;
                                        let ptr0 = vec0.as_ptr().cast::<u8>();
                                        let len0 = vec0.len();
                                        (9i32, ptr0.cast_mut(), len0)
                                    }
                                };
                                #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[method]outgoing-request.set-method"]
                                    fn wit_import2(_: i32, _: i32, _: *mut u8, _: usize) -> i32;
                                }
                                let ret = wit_import2(
                                    (self).handle() as i32,
                                    result1_0,
                                    result1_1,
                                    result1_2,
                                );
                                match ret {
                                    0 => {
                                        let e = ();
                                        Ok(e)
                                    }
                                    1 => {
                                        let e = ();
                                        Err(e)
                                    }
                                    _ => _rt::invalid_enum_discriminant(),
                                }
                            }
                        }
                    }
                    impl OutgoingRequest {
                        #[allow(unused_unsafe, clippy::all)]
                        /// Get the combination of the HTTP Path and Query for the Request.
                        /// When `none`, this represents an empty Path and empty Query.
                        #[allow(async_fn_in_trait)]
                        pub fn path_with_query(&self) -> Option<_rt::String> {
                            unsafe {
                                #[repr(align(4))]
                                struct RetArea(
                                    [::core::mem::MaybeUninit<
                                        u8,
                                    >; 3 * ::core::mem::size_of::<*const u8>()],
                                );
                                let mut ret_area = RetArea(
                                    [::core::mem::MaybeUninit::uninit(); 3
                                        * ::core::mem::size_of::<*const u8>()],
                                );
                                let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                                #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[method]outgoing-request.path-with-query"]
                                    fn wit_import1(_: i32, _: *mut u8);
                                }
                                wit_import1((self).handle() as i32, ptr0);
                                let l2 = i32::from(*ptr0.add(0).cast::<u8>());
                                let result6 = match l2 {
                                    0 => None,
                                    1 => {
                                        let e = {
                                            let l3 = *ptr0
                                                .add(::core::mem::size_of::<*const u8>())
                                                .cast::<*mut u8>();
                                            let l4 = *ptr0
                                                .add(2 * ::core::mem::size_of::<*const u8>())
                                                .cast::<usize>();
                                            let len5 = l4;
                                            let bytes5 = _rt::Vec::from_raw_parts(
                                                l3.cast(),
                                                len5,
                                                len5,
                                            );
                                            _rt::string_lift(bytes5)
                                        };
                                        Some(e)
                                    }
                                    _ => _rt::invalid_enum_discriminant(),
                                };
                                result6
                            }
                        }
                    }
                    impl OutgoingRequest {
                        #[allow(unused_unsafe, clippy::all)]
                        /// Set the combination of the HTTP Path and Query for the Request.
                        /// When `none`, this represents an empty Path and empty Query. Fails is the
                        /// string given is not a syntactically valid path and query uri component.
                        #[allow(async_fn_in_trait)]
                        pub fn set_path_with_query(
                            &self,
                            path_with_query: Option<&str>,
                        ) -> Result<(), ()> {
                            unsafe {
                                let (result1_0, result1_1, result1_2) = match path_with_query {
                                    Some(e) => {
                                        let vec0 = e;
                                        let ptr0 = vec0.as_ptr().cast::<u8>();
                                        let len0 = vec0.len();
                                        (1i32, ptr0.cast_mut(), len0)
                                    }
                                    None => (0i32, ::core::ptr::null_mut(), 0usize),
                                };
                                #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[method]outgoing-request.set-path-with-query"]
                                    fn wit_import2(_: i32, _: i32, _: *mut u8, _: usize) -> i32;
                                }
                                let ret = wit_import2(
                                    (self).handle() as i32,
                                    result1_0,
                                    result1_1,
                                    result1_2,
                                );
                                match ret {
                                    0 => {
                                        let e = ();
                                        Ok(e)
                                    }
                                    1 => {
                                        let e = ();
                                        Err(e)
                                    }
                                    _ => _rt::invalid_enum_discriminant(),
                                }
                            }
                        }
                    }
                    impl OutgoingRequest {
                        #[allow(unused_unsafe, clippy::all)]
                        /// Get the HTTP Related Scheme for the Request. When `none`, the
                        /// implementation may choose an appropriate default scheme.
                        #[allow(async_fn_in_trait)]
                        pub fn scheme(&self) -> Option<Scheme> {
                            unsafe {
                                #[repr(align(4))]
                                struct RetArea(
                                    [::core::mem::MaybeUninit<
                                        u8,
                                    >; 4 * ::core::mem::size_of::<*const u8>()],
                                );
                                let mut ret_area = RetArea(
                                    [::core::mem::MaybeUninit::uninit(); 4
                                        * ::core::mem::size_of::<*const u8>()],
                                );
                                let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                                #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[method]outgoing-request.scheme"]
                                    fn wit_import1(_: i32, _: *mut u8);
                                }
                                wit_import1((self).handle() as i32, ptr0);
                                let l2 = i32::from(*ptr0.add(0).cast::<u8>());
                                let result8 = match l2 {
                                    0 => None,
                                    1 => {
                                        let e = {
                                            let l3 = i32::from(
                                                *ptr0.add(::core::mem::size_of::<*const u8>()).cast::<u8>(),
                                            );
                                            let v7 = match l3 {
                                                0 => Scheme::Http,
                                                1 => Scheme::Https,
                                                n => {
                                                    if true {
                                                        match (&n, &2) {
                                                            (left_val, right_val) => {
                                                                if !(*left_val == *right_val) {
                                                                    let kind = ::core::panicking::AssertKind::Eq;
                                                                    ::core::panicking::assert_failed(
                                                                        kind,
                                                                        &*left_val,
                                                                        &*right_val,
                                                                        ::core::option::Option::Some(
                                                                            format_args!("invalid enum discriminant"),
                                                                        ),
                                                                    );
                                                                }
                                                            }
                                                        };
                                                    }
                                                    let e7 = {
                                                        let l4 = *ptr0
                                                            .add(2 * ::core::mem::size_of::<*const u8>())
                                                            .cast::<*mut u8>();
                                                        let l5 = *ptr0
                                                            .add(3 * ::core::mem::size_of::<*const u8>())
                                                            .cast::<usize>();
                                                        let len6 = l5;
                                                        let bytes6 = _rt::Vec::from_raw_parts(
                                                            l4.cast(),
                                                            len6,
                                                            len6,
                                                        );
                                                        _rt::string_lift(bytes6)
                                                    };
                                                    Scheme::Other(e7)
                                                }
                                            };
                                            v7
                                        };
                                        Some(e)
                                    }
                                    _ => _rt::invalid_enum_discriminant(),
                                };
                                result8
                            }
                        }
                    }
                    impl OutgoingRequest {
                        #[allow(unused_unsafe, clippy::all)]
                        /// Set the HTTP Related Scheme for the Request. When `none`, the
                        /// implementation may choose an appropriate default scheme. Fails if the
                        /// string given is not a syntactically valid uri scheme.
                        #[allow(async_fn_in_trait)]
                        pub fn set_scheme(
                            &self,
                            scheme: Option<&Scheme>,
                        ) -> Result<(), ()> {
                            unsafe {
                                let (result2_0, result2_1, result2_2, result2_3) = match scheme {
                                    Some(e) => {
                                        let (result1_0, result1_1, result1_2) = match e {
                                            Scheme::Http => (0i32, ::core::ptr::null_mut(), 0usize),
                                            Scheme::Https => (1i32, ::core::ptr::null_mut(), 0usize),
                                            Scheme::Other(e) => {
                                                let vec0 = e;
                                                let ptr0 = vec0.as_ptr().cast::<u8>();
                                                let len0 = vec0.len();
                                                (2i32, ptr0.cast_mut(), len0)
                                            }
                                        };
                                        (1i32, result1_0, result1_1, result1_2)
                                    }
                                    None => (0i32, 0i32, ::core::ptr::null_mut(), 0usize),
                                };
                                #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[method]outgoing-request.set-scheme"]
                                    fn wit_import3(
                                        _: i32,
                                        _: i32,
                                        _: i32,
                                        _: *mut u8,
                                        _: usize,
                                    ) -> i32;
                                }
                                let ret = wit_import3(
                                    (self).handle() as i32,
                                    result2_0,
                                    result2_1,
                                    result2_2,
                                    result2_3,
                                );
                                match ret {
                                    0 => {
                                        let e = ();
                                        Ok(e)
                                    }
                                    1 => {
                                        let e = ();
                                        Err(e)
                                    }
                                    _ => _rt::invalid_enum_discriminant(),
                                }
                            }
                        }
                    }
                    impl OutgoingRequest {
                        #[allow(unused_unsafe, clippy::all)]
                        /// Get the HTTP Authority for the Request. A value of `none` may be used
                        /// with Related Schemes which do not require an Authority. The HTTP and
                        /// HTTPS schemes always require an authority.
                        #[allow(async_fn_in_trait)]
                        pub fn authority(&self) -> Option<_rt::String> {
                            unsafe {
                                #[repr(align(4))]
                                struct RetArea(
                                    [::core::mem::MaybeUninit<
                                        u8,
                                    >; 3 * ::core::mem::size_of::<*const u8>()],
                                );
                                let mut ret_area = RetArea(
                                    [::core::mem::MaybeUninit::uninit(); 3
                                        * ::core::mem::size_of::<*const u8>()],
                                );
                                let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                                #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[method]outgoing-request.authority"]
                                    fn wit_import1(_: i32, _: *mut u8);
                                }
                                wit_import1((self).handle() as i32, ptr0);
                                let l2 = i32::from(*ptr0.add(0).cast::<u8>());
                                let result6 = match l2 {
                                    0 => None,
                                    1 => {
                                        let e = {
                                            let l3 = *ptr0
                                                .add(::core::mem::size_of::<*const u8>())
                                                .cast::<*mut u8>();
                                            let l4 = *ptr0
                                                .add(2 * ::core::mem::size_of::<*const u8>())
                                                .cast::<usize>();
                                            let len5 = l4;
                                            let bytes5 = _rt::Vec::from_raw_parts(
                                                l3.cast(),
                                                len5,
                                                len5,
                                            );
                                            _rt::string_lift(bytes5)
                                        };
                                        Some(e)
                                    }
                                    _ => _rt::invalid_enum_discriminant(),
                                };
                                result6
                            }
                        }
                    }
                    impl OutgoingRequest {
                        #[allow(unused_unsafe, clippy::all)]
                        /// Set the HTTP Authority for the Request. A value of `none` may be used
                        /// with Related Schemes which do not require an Authority. The HTTP and
                        /// HTTPS schemes always require an authority. Fails if the string given is
                        /// not a syntactically valid uri authority.
                        #[allow(async_fn_in_trait)]
                        pub fn set_authority(
                            &self,
                            authority: Option<&str>,
                        ) -> Result<(), ()> {
                            unsafe {
                                let (result1_0, result1_1, result1_2) = match authority {
                                    Some(e) => {
                                        let vec0 = e;
                                        let ptr0 = vec0.as_ptr().cast::<u8>();
                                        let len0 = vec0.len();
                                        (1i32, ptr0.cast_mut(), len0)
                                    }
                                    None => (0i32, ::core::ptr::null_mut(), 0usize),
                                };
                                #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[method]outgoing-request.set-authority"]
                                    fn wit_import2(_: i32, _: i32, _: *mut u8, _: usize) -> i32;
                                }
                                let ret = wit_import2(
                                    (self).handle() as i32,
                                    result1_0,
                                    result1_1,
                                    result1_2,
                                );
                                match ret {
                                    0 => {
                                        let e = ();
                                        Ok(e)
                                    }
                                    1 => {
                                        let e = ();
                                        Err(e)
                                    }
                                    _ => _rt::invalid_enum_discriminant(),
                                }
                            }
                        }
                    }
                    impl OutgoingRequest {
                        #[allow(unused_unsafe, clippy::all)]
                        /// Get the headers associated with the Request.
                        ///
                        /// The returned `headers` resource is immutable: `set`, `append`, and
                        /// `delete` operations will fail with `header-error.immutable`.
                        ///
                        /// This headers resource is a child: it must be dropped before the parent
                        /// `outgoing-request` is dropped, or its ownership is transfered to
                        /// another component by e.g. `outgoing-handler.handle`.
                        #[allow(async_fn_in_trait)]
                        pub fn headers(&self) -> Headers {
                            unsafe {
                                #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[method]outgoing-request.headers"]
                                    fn wit_import0(_: i32) -> i32;
                                }
                                let ret = wit_import0((self).handle() as i32);
                                Fields::from_handle(ret as u32)
                            }
                        }
                    }
                    impl RequestOptions {
                        #[allow(unused_unsafe, clippy::all)]
                        /// Construct a default `request-options` value.
                        #[allow(async_fn_in_trait)]
                        pub fn new() -> Self {
                            unsafe {
                                #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[constructor]request-options"]
                                    fn wit_import0() -> i32;
                                }
                                let ret = wit_import0();
                                RequestOptions::from_handle(ret as u32)
                            }
                        }
                    }
                    impl RequestOptions {
                        #[allow(unused_unsafe, clippy::all)]
                        /// The timeout for the initial connect to the HTTP Server.
                        #[allow(async_fn_in_trait)]
                        pub fn connect_timeout(&self) -> Option<Duration> {
                            unsafe {
                                #[repr(align(8))]
                                struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                                let mut ret_area = RetArea(
                                    [::core::mem::MaybeUninit::uninit(); 16],
                                );
                                let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                                #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[method]request-options.connect-timeout"]
                                    fn wit_import1(_: i32, _: *mut u8);
                                }
                                wit_import1((self).handle() as i32, ptr0);
                                let l2 = i32::from(*ptr0.add(0).cast::<u8>());
                                let result4 = match l2 {
                                    0 => None,
                                    1 => {
                                        let e = {
                                            let l3 = *ptr0.add(8).cast::<i64>();
                                            l3 as u64
                                        };
                                        Some(e)
                                    }
                                    _ => _rt::invalid_enum_discriminant(),
                                };
                                result4
                            }
                        }
                    }
                    impl RequestOptions {
                        #[allow(unused_unsafe, clippy::all)]
                        /// Set the timeout for the initial connect to the HTTP Server. An error
                        /// return value indicates that this timeout is not supported.
                        #[allow(async_fn_in_trait)]
                        pub fn set_connect_timeout(
                            &self,
                            duration: Option<Duration>,
                        ) -> Result<(), ()> {
                            unsafe {
                                let (result0_0, result0_1) = match duration {
                                    Some(e) => (1i32, _rt::as_i64(e)),
                                    None => (0i32, 0i64),
                                };
                                #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[method]request-options.set-connect-timeout"]
                                    fn wit_import1(_: i32, _: i32, _: i64) -> i32;
                                }
                                let ret = wit_import1(
                                    (self).handle() as i32,
                                    result0_0,
                                    result0_1,
                                );
                                match ret {
                                    0 => {
                                        let e = ();
                                        Ok(e)
                                    }
                                    1 => {
                                        let e = ();
                                        Err(e)
                                    }
                                    _ => _rt::invalid_enum_discriminant(),
                                }
                            }
                        }
                    }
                    impl RequestOptions {
                        #[allow(unused_unsafe, clippy::all)]
                        /// The timeout for receiving the first byte of the Response body.
                        #[allow(async_fn_in_trait)]
                        pub fn first_byte_timeout(&self) -> Option<Duration> {
                            unsafe {
                                #[repr(align(8))]
                                struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                                let mut ret_area = RetArea(
                                    [::core::mem::MaybeUninit::uninit(); 16],
                                );
                                let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                                #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[method]request-options.first-byte-timeout"]
                                    fn wit_import1(_: i32, _: *mut u8);
                                }
                                wit_import1((self).handle() as i32, ptr0);
                                let l2 = i32::from(*ptr0.add(0).cast::<u8>());
                                let result4 = match l2 {
                                    0 => None,
                                    1 => {
                                        let e = {
                                            let l3 = *ptr0.add(8).cast::<i64>();
                                            l3 as u64
                                        };
                                        Some(e)
                                    }
                                    _ => _rt::invalid_enum_discriminant(),
                                };
                                result4
                            }
                        }
                    }
                    impl RequestOptions {
                        #[allow(unused_unsafe, clippy::all)]
                        /// Set the timeout for receiving the first byte of the Response body. An
                        /// error return value indicates that this timeout is not supported.
                        #[allow(async_fn_in_trait)]
                        pub fn set_first_byte_timeout(
                            &self,
                            duration: Option<Duration>,
                        ) -> Result<(), ()> {
                            unsafe {
                                let (result0_0, result0_1) = match duration {
                                    Some(e) => (1i32, _rt::as_i64(e)),
                                    None => (0i32, 0i64),
                                };
                                #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[method]request-options.set-first-byte-timeout"]
                                    fn wit_import1(_: i32, _: i32, _: i64) -> i32;
                                }
                                let ret = wit_import1(
                                    (self).handle() as i32,
                                    result0_0,
                                    result0_1,
                                );
                                match ret {
                                    0 => {
                                        let e = ();
                                        Ok(e)
                                    }
                                    1 => {
                                        let e = ();
                                        Err(e)
                                    }
                                    _ => _rt::invalid_enum_discriminant(),
                                }
                            }
                        }
                    }
                    impl RequestOptions {
                        #[allow(unused_unsafe, clippy::all)]
                        /// The timeout for receiving subsequent chunks of bytes in the Response
                        /// body stream.
                        #[allow(async_fn_in_trait)]
                        pub fn between_bytes_timeout(&self) -> Option<Duration> {
                            unsafe {
                                #[repr(align(8))]
                                struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                                let mut ret_area = RetArea(
                                    [::core::mem::MaybeUninit::uninit(); 16],
                                );
                                let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                                #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[method]request-options.between-bytes-timeout"]
                                    fn wit_import1(_: i32, _: *mut u8);
                                }
                                wit_import1((self).handle() as i32, ptr0);
                                let l2 = i32::from(*ptr0.add(0).cast::<u8>());
                                let result4 = match l2 {
                                    0 => None,
                                    1 => {
                                        let e = {
                                            let l3 = *ptr0.add(8).cast::<i64>();
                                            l3 as u64
                                        };
                                        Some(e)
                                    }
                                    _ => _rt::invalid_enum_discriminant(),
                                };
                                result4
                            }
                        }
                    }
                    impl RequestOptions {
                        #[allow(unused_unsafe, clippy::all)]
                        /// Set the timeout for receiving subsequent chunks of bytes in the Response
                        /// body stream. An error return value indicates that this timeout is not
                        /// supported.
                        #[allow(async_fn_in_trait)]
                        pub fn set_between_bytes_timeout(
                            &self,
                            duration: Option<Duration>,
                        ) -> Result<(), ()> {
                            unsafe {
                                let (result0_0, result0_1) = match duration {
                                    Some(e) => (1i32, _rt::as_i64(e)),
                                    None => (0i32, 0i64),
                                };
                                #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[method]request-options.set-between-bytes-timeout"]
                                    fn wit_import1(_: i32, _: i32, _: i64) -> i32;
                                }
                                let ret = wit_import1(
                                    (self).handle() as i32,
                                    result0_0,
                                    result0_1,
                                );
                                match ret {
                                    0 => {
                                        let e = ();
                                        Ok(e)
                                    }
                                    1 => {
                                        let e = ();
                                        Err(e)
                                    }
                                    _ => _rt::invalid_enum_discriminant(),
                                }
                            }
                        }
                    }
                    impl ResponseOutparam {
                        #[allow(unused_unsafe, clippy::all)]
                        /// Set the value of the `response-outparam` to either send a response,
                        /// or indicate an error.
                        ///
                        /// This method consumes the `response-outparam` to ensure that it is
                        /// called at most once. If it is never called, the implementation
                        /// will respond with an error.
                        ///
                        /// The user may provide an `error` to `response` to allow the
                        /// implementation determine how to respond with an HTTP error response.
                        #[allow(async_fn_in_trait)]
                        pub fn set(
                            param: ResponseOutparam,
                            response: Result<OutgoingResponse, ErrorCode>,
                        ) -> () {
                            unsafe {
                                let (
                                    result38_0,
                                    result38_1,
                                    result38_2,
                                    result38_3,
                                    result38_4,
                                    result38_5,
                                    result38_6,
                                    result38_7,
                                ) = match &response {
                                    Ok(e) => {
                                        (
                                            0i32,
                                            (e).take_handle() as i32,
                                            0i32,
                                            ::core::mem::MaybeUninit::<u64>::zeroed(),
                                            ::core::ptr::null_mut(),
                                            ::core::ptr::null_mut(),
                                            0usize,
                                            0i32,
                                        )
                                    }
                                    Err(e) => {
                                        let (
                                            result37_0,
                                            result37_1,
                                            result37_2,
                                            result37_3,
                                            result37_4,
                                            result37_5,
                                            result37_6,
                                        ) = match e {
                                            ErrorCode::DnsTimeout => {
                                                (
                                                    0i32,
                                                    0i32,
                                                    ::core::mem::MaybeUninit::<u64>::zeroed(),
                                                    ::core::ptr::null_mut(),
                                                    ::core::ptr::null_mut(),
                                                    0usize,
                                                    0i32,
                                                )
                                            }
                                            ErrorCode::DnsError(e) => {
                                                let DnsErrorPayload {
                                                    rcode: rcode0,
                                                    info_code: info_code0,
                                                } = e;
                                                let (result2_0, result2_1, result2_2) = match rcode0 {
                                                    Some(e) => {
                                                        let vec1 = e;
                                                        let ptr1 = vec1.as_ptr().cast::<u8>();
                                                        let len1 = vec1.len();
                                                        (1i32, ptr1.cast_mut(), len1)
                                                    }
                                                    None => (0i32, ::core::ptr::null_mut(), 0usize),
                                                };
                                                let (result3_0, result3_1) = match info_code0 {
                                                    Some(e) => (1i32, _rt::as_i32(e)),
                                                    None => (0i32, 0i32),
                                                };
                                                (
                                                    1i32,
                                                    result2_0,
                                                    {
                                                        let mut t = ::core::mem::MaybeUninit::<u64>::uninit();
                                                        t.as_mut_ptr().cast::<*mut u8>().write(result2_1);
                                                        t
                                                    },
                                                    result2_2 as *mut u8,
                                                    result3_0 as *mut u8,
                                                    result3_1 as usize,
                                                    0i32,
                                                )
                                            }
                                            ErrorCode::DestinationNotFound => {
                                                (
                                                    2i32,
                                                    0i32,
                                                    ::core::mem::MaybeUninit::<u64>::zeroed(),
                                                    ::core::ptr::null_mut(),
                                                    ::core::ptr::null_mut(),
                                                    0usize,
                                                    0i32,
                                                )
                                            }
                                            ErrorCode::DestinationUnavailable => {
                                                (
                                                    3i32,
                                                    0i32,
                                                    ::core::mem::MaybeUninit::<u64>::zeroed(),
                                                    ::core::ptr::null_mut(),
                                                    ::core::ptr::null_mut(),
                                                    0usize,
                                                    0i32,
                                                )
                                            }
                                            ErrorCode::DestinationIpProhibited => {
                                                (
                                                    4i32,
                                                    0i32,
                                                    ::core::mem::MaybeUninit::<u64>::zeroed(),
                                                    ::core::ptr::null_mut(),
                                                    ::core::ptr::null_mut(),
                                                    0usize,
                                                    0i32,
                                                )
                                            }
                                            ErrorCode::DestinationIpUnroutable => {
                                                (
                                                    5i32,
                                                    0i32,
                                                    ::core::mem::MaybeUninit::<u64>::zeroed(),
                                                    ::core::ptr::null_mut(),
                                                    ::core::ptr::null_mut(),
                                                    0usize,
                                                    0i32,
                                                )
                                            }
                                            ErrorCode::ConnectionRefused => {
                                                (
                                                    6i32,
                                                    0i32,
                                                    ::core::mem::MaybeUninit::<u64>::zeroed(),
                                                    ::core::ptr::null_mut(),
                                                    ::core::ptr::null_mut(),
                                                    0usize,
                                                    0i32,
                                                )
                                            }
                                            ErrorCode::ConnectionTerminated => {
                                                (
                                                    7i32,
                                                    0i32,
                                                    ::core::mem::MaybeUninit::<u64>::zeroed(),
                                                    ::core::ptr::null_mut(),
                                                    ::core::ptr::null_mut(),
                                                    0usize,
                                                    0i32,
                                                )
                                            }
                                            ErrorCode::ConnectionTimeout => {
                                                (
                                                    8i32,
                                                    0i32,
                                                    ::core::mem::MaybeUninit::<u64>::zeroed(),
                                                    ::core::ptr::null_mut(),
                                                    ::core::ptr::null_mut(),
                                                    0usize,
                                                    0i32,
                                                )
                                            }
                                            ErrorCode::ConnectionReadTimeout => {
                                                (
                                                    9i32,
                                                    0i32,
                                                    ::core::mem::MaybeUninit::<u64>::zeroed(),
                                                    ::core::ptr::null_mut(),
                                                    ::core::ptr::null_mut(),
                                                    0usize,
                                                    0i32,
                                                )
                                            }
                                            ErrorCode::ConnectionWriteTimeout => {
                                                (
                                                    10i32,
                                                    0i32,
                                                    ::core::mem::MaybeUninit::<u64>::zeroed(),
                                                    ::core::ptr::null_mut(),
                                                    ::core::ptr::null_mut(),
                                                    0usize,
                                                    0i32,
                                                )
                                            }
                                            ErrorCode::ConnectionLimitReached => {
                                                (
                                                    11i32,
                                                    0i32,
                                                    ::core::mem::MaybeUninit::<u64>::zeroed(),
                                                    ::core::ptr::null_mut(),
                                                    ::core::ptr::null_mut(),
                                                    0usize,
                                                    0i32,
                                                )
                                            }
                                            ErrorCode::TlsProtocolError => {
                                                (
                                                    12i32,
                                                    0i32,
                                                    ::core::mem::MaybeUninit::<u64>::zeroed(),
                                                    ::core::ptr::null_mut(),
                                                    ::core::ptr::null_mut(),
                                                    0usize,
                                                    0i32,
                                                )
                                            }
                                            ErrorCode::TlsCertificateError => {
                                                (
                                                    13i32,
                                                    0i32,
                                                    ::core::mem::MaybeUninit::<u64>::zeroed(),
                                                    ::core::ptr::null_mut(),
                                                    ::core::ptr::null_mut(),
                                                    0usize,
                                                    0i32,
                                                )
                                            }
                                            ErrorCode::TlsAlertReceived(e) => {
                                                let TlsAlertReceivedPayload {
                                                    alert_id: alert_id4,
                                                    alert_message: alert_message4,
                                                } = e;
                                                let (result5_0, result5_1) = match alert_id4 {
                                                    Some(e) => (1i32, _rt::as_i32(e)),
                                                    None => (0i32, 0i32),
                                                };
                                                let (result7_0, result7_1, result7_2) = match alert_message4 {
                                                    Some(e) => {
                                                        let vec6 = e;
                                                        let ptr6 = vec6.as_ptr().cast::<u8>();
                                                        let len6 = vec6.len();
                                                        (1i32, ptr6.cast_mut(), len6)
                                                    }
                                                    None => (0i32, ::core::ptr::null_mut(), 0usize),
                                                };
                                                (
                                                    14i32,
                                                    result5_0,
                                                    ::core::mem::MaybeUninit::new(i64::from(result5_1) as u64),
                                                    result7_0 as *mut u8,
                                                    result7_1,
                                                    result7_2,
                                                    0i32,
                                                )
                                            }
                                            ErrorCode::HttpRequestDenied => {
                                                (
                                                    15i32,
                                                    0i32,
                                                    ::core::mem::MaybeUninit::<u64>::zeroed(),
                                                    ::core::ptr::null_mut(),
                                                    ::core::ptr::null_mut(),
                                                    0usize,
                                                    0i32,
                                                )
                                            }
                                            ErrorCode::HttpRequestLengthRequired => {
                                                (
                                                    16i32,
                                                    0i32,
                                                    ::core::mem::MaybeUninit::<u64>::zeroed(),
                                                    ::core::ptr::null_mut(),
                                                    ::core::ptr::null_mut(),
                                                    0usize,
                                                    0i32,
                                                )
                                            }
                                            ErrorCode::HttpRequestBodySize(e) => {
                                                let (result8_0, result8_1) = match e {
                                                    Some(e) => (1i32, _rt::as_i64(e)),
                                                    None => (0i32, 0i64),
                                                };
                                                (
                                                    17i32,
                                                    result8_0,
                                                    ::core::mem::MaybeUninit::new(result8_1 as u64),
                                                    ::core::ptr::null_mut(),
                                                    ::core::ptr::null_mut(),
                                                    0usize,
                                                    0i32,
                                                )
                                            }
                                            ErrorCode::HttpRequestMethodInvalid => {
                                                (
                                                    18i32,
                                                    0i32,
                                                    ::core::mem::MaybeUninit::<u64>::zeroed(),
                                                    ::core::ptr::null_mut(),
                                                    ::core::ptr::null_mut(),
                                                    0usize,
                                                    0i32,
                                                )
                                            }
                                            ErrorCode::HttpRequestUriInvalid => {
                                                (
                                                    19i32,
                                                    0i32,
                                                    ::core::mem::MaybeUninit::<u64>::zeroed(),
                                                    ::core::ptr::null_mut(),
                                                    ::core::ptr::null_mut(),
                                                    0usize,
                                                    0i32,
                                                )
                                            }
                                            ErrorCode::HttpRequestUriTooLong => {
                                                (
                                                    20i32,
                                                    0i32,
                                                    ::core::mem::MaybeUninit::<u64>::zeroed(),
                                                    ::core::ptr::null_mut(),
                                                    ::core::ptr::null_mut(),
                                                    0usize,
                                                    0i32,
                                                )
                                            }
                                            ErrorCode::HttpRequestHeaderSectionSize(e) => {
                                                let (result9_0, result9_1) = match e {
                                                    Some(e) => (1i32, _rt::as_i32(e)),
                                                    None => (0i32, 0i32),
                                                };
                                                (
                                                    21i32,
                                                    result9_0,
                                                    ::core::mem::MaybeUninit::new(i64::from(result9_1) as u64),
                                                    ::core::ptr::null_mut(),
                                                    ::core::ptr::null_mut(),
                                                    0usize,
                                                    0i32,
                                                )
                                            }
                                            ErrorCode::HttpRequestHeaderSize(e) => {
                                                let (
                                                    result14_0,
                                                    result14_1,
                                                    result14_2,
                                                    result14_3,
                                                    result14_4,
                                                    result14_5,
                                                ) = match e {
                                                    Some(e) => {
                                                        let FieldSizePayload {
                                                            field_name: field_name10,
                                                            field_size: field_size10,
                                                        } = e;
                                                        let (result12_0, result12_1, result12_2) = match field_name10 {
                                                            Some(e) => {
                                                                let vec11 = e;
                                                                let ptr11 = vec11.as_ptr().cast::<u8>();
                                                                let len11 = vec11.len();
                                                                (1i32, ptr11.cast_mut(), len11)
                                                            }
                                                            None => (0i32, ::core::ptr::null_mut(), 0usize),
                                                        };
                                                        let (result13_0, result13_1) = match field_size10 {
                                                            Some(e) => (1i32, _rt::as_i32(e)),
                                                            None => (0i32, 0i32),
                                                        };
                                                        (
                                                            1i32,
                                                            result12_0,
                                                            result12_1,
                                                            result12_2,
                                                            result13_0,
                                                            result13_1,
                                                        )
                                                    }
                                                    None => {
                                                        (0i32, 0i32, ::core::ptr::null_mut(), 0usize, 0i32, 0i32)
                                                    }
                                                };
                                                (
                                                    22i32,
                                                    result14_0,
                                                    ::core::mem::MaybeUninit::new(i64::from(result14_1) as u64),
                                                    result14_2,
                                                    result14_3 as *mut u8,
                                                    result14_4 as usize,
                                                    result14_5,
                                                )
                                            }
                                            ErrorCode::HttpRequestTrailerSectionSize(e) => {
                                                let (result15_0, result15_1) = match e {
                                                    Some(e) => (1i32, _rt::as_i32(e)),
                                                    None => (0i32, 0i32),
                                                };
                                                (
                                                    23i32,
                                                    result15_0,
                                                    ::core::mem::MaybeUninit::new(i64::from(result15_1) as u64),
                                                    ::core::ptr::null_mut(),
                                                    ::core::ptr::null_mut(),
                                                    0usize,
                                                    0i32,
                                                )
                                            }
                                            ErrorCode::HttpRequestTrailerSize(e) => {
                                                let FieldSizePayload {
                                                    field_name: field_name16,
                                                    field_size: field_size16,
                                                } = e;
                                                let (result18_0, result18_1, result18_2) = match field_name16 {
                                                    Some(e) => {
                                                        let vec17 = e;
                                                        let ptr17 = vec17.as_ptr().cast::<u8>();
                                                        let len17 = vec17.len();
                                                        (1i32, ptr17.cast_mut(), len17)
                                                    }
                                                    None => (0i32, ::core::ptr::null_mut(), 0usize),
                                                };
                                                let (result19_0, result19_1) = match field_size16 {
                                                    Some(e) => (1i32, _rt::as_i32(e)),
                                                    None => (0i32, 0i32),
                                                };
                                                (
                                                    24i32,
                                                    result18_0,
                                                    {
                                                        let mut t = ::core::mem::MaybeUninit::<u64>::uninit();
                                                        t.as_mut_ptr().cast::<*mut u8>().write(result18_1);
                                                        t
                                                    },
                                                    result18_2 as *mut u8,
                                                    result19_0 as *mut u8,
                                                    result19_1 as usize,
                                                    0i32,
                                                )
                                            }
                                            ErrorCode::HttpResponseIncomplete => {
                                                (
                                                    25i32,
                                                    0i32,
                                                    ::core::mem::MaybeUninit::<u64>::zeroed(),
                                                    ::core::ptr::null_mut(),
                                                    ::core::ptr::null_mut(),
                                                    0usize,
                                                    0i32,
                                                )
                                            }
                                            ErrorCode::HttpResponseHeaderSectionSize(e) => {
                                                let (result20_0, result20_1) = match e {
                                                    Some(e) => (1i32, _rt::as_i32(e)),
                                                    None => (0i32, 0i32),
                                                };
                                                (
                                                    26i32,
                                                    result20_0,
                                                    ::core::mem::MaybeUninit::new(i64::from(result20_1) as u64),
                                                    ::core::ptr::null_mut(),
                                                    ::core::ptr::null_mut(),
                                                    0usize,
                                                    0i32,
                                                )
                                            }
                                            ErrorCode::HttpResponseHeaderSize(e) => {
                                                let FieldSizePayload {
                                                    field_name: field_name21,
                                                    field_size: field_size21,
                                                } = e;
                                                let (result23_0, result23_1, result23_2) = match field_name21 {
                                                    Some(e) => {
                                                        let vec22 = e;
                                                        let ptr22 = vec22.as_ptr().cast::<u8>();
                                                        let len22 = vec22.len();
                                                        (1i32, ptr22.cast_mut(), len22)
                                                    }
                                                    None => (0i32, ::core::ptr::null_mut(), 0usize),
                                                };
                                                let (result24_0, result24_1) = match field_size21 {
                                                    Some(e) => (1i32, _rt::as_i32(e)),
                                                    None => (0i32, 0i32),
                                                };
                                                (
                                                    27i32,
                                                    result23_0,
                                                    {
                                                        let mut t = ::core::mem::MaybeUninit::<u64>::uninit();
                                                        t.as_mut_ptr().cast::<*mut u8>().write(result23_1);
                                                        t
                                                    },
                                                    result23_2 as *mut u8,
                                                    result24_0 as *mut u8,
                                                    result24_1 as usize,
                                                    0i32,
                                                )
                                            }
                                            ErrorCode::HttpResponseBodySize(e) => {
                                                let (result25_0, result25_1) = match e {
                                                    Some(e) => (1i32, _rt::as_i64(e)),
                                                    None => (0i32, 0i64),
                                                };
                                                (
                                                    28i32,
                                                    result25_0,
                                                    ::core::mem::MaybeUninit::new(result25_1 as u64),
                                                    ::core::ptr::null_mut(),
                                                    ::core::ptr::null_mut(),
                                                    0usize,
                                                    0i32,
                                                )
                                            }
                                            ErrorCode::HttpResponseTrailerSectionSize(e) => {
                                                let (result26_0, result26_1) = match e {
                                                    Some(e) => (1i32, _rt::as_i32(e)),
                                                    None => (0i32, 0i32),
                                                };
                                                (
                                                    29i32,
                                                    result26_0,
                                                    ::core::mem::MaybeUninit::new(i64::from(result26_1) as u64),
                                                    ::core::ptr::null_mut(),
                                                    ::core::ptr::null_mut(),
                                                    0usize,
                                                    0i32,
                                                )
                                            }
                                            ErrorCode::HttpResponseTrailerSize(e) => {
                                                let FieldSizePayload {
                                                    field_name: field_name27,
                                                    field_size: field_size27,
                                                } = e;
                                                let (result29_0, result29_1, result29_2) = match field_name27 {
                                                    Some(e) => {
                                                        let vec28 = e;
                                                        let ptr28 = vec28.as_ptr().cast::<u8>();
                                                        let len28 = vec28.len();
                                                        (1i32, ptr28.cast_mut(), len28)
                                                    }
                                                    None => (0i32, ::core::ptr::null_mut(), 0usize),
                                                };
                                                let (result30_0, result30_1) = match field_size27 {
                                                    Some(e) => (1i32, _rt::as_i32(e)),
                                                    None => (0i32, 0i32),
                                                };
                                                (
                                                    30i32,
                                                    result29_0,
                                                    {
                                                        let mut t = ::core::mem::MaybeUninit::<u64>::uninit();
                                                        t.as_mut_ptr().cast::<*mut u8>().write(result29_1);
                                                        t
                                                    },
                                                    result29_2 as *mut u8,
                                                    result30_0 as *mut u8,
                                                    result30_1 as usize,
                                                    0i32,
                                                )
                                            }
                                            ErrorCode::HttpResponseTransferCoding(e) => {
                                                let (result32_0, result32_1, result32_2) = match e {
                                                    Some(e) => {
                                                        let vec31 = e;
                                                        let ptr31 = vec31.as_ptr().cast::<u8>();
                                                        let len31 = vec31.len();
                                                        (1i32, ptr31.cast_mut(), len31)
                                                    }
                                                    None => (0i32, ::core::ptr::null_mut(), 0usize),
                                                };
                                                (
                                                    31i32,
                                                    result32_0,
                                                    {
                                                        let mut t = ::core::mem::MaybeUninit::<u64>::uninit();
                                                        t.as_mut_ptr().cast::<*mut u8>().write(result32_1);
                                                        t
                                                    },
                                                    result32_2 as *mut u8,
                                                    ::core::ptr::null_mut(),
                                                    0usize,
                                                    0i32,
                                                )
                                            }
                                            ErrorCode::HttpResponseContentCoding(e) => {
                                                let (result34_0, result34_1, result34_2) = match e {
                                                    Some(e) => {
                                                        let vec33 = e;
                                                        let ptr33 = vec33.as_ptr().cast::<u8>();
                                                        let len33 = vec33.len();
                                                        (1i32, ptr33.cast_mut(), len33)
                                                    }
                                                    None => (0i32, ::core::ptr::null_mut(), 0usize),
                                                };
                                                (
                                                    32i32,
                                                    result34_0,
                                                    {
                                                        let mut t = ::core::mem::MaybeUninit::<u64>::uninit();
                                                        t.as_mut_ptr().cast::<*mut u8>().write(result34_1);
                                                        t
                                                    },
                                                    result34_2 as *mut u8,
                                                    ::core::ptr::null_mut(),
                                                    0usize,
                                                    0i32,
                                                )
                                            }
                                            ErrorCode::HttpResponseTimeout => {
                                                (
                                                    33i32,
                                                    0i32,
                                                    ::core::mem::MaybeUninit::<u64>::zeroed(),
                                                    ::core::ptr::null_mut(),
                                                    ::core::ptr::null_mut(),
                                                    0usize,
                                                    0i32,
                                                )
                                            }
                                            ErrorCode::HttpUpgradeFailed => {
                                                (
                                                    34i32,
                                                    0i32,
                                                    ::core::mem::MaybeUninit::<u64>::zeroed(),
                                                    ::core::ptr::null_mut(),
                                                    ::core::ptr::null_mut(),
                                                    0usize,
                                                    0i32,
                                                )
                                            }
                                            ErrorCode::HttpProtocolError => {
                                                (
                                                    35i32,
                                                    0i32,
                                                    ::core::mem::MaybeUninit::<u64>::zeroed(),
                                                    ::core::ptr::null_mut(),
                                                    ::core::ptr::null_mut(),
                                                    0usize,
                                                    0i32,
                                                )
                                            }
                                            ErrorCode::LoopDetected => {
                                                (
                                                    36i32,
                                                    0i32,
                                                    ::core::mem::MaybeUninit::<u64>::zeroed(),
                                                    ::core::ptr::null_mut(),
                                                    ::core::ptr::null_mut(),
                                                    0usize,
                                                    0i32,
                                                )
                                            }
                                            ErrorCode::ConfigurationError => {
                                                (
                                                    37i32,
                                                    0i32,
                                                    ::core::mem::MaybeUninit::<u64>::zeroed(),
                                                    ::core::ptr::null_mut(),
                                                    ::core::ptr::null_mut(),
                                                    0usize,
                                                    0i32,
                                                )
                                            }
                                            ErrorCode::InternalError(e) => {
                                                let (result36_0, result36_1, result36_2) = match e {
                                                    Some(e) => {
                                                        let vec35 = e;
                                                        let ptr35 = vec35.as_ptr().cast::<u8>();
                                                        let len35 = vec35.len();
                                                        (1i32, ptr35.cast_mut(), len35)
                                                    }
                                                    None => (0i32, ::core::ptr::null_mut(), 0usize),
                                                };
                                                (
                                                    38i32,
                                                    result36_0,
                                                    {
                                                        let mut t = ::core::mem::MaybeUninit::<u64>::uninit();
                                                        t.as_mut_ptr().cast::<*mut u8>().write(result36_1);
                                                        t
                                                    },
                                                    result36_2 as *mut u8,
                                                    ::core::ptr::null_mut(),
                                                    0usize,
                                                    0i32,
                                                )
                                            }
                                        };
                                        (
                                            1i32,
                                            result37_0,
                                            result37_1,
                                            result37_2,
                                            result37_3,
                                            result37_4,
                                            result37_5,
                                            result37_6,
                                        )
                                    }
                                };
                                #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[static]response-outparam.set"]
                                    fn wit_import39(
                                        _: i32,
                                        _: i32,
                                        _: i32,
                                        _: i32,
                                        _: ::core::mem::MaybeUninit<u64>,
                                        _: *mut u8,
                                        _: *mut u8,
                                        _: usize,
                                        _: i32,
                                    );
                                }
                                wit_import39(
                                    (&param).take_handle() as i32,
                                    result38_0,
                                    result38_1,
                                    result38_2,
                                    result38_3,
                                    result38_4,
                                    result38_5,
                                    result38_6,
                                    result38_7,
                                );
                            }
                        }
                    }
                    impl IncomingResponse {
                        #[allow(unused_unsafe, clippy::all)]
                        /// Returns the status code from the incoming response.
                        #[allow(async_fn_in_trait)]
                        pub fn status(&self) -> StatusCode {
                            unsafe {
                                #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[method]incoming-response.status"]
                                    fn wit_import0(_: i32) -> i32;
                                }
                                let ret = wit_import0((self).handle() as i32);
                                ret as u16
                            }
                        }
                    }
                    impl IncomingResponse {
                        #[allow(unused_unsafe, clippy::all)]
                        /// Returns the headers from the incoming response.
                        ///
                        /// The returned `headers` resource is immutable: `set`, `append`, and
                        /// `delete` operations will fail with `header-error.immutable`.
                        ///
                        /// This headers resource is a child: it must be dropped before the parent
                        /// `incoming-response` is dropped.
                        #[allow(async_fn_in_trait)]
                        pub fn headers(&self) -> Headers {
                            unsafe {
                                #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[method]incoming-response.headers"]
                                    fn wit_import0(_: i32) -> i32;
                                }
                                let ret = wit_import0((self).handle() as i32);
                                Fields::from_handle(ret as u32)
                            }
                        }
                    }
                    impl IncomingResponse {
                        #[allow(unused_unsafe, clippy::all)]
                        /// Returns the incoming body. May be called at most once. Returns error
                        /// if called additional times.
                        #[allow(async_fn_in_trait)]
                        pub fn consume(&self) -> Result<IncomingBody, ()> {
                            unsafe {
                                #[repr(align(4))]
                                struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                                let mut ret_area = RetArea(
                                    [::core::mem::MaybeUninit::uninit(); 8],
                                );
                                let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                                #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[method]incoming-response.consume"]
                                    fn wit_import1(_: i32, _: *mut u8);
                                }
                                wit_import1((self).handle() as i32, ptr0);
                                let l2 = i32::from(*ptr0.add(0).cast::<u8>());
                                let result4 = match l2 {
                                    0 => {
                                        let e = {
                                            let l3 = *ptr0.add(4).cast::<i32>();
                                            IncomingBody::from_handle(l3 as u32)
                                        };
                                        Ok(e)
                                    }
                                    1 => {
                                        let e = ();
                                        Err(e)
                                    }
                                    _ => _rt::invalid_enum_discriminant(),
                                };
                                result4
                            }
                        }
                    }
                    impl IncomingBody {
                        #[allow(unused_unsafe, clippy::all)]
                        /// Returns the contents of the body, as a stream of bytes.
                        ///
                        /// Returns success on first call: the stream representing the contents
                        /// can be retrieved at most once. Subsequent calls will return error.
                        ///
                        /// The returned `input-stream` resource is a child: it must be dropped
                        /// before the parent `incoming-body` is dropped, or consumed by
                        /// `incoming-body.finish`.
                        ///
                        /// This invariant ensures that the implementation can determine whether
                        /// the user is consuming the contents of the body, waiting on the
                        /// `future-trailers` to be ready, or neither. This allows for network
                        /// backpressure is to be applied when the user is consuming the body,
                        /// and for that backpressure to not inhibit delivery of the trailers if
                        /// the user does not read the entire body.
                        #[allow(async_fn_in_trait)]
                        pub fn stream(&self) -> Result<InputStream, ()> {
                            unsafe {
                                #[repr(align(4))]
                                struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                                let mut ret_area = RetArea(
                                    [::core::mem::MaybeUninit::uninit(); 8],
                                );
                                let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                                #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[method]incoming-body.stream"]
                                    fn wit_import1(_: i32, _: *mut u8);
                                }
                                wit_import1((self).handle() as i32, ptr0);
                                let l2 = i32::from(*ptr0.add(0).cast::<u8>());
                                let result4 = match l2 {
                                    0 => {
                                        let e = {
                                            let l3 = *ptr0.add(4).cast::<i32>();
                                            super::super::super::wasi::io::streams::InputStream::from_handle(
                                                l3 as u32,
                                            )
                                        };
                                        Ok(e)
                                    }
                                    1 => {
                                        let e = ();
                                        Err(e)
                                    }
                                    _ => _rt::invalid_enum_discriminant(),
                                };
                                result4
                            }
                        }
                    }
                    impl IncomingBody {
                        #[allow(unused_unsafe, clippy::all)]
                        /// Takes ownership of `incoming-body`, and returns a `future-trailers`.
                        /// This function will trap if the `input-stream` child is still alive.
                        #[allow(async_fn_in_trait)]
                        pub fn finish(this: IncomingBody) -> FutureTrailers {
                            unsafe {
                                #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[static]incoming-body.finish"]
                                    fn wit_import0(_: i32) -> i32;
                                }
                                let ret = wit_import0((&this).take_handle() as i32);
                                FutureTrailers::from_handle(ret as u32)
                            }
                        }
                    }
                    impl FutureTrailers {
                        #[allow(unused_unsafe, clippy::all)]
                        /// Returns a pollable which becomes ready when either the trailers have
                        /// been received, or an error has occured. When this pollable is ready,
                        /// the `get` method will return `some`.
                        #[allow(async_fn_in_trait)]
                        pub fn subscribe(&self) -> Pollable {
                            unsafe {
                                #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[method]future-trailers.subscribe"]
                                    fn wit_import0(_: i32) -> i32;
                                }
                                let ret = wit_import0((self).handle() as i32);
                                super::super::super::wasi::io::poll::Pollable::from_handle(
                                    ret as u32,
                                )
                            }
                        }
                    }
                    impl FutureTrailers {
                        #[allow(unused_unsafe, clippy::all)]
                        /// Returns the contents of the trailers, or an error which occured,
                        /// once the future is ready.
                        ///
                        /// The outer `option` represents future readiness. Users can wait on this
                        /// `option` to become `some` using the `subscribe` method.
                        ///
                        /// The outer `result` is used to retrieve the trailers or error at most
                        /// once. It will be success on the first call in which the outer option
                        /// is `some`, and error on subsequent calls.
                        ///
                        /// The inner `result` represents that either the HTTP Request or Response
                        /// body, as well as any trailers, were received successfully, or that an
                        /// error occured receiving them. The optional `trailers` indicates whether
                        /// or not trailers were present in the body.
                        ///
                        /// When some `trailers` are returned by this method, the `trailers`
                        /// resource is immutable, and a child. Use of the `set`, `append`, or
                        /// `delete` methods will return an error, and the resource must be
                        /// dropped before the parent `future-trailers` is dropped.
                        #[allow(async_fn_in_trait)]
                        pub fn get(
                            &self,
                        ) -> Option<Result<Result<Option<Trailers>, ErrorCode>, ()>> {
                            unsafe {
                                #[repr(align(8))]
                                struct RetArea(
                                    [::core::mem::MaybeUninit<
                                        u8,
                                    >; 40 + 4 * ::core::mem::size_of::<*const u8>()],
                                );
                                let mut ret_area = RetArea(
                                    [::core::mem::MaybeUninit::uninit(); 40
                                        + 4 * ::core::mem::size_of::<*const u8>()],
                                );
                                let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                                #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[method]future-trailers.get"]
                                    fn wit_import1(_: i32, _: *mut u8);
                                }
                                wit_import1((self).handle() as i32, ptr0);
                                let l2 = i32::from(*ptr0.add(0).cast::<u8>());
                                let result70 = match l2 {
                                    0 => None,
                                    1 => {
                                        let e = {
                                            let l3 = i32::from(*ptr0.add(8).cast::<u8>());
                                            match l3 {
                                                0 => {
                                                    let e = {
                                                        let l4 = i32::from(*ptr0.add(16).cast::<u8>());
                                                        match l4 {
                                                            0 => {
                                                                let e = {
                                                                    let l5 = i32::from(*ptr0.add(24).cast::<u8>());
                                                                    match l5 {
                                                                        0 => None,
                                                                        1 => {
                                                                            let e = {
                                                                                let l6 = *ptr0.add(28).cast::<i32>();
                                                                                Fields::from_handle(l6 as u32)
                                                                            };
                                                                            Some(e)
                                                                        }
                                                                        _ => _rt::invalid_enum_discriminant(),
                                                                    }
                                                                };
                                                                Ok(e)
                                                            }
                                                            1 => {
                                                                let e = {
                                                                    let l7 = i32::from(*ptr0.add(24).cast::<u8>());
                                                                    let v69 = match l7 {
                                                                        0 => ErrorCode::DnsTimeout,
                                                                        1 => {
                                                                            let e69 = {
                                                                                let l8 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                                let l12 = i32::from(
                                                                                    *ptr0
                                                                                        .add(32 + 3 * ::core::mem::size_of::<*const u8>())
                                                                                        .cast::<u8>(),
                                                                                );
                                                                                DnsErrorPayload {
                                                                                    rcode: match l8 {
                                                                                        0 => None,
                                                                                        1 => {
                                                                                            let e = {
                                                                                                let l9 = *ptr0
                                                                                                    .add(32 + 1 * ::core::mem::size_of::<*const u8>())
                                                                                                    .cast::<*mut u8>();
                                                                                                let l10 = *ptr0
                                                                                                    .add(32 + 2 * ::core::mem::size_of::<*const u8>())
                                                                                                    .cast::<usize>();
                                                                                                let len11 = l10;
                                                                                                let bytes11 = _rt::Vec::from_raw_parts(
                                                                                                    l9.cast(),
                                                                                                    len11,
                                                                                                    len11,
                                                                                                );
                                                                                                _rt::string_lift(bytes11)
                                                                                            };
                                                                                            Some(e)
                                                                                        }
                                                                                        _ => _rt::invalid_enum_discriminant(),
                                                                                    },
                                                                                    info_code: match l12 {
                                                                                        0 => None,
                                                                                        1 => {
                                                                                            let e = {
                                                                                                let l13 = i32::from(
                                                                                                    *ptr0
                                                                                                        .add(34 + 3 * ::core::mem::size_of::<*const u8>())
                                                                                                        .cast::<u16>(),
                                                                                                );
                                                                                                l13 as u16
                                                                                            };
                                                                                            Some(e)
                                                                                        }
                                                                                        _ => _rt::invalid_enum_discriminant(),
                                                                                    },
                                                                                }
                                                                            };
                                                                            ErrorCode::DnsError(e69)
                                                                        }
                                                                        2 => ErrorCode::DestinationNotFound,
                                                                        3 => ErrorCode::DestinationUnavailable,
                                                                        4 => ErrorCode::DestinationIpProhibited,
                                                                        5 => ErrorCode::DestinationIpUnroutable,
                                                                        6 => ErrorCode::ConnectionRefused,
                                                                        7 => ErrorCode::ConnectionTerminated,
                                                                        8 => ErrorCode::ConnectionTimeout,
                                                                        9 => ErrorCode::ConnectionReadTimeout,
                                                                        10 => ErrorCode::ConnectionWriteTimeout,
                                                                        11 => ErrorCode::ConnectionLimitReached,
                                                                        12 => ErrorCode::TlsProtocolError,
                                                                        13 => ErrorCode::TlsCertificateError,
                                                                        14 => {
                                                                            let e69 = {
                                                                                let l14 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                                let l16 = i32::from(
                                                                                    *ptr0
                                                                                        .add(32 + 1 * ::core::mem::size_of::<*const u8>())
                                                                                        .cast::<u8>(),
                                                                                );
                                                                                TlsAlertReceivedPayload {
                                                                                    alert_id: match l14 {
                                                                                        0 => None,
                                                                                        1 => {
                                                                                            let e = {
                                                                                                let l15 = i32::from(*ptr0.add(33).cast::<u8>());
                                                                                                l15 as u8
                                                                                            };
                                                                                            Some(e)
                                                                                        }
                                                                                        _ => _rt::invalid_enum_discriminant(),
                                                                                    },
                                                                                    alert_message: match l16 {
                                                                                        0 => None,
                                                                                        1 => {
                                                                                            let e = {
                                                                                                let l17 = *ptr0
                                                                                                    .add(32 + 2 * ::core::mem::size_of::<*const u8>())
                                                                                                    .cast::<*mut u8>();
                                                                                                let l18 = *ptr0
                                                                                                    .add(32 + 3 * ::core::mem::size_of::<*const u8>())
                                                                                                    .cast::<usize>();
                                                                                                let len19 = l18;
                                                                                                let bytes19 = _rt::Vec::from_raw_parts(
                                                                                                    l17.cast(),
                                                                                                    len19,
                                                                                                    len19,
                                                                                                );
                                                                                                _rt::string_lift(bytes19)
                                                                                            };
                                                                                            Some(e)
                                                                                        }
                                                                                        _ => _rt::invalid_enum_discriminant(),
                                                                                    },
                                                                                }
                                                                            };
                                                                            ErrorCode::TlsAlertReceived(e69)
                                                                        }
                                                                        15 => ErrorCode::HttpRequestDenied,
                                                                        16 => ErrorCode::HttpRequestLengthRequired,
                                                                        17 => {
                                                                            let e69 = {
                                                                                let l20 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                                match l20 {
                                                                                    0 => None,
                                                                                    1 => {
                                                                                        let e = {
                                                                                            let l21 = *ptr0.add(40).cast::<i64>();
                                                                                            l21 as u64
                                                                                        };
                                                                                        Some(e)
                                                                                    }
                                                                                    _ => _rt::invalid_enum_discriminant(),
                                                                                }
                                                                            };
                                                                            ErrorCode::HttpRequestBodySize(e69)
                                                                        }
                                                                        18 => ErrorCode::HttpRequestMethodInvalid,
                                                                        19 => ErrorCode::HttpRequestUriInvalid,
                                                                        20 => ErrorCode::HttpRequestUriTooLong,
                                                                        21 => {
                                                                            let e69 = {
                                                                                let l22 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                                match l22 {
                                                                                    0 => None,
                                                                                    1 => {
                                                                                        let e = {
                                                                                            let l23 = *ptr0.add(36).cast::<i32>();
                                                                                            l23 as u32
                                                                                        };
                                                                                        Some(e)
                                                                                    }
                                                                                    _ => _rt::invalid_enum_discriminant(),
                                                                                }
                                                                            };
                                                                            ErrorCode::HttpRequestHeaderSectionSize(e69)
                                                                        }
                                                                        22 => {
                                                                            let e69 = {
                                                                                let l24 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                                match l24 {
                                                                                    0 => None,
                                                                                    1 => {
                                                                                        let e = {
                                                                                            let l25 = i32::from(
                                                                                                *ptr0
                                                                                                    .add(32 + 1 * ::core::mem::size_of::<*const u8>())
                                                                                                    .cast::<u8>(),
                                                                                            );
                                                                                            let l29 = i32::from(
                                                                                                *ptr0
                                                                                                    .add(32 + 4 * ::core::mem::size_of::<*const u8>())
                                                                                                    .cast::<u8>(),
                                                                                            );
                                                                                            FieldSizePayload {
                                                                                                field_name: match l25 {
                                                                                                    0 => None,
                                                                                                    1 => {
                                                                                                        let e = {
                                                                                                            let l26 = *ptr0
                                                                                                                .add(32 + 2 * ::core::mem::size_of::<*const u8>())
                                                                                                                .cast::<*mut u8>();
                                                                                                            let l27 = *ptr0
                                                                                                                .add(32 + 3 * ::core::mem::size_of::<*const u8>())
                                                                                                                .cast::<usize>();
                                                                                                            let len28 = l27;
                                                                                                            let bytes28 = _rt::Vec::from_raw_parts(
                                                                                                                l26.cast(),
                                                                                                                len28,
                                                                                                                len28,
                                                                                                            );
                                                                                                            _rt::string_lift(bytes28)
                                                                                                        };
                                                                                                        Some(e)
                                                                                                    }
                                                                                                    _ => _rt::invalid_enum_discriminant(),
                                                                                                },
                                                                                                field_size: match l29 {
                                                                                                    0 => None,
                                                                                                    1 => {
                                                                                                        let e = {
                                                                                                            let l30 = *ptr0
                                                                                                                .add(36 + 4 * ::core::mem::size_of::<*const u8>())
                                                                                                                .cast::<i32>();
                                                                                                            l30 as u32
                                                                                                        };
                                                                                                        Some(e)
                                                                                                    }
                                                                                                    _ => _rt::invalid_enum_discriminant(),
                                                                                                },
                                                                                            }
                                                                                        };
                                                                                        Some(e)
                                                                                    }
                                                                                    _ => _rt::invalid_enum_discriminant(),
                                                                                }
                                                                            };
                                                                            ErrorCode::HttpRequestHeaderSize(e69)
                                                                        }
                                                                        23 => {
                                                                            let e69 = {
                                                                                let l31 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                                match l31 {
                                                                                    0 => None,
                                                                                    1 => {
                                                                                        let e = {
                                                                                            let l32 = *ptr0.add(36).cast::<i32>();
                                                                                            l32 as u32
                                                                                        };
                                                                                        Some(e)
                                                                                    }
                                                                                    _ => _rt::invalid_enum_discriminant(),
                                                                                }
                                                                            };
                                                                            ErrorCode::HttpRequestTrailerSectionSize(e69)
                                                                        }
                                                                        24 => {
                                                                            let e69 = {
                                                                                let l33 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                                let l37 = i32::from(
                                                                                    *ptr0
                                                                                        .add(32 + 3 * ::core::mem::size_of::<*const u8>())
                                                                                        .cast::<u8>(),
                                                                                );
                                                                                FieldSizePayload {
                                                                                    field_name: match l33 {
                                                                                        0 => None,
                                                                                        1 => {
                                                                                            let e = {
                                                                                                let l34 = *ptr0
                                                                                                    .add(32 + 1 * ::core::mem::size_of::<*const u8>())
                                                                                                    .cast::<*mut u8>();
                                                                                                let l35 = *ptr0
                                                                                                    .add(32 + 2 * ::core::mem::size_of::<*const u8>())
                                                                                                    .cast::<usize>();
                                                                                                let len36 = l35;
                                                                                                let bytes36 = _rt::Vec::from_raw_parts(
                                                                                                    l34.cast(),
                                                                                                    len36,
                                                                                                    len36,
                                                                                                );
                                                                                                _rt::string_lift(bytes36)
                                                                                            };
                                                                                            Some(e)
                                                                                        }
                                                                                        _ => _rt::invalid_enum_discriminant(),
                                                                                    },
                                                                                    field_size: match l37 {
                                                                                        0 => None,
                                                                                        1 => {
                                                                                            let e = {
                                                                                                let l38 = *ptr0
                                                                                                    .add(36 + 3 * ::core::mem::size_of::<*const u8>())
                                                                                                    .cast::<i32>();
                                                                                                l38 as u32
                                                                                            };
                                                                                            Some(e)
                                                                                        }
                                                                                        _ => _rt::invalid_enum_discriminant(),
                                                                                    },
                                                                                }
                                                                            };
                                                                            ErrorCode::HttpRequestTrailerSize(e69)
                                                                        }
                                                                        25 => ErrorCode::HttpResponseIncomplete,
                                                                        26 => {
                                                                            let e69 = {
                                                                                let l39 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                                match l39 {
                                                                                    0 => None,
                                                                                    1 => {
                                                                                        let e = {
                                                                                            let l40 = *ptr0.add(36).cast::<i32>();
                                                                                            l40 as u32
                                                                                        };
                                                                                        Some(e)
                                                                                    }
                                                                                    _ => _rt::invalid_enum_discriminant(),
                                                                                }
                                                                            };
                                                                            ErrorCode::HttpResponseHeaderSectionSize(e69)
                                                                        }
                                                                        27 => {
                                                                            let e69 = {
                                                                                let l41 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                                let l45 = i32::from(
                                                                                    *ptr0
                                                                                        .add(32 + 3 * ::core::mem::size_of::<*const u8>())
                                                                                        .cast::<u8>(),
                                                                                );
                                                                                FieldSizePayload {
                                                                                    field_name: match l41 {
                                                                                        0 => None,
                                                                                        1 => {
                                                                                            let e = {
                                                                                                let l42 = *ptr0
                                                                                                    .add(32 + 1 * ::core::mem::size_of::<*const u8>())
                                                                                                    .cast::<*mut u8>();
                                                                                                let l43 = *ptr0
                                                                                                    .add(32 + 2 * ::core::mem::size_of::<*const u8>())
                                                                                                    .cast::<usize>();
                                                                                                let len44 = l43;
                                                                                                let bytes44 = _rt::Vec::from_raw_parts(
                                                                                                    l42.cast(),
                                                                                                    len44,
                                                                                                    len44,
                                                                                                );
                                                                                                _rt::string_lift(bytes44)
                                                                                            };
                                                                                            Some(e)
                                                                                        }
                                                                                        _ => _rt::invalid_enum_discriminant(),
                                                                                    },
                                                                                    field_size: match l45 {
                                                                                        0 => None,
                                                                                        1 => {
                                                                                            let e = {
                                                                                                let l46 = *ptr0
                                                                                                    .add(36 + 3 * ::core::mem::size_of::<*const u8>())
                                                                                                    .cast::<i32>();
                                                                                                l46 as u32
                                                                                            };
                                                                                            Some(e)
                                                                                        }
                                                                                        _ => _rt::invalid_enum_discriminant(),
                                                                                    },
                                                                                }
                                                                            };
                                                                            ErrorCode::HttpResponseHeaderSize(e69)
                                                                        }
                                                                        28 => {
                                                                            let e69 = {
                                                                                let l47 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                                match l47 {
                                                                                    0 => None,
                                                                                    1 => {
                                                                                        let e = {
                                                                                            let l48 = *ptr0.add(40).cast::<i64>();
                                                                                            l48 as u64
                                                                                        };
                                                                                        Some(e)
                                                                                    }
                                                                                    _ => _rt::invalid_enum_discriminant(),
                                                                                }
                                                                            };
                                                                            ErrorCode::HttpResponseBodySize(e69)
                                                                        }
                                                                        29 => {
                                                                            let e69 = {
                                                                                let l49 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                                match l49 {
                                                                                    0 => None,
                                                                                    1 => {
                                                                                        let e = {
                                                                                            let l50 = *ptr0.add(36).cast::<i32>();
                                                                                            l50 as u32
                                                                                        };
                                                                                        Some(e)
                                                                                    }
                                                                                    _ => _rt::invalid_enum_discriminant(),
                                                                                }
                                                                            };
                                                                            ErrorCode::HttpResponseTrailerSectionSize(e69)
                                                                        }
                                                                        30 => {
                                                                            let e69 = {
                                                                                let l51 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                                let l55 = i32::from(
                                                                                    *ptr0
                                                                                        .add(32 + 3 * ::core::mem::size_of::<*const u8>())
                                                                                        .cast::<u8>(),
                                                                                );
                                                                                FieldSizePayload {
                                                                                    field_name: match l51 {
                                                                                        0 => None,
                                                                                        1 => {
                                                                                            let e = {
                                                                                                let l52 = *ptr0
                                                                                                    .add(32 + 1 * ::core::mem::size_of::<*const u8>())
                                                                                                    .cast::<*mut u8>();
                                                                                                let l53 = *ptr0
                                                                                                    .add(32 + 2 * ::core::mem::size_of::<*const u8>())
                                                                                                    .cast::<usize>();
                                                                                                let len54 = l53;
                                                                                                let bytes54 = _rt::Vec::from_raw_parts(
                                                                                                    l52.cast(),
                                                                                                    len54,
                                                                                                    len54,
                                                                                                );
                                                                                                _rt::string_lift(bytes54)
                                                                                            };
                                                                                            Some(e)
                                                                                        }
                                                                                        _ => _rt::invalid_enum_discriminant(),
                                                                                    },
                                                                                    field_size: match l55 {
                                                                                        0 => None,
                                                                                        1 => {
                                                                                            let e = {
                                                                                                let l56 = *ptr0
                                                                                                    .add(36 + 3 * ::core::mem::size_of::<*const u8>())
                                                                                                    .cast::<i32>();
                                                                                                l56 as u32
                                                                                            };
                                                                                            Some(e)
                                                                                        }
                                                                                        _ => _rt::invalid_enum_discriminant(),
                                                                                    },
                                                                                }
                                                                            };
                                                                            ErrorCode::HttpResponseTrailerSize(e69)
                                                                        }
                                                                        31 => {
                                                                            let e69 = {
                                                                                let l57 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                                match l57 {
                                                                                    0 => None,
                                                                                    1 => {
                                                                                        let e = {
                                                                                            let l58 = *ptr0
                                                                                                .add(32 + 1 * ::core::mem::size_of::<*const u8>())
                                                                                                .cast::<*mut u8>();
                                                                                            let l59 = *ptr0
                                                                                                .add(32 + 2 * ::core::mem::size_of::<*const u8>())
                                                                                                .cast::<usize>();
                                                                                            let len60 = l59;
                                                                                            let bytes60 = _rt::Vec::from_raw_parts(
                                                                                                l58.cast(),
                                                                                                len60,
                                                                                                len60,
                                                                                            );
                                                                                            _rt::string_lift(bytes60)
                                                                                        };
                                                                                        Some(e)
                                                                                    }
                                                                                    _ => _rt::invalid_enum_discriminant(),
                                                                                }
                                                                            };
                                                                            ErrorCode::HttpResponseTransferCoding(e69)
                                                                        }
                                                                        32 => {
                                                                            let e69 = {
                                                                                let l61 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                                match l61 {
                                                                                    0 => None,
                                                                                    1 => {
                                                                                        let e = {
                                                                                            let l62 = *ptr0
                                                                                                .add(32 + 1 * ::core::mem::size_of::<*const u8>())
                                                                                                .cast::<*mut u8>();
                                                                                            let l63 = *ptr0
                                                                                                .add(32 + 2 * ::core::mem::size_of::<*const u8>())
                                                                                                .cast::<usize>();
                                                                                            let len64 = l63;
                                                                                            let bytes64 = _rt::Vec::from_raw_parts(
                                                                                                l62.cast(),
                                                                                                len64,
                                                                                                len64,
                                                                                            );
                                                                                            _rt::string_lift(bytes64)
                                                                                        };
                                                                                        Some(e)
                                                                                    }
                                                                                    _ => _rt::invalid_enum_discriminant(),
                                                                                }
                                                                            };
                                                                            ErrorCode::HttpResponseContentCoding(e69)
                                                                        }
                                                                        33 => ErrorCode::HttpResponseTimeout,
                                                                        34 => ErrorCode::HttpUpgradeFailed,
                                                                        35 => ErrorCode::HttpProtocolError,
                                                                        36 => ErrorCode::LoopDetected,
                                                                        37 => ErrorCode::ConfigurationError,
                                                                        n => {
                                                                            if true {
                                                                                match (&n, &38) {
                                                                                    (left_val, right_val) => {
                                                                                        if !(*left_val == *right_val) {
                                                                                            let kind = ::core::panicking::AssertKind::Eq;
                                                                                            ::core::panicking::assert_failed(
                                                                                                kind,
                                                                                                &*left_val,
                                                                                                &*right_val,
                                                                                                ::core::option::Option::Some(
                                                                                                    format_args!("invalid enum discriminant"),
                                                                                                ),
                                                                                            );
                                                                                        }
                                                                                    }
                                                                                };
                                                                            }
                                                                            let e69 = {
                                                                                let l65 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                                match l65 {
                                                                                    0 => None,
                                                                                    1 => {
                                                                                        let e = {
                                                                                            let l66 = *ptr0
                                                                                                .add(32 + 1 * ::core::mem::size_of::<*const u8>())
                                                                                                .cast::<*mut u8>();
                                                                                            let l67 = *ptr0
                                                                                                .add(32 + 2 * ::core::mem::size_of::<*const u8>())
                                                                                                .cast::<usize>();
                                                                                            let len68 = l67;
                                                                                            let bytes68 = _rt::Vec::from_raw_parts(
                                                                                                l66.cast(),
                                                                                                len68,
                                                                                                len68,
                                                                                            );
                                                                                            _rt::string_lift(bytes68)
                                                                                        };
                                                                                        Some(e)
                                                                                    }
                                                                                    _ => _rt::invalid_enum_discriminant(),
                                                                                }
                                                                            };
                                                                            ErrorCode::InternalError(e69)
                                                                        }
                                                                    };
                                                                    v69
                                                                };
                                                                Err(e)
                                                            }
                                                            _ => _rt::invalid_enum_discriminant(),
                                                        }
                                                    };
                                                    Ok(e)
                                                }
                                                1 => {
                                                    let e = ();
                                                    Err(e)
                                                }
                                                _ => _rt::invalid_enum_discriminant(),
                                            }
                                        };
                                        Some(e)
                                    }
                                    _ => _rt::invalid_enum_discriminant(),
                                };
                                result70
                            }
                        }
                    }
                    impl OutgoingResponse {
                        #[allow(unused_unsafe, clippy::all)]
                        /// Construct an `outgoing-response`, with a default `status-code` of `200`.
                        /// If a different `status-code` is needed, it must be set via the
                        /// `set-status-code` method.
                        ///
                        /// * `headers` is the HTTP Headers for the Response.
                        #[allow(async_fn_in_trait)]
                        pub fn new(headers: Headers) -> Self {
                            unsafe {
                                #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[constructor]outgoing-response"]
                                    fn wit_import0(_: i32) -> i32;
                                }
                                let ret = wit_import0((&headers).take_handle() as i32);
                                OutgoingResponse::from_handle(ret as u32)
                            }
                        }
                    }
                    impl OutgoingResponse {
                        #[allow(unused_unsafe, clippy::all)]
                        /// Get the HTTP Status Code for the Response.
                        #[allow(async_fn_in_trait)]
                        pub fn status_code(&self) -> StatusCode {
                            unsafe {
                                #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[method]outgoing-response.status-code"]
                                    fn wit_import0(_: i32) -> i32;
                                }
                                let ret = wit_import0((self).handle() as i32);
                                ret as u16
                            }
                        }
                    }
                    impl OutgoingResponse {
                        #[allow(unused_unsafe, clippy::all)]
                        /// Set the HTTP Status Code for the Response. Fails if the status-code
                        /// given is not a valid http status code.
                        #[allow(async_fn_in_trait)]
                        pub fn set_status_code(
                            &self,
                            status_code: StatusCode,
                        ) -> Result<(), ()> {
                            unsafe {
                                #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[method]outgoing-response.set-status-code"]
                                    fn wit_import0(_: i32, _: i32) -> i32;
                                }
                                let ret = wit_import0(
                                    (self).handle() as i32,
                                    _rt::as_i32(status_code),
                                );
                                match ret {
                                    0 => {
                                        let e = ();
                                        Ok(e)
                                    }
                                    1 => {
                                        let e = ();
                                        Err(e)
                                    }
                                    _ => _rt::invalid_enum_discriminant(),
                                }
                            }
                        }
                    }
                    impl OutgoingResponse {
                        #[allow(unused_unsafe, clippy::all)]
                        /// Get the headers associated with the Request.
                        ///
                        /// The returned `headers` resource is immutable: `set`, `append`, and
                        /// `delete` operations will fail with `header-error.immutable`.
                        ///
                        /// This headers resource is a child: it must be dropped before the parent
                        /// `outgoing-request` is dropped, or its ownership is transfered to
                        /// another component by e.g. `outgoing-handler.handle`.
                        #[allow(async_fn_in_trait)]
                        pub fn headers(&self) -> Headers {
                            unsafe {
                                #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[method]outgoing-response.headers"]
                                    fn wit_import0(_: i32) -> i32;
                                }
                                let ret = wit_import0((self).handle() as i32);
                                Fields::from_handle(ret as u32)
                            }
                        }
                    }
                    impl OutgoingResponse {
                        #[allow(unused_unsafe, clippy::all)]
                        /// Returns the resource corresponding to the outgoing Body for this Response.
                        ///
                        /// Returns success on the first call: the `outgoing-body` resource for
                        /// this `outgoing-response` can be retrieved at most once. Subsequent
                        /// calls will return error.
                        #[allow(async_fn_in_trait)]
                        pub fn body(&self) -> Result<OutgoingBody, ()> {
                            unsafe {
                                #[repr(align(4))]
                                struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                                let mut ret_area = RetArea(
                                    [::core::mem::MaybeUninit::uninit(); 8],
                                );
                                let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                                #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[method]outgoing-response.body"]
                                    fn wit_import1(_: i32, _: *mut u8);
                                }
                                wit_import1((self).handle() as i32, ptr0);
                                let l2 = i32::from(*ptr0.add(0).cast::<u8>());
                                let result4 = match l2 {
                                    0 => {
                                        let e = {
                                            let l3 = *ptr0.add(4).cast::<i32>();
                                            OutgoingBody::from_handle(l3 as u32)
                                        };
                                        Ok(e)
                                    }
                                    1 => {
                                        let e = ();
                                        Err(e)
                                    }
                                    _ => _rt::invalid_enum_discriminant(),
                                };
                                result4
                            }
                        }
                    }
                    impl OutgoingBody {
                        #[allow(unused_unsafe, clippy::all)]
                        /// Returns a stream for writing the body contents.
                        ///
                        /// The returned `output-stream` is a child resource: it must be dropped
                        /// before the parent `outgoing-body` resource is dropped (or finished),
                        /// otherwise the `outgoing-body` drop or `finish` will trap.
                        ///
                        /// Returns success on the first call: the `output-stream` resource for
                        /// this `outgoing-body` may be retrieved at most once. Subsequent calls
                        /// will return error.
                        #[allow(async_fn_in_trait)]
                        pub fn write(&self) -> Result<OutputStream, ()> {
                            unsafe {
                                #[repr(align(4))]
                                struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                                let mut ret_area = RetArea(
                                    [::core::mem::MaybeUninit::uninit(); 8],
                                );
                                let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                                #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[method]outgoing-body.write"]
                                    fn wit_import1(_: i32, _: *mut u8);
                                }
                                wit_import1((self).handle() as i32, ptr0);
                                let l2 = i32::from(*ptr0.add(0).cast::<u8>());
                                let result4 = match l2 {
                                    0 => {
                                        let e = {
                                            let l3 = *ptr0.add(4).cast::<i32>();
                                            super::super::super::wasi::io::streams::OutputStream::from_handle(
                                                l3 as u32,
                                            )
                                        };
                                        Ok(e)
                                    }
                                    1 => {
                                        let e = ();
                                        Err(e)
                                    }
                                    _ => _rt::invalid_enum_discriminant(),
                                };
                                result4
                            }
                        }
                    }
                    impl OutgoingBody {
                        #[allow(unused_unsafe, clippy::all)]
                        /// Finalize an outgoing body, optionally providing trailers. This must be
                        /// called to signal that the response is complete. If the `outgoing-body`
                        /// is dropped without calling `outgoing-body.finalize`, the implementation
                        /// should treat the body as corrupted.
                        ///
                        /// Fails if the body's `outgoing-request` or `outgoing-response` was
                        /// constructed with a Content-Length header, and the contents written
                        /// to the body (via `write`) does not match the value given in the
                        /// Content-Length.
                        #[allow(async_fn_in_trait)]
                        pub fn finish(
                            this: OutgoingBody,
                            trailers: Option<Trailers>,
                        ) -> Result<(), ErrorCode> {
                            unsafe {
                                #[repr(align(8))]
                                struct RetArea(
                                    [::core::mem::MaybeUninit<
                                        u8,
                                    >; 24 + 4 * ::core::mem::size_of::<*const u8>()],
                                );
                                let mut ret_area = RetArea(
                                    [::core::mem::MaybeUninit::uninit(); 24
                                        + 4 * ::core::mem::size_of::<*const u8>()],
                                );
                                let (result0_0, result0_1) = match &trailers {
                                    Some(e) => (1i32, (e).take_handle() as i32),
                                    None => (0i32, 0i32),
                                };
                                let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                                #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[static]outgoing-body.finish"]
                                    fn wit_import2(_: i32, _: i32, _: i32, _: *mut u8);
                                }
                                wit_import2(
                                    (&this).take_handle() as i32,
                                    result0_0,
                                    result0_1,
                                    ptr1,
                                );
                                let l3 = i32::from(*ptr1.add(0).cast::<u8>());
                                let result67 = match l3 {
                                    0 => {
                                        let e = ();
                                        Ok(e)
                                    }
                                    1 => {
                                        let e = {
                                            let l4 = i32::from(*ptr1.add(8).cast::<u8>());
                                            let v66 = match l4 {
                                                0 => ErrorCode::DnsTimeout,
                                                1 => {
                                                    let e66 = {
                                                        let l5 = i32::from(*ptr1.add(16).cast::<u8>());
                                                        let l9 = i32::from(
                                                            *ptr1
                                                                .add(16 + 3 * ::core::mem::size_of::<*const u8>())
                                                                .cast::<u8>(),
                                                        );
                                                        DnsErrorPayload {
                                                            rcode: match l5 {
                                                                0 => None,
                                                                1 => {
                                                                    let e = {
                                                                        let l6 = *ptr1
                                                                            .add(16 + 1 * ::core::mem::size_of::<*const u8>())
                                                                            .cast::<*mut u8>();
                                                                        let l7 = *ptr1
                                                                            .add(16 + 2 * ::core::mem::size_of::<*const u8>())
                                                                            .cast::<usize>();
                                                                        let len8 = l7;
                                                                        let bytes8 = _rt::Vec::from_raw_parts(
                                                                            l6.cast(),
                                                                            len8,
                                                                            len8,
                                                                        );
                                                                        _rt::string_lift(bytes8)
                                                                    };
                                                                    Some(e)
                                                                }
                                                                _ => _rt::invalid_enum_discriminant(),
                                                            },
                                                            info_code: match l9 {
                                                                0 => None,
                                                                1 => {
                                                                    let e = {
                                                                        let l10 = i32::from(
                                                                            *ptr1
                                                                                .add(18 + 3 * ::core::mem::size_of::<*const u8>())
                                                                                .cast::<u16>(),
                                                                        );
                                                                        l10 as u16
                                                                    };
                                                                    Some(e)
                                                                }
                                                                _ => _rt::invalid_enum_discriminant(),
                                                            },
                                                        }
                                                    };
                                                    ErrorCode::DnsError(e66)
                                                }
                                                2 => ErrorCode::DestinationNotFound,
                                                3 => ErrorCode::DestinationUnavailable,
                                                4 => ErrorCode::DestinationIpProhibited,
                                                5 => ErrorCode::DestinationIpUnroutable,
                                                6 => ErrorCode::ConnectionRefused,
                                                7 => ErrorCode::ConnectionTerminated,
                                                8 => ErrorCode::ConnectionTimeout,
                                                9 => ErrorCode::ConnectionReadTimeout,
                                                10 => ErrorCode::ConnectionWriteTimeout,
                                                11 => ErrorCode::ConnectionLimitReached,
                                                12 => ErrorCode::TlsProtocolError,
                                                13 => ErrorCode::TlsCertificateError,
                                                14 => {
                                                    let e66 = {
                                                        let l11 = i32::from(*ptr1.add(16).cast::<u8>());
                                                        let l13 = i32::from(
                                                            *ptr1
                                                                .add(16 + 1 * ::core::mem::size_of::<*const u8>())
                                                                .cast::<u8>(),
                                                        );
                                                        TlsAlertReceivedPayload {
                                                            alert_id: match l11 {
                                                                0 => None,
                                                                1 => {
                                                                    let e = {
                                                                        let l12 = i32::from(*ptr1.add(17).cast::<u8>());
                                                                        l12 as u8
                                                                    };
                                                                    Some(e)
                                                                }
                                                                _ => _rt::invalid_enum_discriminant(),
                                                            },
                                                            alert_message: match l13 {
                                                                0 => None,
                                                                1 => {
                                                                    let e = {
                                                                        let l14 = *ptr1
                                                                            .add(16 + 2 * ::core::mem::size_of::<*const u8>())
                                                                            .cast::<*mut u8>();
                                                                        let l15 = *ptr1
                                                                            .add(16 + 3 * ::core::mem::size_of::<*const u8>())
                                                                            .cast::<usize>();
                                                                        let len16 = l15;
                                                                        let bytes16 = _rt::Vec::from_raw_parts(
                                                                            l14.cast(),
                                                                            len16,
                                                                            len16,
                                                                        );
                                                                        _rt::string_lift(bytes16)
                                                                    };
                                                                    Some(e)
                                                                }
                                                                _ => _rt::invalid_enum_discriminant(),
                                                            },
                                                        }
                                                    };
                                                    ErrorCode::TlsAlertReceived(e66)
                                                }
                                                15 => ErrorCode::HttpRequestDenied,
                                                16 => ErrorCode::HttpRequestLengthRequired,
                                                17 => {
                                                    let e66 = {
                                                        let l17 = i32::from(*ptr1.add(16).cast::<u8>());
                                                        match l17 {
                                                            0 => None,
                                                            1 => {
                                                                let e = {
                                                                    let l18 = *ptr1.add(24).cast::<i64>();
                                                                    l18 as u64
                                                                };
                                                                Some(e)
                                                            }
                                                            _ => _rt::invalid_enum_discriminant(),
                                                        }
                                                    };
                                                    ErrorCode::HttpRequestBodySize(e66)
                                                }
                                                18 => ErrorCode::HttpRequestMethodInvalid,
                                                19 => ErrorCode::HttpRequestUriInvalid,
                                                20 => ErrorCode::HttpRequestUriTooLong,
                                                21 => {
                                                    let e66 = {
                                                        let l19 = i32::from(*ptr1.add(16).cast::<u8>());
                                                        match l19 {
                                                            0 => None,
                                                            1 => {
                                                                let e = {
                                                                    let l20 = *ptr1.add(20).cast::<i32>();
                                                                    l20 as u32
                                                                };
                                                                Some(e)
                                                            }
                                                            _ => _rt::invalid_enum_discriminant(),
                                                        }
                                                    };
                                                    ErrorCode::HttpRequestHeaderSectionSize(e66)
                                                }
                                                22 => {
                                                    let e66 = {
                                                        let l21 = i32::from(*ptr1.add(16).cast::<u8>());
                                                        match l21 {
                                                            0 => None,
                                                            1 => {
                                                                let e = {
                                                                    let l22 = i32::from(
                                                                        *ptr1
                                                                            .add(16 + 1 * ::core::mem::size_of::<*const u8>())
                                                                            .cast::<u8>(),
                                                                    );
                                                                    let l26 = i32::from(
                                                                        *ptr1
                                                                            .add(16 + 4 * ::core::mem::size_of::<*const u8>())
                                                                            .cast::<u8>(),
                                                                    );
                                                                    FieldSizePayload {
                                                                        field_name: match l22 {
                                                                            0 => None,
                                                                            1 => {
                                                                                let e = {
                                                                                    let l23 = *ptr1
                                                                                        .add(16 + 2 * ::core::mem::size_of::<*const u8>())
                                                                                        .cast::<*mut u8>();
                                                                                    let l24 = *ptr1
                                                                                        .add(16 + 3 * ::core::mem::size_of::<*const u8>())
                                                                                        .cast::<usize>();
                                                                                    let len25 = l24;
                                                                                    let bytes25 = _rt::Vec::from_raw_parts(
                                                                                        l23.cast(),
                                                                                        len25,
                                                                                        len25,
                                                                                    );
                                                                                    _rt::string_lift(bytes25)
                                                                                };
                                                                                Some(e)
                                                                            }
                                                                            _ => _rt::invalid_enum_discriminant(),
                                                                        },
                                                                        field_size: match l26 {
                                                                            0 => None,
                                                                            1 => {
                                                                                let e = {
                                                                                    let l27 = *ptr1
                                                                                        .add(20 + 4 * ::core::mem::size_of::<*const u8>())
                                                                                        .cast::<i32>();
                                                                                    l27 as u32
                                                                                };
                                                                                Some(e)
                                                                            }
                                                                            _ => _rt::invalid_enum_discriminant(),
                                                                        },
                                                                    }
                                                                };
                                                                Some(e)
                                                            }
                                                            _ => _rt::invalid_enum_discriminant(),
                                                        }
                                                    };
                                                    ErrorCode::HttpRequestHeaderSize(e66)
                                                }
                                                23 => {
                                                    let e66 = {
                                                        let l28 = i32::from(*ptr1.add(16).cast::<u8>());
                                                        match l28 {
                                                            0 => None,
                                                            1 => {
                                                                let e = {
                                                                    let l29 = *ptr1.add(20).cast::<i32>();
                                                                    l29 as u32
                                                                };
                                                                Some(e)
                                                            }
                                                            _ => _rt::invalid_enum_discriminant(),
                                                        }
                                                    };
                                                    ErrorCode::HttpRequestTrailerSectionSize(e66)
                                                }
                                                24 => {
                                                    let e66 = {
                                                        let l30 = i32::from(*ptr1.add(16).cast::<u8>());
                                                        let l34 = i32::from(
                                                            *ptr1
                                                                .add(16 + 3 * ::core::mem::size_of::<*const u8>())
                                                                .cast::<u8>(),
                                                        );
                                                        FieldSizePayload {
                                                            field_name: match l30 {
                                                                0 => None,
                                                                1 => {
                                                                    let e = {
                                                                        let l31 = *ptr1
                                                                            .add(16 + 1 * ::core::mem::size_of::<*const u8>())
                                                                            .cast::<*mut u8>();
                                                                        let l32 = *ptr1
                                                                            .add(16 + 2 * ::core::mem::size_of::<*const u8>())
                                                                            .cast::<usize>();
                                                                        let len33 = l32;
                                                                        let bytes33 = _rt::Vec::from_raw_parts(
                                                                            l31.cast(),
                                                                            len33,
                                                                            len33,
                                                                        );
                                                                        _rt::string_lift(bytes33)
                                                                    };
                                                                    Some(e)
                                                                }
                                                                _ => _rt::invalid_enum_discriminant(),
                                                            },
                                                            field_size: match l34 {
                                                                0 => None,
                                                                1 => {
                                                                    let e = {
                                                                        let l35 = *ptr1
                                                                            .add(20 + 3 * ::core::mem::size_of::<*const u8>())
                                                                            .cast::<i32>();
                                                                        l35 as u32
                                                                    };
                                                                    Some(e)
                                                                }
                                                                _ => _rt::invalid_enum_discriminant(),
                                                            },
                                                        }
                                                    };
                                                    ErrorCode::HttpRequestTrailerSize(e66)
                                                }
                                                25 => ErrorCode::HttpResponseIncomplete,
                                                26 => {
                                                    let e66 = {
                                                        let l36 = i32::from(*ptr1.add(16).cast::<u8>());
                                                        match l36 {
                                                            0 => None,
                                                            1 => {
                                                                let e = {
                                                                    let l37 = *ptr1.add(20).cast::<i32>();
                                                                    l37 as u32
                                                                };
                                                                Some(e)
                                                            }
                                                            _ => _rt::invalid_enum_discriminant(),
                                                        }
                                                    };
                                                    ErrorCode::HttpResponseHeaderSectionSize(e66)
                                                }
                                                27 => {
                                                    let e66 = {
                                                        let l38 = i32::from(*ptr1.add(16).cast::<u8>());
                                                        let l42 = i32::from(
                                                            *ptr1
                                                                .add(16 + 3 * ::core::mem::size_of::<*const u8>())
                                                                .cast::<u8>(),
                                                        );
                                                        FieldSizePayload {
                                                            field_name: match l38 {
                                                                0 => None,
                                                                1 => {
                                                                    let e = {
                                                                        let l39 = *ptr1
                                                                            .add(16 + 1 * ::core::mem::size_of::<*const u8>())
                                                                            .cast::<*mut u8>();
                                                                        let l40 = *ptr1
                                                                            .add(16 + 2 * ::core::mem::size_of::<*const u8>())
                                                                            .cast::<usize>();
                                                                        let len41 = l40;
                                                                        let bytes41 = _rt::Vec::from_raw_parts(
                                                                            l39.cast(),
                                                                            len41,
                                                                            len41,
                                                                        );
                                                                        _rt::string_lift(bytes41)
                                                                    };
                                                                    Some(e)
                                                                }
                                                                _ => _rt::invalid_enum_discriminant(),
                                                            },
                                                            field_size: match l42 {
                                                                0 => None,
                                                                1 => {
                                                                    let e = {
                                                                        let l43 = *ptr1
                                                                            .add(20 + 3 * ::core::mem::size_of::<*const u8>())
                                                                            .cast::<i32>();
                                                                        l43 as u32
                                                                    };
                                                                    Some(e)
                                                                }
                                                                _ => _rt::invalid_enum_discriminant(),
                                                            },
                                                        }
                                                    };
                                                    ErrorCode::HttpResponseHeaderSize(e66)
                                                }
                                                28 => {
                                                    let e66 = {
                                                        let l44 = i32::from(*ptr1.add(16).cast::<u8>());
                                                        match l44 {
                                                            0 => None,
                                                            1 => {
                                                                let e = {
                                                                    let l45 = *ptr1.add(24).cast::<i64>();
                                                                    l45 as u64
                                                                };
                                                                Some(e)
                                                            }
                                                            _ => _rt::invalid_enum_discriminant(),
                                                        }
                                                    };
                                                    ErrorCode::HttpResponseBodySize(e66)
                                                }
                                                29 => {
                                                    let e66 = {
                                                        let l46 = i32::from(*ptr1.add(16).cast::<u8>());
                                                        match l46 {
                                                            0 => None,
                                                            1 => {
                                                                let e = {
                                                                    let l47 = *ptr1.add(20).cast::<i32>();
                                                                    l47 as u32
                                                                };
                                                                Some(e)
                                                            }
                                                            _ => _rt::invalid_enum_discriminant(),
                                                        }
                                                    };
                                                    ErrorCode::HttpResponseTrailerSectionSize(e66)
                                                }
                                                30 => {
                                                    let e66 = {
                                                        let l48 = i32::from(*ptr1.add(16).cast::<u8>());
                                                        let l52 = i32::from(
                                                            *ptr1
                                                                .add(16 + 3 * ::core::mem::size_of::<*const u8>())
                                                                .cast::<u8>(),
                                                        );
                                                        FieldSizePayload {
                                                            field_name: match l48 {
                                                                0 => None,
                                                                1 => {
                                                                    let e = {
                                                                        let l49 = *ptr1
                                                                            .add(16 + 1 * ::core::mem::size_of::<*const u8>())
                                                                            .cast::<*mut u8>();
                                                                        let l50 = *ptr1
                                                                            .add(16 + 2 * ::core::mem::size_of::<*const u8>())
                                                                            .cast::<usize>();
                                                                        let len51 = l50;
                                                                        let bytes51 = _rt::Vec::from_raw_parts(
                                                                            l49.cast(),
                                                                            len51,
                                                                            len51,
                                                                        );
                                                                        _rt::string_lift(bytes51)
                                                                    };
                                                                    Some(e)
                                                                }
                                                                _ => _rt::invalid_enum_discriminant(),
                                                            },
                                                            field_size: match l52 {
                                                                0 => None,
                                                                1 => {
                                                                    let e = {
                                                                        let l53 = *ptr1
                                                                            .add(20 + 3 * ::core::mem::size_of::<*const u8>())
                                                                            .cast::<i32>();
                                                                        l53 as u32
                                                                    };
                                                                    Some(e)
                                                                }
                                                                _ => _rt::invalid_enum_discriminant(),
                                                            },
                                                        }
                                                    };
                                                    ErrorCode::HttpResponseTrailerSize(e66)
                                                }
                                                31 => {
                                                    let e66 = {
                                                        let l54 = i32::from(*ptr1.add(16).cast::<u8>());
                                                        match l54 {
                                                            0 => None,
                                                            1 => {
                                                                let e = {
                                                                    let l55 = *ptr1
                                                                        .add(16 + 1 * ::core::mem::size_of::<*const u8>())
                                                                        .cast::<*mut u8>();
                                                                    let l56 = *ptr1
                                                                        .add(16 + 2 * ::core::mem::size_of::<*const u8>())
                                                                        .cast::<usize>();
                                                                    let len57 = l56;
                                                                    let bytes57 = _rt::Vec::from_raw_parts(
                                                                        l55.cast(),
                                                                        len57,
                                                                        len57,
                                                                    );
                                                                    _rt::string_lift(bytes57)
                                                                };
                                                                Some(e)
                                                            }
                                                            _ => _rt::invalid_enum_discriminant(),
                                                        }
                                                    };
                                                    ErrorCode::HttpResponseTransferCoding(e66)
                                                }
                                                32 => {
                                                    let e66 = {
                                                        let l58 = i32::from(*ptr1.add(16).cast::<u8>());
                                                        match l58 {
                                                            0 => None,
                                                            1 => {
                                                                let e = {
                                                                    let l59 = *ptr1
                                                                        .add(16 + 1 * ::core::mem::size_of::<*const u8>())
                                                                        .cast::<*mut u8>();
                                                                    let l60 = *ptr1
                                                                        .add(16 + 2 * ::core::mem::size_of::<*const u8>())
                                                                        .cast::<usize>();
                                                                    let len61 = l60;
                                                                    let bytes61 = _rt::Vec::from_raw_parts(
                                                                        l59.cast(),
                                                                        len61,
                                                                        len61,
                                                                    );
                                                                    _rt::string_lift(bytes61)
                                                                };
                                                                Some(e)
                                                            }
                                                            _ => _rt::invalid_enum_discriminant(),
                                                        }
                                                    };
                                                    ErrorCode::HttpResponseContentCoding(e66)
                                                }
                                                33 => ErrorCode::HttpResponseTimeout,
                                                34 => ErrorCode::HttpUpgradeFailed,
                                                35 => ErrorCode::HttpProtocolError,
                                                36 => ErrorCode::LoopDetected,
                                                37 => ErrorCode::ConfigurationError,
                                                n => {
                                                    if true {
                                                        match (&n, &38) {
                                                            (left_val, right_val) => {
                                                                if !(*left_val == *right_val) {
                                                                    let kind = ::core::panicking::AssertKind::Eq;
                                                                    ::core::panicking::assert_failed(
                                                                        kind,
                                                                        &*left_val,
                                                                        &*right_val,
                                                                        ::core::option::Option::Some(
                                                                            format_args!("invalid enum discriminant"),
                                                                        ),
                                                                    );
                                                                }
                                                            }
                                                        };
                                                    }
                                                    let e66 = {
                                                        let l62 = i32::from(*ptr1.add(16).cast::<u8>());
                                                        match l62 {
                                                            0 => None,
                                                            1 => {
                                                                let e = {
                                                                    let l63 = *ptr1
                                                                        .add(16 + 1 * ::core::mem::size_of::<*const u8>())
                                                                        .cast::<*mut u8>();
                                                                    let l64 = *ptr1
                                                                        .add(16 + 2 * ::core::mem::size_of::<*const u8>())
                                                                        .cast::<usize>();
                                                                    let len65 = l64;
                                                                    let bytes65 = _rt::Vec::from_raw_parts(
                                                                        l63.cast(),
                                                                        len65,
                                                                        len65,
                                                                    );
                                                                    _rt::string_lift(bytes65)
                                                                };
                                                                Some(e)
                                                            }
                                                            _ => _rt::invalid_enum_discriminant(),
                                                        }
                                                    };
                                                    ErrorCode::InternalError(e66)
                                                }
                                            };
                                            v66
                                        };
                                        Err(e)
                                    }
                                    _ => _rt::invalid_enum_discriminant(),
                                };
                                result67
                            }
                        }
                    }
                    impl FutureIncomingResponse {
                        #[allow(unused_unsafe, clippy::all)]
                        /// Returns a pollable which becomes ready when either the Response has
                        /// been received, or an error has occured. When this pollable is ready,
                        /// the `get` method will return `some`.
                        #[allow(async_fn_in_trait)]
                        pub fn subscribe(&self) -> Pollable {
                            unsafe {
                                #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[method]future-incoming-response.subscribe"]
                                    fn wit_import0(_: i32) -> i32;
                                }
                                let ret = wit_import0((self).handle() as i32);
                                super::super::super::wasi::io::poll::Pollable::from_handle(
                                    ret as u32,
                                )
                            }
                        }
                    }
                    impl FutureIncomingResponse {
                        #[allow(unused_unsafe, clippy::all)]
                        /// Returns the incoming HTTP Response, or an error, once one is ready.
                        ///
                        /// The outer `option` represents future readiness. Users can wait on this
                        /// `option` to become `some` using the `subscribe` method.
                        ///
                        /// The outer `result` is used to retrieve the response or error at most
                        /// once. It will be success on the first call in which the outer option
                        /// is `some`, and error on subsequent calls.
                        ///
                        /// The inner `result` represents that either the incoming HTTP Response
                        /// status and headers have recieved successfully, or that an error
                        /// occured. Errors may also occur while consuming the response body,
                        /// but those will be reported by the `incoming-body` and its
                        /// `output-stream` child.
                        #[allow(async_fn_in_trait)]
                        pub fn get(
                            &self,
                        ) -> Option<Result<Result<IncomingResponse, ErrorCode>, ()>> {
                            unsafe {
                                #[repr(align(8))]
                                struct RetArea(
                                    [::core::mem::MaybeUninit<
                                        u8,
                                    >; 40 + 4 * ::core::mem::size_of::<*const u8>()],
                                );
                                let mut ret_area = RetArea(
                                    [::core::mem::MaybeUninit::uninit(); 40
                                        + 4 * ::core::mem::size_of::<*const u8>()],
                                );
                                let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                                #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[method]future-incoming-response.get"]
                                    fn wit_import1(_: i32, _: *mut u8);
                                }
                                wit_import1((self).handle() as i32, ptr0);
                                let l2 = i32::from(*ptr0.add(0).cast::<u8>());
                                let result69 = match l2 {
                                    0 => None,
                                    1 => {
                                        let e = {
                                            let l3 = i32::from(*ptr0.add(8).cast::<u8>());
                                            match l3 {
                                                0 => {
                                                    let e = {
                                                        let l4 = i32::from(*ptr0.add(16).cast::<u8>());
                                                        match l4 {
                                                            0 => {
                                                                let e = {
                                                                    let l5 = *ptr0.add(24).cast::<i32>();
                                                                    IncomingResponse::from_handle(l5 as u32)
                                                                };
                                                                Ok(e)
                                                            }
                                                            1 => {
                                                                let e = {
                                                                    let l6 = i32::from(*ptr0.add(24).cast::<u8>());
                                                                    let v68 = match l6 {
                                                                        0 => ErrorCode::DnsTimeout,
                                                                        1 => {
                                                                            let e68 = {
                                                                                let l7 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                                let l11 = i32::from(
                                                                                    *ptr0
                                                                                        .add(32 + 3 * ::core::mem::size_of::<*const u8>())
                                                                                        .cast::<u8>(),
                                                                                );
                                                                                DnsErrorPayload {
                                                                                    rcode: match l7 {
                                                                                        0 => None,
                                                                                        1 => {
                                                                                            let e = {
                                                                                                let l8 = *ptr0
                                                                                                    .add(32 + 1 * ::core::mem::size_of::<*const u8>())
                                                                                                    .cast::<*mut u8>();
                                                                                                let l9 = *ptr0
                                                                                                    .add(32 + 2 * ::core::mem::size_of::<*const u8>())
                                                                                                    .cast::<usize>();
                                                                                                let len10 = l9;
                                                                                                let bytes10 = _rt::Vec::from_raw_parts(
                                                                                                    l8.cast(),
                                                                                                    len10,
                                                                                                    len10,
                                                                                                );
                                                                                                _rt::string_lift(bytes10)
                                                                                            };
                                                                                            Some(e)
                                                                                        }
                                                                                        _ => _rt::invalid_enum_discriminant(),
                                                                                    },
                                                                                    info_code: match l11 {
                                                                                        0 => None,
                                                                                        1 => {
                                                                                            let e = {
                                                                                                let l12 = i32::from(
                                                                                                    *ptr0
                                                                                                        .add(34 + 3 * ::core::mem::size_of::<*const u8>())
                                                                                                        .cast::<u16>(),
                                                                                                );
                                                                                                l12 as u16
                                                                                            };
                                                                                            Some(e)
                                                                                        }
                                                                                        _ => _rt::invalid_enum_discriminant(),
                                                                                    },
                                                                                }
                                                                            };
                                                                            ErrorCode::DnsError(e68)
                                                                        }
                                                                        2 => ErrorCode::DestinationNotFound,
                                                                        3 => ErrorCode::DestinationUnavailable,
                                                                        4 => ErrorCode::DestinationIpProhibited,
                                                                        5 => ErrorCode::DestinationIpUnroutable,
                                                                        6 => ErrorCode::ConnectionRefused,
                                                                        7 => ErrorCode::ConnectionTerminated,
                                                                        8 => ErrorCode::ConnectionTimeout,
                                                                        9 => ErrorCode::ConnectionReadTimeout,
                                                                        10 => ErrorCode::ConnectionWriteTimeout,
                                                                        11 => ErrorCode::ConnectionLimitReached,
                                                                        12 => ErrorCode::TlsProtocolError,
                                                                        13 => ErrorCode::TlsCertificateError,
                                                                        14 => {
                                                                            let e68 = {
                                                                                let l13 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                                let l15 = i32::from(
                                                                                    *ptr0
                                                                                        .add(32 + 1 * ::core::mem::size_of::<*const u8>())
                                                                                        .cast::<u8>(),
                                                                                );
                                                                                TlsAlertReceivedPayload {
                                                                                    alert_id: match l13 {
                                                                                        0 => None,
                                                                                        1 => {
                                                                                            let e = {
                                                                                                let l14 = i32::from(*ptr0.add(33).cast::<u8>());
                                                                                                l14 as u8
                                                                                            };
                                                                                            Some(e)
                                                                                        }
                                                                                        _ => _rt::invalid_enum_discriminant(),
                                                                                    },
                                                                                    alert_message: match l15 {
                                                                                        0 => None,
                                                                                        1 => {
                                                                                            let e = {
                                                                                                let l16 = *ptr0
                                                                                                    .add(32 + 2 * ::core::mem::size_of::<*const u8>())
                                                                                                    .cast::<*mut u8>();
                                                                                                let l17 = *ptr0
                                                                                                    .add(32 + 3 * ::core::mem::size_of::<*const u8>())
                                                                                                    .cast::<usize>();
                                                                                                let len18 = l17;
                                                                                                let bytes18 = _rt::Vec::from_raw_parts(
                                                                                                    l16.cast(),
                                                                                                    len18,
                                                                                                    len18,
                                                                                                );
                                                                                                _rt::string_lift(bytes18)
                                                                                            };
                                                                                            Some(e)
                                                                                        }
                                                                                        _ => _rt::invalid_enum_discriminant(),
                                                                                    },
                                                                                }
                                                                            };
                                                                            ErrorCode::TlsAlertReceived(e68)
                                                                        }
                                                                        15 => ErrorCode::HttpRequestDenied,
                                                                        16 => ErrorCode::HttpRequestLengthRequired,
                                                                        17 => {
                                                                            let e68 = {
                                                                                let l19 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                                match l19 {
                                                                                    0 => None,
                                                                                    1 => {
                                                                                        let e = {
                                                                                            let l20 = *ptr0.add(40).cast::<i64>();
                                                                                            l20 as u64
                                                                                        };
                                                                                        Some(e)
                                                                                    }
                                                                                    _ => _rt::invalid_enum_discriminant(),
                                                                                }
                                                                            };
                                                                            ErrorCode::HttpRequestBodySize(e68)
                                                                        }
                                                                        18 => ErrorCode::HttpRequestMethodInvalid,
                                                                        19 => ErrorCode::HttpRequestUriInvalid,
                                                                        20 => ErrorCode::HttpRequestUriTooLong,
                                                                        21 => {
                                                                            let e68 = {
                                                                                let l21 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                                match l21 {
                                                                                    0 => None,
                                                                                    1 => {
                                                                                        let e = {
                                                                                            let l22 = *ptr0.add(36).cast::<i32>();
                                                                                            l22 as u32
                                                                                        };
                                                                                        Some(e)
                                                                                    }
                                                                                    _ => _rt::invalid_enum_discriminant(),
                                                                                }
                                                                            };
                                                                            ErrorCode::HttpRequestHeaderSectionSize(e68)
                                                                        }
                                                                        22 => {
                                                                            let e68 = {
                                                                                let l23 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                                match l23 {
                                                                                    0 => None,
                                                                                    1 => {
                                                                                        let e = {
                                                                                            let l24 = i32::from(
                                                                                                *ptr0
                                                                                                    .add(32 + 1 * ::core::mem::size_of::<*const u8>())
                                                                                                    .cast::<u8>(),
                                                                                            );
                                                                                            let l28 = i32::from(
                                                                                                *ptr0
                                                                                                    .add(32 + 4 * ::core::mem::size_of::<*const u8>())
                                                                                                    .cast::<u8>(),
                                                                                            );
                                                                                            FieldSizePayload {
                                                                                                field_name: match l24 {
                                                                                                    0 => None,
                                                                                                    1 => {
                                                                                                        let e = {
                                                                                                            let l25 = *ptr0
                                                                                                                .add(32 + 2 * ::core::mem::size_of::<*const u8>())
                                                                                                                .cast::<*mut u8>();
                                                                                                            let l26 = *ptr0
                                                                                                                .add(32 + 3 * ::core::mem::size_of::<*const u8>())
                                                                                                                .cast::<usize>();
                                                                                                            let len27 = l26;
                                                                                                            let bytes27 = _rt::Vec::from_raw_parts(
                                                                                                                l25.cast(),
                                                                                                                len27,
                                                                                                                len27,
                                                                                                            );
                                                                                                            _rt::string_lift(bytes27)
                                                                                                        };
                                                                                                        Some(e)
                                                                                                    }
                                                                                                    _ => _rt::invalid_enum_discriminant(),
                                                                                                },
                                                                                                field_size: match l28 {
                                                                                                    0 => None,
                                                                                                    1 => {
                                                                                                        let e = {
                                                                                                            let l29 = *ptr0
                                                                                                                .add(36 + 4 * ::core::mem::size_of::<*const u8>())
                                                                                                                .cast::<i32>();
                                                                                                            l29 as u32
                                                                                                        };
                                                                                                        Some(e)
                                                                                                    }
                                                                                                    _ => _rt::invalid_enum_discriminant(),
                                                                                                },
                                                                                            }
                                                                                        };
                                                                                        Some(e)
                                                                                    }
                                                                                    _ => _rt::invalid_enum_discriminant(),
                                                                                }
                                                                            };
                                                                            ErrorCode::HttpRequestHeaderSize(e68)
                                                                        }
                                                                        23 => {
                                                                            let e68 = {
                                                                                let l30 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                                match l30 {
                                                                                    0 => None,
                                                                                    1 => {
                                                                                        let e = {
                                                                                            let l31 = *ptr0.add(36).cast::<i32>();
                                                                                            l31 as u32
                                                                                        };
                                                                                        Some(e)
                                                                                    }
                                                                                    _ => _rt::invalid_enum_discriminant(),
                                                                                }
                                                                            };
                                                                            ErrorCode::HttpRequestTrailerSectionSize(e68)
                                                                        }
                                                                        24 => {
                                                                            let e68 = {
                                                                                let l32 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                                let l36 = i32::from(
                                                                                    *ptr0
                                                                                        .add(32 + 3 * ::core::mem::size_of::<*const u8>())
                                                                                        .cast::<u8>(),
                                                                                );
                                                                                FieldSizePayload {
                                                                                    field_name: match l32 {
                                                                                        0 => None,
                                                                                        1 => {
                                                                                            let e = {
                                                                                                let l33 = *ptr0
                                                                                                    .add(32 + 1 * ::core::mem::size_of::<*const u8>())
                                                                                                    .cast::<*mut u8>();
                                                                                                let l34 = *ptr0
                                                                                                    .add(32 + 2 * ::core::mem::size_of::<*const u8>())
                                                                                                    .cast::<usize>();
                                                                                                let len35 = l34;
                                                                                                let bytes35 = _rt::Vec::from_raw_parts(
                                                                                                    l33.cast(),
                                                                                                    len35,
                                                                                                    len35,
                                                                                                );
                                                                                                _rt::string_lift(bytes35)
                                                                                            };
                                                                                            Some(e)
                                                                                        }
                                                                                        _ => _rt::invalid_enum_discriminant(),
                                                                                    },
                                                                                    field_size: match l36 {
                                                                                        0 => None,
                                                                                        1 => {
                                                                                            let e = {
                                                                                                let l37 = *ptr0
                                                                                                    .add(36 + 3 * ::core::mem::size_of::<*const u8>())
                                                                                                    .cast::<i32>();
                                                                                                l37 as u32
                                                                                            };
                                                                                            Some(e)
                                                                                        }
                                                                                        _ => _rt::invalid_enum_discriminant(),
                                                                                    },
                                                                                }
                                                                            };
                                                                            ErrorCode::HttpRequestTrailerSize(e68)
                                                                        }
                                                                        25 => ErrorCode::HttpResponseIncomplete,
                                                                        26 => {
                                                                            let e68 = {
                                                                                let l38 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                                match l38 {
                                                                                    0 => None,
                                                                                    1 => {
                                                                                        let e = {
                                                                                            let l39 = *ptr0.add(36).cast::<i32>();
                                                                                            l39 as u32
                                                                                        };
                                                                                        Some(e)
                                                                                    }
                                                                                    _ => _rt::invalid_enum_discriminant(),
                                                                                }
                                                                            };
                                                                            ErrorCode::HttpResponseHeaderSectionSize(e68)
                                                                        }
                                                                        27 => {
                                                                            let e68 = {
                                                                                let l40 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                                let l44 = i32::from(
                                                                                    *ptr0
                                                                                        .add(32 + 3 * ::core::mem::size_of::<*const u8>())
                                                                                        .cast::<u8>(),
                                                                                );
                                                                                FieldSizePayload {
                                                                                    field_name: match l40 {
                                                                                        0 => None,
                                                                                        1 => {
                                                                                            let e = {
                                                                                                let l41 = *ptr0
                                                                                                    .add(32 + 1 * ::core::mem::size_of::<*const u8>())
                                                                                                    .cast::<*mut u8>();
                                                                                                let l42 = *ptr0
                                                                                                    .add(32 + 2 * ::core::mem::size_of::<*const u8>())
                                                                                                    .cast::<usize>();
                                                                                                let len43 = l42;
                                                                                                let bytes43 = _rt::Vec::from_raw_parts(
                                                                                                    l41.cast(),
                                                                                                    len43,
                                                                                                    len43,
                                                                                                );
                                                                                                _rt::string_lift(bytes43)
                                                                                            };
                                                                                            Some(e)
                                                                                        }
                                                                                        _ => _rt::invalid_enum_discriminant(),
                                                                                    },
                                                                                    field_size: match l44 {
                                                                                        0 => None,
                                                                                        1 => {
                                                                                            let e = {
                                                                                                let l45 = *ptr0
                                                                                                    .add(36 + 3 * ::core::mem::size_of::<*const u8>())
                                                                                                    .cast::<i32>();
                                                                                                l45 as u32
                                                                                            };
                                                                                            Some(e)
                                                                                        }
                                                                                        _ => _rt::invalid_enum_discriminant(),
                                                                                    },
                                                                                }
                                                                            };
                                                                            ErrorCode::HttpResponseHeaderSize(e68)
                                                                        }
                                                                        28 => {
                                                                            let e68 = {
                                                                                let l46 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                                match l46 {
                                                                                    0 => None,
                                                                                    1 => {
                                                                                        let e = {
                                                                                            let l47 = *ptr0.add(40).cast::<i64>();
                                                                                            l47 as u64
                                                                                        };
                                                                                        Some(e)
                                                                                    }
                                                                                    _ => _rt::invalid_enum_discriminant(),
                                                                                }
                                                                            };
                                                                            ErrorCode::HttpResponseBodySize(e68)
                                                                        }
                                                                        29 => {
                                                                            let e68 = {
                                                                                let l48 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                                match l48 {
                                                                                    0 => None,
                                                                                    1 => {
                                                                                        let e = {
                                                                                            let l49 = *ptr0.add(36).cast::<i32>();
                                                                                            l49 as u32
                                                                                        };
                                                                                        Some(e)
                                                                                    }
                                                                                    _ => _rt::invalid_enum_discriminant(),
                                                                                }
                                                                            };
                                                                            ErrorCode::HttpResponseTrailerSectionSize(e68)
                                                                        }
                                                                        30 => {
                                                                            let e68 = {
                                                                                let l50 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                                let l54 = i32::from(
                                                                                    *ptr0
                                                                                        .add(32 + 3 * ::core::mem::size_of::<*const u8>())
                                                                                        .cast::<u8>(),
                                                                                );
                                                                                FieldSizePayload {
                                                                                    field_name: match l50 {
                                                                                        0 => None,
                                                                                        1 => {
                                                                                            let e = {
                                                                                                let l51 = *ptr0
                                                                                                    .add(32 + 1 * ::core::mem::size_of::<*const u8>())
                                                                                                    .cast::<*mut u8>();
                                                                                                let l52 = *ptr0
                                                                                                    .add(32 + 2 * ::core::mem::size_of::<*const u8>())
                                                                                                    .cast::<usize>();
                                                                                                let len53 = l52;
                                                                                                let bytes53 = _rt::Vec::from_raw_parts(
                                                                                                    l51.cast(),
                                                                                                    len53,
                                                                                                    len53,
                                                                                                );
                                                                                                _rt::string_lift(bytes53)
                                                                                            };
                                                                                            Some(e)
                                                                                        }
                                                                                        _ => _rt::invalid_enum_discriminant(),
                                                                                    },
                                                                                    field_size: match l54 {
                                                                                        0 => None,
                                                                                        1 => {
                                                                                            let e = {
                                                                                                let l55 = *ptr0
                                                                                                    .add(36 + 3 * ::core::mem::size_of::<*const u8>())
                                                                                                    .cast::<i32>();
                                                                                                l55 as u32
                                                                                            };
                                                                                            Some(e)
                                                                                        }
                                                                                        _ => _rt::invalid_enum_discriminant(),
                                                                                    },
                                                                                }
                                                                            };
                                                                            ErrorCode::HttpResponseTrailerSize(e68)
                                                                        }
                                                                        31 => {
                                                                            let e68 = {
                                                                                let l56 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                                match l56 {
                                                                                    0 => None,
                                                                                    1 => {
                                                                                        let e = {
                                                                                            let l57 = *ptr0
                                                                                                .add(32 + 1 * ::core::mem::size_of::<*const u8>())
                                                                                                .cast::<*mut u8>();
                                                                                            let l58 = *ptr0
                                                                                                .add(32 + 2 * ::core::mem::size_of::<*const u8>())
                                                                                                .cast::<usize>();
                                                                                            let len59 = l58;
                                                                                            let bytes59 = _rt::Vec::from_raw_parts(
                                                                                                l57.cast(),
                                                                                                len59,
                                                                                                len59,
                                                                                            );
                                                                                            _rt::string_lift(bytes59)
                                                                                        };
                                                                                        Some(e)
                                                                                    }
                                                                                    _ => _rt::invalid_enum_discriminant(),
                                                                                }
                                                                            };
                                                                            ErrorCode::HttpResponseTransferCoding(e68)
                                                                        }
                                                                        32 => {
                                                                            let e68 = {
                                                                                let l60 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                                match l60 {
                                                                                    0 => None,
                                                                                    1 => {
                                                                                        let e = {
                                                                                            let l61 = *ptr0
                                                                                                .add(32 + 1 * ::core::mem::size_of::<*const u8>())
                                                                                                .cast::<*mut u8>();
                                                                                            let l62 = *ptr0
                                                                                                .add(32 + 2 * ::core::mem::size_of::<*const u8>())
                                                                                                .cast::<usize>();
                                                                                            let len63 = l62;
                                                                                            let bytes63 = _rt::Vec::from_raw_parts(
                                                                                                l61.cast(),
                                                                                                len63,
                                                                                                len63,
                                                                                            );
                                                                                            _rt::string_lift(bytes63)
                                                                                        };
                                                                                        Some(e)
                                                                                    }
                                                                                    _ => _rt::invalid_enum_discriminant(),
                                                                                }
                                                                            };
                                                                            ErrorCode::HttpResponseContentCoding(e68)
                                                                        }
                                                                        33 => ErrorCode::HttpResponseTimeout,
                                                                        34 => ErrorCode::HttpUpgradeFailed,
                                                                        35 => ErrorCode::HttpProtocolError,
                                                                        36 => ErrorCode::LoopDetected,
                                                                        37 => ErrorCode::ConfigurationError,
                                                                        n => {
                                                                            if true {
                                                                                match (&n, &38) {
                                                                                    (left_val, right_val) => {
                                                                                        if !(*left_val == *right_val) {
                                                                                            let kind = ::core::panicking::AssertKind::Eq;
                                                                                            ::core::panicking::assert_failed(
                                                                                                kind,
                                                                                                &*left_val,
                                                                                                &*right_val,
                                                                                                ::core::option::Option::Some(
                                                                                                    format_args!("invalid enum discriminant"),
                                                                                                ),
                                                                                            );
                                                                                        }
                                                                                    }
                                                                                };
                                                                            }
                                                                            let e68 = {
                                                                                let l64 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                                match l64 {
                                                                                    0 => None,
                                                                                    1 => {
                                                                                        let e = {
                                                                                            let l65 = *ptr0
                                                                                                .add(32 + 1 * ::core::mem::size_of::<*const u8>())
                                                                                                .cast::<*mut u8>();
                                                                                            let l66 = *ptr0
                                                                                                .add(32 + 2 * ::core::mem::size_of::<*const u8>())
                                                                                                .cast::<usize>();
                                                                                            let len67 = l66;
                                                                                            let bytes67 = _rt::Vec::from_raw_parts(
                                                                                                l65.cast(),
                                                                                                len67,
                                                                                                len67,
                                                                                            );
                                                                                            _rt::string_lift(bytes67)
                                                                                        };
                                                                                        Some(e)
                                                                                    }
                                                                                    _ => _rt::invalid_enum_discriminant(),
                                                                                }
                                                                            };
                                                                            ErrorCode::InternalError(e68)
                                                                        }
                                                                    };
                                                                    v68
                                                                };
                                                                Err(e)
                                                            }
                                                            _ => _rt::invalid_enum_discriminant(),
                                                        }
                                                    };
                                                    Ok(e)
                                                }
                                                1 => {
                                                    let e = ();
                                                    Err(e)
                                                }
                                                _ => _rt::invalid_enum_discriminant(),
                                            }
                                        };
                                        Some(e)
                                    }
                                    _ => _rt::invalid_enum_discriminant(),
                                };
                                result69
                            }
                        }
                    }
                }
                /// This interface defines a handler of outgoing HTTP Requests. It should be
                /// imported by components which wish to make HTTP Requests.
                #[allow(dead_code, async_fn_in_trait, unused_imports, clippy::all)]
                pub mod outgoing_handler {
                    #[used]
                    #[doc(hidden)]
                    static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
                    use super::super::super::_rt;
                    pub type OutgoingRequest = super::super::super::wasi::http::types::OutgoingRequest;
                    pub type RequestOptions = super::super::super::wasi::http::types::RequestOptions;
                    pub type FutureIncomingResponse = super::super::super::wasi::http::types::FutureIncomingResponse;
                    pub type ErrorCode = super::super::super::wasi::http::types::ErrorCode;
                    #[allow(unused_unsafe, clippy::all)]
                    /// This function is invoked with an outgoing HTTP Request, and it returns
                    /// a resource `future-incoming-response` which represents an HTTP Response
                    /// which may arrive in the future.
                    ///
                    /// The `options` argument accepts optional parameters for the HTTP
                    /// protocol's transport layer.
                    ///
                    /// This function may return an error if the `outgoing-request` is invalid
                    /// or not allowed to be made. Otherwise, protocol errors are reported
                    /// through the `future-incoming-response`.
                    #[allow(async_fn_in_trait)]
                    pub fn handle(
                        request: OutgoingRequest,
                        options: Option<RequestOptions>,
                    ) -> Result<FutureIncomingResponse, ErrorCode> {
                        unsafe {
                            #[repr(align(8))]
                            struct RetArea(
                                [::core::mem::MaybeUninit<
                                    u8,
                                >; 24 + 4 * ::core::mem::size_of::<*const u8>()],
                            );
                            let mut ret_area = RetArea(
                                [::core::mem::MaybeUninit::uninit(); 24
                                    + 4 * ::core::mem::size_of::<*const u8>()],
                            );
                            let (result0_0, result0_1) = match &options {
                                Some(e) => (1i32, (e).take_handle() as i32),
                                None => (0i32, 0i32),
                            };
                            let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                            #[link(
                                wasm_import_module = "wasi:http/outgoing-handler@0.2.0"
                            )]
                            unsafe extern "C" {
                                #[link_name = "handle"]
                                fn wit_import2(_: i32, _: i32, _: i32, _: *mut u8);
                            }
                            wit_import2(
                                (&request).take_handle() as i32,
                                result0_0,
                                result0_1,
                                ptr1,
                            );
                            let l3 = i32::from(*ptr1.add(0).cast::<u8>());
                            let result68 = match l3 {
                                0 => {
                                    let e = {
                                        let l4 = *ptr1.add(8).cast::<i32>();
                                        super::super::super::wasi::http::types::FutureIncomingResponse::from_handle(
                                            l4 as u32,
                                        )
                                    };
                                    Ok(e)
                                }
                                1 => {
                                    let e = {
                                        let l5 = i32::from(*ptr1.add(8).cast::<u8>());
                                        use super::super::super::wasi::http::types::ErrorCode as V67;
                                        let v67 = match l5 {
                                            0 => V67::DnsTimeout,
                                            1 => {
                                                let e67 = {
                                                    let l6 = i32::from(*ptr1.add(16).cast::<u8>());
                                                    let l10 = i32::from(
                                                        *ptr1
                                                            .add(16 + 3 * ::core::mem::size_of::<*const u8>())
                                                            .cast::<u8>(),
                                                    );
                                                    super::super::super::wasi::http::types::DnsErrorPayload {
                                                        rcode: match l6 {
                                                            0 => None,
                                                            1 => {
                                                                let e = {
                                                                    let l7 = *ptr1
                                                                        .add(16 + 1 * ::core::mem::size_of::<*const u8>())
                                                                        .cast::<*mut u8>();
                                                                    let l8 = *ptr1
                                                                        .add(16 + 2 * ::core::mem::size_of::<*const u8>())
                                                                        .cast::<usize>();
                                                                    let len9 = l8;
                                                                    let bytes9 = _rt::Vec::from_raw_parts(
                                                                        l7.cast(),
                                                                        len9,
                                                                        len9,
                                                                    );
                                                                    _rt::string_lift(bytes9)
                                                                };
                                                                Some(e)
                                                            }
                                                            _ => _rt::invalid_enum_discriminant(),
                                                        },
                                                        info_code: match l10 {
                                                            0 => None,
                                                            1 => {
                                                                let e = {
                                                                    let l11 = i32::from(
                                                                        *ptr1
                                                                            .add(18 + 3 * ::core::mem::size_of::<*const u8>())
                                                                            .cast::<u16>(),
                                                                    );
                                                                    l11 as u16
                                                                };
                                                                Some(e)
                                                            }
                                                            _ => _rt::invalid_enum_discriminant(),
                                                        },
                                                    }
                                                };
                                                V67::DnsError(e67)
                                            }
                                            2 => V67::DestinationNotFound,
                                            3 => V67::DestinationUnavailable,
                                            4 => V67::DestinationIpProhibited,
                                            5 => V67::DestinationIpUnroutable,
                                            6 => V67::ConnectionRefused,
                                            7 => V67::ConnectionTerminated,
                                            8 => V67::ConnectionTimeout,
                                            9 => V67::ConnectionReadTimeout,
                                            10 => V67::ConnectionWriteTimeout,
                                            11 => V67::ConnectionLimitReached,
                                            12 => V67::TlsProtocolError,
                                            13 => V67::TlsCertificateError,
                                            14 => {
                                                let e67 = {
                                                    let l12 = i32::from(*ptr1.add(16).cast::<u8>());
                                                    let l14 = i32::from(
                                                        *ptr1
                                                            .add(16 + 1 * ::core::mem::size_of::<*const u8>())
                                                            .cast::<u8>(),
                                                    );
                                                    super::super::super::wasi::http::types::TlsAlertReceivedPayload {
                                                        alert_id: match l12 {
                                                            0 => None,
                                                            1 => {
                                                                let e = {
                                                                    let l13 = i32::from(*ptr1.add(17).cast::<u8>());
                                                                    l13 as u8
                                                                };
                                                                Some(e)
                                                            }
                                                            _ => _rt::invalid_enum_discriminant(),
                                                        },
                                                        alert_message: match l14 {
                                                            0 => None,
                                                            1 => {
                                                                let e = {
                                                                    let l15 = *ptr1
                                                                        .add(16 + 2 * ::core::mem::size_of::<*const u8>())
                                                                        .cast::<*mut u8>();
                                                                    let l16 = *ptr1
                                                                        .add(16 + 3 * ::core::mem::size_of::<*const u8>())
                                                                        .cast::<usize>();
                                                                    let len17 = l16;
                                                                    let bytes17 = _rt::Vec::from_raw_parts(
                                                                        l15.cast(),
                                                                        len17,
                                                                        len17,
                                                                    );
                                                                    _rt::string_lift(bytes17)
                                                                };
                                                                Some(e)
                                                            }
                                                            _ => _rt::invalid_enum_discriminant(),
                                                        },
                                                    }
                                                };
                                                V67::TlsAlertReceived(e67)
                                            }
                                            15 => V67::HttpRequestDenied,
                                            16 => V67::HttpRequestLengthRequired,
                                            17 => {
                                                let e67 = {
                                                    let l18 = i32::from(*ptr1.add(16).cast::<u8>());
                                                    match l18 {
                                                        0 => None,
                                                        1 => {
                                                            let e = {
                                                                let l19 = *ptr1.add(24).cast::<i64>();
                                                                l19 as u64
                                                            };
                                                            Some(e)
                                                        }
                                                        _ => _rt::invalid_enum_discriminant(),
                                                    }
                                                };
                                                V67::HttpRequestBodySize(e67)
                                            }
                                            18 => V67::HttpRequestMethodInvalid,
                                            19 => V67::HttpRequestUriInvalid,
                                            20 => V67::HttpRequestUriTooLong,
                                            21 => {
                                                let e67 = {
                                                    let l20 = i32::from(*ptr1.add(16).cast::<u8>());
                                                    match l20 {
                                                        0 => None,
                                                        1 => {
                                                            let e = {
                                                                let l21 = *ptr1.add(20).cast::<i32>();
                                                                l21 as u32
                                                            };
                                                            Some(e)
                                                        }
                                                        _ => _rt::invalid_enum_discriminant(),
                                                    }
                                                };
                                                V67::HttpRequestHeaderSectionSize(e67)
                                            }
                                            22 => {
                                                let e67 = {
                                                    let l22 = i32::from(*ptr1.add(16).cast::<u8>());
                                                    match l22 {
                                                        0 => None,
                                                        1 => {
                                                            let e = {
                                                                let l23 = i32::from(
                                                                    *ptr1
                                                                        .add(16 + 1 * ::core::mem::size_of::<*const u8>())
                                                                        .cast::<u8>(),
                                                                );
                                                                let l27 = i32::from(
                                                                    *ptr1
                                                                        .add(16 + 4 * ::core::mem::size_of::<*const u8>())
                                                                        .cast::<u8>(),
                                                                );
                                                                super::super::super::wasi::http::types::FieldSizePayload {
                                                                    field_name: match l23 {
                                                                        0 => None,
                                                                        1 => {
                                                                            let e = {
                                                                                let l24 = *ptr1
                                                                                    .add(16 + 2 * ::core::mem::size_of::<*const u8>())
                                                                                    .cast::<*mut u8>();
                                                                                let l25 = *ptr1
                                                                                    .add(16 + 3 * ::core::mem::size_of::<*const u8>())
                                                                                    .cast::<usize>();
                                                                                let len26 = l25;
                                                                                let bytes26 = _rt::Vec::from_raw_parts(
                                                                                    l24.cast(),
                                                                                    len26,
                                                                                    len26,
                                                                                );
                                                                                _rt::string_lift(bytes26)
                                                                            };
                                                                            Some(e)
                                                                        }
                                                                        _ => _rt::invalid_enum_discriminant(),
                                                                    },
                                                                    field_size: match l27 {
                                                                        0 => None,
                                                                        1 => {
                                                                            let e = {
                                                                                let l28 = *ptr1
                                                                                    .add(20 + 4 * ::core::mem::size_of::<*const u8>())
                                                                                    .cast::<i32>();
                                                                                l28 as u32
                                                                            };
                                                                            Some(e)
                                                                        }
                                                                        _ => _rt::invalid_enum_discriminant(),
                                                                    },
                                                                }
                                                            };
                                                            Some(e)
                                                        }
                                                        _ => _rt::invalid_enum_discriminant(),
                                                    }
                                                };
                                                V67::HttpRequestHeaderSize(e67)
                                            }
                                            23 => {
                                                let e67 = {
                                                    let l29 = i32::from(*ptr1.add(16).cast::<u8>());
                                                    match l29 {
                                                        0 => None,
                                                        1 => {
                                                            let e = {
                                                                let l30 = *ptr1.add(20).cast::<i32>();
                                                                l30 as u32
                                                            };
                                                            Some(e)
                                                        }
                                                        _ => _rt::invalid_enum_discriminant(),
                                                    }
                                                };
                                                V67::HttpRequestTrailerSectionSize(e67)
                                            }
                                            24 => {
                                                let e67 = {
                                                    let l31 = i32::from(*ptr1.add(16).cast::<u8>());
                                                    let l35 = i32::from(
                                                        *ptr1
                                                            .add(16 + 3 * ::core::mem::size_of::<*const u8>())
                                                            .cast::<u8>(),
                                                    );
                                                    super::super::super::wasi::http::types::FieldSizePayload {
                                                        field_name: match l31 {
                                                            0 => None,
                                                            1 => {
                                                                let e = {
                                                                    let l32 = *ptr1
                                                                        .add(16 + 1 * ::core::mem::size_of::<*const u8>())
                                                                        .cast::<*mut u8>();
                                                                    let l33 = *ptr1
                                                                        .add(16 + 2 * ::core::mem::size_of::<*const u8>())
                                                                        .cast::<usize>();
                                                                    let len34 = l33;
                                                                    let bytes34 = _rt::Vec::from_raw_parts(
                                                                        l32.cast(),
                                                                        len34,
                                                                        len34,
                                                                    );
                                                                    _rt::string_lift(bytes34)
                                                                };
                                                                Some(e)
                                                            }
                                                            _ => _rt::invalid_enum_discriminant(),
                                                        },
                                                        field_size: match l35 {
                                                            0 => None,
                                                            1 => {
                                                                let e = {
                                                                    let l36 = *ptr1
                                                                        .add(20 + 3 * ::core::mem::size_of::<*const u8>())
                                                                        .cast::<i32>();
                                                                    l36 as u32
                                                                };
                                                                Some(e)
                                                            }
                                                            _ => _rt::invalid_enum_discriminant(),
                                                        },
                                                    }
                                                };
                                                V67::HttpRequestTrailerSize(e67)
                                            }
                                            25 => V67::HttpResponseIncomplete,
                                            26 => {
                                                let e67 = {
                                                    let l37 = i32::from(*ptr1.add(16).cast::<u8>());
                                                    match l37 {
                                                        0 => None,
                                                        1 => {
                                                            let e = {
                                                                let l38 = *ptr1.add(20).cast::<i32>();
                                                                l38 as u32
                                                            };
                                                            Some(e)
                                                        }
                                                        _ => _rt::invalid_enum_discriminant(),
                                                    }
                                                };
                                                V67::HttpResponseHeaderSectionSize(e67)
                                            }
                                            27 => {
                                                let e67 = {
                                                    let l39 = i32::from(*ptr1.add(16).cast::<u8>());
                                                    let l43 = i32::from(
                                                        *ptr1
                                                            .add(16 + 3 * ::core::mem::size_of::<*const u8>())
                                                            .cast::<u8>(),
                                                    );
                                                    super::super::super::wasi::http::types::FieldSizePayload {
                                                        field_name: match l39 {
                                                            0 => None,
                                                            1 => {
                                                                let e = {
                                                                    let l40 = *ptr1
                                                                        .add(16 + 1 * ::core::mem::size_of::<*const u8>())
                                                                        .cast::<*mut u8>();
                                                                    let l41 = *ptr1
                                                                        .add(16 + 2 * ::core::mem::size_of::<*const u8>())
                                                                        .cast::<usize>();
                                                                    let len42 = l41;
                                                                    let bytes42 = _rt::Vec::from_raw_parts(
                                                                        l40.cast(),
                                                                        len42,
                                                                        len42,
                                                                    );
                                                                    _rt::string_lift(bytes42)
                                                                };
                                                                Some(e)
                                                            }
                                                            _ => _rt::invalid_enum_discriminant(),
                                                        },
                                                        field_size: match l43 {
                                                            0 => None,
                                                            1 => {
                                                                let e = {
                                                                    let l44 = *ptr1
                                                                        .add(20 + 3 * ::core::mem::size_of::<*const u8>())
                                                                        .cast::<i32>();
                                                                    l44 as u32
                                                                };
                                                                Some(e)
                                                            }
                                                            _ => _rt::invalid_enum_discriminant(),
                                                        },
                                                    }
                                                };
                                                V67::HttpResponseHeaderSize(e67)
                                            }
                                            28 => {
                                                let e67 = {
                                                    let l45 = i32::from(*ptr1.add(16).cast::<u8>());
                                                    match l45 {
                                                        0 => None,
                                                        1 => {
                                                            let e = {
                                                                let l46 = *ptr1.add(24).cast::<i64>();
                                                                l46 as u64
                                                            };
                                                            Some(e)
                                                        }
                                                        _ => _rt::invalid_enum_discriminant(),
                                                    }
                                                };
                                                V67::HttpResponseBodySize(e67)
                                            }
                                            29 => {
                                                let e67 = {
                                                    let l47 = i32::from(*ptr1.add(16).cast::<u8>());
                                                    match l47 {
                                                        0 => None,
                                                        1 => {
                                                            let e = {
                                                                let l48 = *ptr1.add(20).cast::<i32>();
                                                                l48 as u32
                                                            };
                                                            Some(e)
                                                        }
                                                        _ => _rt::invalid_enum_discriminant(),
                                                    }
                                                };
                                                V67::HttpResponseTrailerSectionSize(e67)
                                            }
                                            30 => {
                                                let e67 = {
                                                    let l49 = i32::from(*ptr1.add(16).cast::<u8>());
                                                    let l53 = i32::from(
                                                        *ptr1
                                                            .add(16 + 3 * ::core::mem::size_of::<*const u8>())
                                                            .cast::<u8>(),
                                                    );
                                                    super::super::super::wasi::http::types::FieldSizePayload {
                                                        field_name: match l49 {
                                                            0 => None,
                                                            1 => {
                                                                let e = {
                                                                    let l50 = *ptr1
                                                                        .add(16 + 1 * ::core::mem::size_of::<*const u8>())
                                                                        .cast::<*mut u8>();
                                                                    let l51 = *ptr1
                                                                        .add(16 + 2 * ::core::mem::size_of::<*const u8>())
                                                                        .cast::<usize>();
                                                                    let len52 = l51;
                                                                    let bytes52 = _rt::Vec::from_raw_parts(
                                                                        l50.cast(),
                                                                        len52,
                                                                        len52,
                                                                    );
                                                                    _rt::string_lift(bytes52)
                                                                };
                                                                Some(e)
                                                            }
                                                            _ => _rt::invalid_enum_discriminant(),
                                                        },
                                                        field_size: match l53 {
                                                            0 => None,
                                                            1 => {
                                                                let e = {
                                                                    let l54 = *ptr1
                                                                        .add(20 + 3 * ::core::mem::size_of::<*const u8>())
                                                                        .cast::<i32>();
                                                                    l54 as u32
                                                                };
                                                                Some(e)
                                                            }
                                                            _ => _rt::invalid_enum_discriminant(),
                                                        },
                                                    }
                                                };
                                                V67::HttpResponseTrailerSize(e67)
                                            }
                                            31 => {
                                                let e67 = {
                                                    let l55 = i32::from(*ptr1.add(16).cast::<u8>());
                                                    match l55 {
                                                        0 => None,
                                                        1 => {
                                                            let e = {
                                                                let l56 = *ptr1
                                                                    .add(16 + 1 * ::core::mem::size_of::<*const u8>())
                                                                    .cast::<*mut u8>();
                                                                let l57 = *ptr1
                                                                    .add(16 + 2 * ::core::mem::size_of::<*const u8>())
                                                                    .cast::<usize>();
                                                                let len58 = l57;
                                                                let bytes58 = _rt::Vec::from_raw_parts(
                                                                    l56.cast(),
                                                                    len58,
                                                                    len58,
                                                                );
                                                                _rt::string_lift(bytes58)
                                                            };
                                                            Some(e)
                                                        }
                                                        _ => _rt::invalid_enum_discriminant(),
                                                    }
                                                };
                                                V67::HttpResponseTransferCoding(e67)
                                            }
                                            32 => {
                                                let e67 = {
                                                    let l59 = i32::from(*ptr1.add(16).cast::<u8>());
                                                    match l59 {
                                                        0 => None,
                                                        1 => {
                                                            let e = {
                                                                let l60 = *ptr1
                                                                    .add(16 + 1 * ::core::mem::size_of::<*const u8>())
                                                                    .cast::<*mut u8>();
                                                                let l61 = *ptr1
                                                                    .add(16 + 2 * ::core::mem::size_of::<*const u8>())
                                                                    .cast::<usize>();
                                                                let len62 = l61;
                                                                let bytes62 = _rt::Vec::from_raw_parts(
                                                                    l60.cast(),
                                                                    len62,
                                                                    len62,
                                                                );
                                                                _rt::string_lift(bytes62)
                                                            };
                                                            Some(e)
                                                        }
                                                        _ => _rt::invalid_enum_discriminant(),
                                                    }
                                                };
                                                V67::HttpResponseContentCoding(e67)
                                            }
                                            33 => V67::HttpResponseTimeout,
                                            34 => V67::HttpUpgradeFailed,
                                            35 => V67::HttpProtocolError,
                                            36 => V67::LoopDetected,
                                            37 => V67::ConfigurationError,
                                            n => {
                                                if true {
                                                    match (&n, &38) {
                                                        (left_val, right_val) => {
                                                            if !(*left_val == *right_val) {
                                                                let kind = ::core::panicking::AssertKind::Eq;
                                                                ::core::panicking::assert_failed(
                                                                    kind,
                                                                    &*left_val,
                                                                    &*right_val,
                                                                    ::core::option::Option::Some(
                                                                        format_args!("invalid enum discriminant"),
                                                                    ),
                                                                );
                                                            }
                                                        }
                                                    };
                                                }
                                                let e67 = {
                                                    let l63 = i32::from(*ptr1.add(16).cast::<u8>());
                                                    match l63 {
                                                        0 => None,
                                                        1 => {
                                                            let e = {
                                                                let l64 = *ptr1
                                                                    .add(16 + 1 * ::core::mem::size_of::<*const u8>())
                                                                    .cast::<*mut u8>();
                                                                let l65 = *ptr1
                                                                    .add(16 + 2 * ::core::mem::size_of::<*const u8>())
                                                                    .cast::<usize>();
                                                                let len66 = l65;
                                                                let bytes66 = _rt::Vec::from_raw_parts(
                                                                    l64.cast(),
                                                                    len66,
                                                                    len66,
                                                                );
                                                                _rt::string_lift(bytes66)
                                                            };
                                                            Some(e)
                                                        }
                                                        _ => _rt::invalid_enum_discriminant(),
                                                    }
                                                };
                                                V67::InternalError(e67)
                                            }
                                        };
                                        v67
                                    };
                                    Err(e)
                                }
                                _ => _rt::invalid_enum_discriminant(),
                            };
                            result68
                        }
                    }
                }
            }
            pub mod io {
                /// A poll API intended to let users wait for I/O events on multiple handles
                /// at once.
                #[allow(dead_code, async_fn_in_trait, unused_imports, clippy::all)]
                pub mod poll {
                    #[used]
                    #[doc(hidden)]
                    static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
                    use super::super::super::_rt;
                    /// `pollable` represents a single I/O event which may be ready, or not.
                    #[repr(transparent)]
                    pub struct Pollable {
                        handle: _rt::Resource<Pollable>,
                    }
                    #[automatically_derived]
                    impl ::core::fmt::Debug for Pollable {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_struct_field1_finish(
                                f,
                                "Pollable",
                                "handle",
                                &&self.handle,
                            )
                        }
                    }
                    impl Pollable {
                        #[doc(hidden)]
                        pub unsafe fn from_handle(handle: u32) -> Self {
                            Self {
                                handle: unsafe { _rt::Resource::from_handle(handle) },
                            }
                        }
                        #[doc(hidden)]
                        pub fn take_handle(&self) -> u32 {
                            _rt::Resource::take_handle(&self.handle)
                        }
                        #[doc(hidden)]
                        pub fn handle(&self) -> u32 {
                            _rt::Resource::handle(&self.handle)
                        }
                    }
                    unsafe impl _rt::WasmResource for Pollable {
                        #[inline]
                        unsafe fn drop(_handle: u32) {
                            #[link(wasm_import_module = "wasi:io/poll@0.2.0")]
                            unsafe extern "C" {
                                #[link_name = "[resource-drop]pollable"]
                                fn drop(_: i32);
                            }
                            unsafe {
                                drop(_handle as i32);
                            }
                        }
                    }
                    impl Pollable {
                        #[allow(unused_unsafe, clippy::all)]
                        /// Return the readiness of a pollable. This function never blocks.
                        ///
                        /// Returns `true` when the pollable is ready, and `false` otherwise.
                        #[allow(async_fn_in_trait)]
                        pub fn ready(&self) -> bool {
                            unsafe {
                                #[link(wasm_import_module = "wasi:io/poll@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[method]pollable.ready"]
                                    fn wit_import0(_: i32) -> i32;
                                }
                                let ret = wit_import0((self).handle() as i32);
                                _rt::bool_lift(ret as u8)
                            }
                        }
                    }
                    impl Pollable {
                        #[allow(unused_unsafe, clippy::all)]
                        /// `block` returns immediately if the pollable is ready, and otherwise
                        /// blocks until ready.
                        ///
                        /// This function is equivalent to calling `poll.poll` on a list
                        /// containing only this pollable.
                        #[allow(async_fn_in_trait)]
                        pub fn block(&self) -> () {
                            unsafe {
                                #[link(wasm_import_module = "wasi:io/poll@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[method]pollable.block"]
                                    fn wit_import0(_: i32);
                                }
                                wit_import0((self).handle() as i32);
                            }
                        }
                    }
                    #[allow(unused_unsafe, clippy::all)]
                    /// Poll for completion on a set of pollables.
                    ///
                    /// This function takes a list of pollables, which identify I/O sources of
                    /// interest, and waits until one or more of the events is ready for I/O.
                    ///
                    /// The result `list<u32>` contains one or more indices of handles in the
                    /// argument list that is ready for I/O.
                    ///
                    /// If the list contains more elements than can be indexed with a `u32`
                    /// value, this function traps.
                    ///
                    /// A timeout can be implemented by adding a pollable from the
                    /// wasi-clocks API to the list.
                    ///
                    /// This function does not return a `result`; polling in itself does not
                    /// do any I/O so it doesn't fail. If any of the I/O sources identified by
                    /// the pollables has an error, it is indicated by marking the source as
                    /// being reaedy for I/O.
                    #[allow(async_fn_in_trait)]
                    pub fn poll(in_: &[&Pollable]) -> _rt::Vec<u32> {
                        unsafe {
                            #[repr(align(4))]
                            struct RetArea(
                                [::core::mem::MaybeUninit<
                                    u8,
                                >; 2 * ::core::mem::size_of::<*const u8>()],
                            );
                            let mut ret_area = RetArea(
                                [::core::mem::MaybeUninit::uninit(); 2
                                    * ::core::mem::size_of::<*const u8>()],
                            );
                            let vec0 = in_;
                            let len0 = vec0.len();
                            let layout0 = _rt::alloc::Layout::from_size_align(
                                    vec0.len() * 4,
                                    4,
                                )
                                .unwrap();
                            let (result0, _cleanup0) = ::spin_sdk::wit_bindgen::rt::Cleanup::new(
                                layout0,
                            );
                            for (i, e) in vec0.into_iter().enumerate() {
                                let base = result0.add(i * 4);
                                {
                                    *base.add(0).cast::<i32>() = (e).handle() as i32;
                                }
                            }
                            let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                            #[link(wasm_import_module = "wasi:io/poll@0.2.0")]
                            unsafe extern "C" {
                                #[link_name = "poll"]
                                fn wit_import2(_: *mut u8, _: usize, _: *mut u8);
                            }
                            wit_import2(result0, len0, ptr1);
                            let l3 = *ptr1.add(0).cast::<*mut u8>();
                            let l4 = *ptr1
                                .add(::core::mem::size_of::<*const u8>())
                                .cast::<usize>();
                            let len5 = l4;
                            let result6 = _rt::Vec::from_raw_parts(
                                l3.cast(),
                                len5,
                                len5,
                            );
                            result6
                        }
                    }
                }
                #[allow(dead_code, async_fn_in_trait, unused_imports, clippy::all)]
                pub mod error {
                    #[used]
                    #[doc(hidden)]
                    static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
                    use super::super::super::_rt;
                    /// A resource which represents some error information.
                    ///
                    /// The only method provided by this resource is `to-debug-string`,
                    /// which provides some human-readable information about the error.
                    ///
                    /// In the `wasi:io` package, this resource is returned through the
                    /// `wasi:io/streams/stream-error` type.
                    ///
                    /// To provide more specific error information, other interfaces may
                    /// provide functions to further "downcast" this error into more specific
                    /// error information. For example, `error`s returned in streams derived
                    /// from filesystem types to be described using the filesystem's own
                    /// error-code type, using the function
                    /// `wasi:filesystem/types/filesystem-error-code`, which takes a parameter
                    /// `borrow<error>` and returns
                    /// `option<wasi:filesystem/types/error-code>`.
                    ///
                    /// The set of functions which can "downcast" an `error` into a more
                    /// concrete type is open.
                    #[repr(transparent)]
                    pub struct Error {
                        handle: _rt::Resource<Error>,
                    }
                    #[automatically_derived]
                    impl ::core::fmt::Debug for Error {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_struct_field1_finish(
                                f,
                                "Error",
                                "handle",
                                &&self.handle,
                            )
                        }
                    }
                    impl Error {
                        #[doc(hidden)]
                        pub unsafe fn from_handle(handle: u32) -> Self {
                            Self {
                                handle: unsafe { _rt::Resource::from_handle(handle) },
                            }
                        }
                        #[doc(hidden)]
                        pub fn take_handle(&self) -> u32 {
                            _rt::Resource::take_handle(&self.handle)
                        }
                        #[doc(hidden)]
                        pub fn handle(&self) -> u32 {
                            _rt::Resource::handle(&self.handle)
                        }
                    }
                    unsafe impl _rt::WasmResource for Error {
                        #[inline]
                        unsafe fn drop(_handle: u32) {
                            #[link(wasm_import_module = "wasi:io/error@0.2.0")]
                            unsafe extern "C" {
                                #[link_name = "[resource-drop]error"]
                                fn drop(_: i32);
                            }
                            unsafe {
                                drop(_handle as i32);
                            }
                        }
                    }
                    impl Error {
                        #[allow(unused_unsafe, clippy::all)]
                        /// Returns a string that is suitable to assist humans in debugging
                        /// this error.
                        ///
                        /// WARNING: The returned string should not be consumed mechanically!
                        /// It may change across platforms, hosts, or other implementation
                        /// details. Parsing this string is a major platform-compatibility
                        /// hazard.
                        #[allow(async_fn_in_trait)]
                        pub fn to_debug_string(&self) -> _rt::String {
                            unsafe {
                                #[repr(align(4))]
                                struct RetArea(
                                    [::core::mem::MaybeUninit<
                                        u8,
                                    >; 2 * ::core::mem::size_of::<*const u8>()],
                                );
                                let mut ret_area = RetArea(
                                    [::core::mem::MaybeUninit::uninit(); 2
                                        * ::core::mem::size_of::<*const u8>()],
                                );
                                let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                                #[link(wasm_import_module = "wasi:io/error@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[method]error.to-debug-string"]
                                    fn wit_import1(_: i32, _: *mut u8);
                                }
                                wit_import1((self).handle() as i32, ptr0);
                                let l2 = *ptr0.add(0).cast::<*mut u8>();
                                let l3 = *ptr0
                                    .add(::core::mem::size_of::<*const u8>())
                                    .cast::<usize>();
                                let len4 = l3;
                                let bytes4 = _rt::Vec::from_raw_parts(
                                    l2.cast(),
                                    len4,
                                    len4,
                                );
                                let result5 = _rt::string_lift(bytes4);
                                result5
                            }
                        }
                    }
                }
                /// WASI I/O is an I/O abstraction API which is currently focused on providing
                /// stream types.
                ///
                /// In the future, the component model is expected to add built-in stream types;
                /// when it does, they are expected to subsume this API.
                #[allow(dead_code, async_fn_in_trait, unused_imports, clippy::all)]
                pub mod streams {
                    #[used]
                    #[doc(hidden)]
                    static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
                    use super::super::super::_rt;
                    pub type Error = super::super::super::wasi::io::error::Error;
                    pub type Pollable = super::super::super::wasi::io::poll::Pollable;
                    /// An error for input-stream and output-stream operations.
                    pub enum StreamError {
                        /// The last operation (a write or flush) failed before completion.
                        ///
                        /// More information is available in the `error` payload.
                        LastOperationFailed(Error),
                        /// The stream is closed: no more input will be accepted by the
                        /// stream. A closed output-stream will return this error on all
                        /// future operations.
                        Closed,
                    }
                    impl ::core::fmt::Debug for StreamError {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter<'_>,
                        ) -> ::core::fmt::Result {
                            match self {
                                StreamError::LastOperationFailed(e) => {
                                    f.debug_tuple("StreamError::LastOperationFailed")
                                        .field(e)
                                        .finish()
                                }
                                StreamError::Closed => {
                                    f.debug_tuple("StreamError::Closed").finish()
                                }
                            }
                        }
                    }
                    impl ::core::fmt::Display for StreamError {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter<'_>,
                        ) -> ::core::fmt::Result {
                            f.write_fmt(format_args!("{0:?}", self))
                        }
                    }
                    impl std::error::Error for StreamError {}
                    /// An input bytestream.
                    ///
                    /// `input-stream`s are *non-blocking* to the extent practical on underlying
                    /// platforms. I/O operations always return promptly; if fewer bytes are
                    /// promptly available than requested, they return the number of bytes promptly
                    /// available, which could even be zero. To wait for data to be available,
                    /// use the `subscribe` function to obtain a `pollable` which can be polled
                    /// for using `wasi:io/poll`.
                    #[repr(transparent)]
                    pub struct InputStream {
                        handle: _rt::Resource<InputStream>,
                    }
                    #[automatically_derived]
                    impl ::core::fmt::Debug for InputStream {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_struct_field1_finish(
                                f,
                                "InputStream",
                                "handle",
                                &&self.handle,
                            )
                        }
                    }
                    impl InputStream {
                        #[doc(hidden)]
                        pub unsafe fn from_handle(handle: u32) -> Self {
                            Self {
                                handle: unsafe { _rt::Resource::from_handle(handle) },
                            }
                        }
                        #[doc(hidden)]
                        pub fn take_handle(&self) -> u32 {
                            _rt::Resource::take_handle(&self.handle)
                        }
                        #[doc(hidden)]
                        pub fn handle(&self) -> u32 {
                            _rt::Resource::handle(&self.handle)
                        }
                    }
                    unsafe impl _rt::WasmResource for InputStream {
                        #[inline]
                        unsafe fn drop(_handle: u32) {
                            #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                            unsafe extern "C" {
                                #[link_name = "[resource-drop]input-stream"]
                                fn drop(_: i32);
                            }
                            unsafe {
                                drop(_handle as i32);
                            }
                        }
                    }
                    /// An output bytestream.
                    ///
                    /// `output-stream`s are *non-blocking* to the extent practical on
                    /// underlying platforms. Except where specified otherwise, I/O operations also
                    /// always return promptly, after the number of bytes that can be written
                    /// promptly, which could even be zero. To wait for the stream to be ready to
                    /// accept data, the `subscribe` function to obtain a `pollable` which can be
                    /// polled for using `wasi:io/poll`.
                    #[repr(transparent)]
                    pub struct OutputStream {
                        handle: _rt::Resource<OutputStream>,
                    }
                    #[automatically_derived]
                    impl ::core::fmt::Debug for OutputStream {
                        #[inline]
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::debug_struct_field1_finish(
                                f,
                                "OutputStream",
                                "handle",
                                &&self.handle,
                            )
                        }
                    }
                    impl OutputStream {
                        #[doc(hidden)]
                        pub unsafe fn from_handle(handle: u32) -> Self {
                            Self {
                                handle: unsafe { _rt::Resource::from_handle(handle) },
                            }
                        }
                        #[doc(hidden)]
                        pub fn take_handle(&self) -> u32 {
                            _rt::Resource::take_handle(&self.handle)
                        }
                        #[doc(hidden)]
                        pub fn handle(&self) -> u32 {
                            _rt::Resource::handle(&self.handle)
                        }
                    }
                    unsafe impl _rt::WasmResource for OutputStream {
                        #[inline]
                        unsafe fn drop(_handle: u32) {
                            #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                            unsafe extern "C" {
                                #[link_name = "[resource-drop]output-stream"]
                                fn drop(_: i32);
                            }
                            unsafe {
                                drop(_handle as i32);
                            }
                        }
                    }
                    impl InputStream {
                        #[allow(unused_unsafe, clippy::all)]
                        /// Perform a non-blocking read from the stream.
                        ///
                        /// When the source of a `read` is binary data, the bytes from the source
                        /// are returned verbatim. When the source of a `read` is known to the
                        /// implementation to be text, bytes containing the UTF-8 encoding of the
                        /// text are returned.
                        ///
                        /// This function returns a list of bytes containing the read data,
                        /// when successful. The returned list will contain up to `len` bytes;
                        /// it may return fewer than requested, but not more. The list is
                        /// empty when no bytes are available for reading at this time. The
                        /// pollable given by `subscribe` will be ready when more bytes are
                        /// available.
                        ///
                        /// This function fails with a `stream-error` when the operation
                        /// encounters an error, giving `last-operation-failed`, or when the
                        /// stream is closed, giving `closed`.
                        ///
                        /// When the caller gives a `len` of 0, it represents a request to
                        /// read 0 bytes. If the stream is still open, this call should
                        /// succeed and return an empty list, or otherwise fail with `closed`.
                        ///
                        /// The `len` parameter is a `u64`, which could represent a list of u8 which
                        /// is not possible to allocate in wasm32, or not desirable to allocate as
                        /// as a return value by the callee. The callee may return a list of bytes
                        /// less than `len` in size while more bytes are available for reading.
                        #[allow(async_fn_in_trait)]
                        pub fn read(
                            &self,
                            len: u64,
                        ) -> Result<_rt::Vec<u8>, StreamError> {
                            unsafe {
                                #[repr(align(4))]
                                struct RetArea(
                                    [::core::mem::MaybeUninit<
                                        u8,
                                    >; 3 * ::core::mem::size_of::<*const u8>()],
                                );
                                let mut ret_area = RetArea(
                                    [::core::mem::MaybeUninit::uninit(); 3
                                        * ::core::mem::size_of::<*const u8>()],
                                );
                                let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                                #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[method]input-stream.read"]
                                    fn wit_import1(_: i32, _: i64, _: *mut u8);
                                }
                                wit_import1(
                                    (self).handle() as i32,
                                    _rt::as_i64(&len),
                                    ptr0,
                                );
                                let l2 = i32::from(*ptr0.add(0).cast::<u8>());
                                let result9 = match l2 {
                                    0 => {
                                        let e = {
                                            let l3 = *ptr0
                                                .add(::core::mem::size_of::<*const u8>())
                                                .cast::<*mut u8>();
                                            let l4 = *ptr0
                                                .add(2 * ::core::mem::size_of::<*const u8>())
                                                .cast::<usize>();
                                            let len5 = l4;
                                            _rt::Vec::from_raw_parts(l3.cast(), len5, len5)
                                        };
                                        Ok(e)
                                    }
                                    1 => {
                                        let e = {
                                            let l6 = i32::from(
                                                *ptr0.add(::core::mem::size_of::<*const u8>()).cast::<u8>(),
                                            );
                                            let v8 = match l6 {
                                                0 => {
                                                    let e8 = {
                                                        let l7 = *ptr0
                                                            .add(4 + 1 * ::core::mem::size_of::<*const u8>())
                                                            .cast::<i32>();
                                                        super::super::super::wasi::io::error::Error::from_handle(
                                                            l7 as u32,
                                                        )
                                                    };
                                                    StreamError::LastOperationFailed(e8)
                                                }
                                                n => {
                                                    if true {
                                                        match (&n, &1) {
                                                            (left_val, right_val) => {
                                                                if !(*left_val == *right_val) {
                                                                    let kind = ::core::panicking::AssertKind::Eq;
                                                                    ::core::panicking::assert_failed(
                                                                        kind,
                                                                        &*left_val,
                                                                        &*right_val,
                                                                        ::core::option::Option::Some(
                                                                            format_args!("invalid enum discriminant"),
                                                                        ),
                                                                    );
                                                                }
                                                            }
                                                        };
                                                    }
                                                    StreamError::Closed
                                                }
                                            };
                                            v8
                                        };
                                        Err(e)
                                    }
                                    _ => _rt::invalid_enum_discriminant(),
                                };
                                result9
                            }
                        }
                    }
                    impl InputStream {
                        #[allow(unused_unsafe, clippy::all)]
                        /// Read bytes from a stream, after blocking until at least one byte can
                        /// be read. Except for blocking, behavior is identical to `read`.
                        #[allow(async_fn_in_trait)]
                        pub fn blocking_read(
                            &self,
                            len: u64,
                        ) -> Result<_rt::Vec<u8>, StreamError> {
                            unsafe {
                                #[repr(align(4))]
                                struct RetArea(
                                    [::core::mem::MaybeUninit<
                                        u8,
                                    >; 3 * ::core::mem::size_of::<*const u8>()],
                                );
                                let mut ret_area = RetArea(
                                    [::core::mem::MaybeUninit::uninit(); 3
                                        * ::core::mem::size_of::<*const u8>()],
                                );
                                let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                                #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[method]input-stream.blocking-read"]
                                    fn wit_import1(_: i32, _: i64, _: *mut u8);
                                }
                                wit_import1(
                                    (self).handle() as i32,
                                    _rt::as_i64(&len),
                                    ptr0,
                                );
                                let l2 = i32::from(*ptr0.add(0).cast::<u8>());
                                let result9 = match l2 {
                                    0 => {
                                        let e = {
                                            let l3 = *ptr0
                                                .add(::core::mem::size_of::<*const u8>())
                                                .cast::<*mut u8>();
                                            let l4 = *ptr0
                                                .add(2 * ::core::mem::size_of::<*const u8>())
                                                .cast::<usize>();
                                            let len5 = l4;
                                            _rt::Vec::from_raw_parts(l3.cast(), len5, len5)
                                        };
                                        Ok(e)
                                    }
                                    1 => {
                                        let e = {
                                            let l6 = i32::from(
                                                *ptr0.add(::core::mem::size_of::<*const u8>()).cast::<u8>(),
                                            );
                                            let v8 = match l6 {
                                                0 => {
                                                    let e8 = {
                                                        let l7 = *ptr0
                                                            .add(4 + 1 * ::core::mem::size_of::<*const u8>())
                                                            .cast::<i32>();
                                                        super::super::super::wasi::io::error::Error::from_handle(
                                                            l7 as u32,
                                                        )
                                                    };
                                                    StreamError::LastOperationFailed(e8)
                                                }
                                                n => {
                                                    if true {
                                                        match (&n, &1) {
                                                            (left_val, right_val) => {
                                                                if !(*left_val == *right_val) {
                                                                    let kind = ::core::panicking::AssertKind::Eq;
                                                                    ::core::panicking::assert_failed(
                                                                        kind,
                                                                        &*left_val,
                                                                        &*right_val,
                                                                        ::core::option::Option::Some(
                                                                            format_args!("invalid enum discriminant"),
                                                                        ),
                                                                    );
                                                                }
                                                            }
                                                        };
                                                    }
                                                    StreamError::Closed
                                                }
                                            };
                                            v8
                                        };
                                        Err(e)
                                    }
                                    _ => _rt::invalid_enum_discriminant(),
                                };
                                result9
                            }
                        }
                    }
                    impl InputStream {
                        #[allow(unused_unsafe, clippy::all)]
                        /// Skip bytes from a stream. Returns number of bytes skipped.
                        ///
                        /// Behaves identical to `read`, except instead of returning a list
                        /// of bytes, returns the number of bytes consumed from the stream.
                        #[allow(async_fn_in_trait)]
                        pub fn skip(&self, len: u64) -> Result<u64, StreamError> {
                            unsafe {
                                #[repr(align(8))]
                                struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                                let mut ret_area = RetArea(
                                    [::core::mem::MaybeUninit::uninit(); 16],
                                );
                                let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                                #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[method]input-stream.skip"]
                                    fn wit_import1(_: i32, _: i64, _: *mut u8);
                                }
                                wit_import1(
                                    (self).handle() as i32,
                                    _rt::as_i64(&len),
                                    ptr0,
                                );
                                let l2 = i32::from(*ptr0.add(0).cast::<u8>());
                                let result7 = match l2 {
                                    0 => {
                                        let e = {
                                            let l3 = *ptr0.add(8).cast::<i64>();
                                            l3 as u64
                                        };
                                        Ok(e)
                                    }
                                    1 => {
                                        let e = {
                                            let l4 = i32::from(*ptr0.add(8).cast::<u8>());
                                            let v6 = match l4 {
                                                0 => {
                                                    let e6 = {
                                                        let l5 = *ptr0.add(12).cast::<i32>();
                                                        super::super::super::wasi::io::error::Error::from_handle(
                                                            l5 as u32,
                                                        )
                                                    };
                                                    StreamError::LastOperationFailed(e6)
                                                }
                                                n => {
                                                    if true {
                                                        match (&n, &1) {
                                                            (left_val, right_val) => {
                                                                if !(*left_val == *right_val) {
                                                                    let kind = ::core::panicking::AssertKind::Eq;
                                                                    ::core::panicking::assert_failed(
                                                                        kind,
                                                                        &*left_val,
                                                                        &*right_val,
                                                                        ::core::option::Option::Some(
                                                                            format_args!("invalid enum discriminant"),
                                                                        ),
                                                                    );
                                                                }
                                                            }
                                                        };
                                                    }
                                                    StreamError::Closed
                                                }
                                            };
                                            v6
                                        };
                                        Err(e)
                                    }
                                    _ => _rt::invalid_enum_discriminant(),
                                };
                                result7
                            }
                        }
                    }
                    impl InputStream {
                        #[allow(unused_unsafe, clippy::all)]
                        /// Skip bytes from a stream, after blocking until at least one byte
                        /// can be skipped. Except for blocking behavior, identical to `skip`.
                        #[allow(async_fn_in_trait)]
                        pub fn blocking_skip(
                            &self,
                            len: u64,
                        ) -> Result<u64, StreamError> {
                            unsafe {
                                #[repr(align(8))]
                                struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                                let mut ret_area = RetArea(
                                    [::core::mem::MaybeUninit::uninit(); 16],
                                );
                                let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                                #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[method]input-stream.blocking-skip"]
                                    fn wit_import1(_: i32, _: i64, _: *mut u8);
                                }
                                wit_import1(
                                    (self).handle() as i32,
                                    _rt::as_i64(&len),
                                    ptr0,
                                );
                                let l2 = i32::from(*ptr0.add(0).cast::<u8>());
                                let result7 = match l2 {
                                    0 => {
                                        let e = {
                                            let l3 = *ptr0.add(8).cast::<i64>();
                                            l3 as u64
                                        };
                                        Ok(e)
                                    }
                                    1 => {
                                        let e = {
                                            let l4 = i32::from(*ptr0.add(8).cast::<u8>());
                                            let v6 = match l4 {
                                                0 => {
                                                    let e6 = {
                                                        let l5 = *ptr0.add(12).cast::<i32>();
                                                        super::super::super::wasi::io::error::Error::from_handle(
                                                            l5 as u32,
                                                        )
                                                    };
                                                    StreamError::LastOperationFailed(e6)
                                                }
                                                n => {
                                                    if true {
                                                        match (&n, &1) {
                                                            (left_val, right_val) => {
                                                                if !(*left_val == *right_val) {
                                                                    let kind = ::core::panicking::AssertKind::Eq;
                                                                    ::core::panicking::assert_failed(
                                                                        kind,
                                                                        &*left_val,
                                                                        &*right_val,
                                                                        ::core::option::Option::Some(
                                                                            format_args!("invalid enum discriminant"),
                                                                        ),
                                                                    );
                                                                }
                                                            }
                                                        };
                                                    }
                                                    StreamError::Closed
                                                }
                                            };
                                            v6
                                        };
                                        Err(e)
                                    }
                                    _ => _rt::invalid_enum_discriminant(),
                                };
                                result7
                            }
                        }
                    }
                    impl InputStream {
                        #[allow(unused_unsafe, clippy::all)]
                        /// Create a `pollable` which will resolve once either the specified stream
                        /// has bytes available to read or the other end of the stream has been
                        /// closed.
                        /// The created `pollable` is a child resource of the `input-stream`.
                        /// Implementations may trap if the `input-stream` is dropped before
                        /// all derived `pollable`s created with this function are dropped.
                        #[allow(async_fn_in_trait)]
                        pub fn subscribe(&self) -> Pollable {
                            unsafe {
                                #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[method]input-stream.subscribe"]
                                    fn wit_import0(_: i32) -> i32;
                                }
                                let ret = wit_import0((self).handle() as i32);
                                super::super::super::wasi::io::poll::Pollable::from_handle(
                                    ret as u32,
                                )
                            }
                        }
                    }
                    impl OutputStream {
                        #[allow(unused_unsafe, clippy::all)]
                        /// Check readiness for writing. This function never blocks.
                        ///
                        /// Returns the number of bytes permitted for the next call to `write`,
                        /// or an error. Calling `write` with more bytes than this function has
                        /// permitted will trap.
                        ///
                        /// When this function returns 0 bytes, the `subscribe` pollable will
                        /// become ready when this function will report at least 1 byte, or an
                        /// error.
                        #[allow(async_fn_in_trait)]
                        pub fn check_write(&self) -> Result<u64, StreamError> {
                            unsafe {
                                #[repr(align(8))]
                                struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                                let mut ret_area = RetArea(
                                    [::core::mem::MaybeUninit::uninit(); 16],
                                );
                                let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                                #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[method]output-stream.check-write"]
                                    fn wit_import1(_: i32, _: *mut u8);
                                }
                                wit_import1((self).handle() as i32, ptr0);
                                let l2 = i32::from(*ptr0.add(0).cast::<u8>());
                                let result7 = match l2 {
                                    0 => {
                                        let e = {
                                            let l3 = *ptr0.add(8).cast::<i64>();
                                            l3 as u64
                                        };
                                        Ok(e)
                                    }
                                    1 => {
                                        let e = {
                                            let l4 = i32::from(*ptr0.add(8).cast::<u8>());
                                            let v6 = match l4 {
                                                0 => {
                                                    let e6 = {
                                                        let l5 = *ptr0.add(12).cast::<i32>();
                                                        super::super::super::wasi::io::error::Error::from_handle(
                                                            l5 as u32,
                                                        )
                                                    };
                                                    StreamError::LastOperationFailed(e6)
                                                }
                                                n => {
                                                    if true {
                                                        match (&n, &1) {
                                                            (left_val, right_val) => {
                                                                if !(*left_val == *right_val) {
                                                                    let kind = ::core::panicking::AssertKind::Eq;
                                                                    ::core::panicking::assert_failed(
                                                                        kind,
                                                                        &*left_val,
                                                                        &*right_val,
                                                                        ::core::option::Option::Some(
                                                                            format_args!("invalid enum discriminant"),
                                                                        ),
                                                                    );
                                                                }
                                                            }
                                                        };
                                                    }
                                                    StreamError::Closed
                                                }
                                            };
                                            v6
                                        };
                                        Err(e)
                                    }
                                    _ => _rt::invalid_enum_discriminant(),
                                };
                                result7
                            }
                        }
                    }
                    impl OutputStream {
                        #[allow(unused_unsafe, clippy::all)]
                        /// Perform a write. This function never blocks.
                        ///
                        /// When the destination of a `write` is binary data, the bytes from
                        /// `contents` are written verbatim. When the destination of a `write` is
                        /// known to the implementation to be text, the bytes of `contents` are
                        /// transcoded from UTF-8 into the encoding of the destination and then
                        /// written.
                        ///
                        /// Precondition: check-write gave permit of Ok(n) and contents has a
                        /// length of less than or equal to n. Otherwise, this function will trap.
                        ///
                        /// returns Err(closed) without writing if the stream has closed since
                        /// the last call to check-write provided a permit.
                        #[allow(async_fn_in_trait)]
                        pub fn write(&self, contents: &[u8]) -> Result<(), StreamError> {
                            unsafe {
                                #[repr(align(4))]
                                struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                                let mut ret_area = RetArea(
                                    [::core::mem::MaybeUninit::uninit(); 12],
                                );
                                let vec0 = contents;
                                let ptr0 = vec0.as_ptr().cast::<u8>();
                                let len0 = vec0.len();
                                let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                                #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[method]output-stream.write"]
                                    fn wit_import2(_: i32, _: *mut u8, _: usize, _: *mut u8);
                                }
                                wit_import2(
                                    (self).handle() as i32,
                                    ptr0.cast_mut(),
                                    len0,
                                    ptr1,
                                );
                                let l3 = i32::from(*ptr1.add(0).cast::<u8>());
                                let result7 = match l3 {
                                    0 => {
                                        let e = ();
                                        Ok(e)
                                    }
                                    1 => {
                                        let e = {
                                            let l4 = i32::from(*ptr1.add(4).cast::<u8>());
                                            let v6 = match l4 {
                                                0 => {
                                                    let e6 = {
                                                        let l5 = *ptr1.add(8).cast::<i32>();
                                                        super::super::super::wasi::io::error::Error::from_handle(
                                                            l5 as u32,
                                                        )
                                                    };
                                                    StreamError::LastOperationFailed(e6)
                                                }
                                                n => {
                                                    if true {
                                                        match (&n, &1) {
                                                            (left_val, right_val) => {
                                                                if !(*left_val == *right_val) {
                                                                    let kind = ::core::panicking::AssertKind::Eq;
                                                                    ::core::panicking::assert_failed(
                                                                        kind,
                                                                        &*left_val,
                                                                        &*right_val,
                                                                        ::core::option::Option::Some(
                                                                            format_args!("invalid enum discriminant"),
                                                                        ),
                                                                    );
                                                                }
                                                            }
                                                        };
                                                    }
                                                    StreamError::Closed
                                                }
                                            };
                                            v6
                                        };
                                        Err(e)
                                    }
                                    _ => _rt::invalid_enum_discriminant(),
                                };
                                result7
                            }
                        }
                    }
                    impl OutputStream {
                        #[allow(unused_unsafe, clippy::all)]
                        /// Perform a write of up to 4096 bytes, and then flush the stream. Block
                        /// until all of these operations are complete, or an error occurs.
                        ///
                        /// This is a convenience wrapper around the use of `check-write`,
                        /// `subscribe`, `write`, and `flush`, and is implemented with the
                        /// following pseudo-code:
                        ///
                        /// ```text
                        /// let pollable = this.subscribe();
                        /// while !contents.is_empty() {
                        ///     // Wait for the stream to become writable
                        ///     pollable.block();
                        ///     let Ok(n) = this.check-write(); // eliding error handling
                        ///     let len = min(n, contents.len());
                        ///     let (chunk, rest) = contents.split_at(len);
                        ///     this.write(chunk  );            // eliding error handling
                        ///     contents = rest;
                        /// }
                        /// this.flush();
                        /// // Wait for completion of `flush`
                        /// pollable.block();
                        /// // Check for any errors that arose during `flush`
                        /// let _ = this.check-write();         // eliding error handling
                        /// ```
                        #[allow(async_fn_in_trait)]
                        pub fn blocking_write_and_flush(
                            &self,
                            contents: &[u8],
                        ) -> Result<(), StreamError> {
                            unsafe {
                                #[repr(align(4))]
                                struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                                let mut ret_area = RetArea(
                                    [::core::mem::MaybeUninit::uninit(); 12],
                                );
                                let vec0 = contents;
                                let ptr0 = vec0.as_ptr().cast::<u8>();
                                let len0 = vec0.len();
                                let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                                #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[method]output-stream.blocking-write-and-flush"]
                                    fn wit_import2(_: i32, _: *mut u8, _: usize, _: *mut u8);
                                }
                                wit_import2(
                                    (self).handle() as i32,
                                    ptr0.cast_mut(),
                                    len0,
                                    ptr1,
                                );
                                let l3 = i32::from(*ptr1.add(0).cast::<u8>());
                                let result7 = match l3 {
                                    0 => {
                                        let e = ();
                                        Ok(e)
                                    }
                                    1 => {
                                        let e = {
                                            let l4 = i32::from(*ptr1.add(4).cast::<u8>());
                                            let v6 = match l4 {
                                                0 => {
                                                    let e6 = {
                                                        let l5 = *ptr1.add(8).cast::<i32>();
                                                        super::super::super::wasi::io::error::Error::from_handle(
                                                            l5 as u32,
                                                        )
                                                    };
                                                    StreamError::LastOperationFailed(e6)
                                                }
                                                n => {
                                                    if true {
                                                        match (&n, &1) {
                                                            (left_val, right_val) => {
                                                                if !(*left_val == *right_val) {
                                                                    let kind = ::core::panicking::AssertKind::Eq;
                                                                    ::core::panicking::assert_failed(
                                                                        kind,
                                                                        &*left_val,
                                                                        &*right_val,
                                                                        ::core::option::Option::Some(
                                                                            format_args!("invalid enum discriminant"),
                                                                        ),
                                                                    );
                                                                }
                                                            }
                                                        };
                                                    }
                                                    StreamError::Closed
                                                }
                                            };
                                            v6
                                        };
                                        Err(e)
                                    }
                                    _ => _rt::invalid_enum_discriminant(),
                                };
                                result7
                            }
                        }
                    }
                    impl OutputStream {
                        #[allow(unused_unsafe, clippy::all)]
                        /// Request to flush buffered output. This function never blocks.
                        ///
                        /// This tells the output-stream that the caller intends any buffered
                        /// output to be flushed. the output which is expected to be flushed
                        /// is all that has been passed to `write` prior to this call.
                        ///
                        /// Upon calling this function, the `output-stream` will not accept any
                        /// writes (`check-write` will return `ok(0)`) until the flush has
                        /// completed. The `subscribe` pollable will become ready when the
                        /// flush has completed and the stream can accept more writes.
                        #[allow(async_fn_in_trait)]
                        pub fn flush(&self) -> Result<(), StreamError> {
                            unsafe {
                                #[repr(align(4))]
                                struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                                let mut ret_area = RetArea(
                                    [::core::mem::MaybeUninit::uninit(); 12],
                                );
                                let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                                #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[method]output-stream.flush"]
                                    fn wit_import1(_: i32, _: *mut u8);
                                }
                                wit_import1((self).handle() as i32, ptr0);
                                let l2 = i32::from(*ptr0.add(0).cast::<u8>());
                                let result6 = match l2 {
                                    0 => {
                                        let e = ();
                                        Ok(e)
                                    }
                                    1 => {
                                        let e = {
                                            let l3 = i32::from(*ptr0.add(4).cast::<u8>());
                                            let v5 = match l3 {
                                                0 => {
                                                    let e5 = {
                                                        let l4 = *ptr0.add(8).cast::<i32>();
                                                        super::super::super::wasi::io::error::Error::from_handle(
                                                            l4 as u32,
                                                        )
                                                    };
                                                    StreamError::LastOperationFailed(e5)
                                                }
                                                n => {
                                                    if true {
                                                        match (&n, &1) {
                                                            (left_val, right_val) => {
                                                                if !(*left_val == *right_val) {
                                                                    let kind = ::core::panicking::AssertKind::Eq;
                                                                    ::core::panicking::assert_failed(
                                                                        kind,
                                                                        &*left_val,
                                                                        &*right_val,
                                                                        ::core::option::Option::Some(
                                                                            format_args!("invalid enum discriminant"),
                                                                        ),
                                                                    );
                                                                }
                                                            }
                                                        };
                                                    }
                                                    StreamError::Closed
                                                }
                                            };
                                            v5
                                        };
                                        Err(e)
                                    }
                                    _ => _rt::invalid_enum_discriminant(),
                                };
                                result6
                            }
                        }
                    }
                    impl OutputStream {
                        #[allow(unused_unsafe, clippy::all)]
                        /// Request to flush buffered output, and block until flush completes
                        /// and stream is ready for writing again.
                        #[allow(async_fn_in_trait)]
                        pub fn blocking_flush(&self) -> Result<(), StreamError> {
                            unsafe {
                                #[repr(align(4))]
                                struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                                let mut ret_area = RetArea(
                                    [::core::mem::MaybeUninit::uninit(); 12],
                                );
                                let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                                #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[method]output-stream.blocking-flush"]
                                    fn wit_import1(_: i32, _: *mut u8);
                                }
                                wit_import1((self).handle() as i32, ptr0);
                                let l2 = i32::from(*ptr0.add(0).cast::<u8>());
                                let result6 = match l2 {
                                    0 => {
                                        let e = ();
                                        Ok(e)
                                    }
                                    1 => {
                                        let e = {
                                            let l3 = i32::from(*ptr0.add(4).cast::<u8>());
                                            let v5 = match l3 {
                                                0 => {
                                                    let e5 = {
                                                        let l4 = *ptr0.add(8).cast::<i32>();
                                                        super::super::super::wasi::io::error::Error::from_handle(
                                                            l4 as u32,
                                                        )
                                                    };
                                                    StreamError::LastOperationFailed(e5)
                                                }
                                                n => {
                                                    if true {
                                                        match (&n, &1) {
                                                            (left_val, right_val) => {
                                                                if !(*left_val == *right_val) {
                                                                    let kind = ::core::panicking::AssertKind::Eq;
                                                                    ::core::panicking::assert_failed(
                                                                        kind,
                                                                        &*left_val,
                                                                        &*right_val,
                                                                        ::core::option::Option::Some(
                                                                            format_args!("invalid enum discriminant"),
                                                                        ),
                                                                    );
                                                                }
                                                            }
                                                        };
                                                    }
                                                    StreamError::Closed
                                                }
                                            };
                                            v5
                                        };
                                        Err(e)
                                    }
                                    _ => _rt::invalid_enum_discriminant(),
                                };
                                result6
                            }
                        }
                    }
                    impl OutputStream {
                        #[allow(unused_unsafe, clippy::all)]
                        /// Create a `pollable` which will resolve once the output-stream
                        /// is ready for more writing, or an error has occured. When this
                        /// pollable is ready, `check-write` will return `ok(n)` with n>0, or an
                        /// error.
                        ///
                        /// If the stream is closed, this pollable is always ready immediately.
                        ///
                        /// The created `pollable` is a child resource of the `output-stream`.
                        /// Implementations may trap if the `output-stream` is dropped before
                        /// all derived `pollable`s created with this function are dropped.
                        #[allow(async_fn_in_trait)]
                        pub fn subscribe(&self) -> Pollable {
                            unsafe {
                                #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[method]output-stream.subscribe"]
                                    fn wit_import0(_: i32) -> i32;
                                }
                                let ret = wit_import0((self).handle() as i32);
                                super::super::super::wasi::io::poll::Pollable::from_handle(
                                    ret as u32,
                                )
                            }
                        }
                    }
                    impl OutputStream {
                        #[allow(unused_unsafe, clippy::all)]
                        /// Write zeroes to a stream.
                        ///
                        /// This should be used precisely like `write` with the exact same
                        /// preconditions (must use check-write first), but instead of
                        /// passing a list of bytes, you simply pass the number of zero-bytes
                        /// that should be written.
                        #[allow(async_fn_in_trait)]
                        pub fn write_zeroes(&self, len: u64) -> Result<(), StreamError> {
                            unsafe {
                                #[repr(align(4))]
                                struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                                let mut ret_area = RetArea(
                                    [::core::mem::MaybeUninit::uninit(); 12],
                                );
                                let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                                #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[method]output-stream.write-zeroes"]
                                    fn wit_import1(_: i32, _: i64, _: *mut u8);
                                }
                                wit_import1(
                                    (self).handle() as i32,
                                    _rt::as_i64(&len),
                                    ptr0,
                                );
                                let l2 = i32::from(*ptr0.add(0).cast::<u8>());
                                let result6 = match l2 {
                                    0 => {
                                        let e = ();
                                        Ok(e)
                                    }
                                    1 => {
                                        let e = {
                                            let l3 = i32::from(*ptr0.add(4).cast::<u8>());
                                            let v5 = match l3 {
                                                0 => {
                                                    let e5 = {
                                                        let l4 = *ptr0.add(8).cast::<i32>();
                                                        super::super::super::wasi::io::error::Error::from_handle(
                                                            l4 as u32,
                                                        )
                                                    };
                                                    StreamError::LastOperationFailed(e5)
                                                }
                                                n => {
                                                    if true {
                                                        match (&n, &1) {
                                                            (left_val, right_val) => {
                                                                if !(*left_val == *right_val) {
                                                                    let kind = ::core::panicking::AssertKind::Eq;
                                                                    ::core::panicking::assert_failed(
                                                                        kind,
                                                                        &*left_val,
                                                                        &*right_val,
                                                                        ::core::option::Option::Some(
                                                                            format_args!("invalid enum discriminant"),
                                                                        ),
                                                                    );
                                                                }
                                                            }
                                                        };
                                                    }
                                                    StreamError::Closed
                                                }
                                            };
                                            v5
                                        };
                                        Err(e)
                                    }
                                    _ => _rt::invalid_enum_discriminant(),
                                };
                                result6
                            }
                        }
                    }
                    impl OutputStream {
                        #[allow(unused_unsafe, clippy::all)]
                        /// Perform a write of up to 4096 zeroes, and then flush the stream.
                        /// Block until all of these operations are complete, or an error
                        /// occurs.
                        ///
                        /// This is a convenience wrapper around the use of `check-write`,
                        /// `subscribe`, `write-zeroes`, and `flush`, and is implemented with
                        /// the following pseudo-code:
                        ///
                        /// ```text
                        /// let pollable = this.subscribe();
                        /// while num_zeroes != 0 {
                        ///     // Wait for the stream to become writable
                        ///     pollable.block();
                        ///     let Ok(n) = this.check-write(); // eliding error handling
                        ///     let len = min(n, num_zeroes);
                        ///     this.write-zeroes(len);         // eliding error handling
                        ///     num_zeroes -= len;
                        /// }
                        /// this.flush();
                        /// // Wait for completion of `flush`
                        /// pollable.block();
                        /// // Check for any errors that arose during `flush`
                        /// let _ = this.check-write();         // eliding error handling
                        /// ```
                        #[allow(async_fn_in_trait)]
                        pub fn blocking_write_zeroes_and_flush(
                            &self,
                            len: u64,
                        ) -> Result<(), StreamError> {
                            unsafe {
                                #[repr(align(4))]
                                struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                                let mut ret_area = RetArea(
                                    [::core::mem::MaybeUninit::uninit(); 12],
                                );
                                let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                                #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[method]output-stream.blocking-write-zeroes-and-flush"]
                                    fn wit_import1(_: i32, _: i64, _: *mut u8);
                                }
                                wit_import1(
                                    (self).handle() as i32,
                                    _rt::as_i64(&len),
                                    ptr0,
                                );
                                let l2 = i32::from(*ptr0.add(0).cast::<u8>());
                                let result6 = match l2 {
                                    0 => {
                                        let e = ();
                                        Ok(e)
                                    }
                                    1 => {
                                        let e = {
                                            let l3 = i32::from(*ptr0.add(4).cast::<u8>());
                                            let v5 = match l3 {
                                                0 => {
                                                    let e5 = {
                                                        let l4 = *ptr0.add(8).cast::<i32>();
                                                        super::super::super::wasi::io::error::Error::from_handle(
                                                            l4 as u32,
                                                        )
                                                    };
                                                    StreamError::LastOperationFailed(e5)
                                                }
                                                n => {
                                                    if true {
                                                        match (&n, &1) {
                                                            (left_val, right_val) => {
                                                                if !(*left_val == *right_val) {
                                                                    let kind = ::core::panicking::AssertKind::Eq;
                                                                    ::core::panicking::assert_failed(
                                                                        kind,
                                                                        &*left_val,
                                                                        &*right_val,
                                                                        ::core::option::Option::Some(
                                                                            format_args!("invalid enum discriminant"),
                                                                        ),
                                                                    );
                                                                }
                                                            }
                                                        };
                                                    }
                                                    StreamError::Closed
                                                }
                                            };
                                            v5
                                        };
                                        Err(e)
                                    }
                                    _ => _rt::invalid_enum_discriminant(),
                                };
                                result6
                            }
                        }
                    }
                    impl OutputStream {
                        #[allow(unused_unsafe, clippy::all)]
                        /// Read from one stream and write to another.
                        ///
                        /// The behavior of splice is equivelant to:
                        /// 1. calling `check-write` on the `output-stream`
                        /// 2. calling `read` on the `input-stream` with the smaller of the
                        /// `check-write` permitted length and the `len` provided to `splice`
                        /// 3. calling `write` on the `output-stream` with that read data.
                        ///
                        /// Any error reported by the call to `check-write`, `read`, or
                        /// `write` ends the splice and reports that error.
                        ///
                        /// This function returns the number of bytes transferred; it may be less
                        /// than `len`.
                        #[allow(async_fn_in_trait)]
                        pub fn splice(
                            &self,
                            src: &InputStream,
                            len: u64,
                        ) -> Result<u64, StreamError> {
                            unsafe {
                                #[repr(align(8))]
                                struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                                let mut ret_area = RetArea(
                                    [::core::mem::MaybeUninit::uninit(); 16],
                                );
                                let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                                #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[method]output-stream.splice"]
                                    fn wit_import1(_: i32, _: i32, _: i64, _: *mut u8);
                                }
                                wit_import1(
                                    (self).handle() as i32,
                                    (src).handle() as i32,
                                    _rt::as_i64(&len),
                                    ptr0,
                                );
                                let l2 = i32::from(*ptr0.add(0).cast::<u8>());
                                let result7 = match l2 {
                                    0 => {
                                        let e = {
                                            let l3 = *ptr0.add(8).cast::<i64>();
                                            l3 as u64
                                        };
                                        Ok(e)
                                    }
                                    1 => {
                                        let e = {
                                            let l4 = i32::from(*ptr0.add(8).cast::<u8>());
                                            let v6 = match l4 {
                                                0 => {
                                                    let e6 = {
                                                        let l5 = *ptr0.add(12).cast::<i32>();
                                                        super::super::super::wasi::io::error::Error::from_handle(
                                                            l5 as u32,
                                                        )
                                                    };
                                                    StreamError::LastOperationFailed(e6)
                                                }
                                                n => {
                                                    if true {
                                                        match (&n, &1) {
                                                            (left_val, right_val) => {
                                                                if !(*left_val == *right_val) {
                                                                    let kind = ::core::panicking::AssertKind::Eq;
                                                                    ::core::panicking::assert_failed(
                                                                        kind,
                                                                        &*left_val,
                                                                        &*right_val,
                                                                        ::core::option::Option::Some(
                                                                            format_args!("invalid enum discriminant"),
                                                                        ),
                                                                    );
                                                                }
                                                            }
                                                        };
                                                    }
                                                    StreamError::Closed
                                                }
                                            };
                                            v6
                                        };
                                        Err(e)
                                    }
                                    _ => _rt::invalid_enum_discriminant(),
                                };
                                result7
                            }
                        }
                    }
                    impl OutputStream {
                        #[allow(unused_unsafe, clippy::all)]
                        /// Read from one stream and write to another, with blocking.
                        ///
                        /// This is similar to `splice`, except that it blocks until the
                        /// `output-stream` is ready for writing, and the `input-stream`
                        /// is ready for reading, before performing the `splice`.
                        #[allow(async_fn_in_trait)]
                        pub fn blocking_splice(
                            &self,
                            src: &InputStream,
                            len: u64,
                        ) -> Result<u64, StreamError> {
                            unsafe {
                                #[repr(align(8))]
                                struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                                let mut ret_area = RetArea(
                                    [::core::mem::MaybeUninit::uninit(); 16],
                                );
                                let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                                #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                                unsafe extern "C" {
                                    #[link_name = "[method]output-stream.blocking-splice"]
                                    fn wit_import1(_: i32, _: i32, _: i64, _: *mut u8);
                                }
                                wit_import1(
                                    (self).handle() as i32,
                                    (src).handle() as i32,
                                    _rt::as_i64(&len),
                                    ptr0,
                                );
                                let l2 = i32::from(*ptr0.add(0).cast::<u8>());
                                let result7 = match l2 {
                                    0 => {
                                        let e = {
                                            let l3 = *ptr0.add(8).cast::<i64>();
                                            l3 as u64
                                        };
                                        Ok(e)
                                    }
                                    1 => {
                                        let e = {
                                            let l4 = i32::from(*ptr0.add(8).cast::<u8>());
                                            let v6 = match l4 {
                                                0 => {
                                                    let e6 = {
                                                        let l5 = *ptr0.add(12).cast::<i32>();
                                                        super::super::super::wasi::io::error::Error::from_handle(
                                                            l5 as u32,
                                                        )
                                                    };
                                                    StreamError::LastOperationFailed(e6)
                                                }
                                                n => {
                                                    if true {
                                                        match (&n, &1) {
                                                            (left_val, right_val) => {
                                                                if !(*left_val == *right_val) {
                                                                    let kind = ::core::panicking::AssertKind::Eq;
                                                                    ::core::panicking::assert_failed(
                                                                        kind,
                                                                        &*left_val,
                                                                        &*right_val,
                                                                        ::core::option::Option::Some(
                                                                            format_args!("invalid enum discriminant"),
                                                                        ),
                                                                    );
                                                                }
                                                            }
                                                        };
                                                    }
                                                    StreamError::Closed
                                                }
                                            };
                                            v6
                                        };
                                        Err(e)
                                    }
                                    _ => _rt::invalid_enum_discriminant(),
                                };
                                result7
                            }
                        }
                    }
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod exports {
            pub mod wasi {
                pub mod http {
                    /// This interface defines a handler of incoming HTTP Requests. It should
                    /// be exported by components which can respond to HTTP Requests.
                    #[allow(dead_code, async_fn_in_trait, unused_imports, clippy::all)]
                    pub mod incoming_handler {
                        #[used]
                        #[doc(hidden)]
                        static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                        use super::super::super::super::_rt;
                        pub type IncomingRequest = super::super::super::super::wasi::http::types::IncomingRequest;
                        pub type ResponseOutparam = super::super::super::super::wasi::http::types::ResponseOutparam;
                        #[doc(hidden)]
                        #[allow(non_snake_case, unused_unsafe)]
                        pub unsafe fn _export_handle_cabi<T: Guest>(
                            arg0: i32,
                            arg1: i32,
                        ) {
                            unsafe {
                                _rt::run_ctors_once();
                                {
                                    T::handle(
                                        super::super::super::super::wasi::http::types::IncomingRequest::from_handle(
                                            arg0 as u32,
                                        ),
                                        super::super::super::super::wasi::http::types::ResponseOutparam::from_handle(
                                            arg1 as u32,
                                        ),
                                    )
                                };
                            }
                        }
                        pub trait Guest {
                            /// This function is invoked with an incoming HTTP Request, and a resource
                            /// `response-outparam` which provides the capability to reply with an HTTP
                            /// Response. The response is sent by calling the `response-outparam.set`
                            /// method, which allows execution to continue after the response has been
                            /// sent. This enables both streaming to the response body, and performing other
                            /// work.
                            ///
                            /// The implementor of this function must write a response to the
                            /// `response-outparam` before returning, or else the caller will respond
                            /// with an error on its behalf.
                            #[allow(async_fn_in_trait)]
                            fn handle(
                                request: IncomingRequest,
                                response_out: ResponseOutparam,
                            ) -> ();
                        }
                        #[doc(hidden)]
                        pub(crate) use __export_wasi_http_incoming_handler_0_2_0_cabi;
                    }
                }
            }
        }
        mod _rt {
            #![allow(dead_code, clippy::all)]
            use core::fmt;
            use core::marker;
            use core::sync::atomic::{AtomicU32, Ordering::Relaxed};
            /// A type which represents a component model resource, either imported or
            /// exported into this component.
            ///
            /// This is a low-level wrapper which handles the lifetime of the resource
            /// (namely this has a destructor). The `T` provided defines the component model
            /// intrinsics that this wrapper uses.
            ///
            /// One of the chief purposes of this type is to provide `Deref` implementations
            /// to access the underlying data when it is owned.
            ///
            /// This type is primarily used in generated code for exported and imported
            /// resources.
            #[repr(transparent)]
            pub struct Resource<T: WasmResource> {
                handle: AtomicU32,
                _marker: marker::PhantomData<T>,
            }
            /// A trait which all wasm resources implement, namely providing the ability to
            /// drop a resource.
            ///
            /// This generally is implemented by generated code, not user-facing code.
            #[allow(clippy::missing_safety_doc)]
            pub unsafe trait WasmResource {
                /// Invokes the `[resource-drop]...` intrinsic.
                unsafe fn drop(handle: u32);
            }
            impl<T: WasmResource> Resource<T> {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    if true {
                        if !(handle != 0 && handle != u32::MAX) {
                            ::core::panicking::panic(
                                "assertion failed: handle != 0 && handle != u32::MAX",
                            )
                        }
                    }
                    Self {
                        handle: AtomicU32::new(handle),
                        _marker: marker::PhantomData,
                    }
                }
                /// Takes ownership of the handle owned by `resource`.
                ///
                /// Note that this ideally would be `into_handle` taking `Resource<T>` by
                /// ownership. The code generator does not enable that in all situations,
                /// unfortunately, so this is provided instead.
                ///
                /// Also note that `take_handle` is in theory only ever called on values
                /// owned by a generated function. For example a generated function might
                /// take `Resource<T>` as an argument but then call `take_handle` on a
                /// reference to that argument. In that sense the dynamic nature of
                /// `take_handle` should only be exposed internally to generated code, not
                /// to user code.
                #[doc(hidden)]
                pub fn take_handle(resource: &Resource<T>) -> u32 {
                    resource.handle.swap(u32::MAX, Relaxed)
                }
                #[doc(hidden)]
                pub fn handle(resource: &Resource<T>) -> u32 {
                    resource.handle.load(Relaxed)
                }
            }
            impl<T: WasmResource> fmt::Debug for Resource<T> {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    f.debug_struct("Resource").field("handle", &self.handle).finish()
                }
            }
            impl<T: WasmResource> Drop for Resource<T> {
                fn drop(&mut self) {
                    unsafe {
                        match self.handle.load(Relaxed) {
                            u32::MAX => {}
                            other => T::drop(other),
                        }
                    }
                }
            }
            pub unsafe fn bool_lift(val: u8) -> bool {
                if true {
                    match val {
                        0 => false,
                        1 => true,
                        _ => {
                            ::core::panicking::panic_fmt(
                                format_args!("invalid bool discriminant"),
                            );
                        }
                    }
                } else {
                    val != 0
                }
            }
            pub use alloc_crate::vec::Vec;
            pub use alloc_crate::alloc;
            pub fn as_i64<T: AsI64>(t: T) -> i64 {
                t.as_i64()
            }
            pub trait AsI64 {
                fn as_i64(self) -> i64;
            }
            impl<'a, T: Copy + AsI64> AsI64 for &'a T {
                fn as_i64(self) -> i64 {
                    (*self).as_i64()
                }
            }
            impl AsI64 for i64 {
                #[inline]
                fn as_i64(self) -> i64 {
                    self as i64
                }
            }
            impl AsI64 for u64 {
                #[inline]
                fn as_i64(self) -> i64 {
                    self as i64
                }
            }
            pub use alloc_crate::string::String;
            pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
                if true {
                    String::from_utf8(bytes).unwrap()
                } else {
                    unsafe { String::from_utf8_unchecked(bytes) }
                }
            }
            pub unsafe fn invalid_enum_discriminant<T>() -> T {
                if true {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("invalid enum discriminant"),
                        );
                    }
                } else {
                    unsafe { core::hint::unreachable_unchecked() }
                }
            }
            pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
                if size == 0 {
                    return;
                }
                unsafe {
                    let layout = alloc::Layout::from_size_align_unchecked(size, align);
                    alloc::dealloc(ptr, layout);
                }
            }
            pub fn as_i32<T: AsI32>(t: T) -> i32 {
                t.as_i32()
            }
            pub trait AsI32 {
                fn as_i32(self) -> i32;
            }
            impl<'a, T: Copy + AsI32> AsI32 for &'a T {
                fn as_i32(self) -> i32 {
                    (*self).as_i32()
                }
            }
            impl AsI32 for i32 {
                #[inline]
                fn as_i32(self) -> i32 {
                    self as i32
                }
            }
            impl AsI32 for u32 {
                #[inline]
                fn as_i32(self) -> i32 {
                    self as i32
                }
            }
            impl AsI32 for i16 {
                #[inline]
                fn as_i32(self) -> i32 {
                    self as i32
                }
            }
            impl AsI32 for u16 {
                #[inline]
                fn as_i32(self) -> i32 {
                    self as i32
                }
            }
            impl AsI32 for i8 {
                #[inline]
                fn as_i32(self) -> i32 {
                    self as i32
                }
            }
            impl AsI32 for u8 {
                #[inline]
                fn as_i32(self) -> i32 {
                    self as i32
                }
            }
            impl AsI32 for char {
                #[inline]
                fn as_i32(self) -> i32 {
                    self as i32
                }
            }
            impl AsI32 for usize {
                #[inline]
                fn as_i32(self) -> i32 {
                    self as i32
                }
            }
            pub fn run_ctors_once() {
                ::spin_sdk::wit_bindgen::rt::run_ctors_once();
            }
            extern crate alloc as alloc_crate;
        }
        #[doc(inline)]
        pub(crate) use __export_wasi_http_trigger_impl as export;
        #[unsafe(
            link_section = "component-type:wit-bindgen:0.43.0:fermyon:spin:wasi-http-trigger:encoded world"
        )]
        #[doc(hidden)]
        #[allow(clippy::octal_escapes)]
        pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 6627] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xdb2\x01A\x02\x01A\x19\
\x01B\x0a\x04\0\x08pollable\x03\x01\x01h\0\x01@\x01\x04self\x01\0\x7f\x04\0\x16[\
method]pollable.ready\x01\x02\x01@\x01\x04self\x01\x01\0\x04\0\x16[method]pollab\
le.block\x01\x03\x01p\x01\x01py\x01@\x01\x02in\x04\0\x05\x04\0\x04poll\x01\x06\x03\
\0\x12wasi:io/poll@0.2.0\x05\0\x02\x03\0\0\x08pollable\x01B\x0f\x02\x03\x02\x01\x01\
\x04\0\x08pollable\x03\0\0\x01w\x04\0\x07instant\x03\0\x02\x01w\x04\0\x08duratio\
n\x03\0\x04\x01@\0\0\x03\x04\0\x03now\x01\x06\x01@\0\0\x05\x04\0\x0aresolution\x01\
\x07\x01i\x01\x01@\x01\x04when\x03\0\x08\x04\0\x11subscribe-instant\x01\x09\x01@\
\x01\x04when\x05\0\x08\x04\0\x12subscribe-duration\x01\x0a\x03\0!wasi:clocks/mon\
otonic-clock@0.2.0\x05\x02\x01B\x04\x04\0\x05error\x03\x01\x01h\0\x01@\x01\x04se\
lf\x01\0s\x04\0\x1d[method]error.to-debug-string\x01\x02\x03\0\x13wasi:io/error@\
0.2.0\x05\x03\x02\x03\0\x02\x05error\x01B(\x02\x03\x02\x01\x04\x04\0\x05error\x03\
\0\0\x02\x03\x02\x01\x01\x04\0\x08pollable\x03\0\x02\x01i\x01\x01q\x02\x15last-o\
peration-failed\x01\x04\0\x06closed\0\0\x04\0\x0cstream-error\x03\0\x05\x04\0\x0c\
input-stream\x03\x01\x04\0\x0doutput-stream\x03\x01\x01h\x07\x01p}\x01j\x01\x0a\x01\
\x06\x01@\x02\x04self\x09\x03lenw\0\x0b\x04\0\x19[method]input-stream.read\x01\x0c\
\x04\0\"[method]input-stream.blocking-read\x01\x0c\x01j\x01w\x01\x06\x01@\x02\x04\
self\x09\x03lenw\0\x0d\x04\0\x19[method]input-stream.skip\x01\x0e\x04\0\"[method\
]input-stream.blocking-skip\x01\x0e\x01i\x03\x01@\x01\x04self\x09\0\x0f\x04\0\x1e\
[method]input-stream.subscribe\x01\x10\x01h\x08\x01@\x01\x04self\x11\0\x0d\x04\0\
![method]output-stream.check-write\x01\x12\x01j\0\x01\x06\x01@\x02\x04self\x11\x08\
contents\x0a\0\x13\x04\0\x1b[method]output-stream.write\x01\x14\x04\0.[method]ou\
tput-stream.blocking-write-and-flush\x01\x14\x01@\x01\x04self\x11\0\x13\x04\0\x1b\
[method]output-stream.flush\x01\x15\x04\0$[method]output-stream.blocking-flush\x01\
\x15\x01@\x01\x04self\x11\0\x0f\x04\0\x1f[method]output-stream.subscribe\x01\x16\
\x01@\x02\x04self\x11\x03lenw\0\x13\x04\0\"[method]output-stream.write-zeroes\x01\
\x17\x04\05[method]output-stream.blocking-write-zeroes-and-flush\x01\x17\x01@\x03\
\x04self\x11\x03src\x09\x03lenw\0\x0d\x04\0\x1c[method]output-stream.splice\x01\x18\
\x04\0%[method]output-stream.blocking-splice\x01\x18\x03\0\x15wasi:io/streams@0.\
2.0\x05\x05\x02\x03\0\x01\x08duration\x02\x03\0\x03\x0cinput-stream\x02\x03\0\x03\
\x0doutput-stream\x01B\xc0\x01\x02\x03\x02\x01\x06\x04\0\x08duration\x03\0\0\x02\
\x03\x02\x01\x07\x04\0\x0cinput-stream\x03\0\x02\x02\x03\x02\x01\x08\x04\0\x0dou\
tput-stream\x03\0\x04\x02\x03\x02\x01\x04\x04\0\x08io-error\x03\0\x06\x02\x03\x02\
\x01\x01\x04\0\x08pollable\x03\0\x08\x01q\x0a\x03get\0\0\x04head\0\0\x04post\0\0\
\x03put\0\0\x06delete\0\0\x07connect\0\0\x07options\0\0\x05trace\0\0\x05patch\0\0\
\x05other\x01s\0\x04\0\x06method\x03\0\x0a\x01q\x03\x04HTTP\0\0\x05HTTPS\0\0\x05\
other\x01s\0\x04\0\x06scheme\x03\0\x0c\x01ks\x01k{\x01r\x02\x05rcode\x0e\x09info\
-code\x0f\x04\0\x11DNS-error-payload\x03\0\x10\x01k}\x01r\x02\x08alert-id\x12\x0d\
alert-message\x0e\x04\0\x1aTLS-alert-received-payload\x03\0\x13\x01ky\x01r\x02\x0a\
field-name\x0e\x0afield-size\x15\x04\0\x12field-size-payload\x03\0\x16\x01kw\x01\
k\x17\x01q'\x0bDNS-timeout\0\0\x09DNS-error\x01\x11\0\x15destination-not-found\0\
\0\x17destination-unavailable\0\0\x19destination-IP-prohibited\0\0\x19destinatio\
n-IP-unroutable\0\0\x12connection-refused\0\0\x15connection-terminated\0\0\x12co\
nnection-timeout\0\0\x17connection-read-timeout\0\0\x18connection-write-timeout\0\
\0\x18connection-limit-reached\0\0\x12TLS-protocol-error\0\0\x15TLS-certificate-\
error\0\0\x12TLS-alert-received\x01\x14\0\x13HTTP-request-denied\0\0\x1cHTTP-req\
uest-length-required\0\0\x16HTTP-request-body-size\x01\x18\0\x1bHTTP-request-met\
hod-invalid\0\0\x18HTTP-request-URI-invalid\0\0\x19HTTP-request-URI-too-long\0\0\
\x20HTTP-request-header-section-size\x01\x15\0\x18HTTP-request-header-size\x01\x19\
\0!HTTP-request-trailer-section-size\x01\x15\0\x19HTTP-request-trailer-size\x01\x17\
\0\x18HTTP-response-incomplete\0\0!HTTP-response-header-section-size\x01\x15\0\x19\
HTTP-response-header-size\x01\x17\0\x17HTTP-response-body-size\x01\x18\0\"HTTP-r\
esponse-trailer-section-size\x01\x15\0\x1aHTTP-response-trailer-size\x01\x17\0\x1d\
HTTP-response-transfer-coding\x01\x0e\0\x1cHTTP-response-content-coding\x01\x0e\0\
\x15HTTP-response-timeout\0\0\x13HTTP-upgrade-failed\0\0\x13HTTP-protocol-error\0\
\0\x0dloop-detected\0\0\x13configuration-error\0\0\x0einternal-error\x01\x0e\0\x04\
\0\x0aerror-code\x03\0\x1a\x01q\x03\x0einvalid-syntax\0\0\x09forbidden\0\0\x09im\
mutable\0\0\x04\0\x0cheader-error\x03\0\x1c\x01s\x04\0\x09field-key\x03\0\x1e\x01\
p}\x04\0\x0bfield-value\x03\0\x20\x04\0\x06fields\x03\x01\x04\0\x07headers\x03\0\
\"\x04\0\x08trailers\x03\0\"\x04\0\x10incoming-request\x03\x01\x04\0\x10outgoing\
-request\x03\x01\x04\0\x0frequest-options\x03\x01\x04\0\x11response-outparam\x03\
\x01\x01{\x04\0\x0bstatus-code\x03\0)\x04\0\x11incoming-response\x03\x01\x04\0\x0d\
incoming-body\x03\x01\x04\0\x0ffuture-trailers\x03\x01\x04\0\x11outgoing-respons\
e\x03\x01\x04\0\x0doutgoing-body\x03\x01\x04\0\x18future-incoming-response\x03\x01\
\x01i\"\x01@\0\01\x04\0\x13[constructor]fields\x012\x01o\x02\x1f!\x01p3\x01j\x01\
1\x01\x1d\x01@\x01\x07entries4\05\x04\0\x18[static]fields.from-list\x016\x01h\"\x01\
p!\x01@\x02\x04self7\x04name\x1f\08\x04\0\x12[method]fields.get\x019\x01@\x02\x04\
self7\x04name\x1f\0\x7f\x04\0\x12[method]fields.has\x01:\x01j\0\x01\x1d\x01@\x03\
\x04self7\x04name\x1f\x05value8\0;\x04\0\x12[method]fields.set\x01<\x01@\x02\x04\
self7\x04name\x1f\0;\x04\0\x15[method]fields.delete\x01=\x01@\x03\x04self7\x04na\
me\x1f\x05value!\0;\x04\0\x15[method]fields.append\x01>\x01@\x01\x04self7\04\x04\
\0\x16[method]fields.entries\x01?\x01@\x01\x04self7\01\x04\0\x14[method]fields.c\
lone\x01@\x01h%\x01@\x01\x04self\xc1\0\0\x0b\x04\0\x1f[method]incoming-request.m\
ethod\x01B\x01@\x01\x04self\xc1\0\0\x0e\x04\0([method]incoming-request.path-with\
-query\x01C\x01k\x0d\x01@\x01\x04self\xc1\0\0\xc4\0\x04\0\x1f[method]incoming-re\
quest.scheme\x01E\x04\0\"[method]incoming-request.authority\x01C\x01i#\x01@\x01\x04\
self\xc1\0\0\xc6\0\x04\0\x20[method]incoming-request.headers\x01G\x01i,\x01j\x01\
\xc8\0\0\x01@\x01\x04self\xc1\0\0\xc9\0\x04\0\x20[method]incoming-request.consum\
e\x01J\x01i&\x01@\x01\x07headers\xc6\0\0\xcb\0\x04\0\x1d[constructor]outgoing-re\
quest\x01L\x01h&\x01i/\x01j\x01\xce\0\0\x01@\x01\x04self\xcd\0\0\xcf\0\x04\0\x1d\
[method]outgoing-request.body\x01P\x01@\x01\x04self\xcd\0\0\x0b\x04\0\x1f[method\
]outgoing-request.method\x01Q\x01j\0\0\x01@\x02\x04self\xcd\0\x06method\x0b\0\xd2\
\0\x04\0#[method]outgoing-request.set-method\x01S\x01@\x01\x04self\xcd\0\0\x0e\x04\
\0([method]outgoing-request.path-with-query\x01T\x01@\x02\x04self\xcd\0\x0fpath-\
with-query\x0e\0\xd2\0\x04\0,[method]outgoing-request.set-path-with-query\x01U\x01\
@\x01\x04self\xcd\0\0\xc4\0\x04\0\x1f[method]outgoing-request.scheme\x01V\x01@\x02\
\x04self\xcd\0\x06scheme\xc4\0\0\xd2\0\x04\0#[method]outgoing-request.set-scheme\
\x01W\x04\0\"[method]outgoing-request.authority\x01T\x01@\x02\x04self\xcd\0\x09a\
uthority\x0e\0\xd2\0\x04\0&[method]outgoing-request.set-authority\x01X\x01@\x01\x04\
self\xcd\0\0\xc6\0\x04\0\x20[method]outgoing-request.headers\x01Y\x01i'\x01@\0\0\
\xda\0\x04\0\x1c[constructor]request-options\x01[\x01h'\x01k\x01\x01@\x01\x04sel\
f\xdc\0\0\xdd\0\x04\0'[method]request-options.connect-timeout\x01^\x01@\x02\x04s\
elf\xdc\0\x08duration\xdd\0\0\xd2\0\x04\0+[method]request-options.set-connect-ti\
meout\x01_\x04\0*[method]request-options.first-byte-timeout\x01^\x04\0.[method]r\
equest-options.set-first-byte-timeout\x01_\x04\0-[method]request-options.between\
-bytes-timeout\x01^\x04\01[method]request-options.set-between-bytes-timeout\x01_\
\x01i(\x01i.\x01j\x01\xe1\0\x01\x1b\x01@\x02\x05param\xe0\0\x08response\xe2\0\x01\
\0\x04\0\x1d[static]response-outparam.set\x01c\x01h+\x01@\x01\x04self\xe4\0\0*\x04\
\0\x20[method]incoming-response.status\x01e\x01@\x01\x04self\xe4\0\0\xc6\0\x04\0\
![method]incoming-response.headers\x01f\x01@\x01\x04self\xe4\0\0\xc9\0\x04\0![me\
thod]incoming-response.consume\x01g\x01h,\x01i\x03\x01j\x01\xe9\0\0\x01@\x01\x04\
self\xe8\0\0\xea\0\x04\0\x1c[method]incoming-body.stream\x01k\x01i-\x01@\x01\x04\
this\xc8\0\0\xec\0\x04\0\x1c[static]incoming-body.finish\x01m\x01h-\x01i\x09\x01\
@\x01\x04self\xee\0\0\xef\0\x04\0![method]future-trailers.subscribe\x01p\x01i$\x01\
k\xf1\0\x01j\x01\xf2\0\x01\x1b\x01j\x01\xf3\0\0\x01k\xf4\0\x01@\x01\x04self\xee\0\
\0\xf5\0\x04\0\x1b[method]future-trailers.get\x01v\x01@\x01\x07headers\xc6\0\0\xe1\
\0\x04\0\x1e[constructor]outgoing-response\x01w\x01h.\x01@\x01\x04self\xf8\0\0*\x04\
\0%[method]outgoing-response.status-code\x01y\x01@\x02\x04self\xf8\0\x0bstatus-c\
ode*\0\xd2\0\x04\0)[method]outgoing-response.set-status-code\x01z\x01@\x01\x04se\
lf\xf8\0\0\xc6\0\x04\0![method]outgoing-response.headers\x01{\x01@\x01\x04self\xf8\
\0\0\xcf\0\x04\0\x1e[method]outgoing-response.body\x01|\x01h/\x01i\x05\x01j\x01\xfe\
\0\0\x01@\x01\x04self\xfd\0\0\xff\0\x04\0\x1b[method]outgoing-body.write\x01\x80\
\x01\x01j\0\x01\x1b\x01@\x02\x04this\xce\0\x08trailers\xf2\0\0\x81\x01\x04\0\x1c\
[static]outgoing-body.finish\x01\x82\x01\x01h0\x01@\x01\x04self\x83\x01\0\xef\0\x04\
\0*[method]future-incoming-response.subscribe\x01\x84\x01\x01i+\x01j\x01\x85\x01\
\x01\x1b\x01j\x01\x86\x01\0\x01k\x87\x01\x01@\x01\x04self\x83\x01\0\x88\x01\x04\0\
$[method]future-incoming-response.get\x01\x89\x01\x01h\x07\x01k\x1b\x01@\x01\x03\
err\x8a\x01\0\x8b\x01\x04\0\x0fhttp-error-code\x01\x8c\x01\x03\0\x15wasi:http/ty\
pes@0.2.0\x05\x09\x02\x03\0\x04\x10outgoing-request\x02\x03\0\x04\x0frequest-opt\
ions\x02\x03\0\x04\x18future-incoming-response\x02\x03\0\x04\x0aerror-code\x01B\x0f\
\x02\x03\x02\x01\x0a\x04\0\x10outgoing-request\x03\0\0\x02\x03\x02\x01\x0b\x04\0\
\x0frequest-options\x03\0\x02\x02\x03\x02\x01\x0c\x04\0\x18future-incoming-respo\
nse\x03\0\x04\x02\x03\x02\x01\x0d\x04\0\x0aerror-code\x03\0\x06\x01i\x01\x01i\x03\
\x01k\x09\x01i\x05\x01j\x01\x0b\x01\x07\x01@\x02\x07request\x08\x07options\x0a\0\
\x0c\x04\0\x06handle\x01\x0d\x03\0\x20wasi:http/outgoing-handler@0.2.0\x05\x0e\x02\
\x03\0\x04\x10incoming-request\x02\x03\0\x04\x11response-outparam\x01B\x08\x02\x03\
\x02\x01\x0f\x04\0\x10incoming-request\x03\0\0\x02\x03\x02\x01\x10\x04\0\x11resp\
onse-outparam\x03\0\x02\x01i\x01\x01i\x03\x01@\x02\x07request\x04\x0cresponse-ou\
t\x05\x01\0\x04\0\x06handle\x01\x06\x04\0\x20wasi:http/incoming-handler@0.2.0\x05\
\x11\x04\0\x1efermyon:spin/wasi-http-trigger\x04\0\x0b\x17\x01\0\x11wasi-http-tr\
igger\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit-component\x070.235.\
0\x10wit-bindgen-rust\x060.43.0";
        #[inline(never)]
        #[doc(hidden)]
        pub fn __link_custom_section_describing_imports() {
            ::spin_sdk::wit_bindgen::rt::maybe_link_cabi_realloc();
        }
        const _: &[u8] = b"/// This interface defines all of the types and methods for implementing\n/// HTTP Requests and Responses, both incoming and outgoing, as well as\n/// their headers, trailers, and bodies.\ninterface types {\n  use wasi:clocks/monotonic-clock@0.2.0.{duration};\n  use wasi:io/streams@0.2.0.{input-stream, output-stream};\n  use wasi:io/error@0.2.0.{error as io-error};\n  use wasi:io/poll@0.2.0.{pollable};\n\n  /// This type corresponds to HTTP standard Methods.\n  variant method {\n    get,\n    head,\n    post,\n    put,\n    delete,\n    connect,\n    options,\n    trace,\n    patch,\n    other(string)\n  }\n\n  /// This type corresponds to HTTP standard Related Schemes.\n  variant scheme {\n    HTTP,\n    HTTPS,\n    other(string)\n  }\n\n  /// These cases are inspired by the IANA HTTP Proxy Error Types:\n  ///   https://www.iana.org/assignments/http-proxy-status/http-proxy-status.xhtml#table-http-proxy-error-types\n  variant error-code {\n    DNS-timeout,\n    DNS-error(DNS-error-payload),\n    destination-not-found,\n    destination-unavailable,\n    destination-IP-prohibited,\n    destination-IP-unroutable,\n    connection-refused,\n    connection-terminated,\n    connection-timeout,\n    connection-read-timeout,\n    connection-write-timeout,\n    connection-limit-reached,\n    TLS-protocol-error,\n    TLS-certificate-error,\n    TLS-alert-received(TLS-alert-received-payload),\n    HTTP-request-denied,\n    HTTP-request-length-required,\n    HTTP-request-body-size(option<u64>),\n    HTTP-request-method-invalid,\n    HTTP-request-URI-invalid,\n    HTTP-request-URI-too-long,\n    HTTP-request-header-section-size(option<u32>),\n    HTTP-request-header-size(option<field-size-payload>),\n    HTTP-request-trailer-section-size(option<u32>),\n    HTTP-request-trailer-size(field-size-payload),\n    HTTP-response-incomplete,\n    HTTP-response-header-section-size(option<u32>),\n    HTTP-response-header-size(field-size-payload),\n    HTTP-response-body-size(option<u64>),\n    HTTP-response-trailer-section-size(option<u32>),\n    HTTP-response-trailer-size(field-size-payload),\n    HTTP-response-transfer-coding(option<string>),\n    HTTP-response-content-coding(option<string>),\n    HTTP-response-timeout,\n    HTTP-upgrade-failed,\n    HTTP-protocol-error,\n    loop-detected,\n    configuration-error,\n    /// This is a catch-all error for anything that doesn\'t fit cleanly into a\n    /// more specific case. It also includes an optional string for an\n    /// unstructured description of the error. Users should not depend on the\n    /// string for diagnosing errors, as it\'s not required to be consistent\n    /// between implementations.\n    internal-error(option<string>)\n  }\n\n  /// Defines the case payload type for `DNS-error` above:\n  record DNS-error-payload {\n    rcode: option<string>,\n    info-code: option<u16>\n  }\n\n  /// Defines the case payload type for `TLS-alert-received` above:\n  record TLS-alert-received-payload {\n    alert-id: option<u8>,\n    alert-message: option<string>\n  }\n\n  /// Defines the case payload type for `HTTP-response-{header,trailer}-size` above:\n  record field-size-payload {\n    field-name: option<string>,\n    field-size: option<u32>\n  }\n\n  /// Attempts to extract a http-related `error` from the wasi:io `error`\n  /// provided.\n  ///\n  /// Stream operations which return\n  /// `wasi:io/stream/stream-error::last-operation-failed` have a payload of\n  /// type `wasi:io/error/error` with more information about the operation\n  /// that failed. This payload can be passed through to this function to see\n  /// if there\'s http-related information about the error to return.\n  ///\n  /// Note that this function is fallible because not all io-errors are\n  /// http-related errors.\n  http-error-code: func(err: borrow<io-error>) -> option<error-code>;\n\n  /// This type enumerates the different kinds of errors that may occur when\n  /// setting or appending to a `fields` resource.\n  variant header-error {\n    /// This error indicates that a `field-key` or `field-value` was\n    /// syntactically invalid when used with an operation that sets headers in a\n    /// `fields`.\n    invalid-syntax,\n\n    /// This error indicates that a forbidden `field-key` was used when trying\n    /// to set a header in a `fields`.\n    forbidden,\n\n    /// This error indicates that the operation on the `fields` was not\n    /// permitted because the fields are immutable.\n    immutable,\n  }\n\n  /// Field keys are always strings.\n  type field-key = string;\n\n  /// Field values should always be ASCII strings. However, in\n  /// reality, HTTP implementations often have to interpret malformed values,\n  /// so they are provided as a list of bytes.\n  type field-value = list<u8>;\n\n  /// This following block defines the `fields` resource which corresponds to\n  /// HTTP standard Fields. Fields are a common representation used for both\n  /// Headers and Trailers.\n  ///\n  /// A `fields` may be mutable or immutable. A `fields` created using the\n  /// constructor, `from-list`, or `clone` will be mutable, but a `fields`\n  /// resource given by other means (including, but not limited to,\n  /// `incoming-request.headers`, `outgoing-request.headers`) might be be\n  /// immutable. In an immutable fields, the `set`, `append`, and `delete`\n  /// operations will fail with `header-error.immutable`.\n  resource fields {\n\n    /// Construct an empty HTTP Fields.\n    ///\n    /// The resulting `fields` is mutable.\n    constructor();\n\n    /// Construct an HTTP Fields.\n    ///\n    /// The resulting `fields` is mutable.\n    ///\n    /// The list represents each key-value pair in the Fields. Keys\n    /// which have multiple values are represented by multiple entries in this\n    /// list with the same key.\n    ///\n    /// The tuple is a pair of the field key, represented as a string, and\n    /// Value, represented as a list of bytes. In a valid Fields, all keys\n    /// and values are valid UTF-8 strings. However, values are not always\n    /// well-formed, so they are represented as a raw list of bytes.\n    ///\n    /// An error result will be returned if any header or value was\n    /// syntactically invalid, or if a header was forbidden.\n    from-list: static func(\n      entries: list<tuple<field-key,field-value>>\n    ) -> result<fields, header-error>;\n\n    /// Get all of the values corresponding to a key. If the key is not present\n    /// in this `fields`, an empty list is returned. However, if the key is\n    /// present but empty, this is represented by a list with one or more\n    /// empty field-values present.\n    get: func(name: field-key) -> list<field-value>;\n\n    /// Returns `true` when the key is present in this `fields`. If the key is\n    /// syntactically invalid, `false` is returned.\n    has: func(name: field-key) -> bool;\n\n    /// Set all of the values for a key. Clears any existing values for that\n    /// key, if they have been set.\n    ///\n    /// Fails with `header-error.immutable` if the `fields` are immutable.\n    set: func(name: field-key, value: list<field-value>) -> result<_, header-error>;\n\n    /// Delete all values for a key. Does nothing if no values for the key\n    /// exist.\n    ///\n    /// Fails with `header-error.immutable` if the `fields` are immutable.\n    delete: func(name: field-key) -> result<_, header-error>;\n\n    /// Append a value for a key. Does not change or delete any existing\n    /// values for that key.\n    ///\n    /// Fails with `header-error.immutable` if the `fields` are immutable.\n    append: func(name: field-key, value: field-value) -> result<_, header-error>;\n\n    /// Retrieve the full set of keys and values in the Fields. Like the\n    /// constructor, the list represents each key-value pair.\n    ///\n    /// The outer list represents each key-value pair in the Fields. Keys\n    /// which have multiple values are represented by multiple entries in this\n    /// list with the same key.\n    entries: func() -> list<tuple<field-key,field-value>>;\n\n    /// Make a deep copy of the Fields. Equivelant in behavior to calling the\n    /// `fields` constructor on the return value of `entries`. The resulting\n    /// `fields` is mutable.\n    clone: func() -> fields;\n  }\n\n  /// Headers is an alias for Fields.\n  type headers = fields;\n\n  /// Trailers is an alias for Fields.\n  type trailers = fields;\n\n  /// Represents an incoming HTTP Request.\n  resource incoming-request {\n\n    /// Returns the method of the incoming request.\n    method: func() -> method;\n\n    /// Returns the path with query parameters from the request, as a string.\n    path-with-query: func() -> option<string>;\n\n    /// Returns the protocol scheme from the request.\n    scheme: func() -> option<scheme>;\n\n    /// Returns the authority from the request, if it was present.\n    authority: func() -> option<string>;\n\n    /// Get the `headers` associated with the request.\n    ///\n    /// The returned `headers` resource is immutable: `set`, `append`, and\n    /// `delete` operations will fail with `header-error.immutable`.\n    ///\n    /// The `headers` returned are a child resource: it must be dropped before\n    /// the parent `incoming-request` is dropped. Dropping this\n    /// `incoming-request` before all children are dropped will trap.\n    headers: func() -> headers;\n\n    /// Gives the `incoming-body` associated with this request. Will only\n    /// return success at most once, and subsequent calls will return error.\n    consume: func() -> result<incoming-body>;\n  }\n\n  /// Represents an outgoing HTTP Request.\n  resource outgoing-request {\n\n    /// Construct a new `outgoing-request` with a default `method` of `GET`, and\n    /// `none` values for `path-with-query`, `scheme`, and `authority`.\n    ///\n    /// * `headers` is the HTTP Headers for the Request.\n    ///\n    /// It is possible to construct, or manipulate with the accessor functions\n    /// below, an `outgoing-request` with an invalid combination of `scheme`\n    /// and `authority`, or `headers` which are not permitted to be sent.\n    /// It is the obligation of the `outgoing-handler.handle` implementation\n    /// to reject invalid constructions of `outgoing-request`.\n    constructor(\n      headers: headers\n    );\n\n    /// Returns the resource corresponding to the outgoing Body for this\n    /// Request.\n    ///\n    /// Returns success on the first call: the `outgoing-body` resource for\n    /// this `outgoing-request` can be retrieved at most once. Subsequent\n    /// calls will return error.\n    body: func() -> result<outgoing-body>;\n\n    /// Get the Method for the Request.\n    method: func() -> method;\n    /// Set the Method for the Request. Fails if the string present in a\n    /// `method.other` argument is not a syntactically valid method.\n    set-method: func(method: method) -> result;\n\n    /// Get the combination of the HTTP Path and Query for the Request.\n    /// When `none`, this represents an empty Path and empty Query.\n    path-with-query: func() -> option<string>;\n    /// Set the combination of the HTTP Path and Query for the Request.\n    /// When `none`, this represents an empty Path and empty Query. Fails is the\n    /// string given is not a syntactically valid path and query uri component.\n    set-path-with-query: func(path-with-query: option<string>) -> result;\n\n    /// Get the HTTP Related Scheme for the Request. When `none`, the\n    /// implementation may choose an appropriate default scheme.\n    scheme: func() -> option<scheme>;\n    /// Set the HTTP Related Scheme for the Request. When `none`, the\n    /// implementation may choose an appropriate default scheme. Fails if the\n    /// string given is not a syntactically valid uri scheme.\n    set-scheme: func(scheme: option<scheme>) -> result;\n\n    /// Get the HTTP Authority for the Request. A value of `none` may be used\n    /// with Related Schemes which do not require an Authority. The HTTP and\n    /// HTTPS schemes always require an authority.\n    authority: func() -> option<string>;\n    /// Set the HTTP Authority for the Request. A value of `none` may be used\n    /// with Related Schemes which do not require an Authority. The HTTP and\n    /// HTTPS schemes always require an authority. Fails if the string given is\n    /// not a syntactically valid uri authority.\n    set-authority: func(authority: option<string>) -> result;\n\n    /// Get the headers associated with the Request.\n    ///\n    /// The returned `headers` resource is immutable: `set`, `append`, and\n    /// `delete` operations will fail with `header-error.immutable`.\n    ///\n    /// This headers resource is a child: it must be dropped before the parent\n    /// `outgoing-request` is dropped, or its ownership is transfered to\n    /// another component by e.g. `outgoing-handler.handle`.\n    headers: func() -> headers;\n  }\n\n  /// Parameters for making an HTTP Request. Each of these parameters is\n  /// currently an optional timeout applicable to the transport layer of the\n  /// HTTP protocol.\n  ///\n  /// These timeouts are separate from any the user may use to bound a\n  /// blocking call to `wasi:io/poll.poll`.\n  resource request-options {\n    /// Construct a default `request-options` value.\n    constructor();\n\n    /// The timeout for the initial connect to the HTTP Server.\n    connect-timeout: func() -> option<duration>;\n\n    /// Set the timeout for the initial connect to the HTTP Server. An error\n    /// return value indicates that this timeout is not supported.\n    set-connect-timeout: func(duration: option<duration>) -> result;\n\n    /// The timeout for receiving the first byte of the Response body.\n    first-byte-timeout: func() -> option<duration>;\n\n    /// Set the timeout for receiving the first byte of the Response body. An\n    /// error return value indicates that this timeout is not supported.\n    set-first-byte-timeout: func(duration: option<duration>) -> result;\n\n    /// The timeout for receiving subsequent chunks of bytes in the Response\n    /// body stream.\n    between-bytes-timeout: func() -> option<duration>;\n\n    /// Set the timeout for receiving subsequent chunks of bytes in the Response\n    /// body stream. An error return value indicates that this timeout is not\n    /// supported.\n    set-between-bytes-timeout: func(duration: option<duration>) -> result;\n  }\n\n  /// Represents the ability to send an HTTP Response.\n  ///\n  /// This resource is used by the `wasi:http/incoming-handler` interface to\n  /// allow a Response to be sent corresponding to the Request provided as the\n  /// other argument to `incoming-handler.handle`.\n  resource response-outparam {\n\n    /// Set the value of the `response-outparam` to either send a response,\n    /// or indicate an error.\n    ///\n    /// This method consumes the `response-outparam` to ensure that it is\n    /// called at most once. If it is never called, the implementation\n    /// will respond with an error.\n    ///\n    /// The user may provide an `error` to `response` to allow the\n    /// implementation determine how to respond with an HTTP error response.\n    set: static func(\n      param: response-outparam,\n      response: result<outgoing-response, error-code>,\n    );\n  }\n\n  /// This type corresponds to the HTTP standard Status Code.\n  type status-code = u16;\n\n  /// Represents an incoming HTTP Response.\n  resource incoming-response {\n\n    /// Returns the status code from the incoming response.\n    status: func() -> status-code;\n\n    /// Returns the headers from the incoming response.\n    ///\n    /// The returned `headers` resource is immutable: `set`, `append`, and\n    /// `delete` operations will fail with `header-error.immutable`.\n    ///\n    /// This headers resource is a child: it must be dropped before the parent\n    /// `incoming-response` is dropped.\n    headers: func() -> headers;\n\n    /// Returns the incoming body. May be called at most once. Returns error\n    /// if called additional times.\n    consume: func() -> result<incoming-body>;\n  }\n\n  /// Represents an incoming HTTP Request or Response\'s Body.\n  ///\n  /// A body has both its contents - a stream of bytes - and a (possibly\n  /// empty) set of trailers, indicating that the full contents of the\n  /// body have been received. This resource represents the contents as\n  /// an `input-stream` and the delivery of trailers as a `future-trailers`,\n  /// and ensures that the user of this interface may only be consuming either\n  /// the body contents or waiting on trailers at any given time.\n  resource incoming-body {\n\n    /// Returns the contents of the body, as a stream of bytes.\n    ///\n    /// Returns success on first call: the stream representing the contents\n    /// can be retrieved at most once. Subsequent calls will return error.\n    ///\n    /// The returned `input-stream` resource is a child: it must be dropped\n    /// before the parent `incoming-body` is dropped, or consumed by\n    /// `incoming-body.finish`.\n    ///\n    /// This invariant ensures that the implementation can determine whether\n    /// the user is consuming the contents of the body, waiting on the\n    /// `future-trailers` to be ready, or neither. This allows for network\n    /// backpressure is to be applied when the user is consuming the body,\n    /// and for that backpressure to not inhibit delivery of the trailers if\n    /// the user does not read the entire body.\n    %stream: func() -> result<input-stream>;\n\n    /// Takes ownership of `incoming-body`, and returns a `future-trailers`.\n    /// This function will trap if the `input-stream` child is still alive.\n    finish: static func(this: incoming-body) -> future-trailers;\n  }\n\n  /// Represents a future which may eventaully return trailers, or an error.\n  ///\n  /// In the case that the incoming HTTP Request or Response did not have any\n  /// trailers, this future will resolve to the empty set of trailers once the\n  /// complete Request or Response body has been received.\n  resource future-trailers {\n\n    /// Returns a pollable which becomes ready when either the trailers have\n    /// been received, or an error has occured. When this pollable is ready,\n    /// the `get` method will return `some`.\n    subscribe: func() -> pollable;\n\n    /// Returns the contents of the trailers, or an error which occured,\n    /// once the future is ready.\n    ///\n    /// The outer `option` represents future readiness. Users can wait on this\n    /// `option` to become `some` using the `subscribe` method.\n    ///\n    /// The outer `result` is used to retrieve the trailers or error at most\n    /// once. It will be success on the first call in which the outer option\n    /// is `some`, and error on subsequent calls.\n    ///\n    /// The inner `result` represents that either the HTTP Request or Response\n    /// body, as well as any trailers, were received successfully, or that an\n    /// error occured receiving them. The optional `trailers` indicates whether\n    /// or not trailers were present in the body.\n    ///\n    /// When some `trailers` are returned by this method, the `trailers`\n    /// resource is immutable, and a child. Use of the `set`, `append`, or\n    /// `delete` methods will return an error, and the resource must be\n    /// dropped before the parent `future-trailers` is dropped.\n    get: func() -> option<result<result<option<trailers>, error-code>>>;\n  }\n\n  /// Represents an outgoing HTTP Response.\n  resource outgoing-response {\n\n    /// Construct an `outgoing-response`, with a default `status-code` of `200`.\n    /// If a different `status-code` is needed, it must be set via the\n    /// `set-status-code` method.\n    ///\n    /// * `headers` is the HTTP Headers for the Response.\n    constructor(headers: headers);\n\n    /// Get the HTTP Status Code for the Response.\n    status-code: func() -> status-code;\n\n    /// Set the HTTP Status Code for the Response. Fails if the status-code\n    /// given is not a valid http status code.\n    set-status-code: func(status-code: status-code) -> result;\n\n    /// Get the headers associated with the Request.\n    ///\n    /// The returned `headers` resource is immutable: `set`, `append`, and\n    /// `delete` operations will fail with `header-error.immutable`.\n    ///\n    /// This headers resource is a child: it must be dropped before the parent\n    /// `outgoing-request` is dropped, or its ownership is transfered to\n    /// another component by e.g. `outgoing-handler.handle`.\n    headers: func() -> headers;\n\n    /// Returns the resource corresponding to the outgoing Body for this Response.\n    ///\n    /// Returns success on the first call: the `outgoing-body` resource for\n    /// this `outgoing-response` can be retrieved at most once. Subsequent\n    /// calls will return error.\n    body: func() -> result<outgoing-body>;\n  }\n\n  /// Represents an outgoing HTTP Request or Response\'s Body.\n  ///\n  /// A body has both its contents - a stream of bytes - and a (possibly\n  /// empty) set of trailers, inducating the full contents of the body\n  /// have been sent. This resource represents the contents as an\n  /// `output-stream` child resource, and the completion of the body (with\n  /// optional trailers) with a static function that consumes the\n  /// `outgoing-body` resource, and ensures that the user of this interface\n  /// may not write to the body contents after the body has been finished.\n  ///\n  /// If the user code drops this resource, as opposed to calling the static\n  /// method `finish`, the implementation should treat the body as incomplete,\n  /// and that an error has occured. The implementation should propogate this\n  /// error to the HTTP protocol by whatever means it has available,\n  /// including: corrupting the body on the wire, aborting the associated\n  /// Request, or sending a late status code for the Response.\n  resource outgoing-body {\n\n    /// Returns a stream for writing the body contents.\n    ///\n    /// The returned `output-stream` is a child resource: it must be dropped\n    /// before the parent `outgoing-body` resource is dropped (or finished),\n    /// otherwise the `outgoing-body` drop or `finish` will trap.\n    ///\n    /// Returns success on the first call: the `output-stream` resource for\n    /// this `outgoing-body` may be retrieved at most once. Subsequent calls\n    /// will return error.\n    write: func() -> result<output-stream>;\n\n    /// Finalize an outgoing body, optionally providing trailers. This must be\n    /// called to signal that the response is complete. If the `outgoing-body`\n    /// is dropped without calling `outgoing-body.finalize`, the implementation\n    /// should treat the body as corrupted.\n    ///\n    /// Fails if the body\'s `outgoing-request` or `outgoing-response` was\n    /// constructed with a Content-Length header, and the contents written\n    /// to the body (via `write`) does not match the value given in the\n    /// Content-Length.\n    finish: static func(\n      this: outgoing-body,\n      trailers: option<trailers>\n    ) -> result<_, error-code>;\n  }\n\n  /// Represents a future which may eventaully return an incoming HTTP\n  /// Response, or an error.\n  ///\n  /// This resource is returned by the `wasi:http/outgoing-handler` interface to\n  /// provide the HTTP Response corresponding to the sent Request.\n  resource future-incoming-response {\n    /// Returns a pollable which becomes ready when either the Response has\n    /// been received, or an error has occured. When this pollable is ready,\n    /// the `get` method will return `some`.\n    subscribe: func() -> pollable;\n\n    /// Returns the incoming HTTP Response, or an error, once one is ready.\n    ///\n    /// The outer `option` represents future readiness. Users can wait on this\n    /// `option` to become `some` using the `subscribe` method.\n    ///\n    /// The outer `result` is used to retrieve the response or error at most\n    /// once. It will be success on the first call in which the outer option\n    /// is `some`, and error on subsequent calls.\n    ///\n    /// The inner `result` represents that either the incoming HTTP Response\n    /// status and headers have recieved successfully, or that an error\n    /// occured. Errors may also occur while consuming the response body,\n    /// but those will be reported by the `incoming-body` and its\n    /// `output-stream` child.\n    get: func() -> option<result<result<incoming-response, error-code>>>;\n\n  }\n}\n";
        const _: &[u8] = b"package fermyon:spin;\n\nworld redis-trigger {\n  export inbound-redis;\n}\n\nworld wasi-http-trigger {\n  import wasi:http/outgoing-handler@0.2.0;\n  export wasi:http/incoming-handler@0.2.0;\n}\n";
        const _: &[u8] = b"package wasi:io@0.2.0;\n\nworld imports {\n    import streams;\n    import poll;\n}\n";
        const _: &[u8] = b"package wasi:filesystem@0.2.0;\n/// WASI filesystem is a filesystem API primarily intended to let users run WASI\n/// programs that access their files on their existing filesystems, without\n/// significant overhead.\n///\n/// It is intended to be roughly portable between Unix-family platforms and\n/// Windows, though it does not hide many of the major differences.\n///\n/// Paths are passed as interface-type `string`s, meaning they must consist of\n/// a sequence of Unicode Scalar Values (USVs). Some filesystems may contain\n/// paths which are not accessible by this API.\n///\n/// The directory separator in WASI is always the forward-slash (`/`).\n///\n/// All paths in WASI are relative paths, and are interpreted relative to a\n/// `descriptor` referring to a base directory. If a `path` argument to any WASI\n/// function starts with `/`, or if any step of resolving a `path`, including\n/// `..` and symbolic link steps, reaches a directory outside of the base\n/// directory, or reaches a symlink to an absolute or rooted path in the\n/// underlying filesystem, the function fails with `error-code::not-permitted`.\n///\n/// For more information about WASI path resolution and sandboxing, see\n/// [WASI filesystem path resolution].\n///\n/// [WASI filesystem path resolution]: https://github.com/WebAssembly/wasi-filesystem/blob/main/path-resolution.md\ninterface types {\n    use wasi:io/streams@0.2.0.{input-stream, output-stream, error};\n    use wasi:clocks/wall-clock@0.2.0.{datetime};\n\n    /// File size or length of a region within a file.\n    type filesize = u64;\n\n    /// The type of a filesystem object referenced by a descriptor.\n    ///\n    /// Note: This was called `filetype` in earlier versions of WASI.\n    enum descriptor-type {\n        /// The type of the descriptor or file is unknown or is different from\n        /// any of the other types specified.\n        unknown,\n        /// The descriptor refers to a block device inode.\n        block-device,\n        /// The descriptor refers to a character device inode.\n        character-device,\n        /// The descriptor refers to a directory inode.\n        directory,\n        /// The descriptor refers to a named pipe.\n        fifo,\n        /// The file refers to a symbolic link inode.\n        symbolic-link,\n        /// The descriptor refers to a regular file inode.\n        regular-file,\n        /// The descriptor refers to a socket.\n        socket,\n    }\n\n    /// Descriptor flags.\n    ///\n    /// Note: This was called `fdflags` in earlier versions of WASI.\n    flags descriptor-flags {\n        /// Read mode: Data can be read.\n        read,\n        /// Write mode: Data can be written to.\n        write,\n        /// Request that writes be performed according to synchronized I/O file\n        /// integrity completion. The data stored in the file and the file\'s\n        /// metadata are synchronized. This is similar to `O_SYNC` in POSIX.\n        ///\n        /// The precise semantics of this operation have not yet been defined for\n        /// WASI. At this time, it should be interpreted as a request, and not a\n        /// requirement.\n        file-integrity-sync,\n        /// Request that writes be performed according to synchronized I/O data\n        /// integrity completion. Only the data stored in the file is\n        /// synchronized. This is similar to `O_DSYNC` in POSIX.\n        ///\n        /// The precise semantics of this operation have not yet been defined for\n        /// WASI. At this time, it should be interpreted as a request, and not a\n        /// requirement.\n        data-integrity-sync,\n        /// Requests that reads be performed at the same level of integrety\n        /// requested for writes. This is similar to `O_RSYNC` in POSIX.\n        ///\n        /// The precise semantics of this operation have not yet been defined for\n        /// WASI. At this time, it should be interpreted as a request, and not a\n        /// requirement.\n        requested-write-sync,\n        /// Mutating directories mode: Directory contents may be mutated.\n        ///\n        /// When this flag is unset on a descriptor, operations using the\n        /// descriptor which would create, rename, delete, modify the data or\n        /// metadata of filesystem objects, or obtain another handle which\n        /// would permit any of those, shall fail with `error-code::read-only` if\n        /// they would otherwise succeed.\n        ///\n        /// This may only be set on directories.\n        mutate-directory,\n    }\n\n    /// File attributes.\n    ///\n    /// Note: This was called `filestat` in earlier versions of WASI.\n    record descriptor-stat {\n        /// File type.\n        %type: descriptor-type,\n        /// Number of hard links to the file.\n        link-count: link-count,\n        /// For regular files, the file size in bytes. For symbolic links, the\n        /// length in bytes of the pathname contained in the symbolic link.\n        size: filesize,\n        /// Last data access timestamp.\n        ///\n        /// If the `option` is none, the platform doesn\'t maintain an access\n        /// timestamp for this file.\n        data-access-timestamp: option<datetime>,\n        /// Last data modification timestamp.\n        ///\n        /// If the `option` is none, the platform doesn\'t maintain a\n        /// modification timestamp for this file.\n        data-modification-timestamp: option<datetime>,\n        /// Last file status-change timestamp.\n        ///\n        /// If the `option` is none, the platform doesn\'t maintain a\n        /// status-change timestamp for this file.\n        status-change-timestamp: option<datetime>,\n    }\n\n    /// Flags determining the method of how paths are resolved.\n    flags path-flags {\n        /// As long as the resolved path corresponds to a symbolic link, it is\n        /// expanded.\n        symlink-follow,\n    }\n\n    /// Open flags used by `open-at`.\n    flags open-flags {\n        /// Create file if it does not exist, similar to `O_CREAT` in POSIX.\n        create,\n        /// Fail if not a directory, similar to `O_DIRECTORY` in POSIX.\n        directory,\n        /// Fail if file already exists, similar to `O_EXCL` in POSIX.\n        exclusive,\n        /// Truncate file to size 0, similar to `O_TRUNC` in POSIX.\n        truncate,\n    }\n\n    /// Number of hard links to an inode.\n    type link-count = u64;\n\n    /// When setting a timestamp, this gives the value to set it to.\n    variant new-timestamp {\n        /// Leave the timestamp set to its previous value.\n        no-change,\n        /// Set the timestamp to the current time of the system clock associated\n        /// with the filesystem.\n        now,\n        /// Set the timestamp to the given value.\n        timestamp(datetime),\n    }\n\n    /// A directory entry.\n    record directory-entry {\n        /// The type of the file referred to by this directory entry.\n        %type: descriptor-type,\n\n        /// The name of the object.\n        name: string,\n    }\n\n    /// Error codes returned by functions, similar to `errno` in POSIX.\n    /// Not all of these error codes are returned by the functions provided by this\n    /// API; some are used in higher-level library layers, and others are provided\n    /// merely for alignment with POSIX.\n    enum error-code {\n        /// Permission denied, similar to `EACCES` in POSIX.\n        access,\n        /// Resource unavailable, or operation would block, similar to `EAGAIN` and `EWOULDBLOCK` in POSIX.\n        would-block,\n        /// Connection already in progress, similar to `EALREADY` in POSIX.\n        already,\n        /// Bad descriptor, similar to `EBADF` in POSIX.\n        bad-descriptor,\n        /// Device or resource busy, similar to `EBUSY` in POSIX.\n        busy,\n        /// Resource deadlock would occur, similar to `EDEADLK` in POSIX.\n        deadlock,\n        /// Storage quota exceeded, similar to `EDQUOT` in POSIX.\n        quota,\n        /// File exists, similar to `EEXIST` in POSIX.\n        exist,\n        /// File too large, similar to `EFBIG` in POSIX.\n        file-too-large,\n        /// Illegal byte sequence, similar to `EILSEQ` in POSIX.\n        illegal-byte-sequence,\n        /// Operation in progress, similar to `EINPROGRESS` in POSIX.\n        in-progress,\n        /// Interrupted function, similar to `EINTR` in POSIX.\n        interrupted,\n        /// Invalid argument, similar to `EINVAL` in POSIX.\n        invalid,\n        /// I/O error, similar to `EIO` in POSIX.\n        io,\n        /// Is a directory, similar to `EISDIR` in POSIX.\n        is-directory,\n        /// Too many levels of symbolic links, similar to `ELOOP` in POSIX.\n        loop,\n        /// Too many links, similar to `EMLINK` in POSIX.\n        too-many-links,\n        /// Message too large, similar to `EMSGSIZE` in POSIX.\n        message-size,\n        /// Filename too long, similar to `ENAMETOOLONG` in POSIX.\n        name-too-long,\n        /// No such device, similar to `ENODEV` in POSIX.\n        no-device,\n        /// No such file or directory, similar to `ENOENT` in POSIX.\n        no-entry,\n        /// No locks available, similar to `ENOLCK` in POSIX.\n        no-lock,\n        /// Not enough space, similar to `ENOMEM` in POSIX.\n        insufficient-memory,\n        /// No space left on device, similar to `ENOSPC` in POSIX.\n        insufficient-space,\n        /// Not a directory or a symbolic link to a directory, similar to `ENOTDIR` in POSIX.\n        not-directory,\n        /// Directory not empty, similar to `ENOTEMPTY` in POSIX.\n        not-empty,\n        /// State not recoverable, similar to `ENOTRECOVERABLE` in POSIX.\n        not-recoverable,\n        /// Not supported, similar to `ENOTSUP` and `ENOSYS` in POSIX.\n        unsupported,\n        /// Inappropriate I/O control operation, similar to `ENOTTY` in POSIX.\n        no-tty,\n        /// No such device or address, similar to `ENXIO` in POSIX.\n        no-such-device,\n        /// Value too large to be stored in data type, similar to `EOVERFLOW` in POSIX.\n        overflow,\n        /// Operation not permitted, similar to `EPERM` in POSIX.\n        not-permitted,\n        /// Broken pipe, similar to `EPIPE` in POSIX.\n        pipe,\n        /// Read-only file system, similar to `EROFS` in POSIX.\n        read-only,\n        /// Invalid seek, similar to `ESPIPE` in POSIX.\n        invalid-seek,\n        /// Text file busy, similar to `ETXTBSY` in POSIX.\n        text-file-busy,\n        /// Cross-device link, similar to `EXDEV` in POSIX.\n        cross-device,\n    }\n\n    /// File or memory access pattern advisory information.\n    enum advice {\n        /// The application has no advice to give on its behavior with respect\n        /// to the specified data.\n        normal,\n        /// The application expects to access the specified data sequentially\n        /// from lower offsets to higher offsets.\n        sequential,\n        /// The application expects to access the specified data in a random\n        /// order.\n        random,\n        /// The application expects to access the specified data in the near\n        /// future.\n        will-need,\n        /// The application expects that it will not access the specified data\n        /// in the near future.\n        dont-need,\n        /// The application expects to access the specified data once and then\n        /// not reuse it thereafter.\n        no-reuse,\n    }\n\n    /// A 128-bit hash value, split into parts because wasm doesn\'t have a\n    /// 128-bit integer type.\n    record metadata-hash-value {\n       /// 64 bits of a 128-bit hash value.\n       lower: u64,\n       /// Another 64 bits of a 128-bit hash value.\n       upper: u64,\n    }\n\n    /// A descriptor is a reference to a filesystem object, which may be a file,\n    /// directory, named pipe, special file, or other object on which filesystem\n    /// calls may be made.\n    resource descriptor {\n        /// Return a stream for reading from a file, if available.\n        ///\n        /// May fail with an error-code describing why the file cannot be read.\n        ///\n        /// Multiple read, write, and append streams may be active on the same open\n        /// file and they do not interfere with each other.\n        ///\n        /// Note: This allows using `read-stream`, which is similar to `read` in POSIX.\n        read-via-stream: func(\n            /// The offset within the file at which to start reading.\n            offset: filesize,\n        ) -> result<input-stream, error-code>;\n\n        /// Return a stream for writing to a file, if available.\n        ///\n        /// May fail with an error-code describing why the file cannot be written.\n        ///\n        /// Note: This allows using `write-stream`, which is similar to `write` in\n        /// POSIX.\n        write-via-stream: func(\n            /// The offset within the file at which to start writing.\n            offset: filesize,\n        ) -> result<output-stream, error-code>;\n\n        /// Return a stream for appending to a file, if available.\n        ///\n        /// May fail with an error-code describing why the file cannot be appended.\n        ///\n        /// Note: This allows using `write-stream`, which is similar to `write` with\n        /// `O_APPEND` in in POSIX.\n        append-via-stream: func() -> result<output-stream, error-code>;\n\n        /// Provide file advisory information on a descriptor.\n        ///\n        /// This is similar to `posix_fadvise` in POSIX.\n        advise: func(\n            /// The offset within the file to which the advisory applies.\n            offset: filesize,\n            /// The length of the region to which the advisory applies.\n            length: filesize,\n            /// The advice.\n            advice: advice\n        ) -> result<_, error-code>;\n\n        /// Synchronize the data of a file to disk.\n        ///\n        /// This function succeeds with no effect if the file descriptor is not\n        /// opened for writing.\n        ///\n        /// Note: This is similar to `fdatasync` in POSIX.\n        sync-data: func() -> result<_, error-code>;\n\n        /// Get flags associated with a descriptor.\n        ///\n        /// Note: This returns similar flags to `fcntl(fd, F_GETFL)` in POSIX.\n        ///\n        /// Note: This returns the value that was the `fs_flags` value returned\n        /// from `fdstat_get` in earlier versions of WASI.\n        get-flags: func() -> result<descriptor-flags, error-code>;\n\n        /// Get the dynamic type of a descriptor.\n        ///\n        /// Note: This returns the same value as the `type` field of the `fd-stat`\n        /// returned by `stat`, `stat-at` and similar.\n        ///\n        /// Note: This returns similar flags to the `st_mode & S_IFMT` value provided\n        /// by `fstat` in POSIX.\n        ///\n        /// Note: This returns the value that was the `fs_filetype` value returned\n        /// from `fdstat_get` in earlier versions of WASI.\n        get-type: func() -> result<descriptor-type, error-code>;\n\n        /// Adjust the size of an open file. If this increases the file\'s size, the\n        /// extra bytes are filled with zeros.\n        ///\n        /// Note: This was called `fd_filestat_set_size` in earlier versions of WASI.\n        set-size: func(size: filesize) -> result<_, error-code>;\n\n        /// Adjust the timestamps of an open file or directory.\n        ///\n        /// Note: This is similar to `futimens` in POSIX.\n        ///\n        /// Note: This was called `fd_filestat_set_times` in earlier versions of WASI.\n        set-times: func(\n            /// The desired values of the data access timestamp.\n            data-access-timestamp: new-timestamp,\n            /// The desired values of the data modification timestamp.\n            data-modification-timestamp: new-timestamp,\n        ) -> result<_, error-code>;\n\n        /// Read from a descriptor, without using and updating the descriptor\'s offset.\n        ///\n        /// This function returns a list of bytes containing the data that was\n        /// read, along with a bool which, when true, indicates that the end of the\n        /// file was reached. The returned list will contain up to `length` bytes; it\n        /// may return fewer than requested, if the end of the file is reached or\n        /// if the I/O operation is interrupted.\n        ///\n        /// In the future, this may change to return a `stream<u8, error-code>`.\n        ///\n        /// Note: This is similar to `pread` in POSIX.\n        read: func(\n            /// The maximum number of bytes to read.\n            length: filesize,\n            /// The offset within the file at which to read.\n            offset: filesize,\n        ) -> result<tuple<list<u8>, bool>, error-code>;\n\n        /// Write to a descriptor, without using and updating the descriptor\'s offset.\n        ///\n        /// It is valid to write past the end of a file; the file is extended to the\n        /// extent of the write, with bytes between the previous end and the start of\n        /// the write set to zero.\n        ///\n        /// In the future, this may change to take a `stream<u8, error-code>`.\n        ///\n        /// Note: This is similar to `pwrite` in POSIX.\n        write: func(\n            /// Data to write\n            buffer: list<u8>,\n            /// The offset within the file at which to write.\n            offset: filesize,\n        ) -> result<filesize, error-code>;\n\n        /// Read directory entries from a directory.\n        ///\n        /// On filesystems where directories contain entries referring to themselves\n        /// and their parents, often named `.` and `..` respectively, these entries\n        /// are omitted.\n        ///\n        /// This always returns a new stream which starts at the beginning of the\n        /// directory. Multiple streams may be active on the same directory, and they\n        /// do not interfere with each other.\n        read-directory: func() -> result<directory-entry-stream, error-code>;\n\n        /// Synchronize the data and metadata of a file to disk.\n        ///\n        /// This function succeeds with no effect if the file descriptor is not\n        /// opened for writing.\n        ///\n        /// Note: This is similar to `fsync` in POSIX.\n        sync: func() -> result<_, error-code>;\n\n        /// Create a directory.\n        ///\n        /// Note: This is similar to `mkdirat` in POSIX.\n        create-directory-at: func(\n            /// The relative path at which to create the directory.\n            path: string,\n        ) -> result<_, error-code>;\n\n        /// Return the attributes of an open file or directory.\n        ///\n        /// Note: This is similar to `fstat` in POSIX, except that it does not return\n        /// device and inode information. For testing whether two descriptors refer to\n        /// the same underlying filesystem object, use `is-same-object`. To obtain\n        /// additional data that can be used do determine whether a file has been\n        /// modified, use `metadata-hash`.\n        ///\n        /// Note: This was called `fd_filestat_get` in earlier versions of WASI.\n        stat: func() -> result<descriptor-stat, error-code>;\n\n        /// Return the attributes of a file or directory.\n        ///\n        /// Note: This is similar to `fstatat` in POSIX, except that it does not\n        /// return device and inode information. See the `stat` description for a\n        /// discussion of alternatives.\n        ///\n        /// Note: This was called `path_filestat_get` in earlier versions of WASI.\n        stat-at: func(\n            /// Flags determining the method of how the path is resolved.\n            path-flags: path-flags,\n            /// The relative path of the file or directory to inspect.\n            path: string,\n        ) -> result<descriptor-stat, error-code>;\n\n        /// Adjust the timestamps of a file or directory.\n        ///\n        /// Note: This is similar to `utimensat` in POSIX.\n        ///\n        /// Note: This was called `path_filestat_set_times` in earlier versions of\n        /// WASI.\n        set-times-at: func(\n            /// Flags determining the method of how the path is resolved.\n            path-flags: path-flags,\n            /// The relative path of the file or directory to operate on.\n            path: string,\n            /// The desired values of the data access timestamp.\n            data-access-timestamp: new-timestamp,\n            /// The desired values of the data modification timestamp.\n            data-modification-timestamp: new-timestamp,\n        ) -> result<_, error-code>;\n\n        /// Create a hard link.\n        ///\n        /// Note: This is similar to `linkat` in POSIX.\n        link-at: func(\n            /// Flags determining the method of how the path is resolved.\n            old-path-flags: path-flags,\n            /// The relative source path from which to link.\n            old-path: string,\n            /// The base directory for `new-path`.\n            new-descriptor: borrow<descriptor>,\n            /// The relative destination path at which to create the hard link.\n            new-path: string,\n        ) -> result<_, error-code>;\n\n        /// Open a file or directory.\n        ///\n        /// The returned descriptor is not guaranteed to be the lowest-numbered\n        /// descriptor not currently open/ it is randomized to prevent applications\n        /// from depending on making assumptions about indexes, since this is\n        /// error-prone in multi-threaded contexts. The returned descriptor is\n        /// guaranteed to be less than 2**31.\n        ///\n        /// If `flags` contains `descriptor-flags::mutate-directory`, and the base\n        /// descriptor doesn\'t have `descriptor-flags::mutate-directory` set,\n        /// `open-at` fails with `error-code::read-only`.\n        ///\n        /// If `flags` contains `write` or `mutate-directory`, or `open-flags`\n        /// contains `truncate` or `create`, and the base descriptor doesn\'t have\n        /// `descriptor-flags::mutate-directory` set, `open-at` fails with\n        /// `error-code::read-only`.\n        ///\n        /// Note: This is similar to `openat` in POSIX.\n        open-at: func(\n            /// Flags determining the method of how the path is resolved.\n            path-flags: path-flags,\n            /// The relative path of the object to open.\n            path: string,\n            /// The method by which to open the file.\n            open-flags: open-flags,\n            /// Flags to use for the resulting descriptor.\n            %flags: descriptor-flags,\n        ) -> result<descriptor, error-code>;\n\n        /// Read the contents of a symbolic link.\n        ///\n        /// If the contents contain an absolute or rooted path in the underlying\n        /// filesystem, this function fails with `error-code::not-permitted`.\n        ///\n        /// Note: This is similar to `readlinkat` in POSIX.\n        readlink-at: func(\n            /// The relative path of the symbolic link from which to read.\n            path: string,\n        ) -> result<string, error-code>;\n\n        /// Remove a directory.\n        ///\n        /// Return `error-code::not-empty` if the directory is not empty.\n        ///\n        /// Note: This is similar to `unlinkat(fd, path, AT_REMOVEDIR)` in POSIX.\n        remove-directory-at: func(\n            /// The relative path to a directory to remove.\n            path: string,\n        ) -> result<_, error-code>;\n\n        /// Rename a filesystem object.\n        ///\n        /// Note: This is similar to `renameat` in POSIX.\n        rename-at: func(\n            /// The relative source path of the file or directory to rename.\n            old-path: string,\n            /// The base directory for `new-path`.\n            new-descriptor: borrow<descriptor>,\n            /// The relative destination path to which to rename the file or directory.\n            new-path: string,\n        ) -> result<_, error-code>;\n\n        /// Create a symbolic link (also known as a \"symlink\").\n        ///\n        /// If `old-path` starts with `/`, the function fails with\n        /// `error-code::not-permitted`.\n        ///\n        /// Note: This is similar to `symlinkat` in POSIX.\n        symlink-at: func(\n            /// The contents of the symbolic link.\n            old-path: string,\n            /// The relative destination path at which to create the symbolic link.\n            new-path: string,\n        ) -> result<_, error-code>;\n\n        /// Unlink a filesystem object that is not a directory.\n        ///\n        /// Return `error-code::is-directory` if the path refers to a directory.\n        /// Note: This is similar to `unlinkat(fd, path, 0)` in POSIX.\n        unlink-file-at: func(\n            /// The relative path to a file to unlink.\n            path: string,\n        ) -> result<_, error-code>;\n\n        /// Test whether two descriptors refer to the same filesystem object.\n        ///\n        /// In POSIX, this corresponds to testing whether the two descriptors have the\n        /// same device (`st_dev`) and inode (`st_ino` or `d_ino`) numbers.\n        /// wasi-filesystem does not expose device and inode numbers, so this function\n        /// may be used instead.\n        is-same-object: func(other: borrow<descriptor>) -> bool;\n\n        /// Return a hash of the metadata associated with a filesystem object referred\n        /// to by a descriptor.\n        ///\n        /// This returns a hash of the last-modification timestamp and file size, and\n        /// may also include the inode number, device number, birth timestamp, and\n        /// other metadata fields that may change when the file is modified or\n        /// replaced. It may also include a secret value chosen by the\n        /// implementation and not otherwise exposed.\n        ///\n        /// Implementations are encourated to provide the following properties:\n        ///\n        ///  - If the file is not modified or replaced, the computed hash value should\n        ///    usually not change.\n        ///  - If the object is modified or replaced, the computed hash value should\n        ///    usually change.\n        ///  - The inputs to the hash should not be easily computable from the\n        ///    computed hash.\n        ///\n        /// However, none of these is required.\n        metadata-hash: func() -> result<metadata-hash-value, error-code>;\n\n        /// Return a hash of the metadata associated with a filesystem object referred\n        /// to by a directory descriptor and a relative path.\n        ///\n        /// This performs the same hash computation as `metadata-hash`.\n        metadata-hash-at: func(\n            /// Flags determining the method of how the path is resolved.\n            path-flags: path-flags,\n            /// The relative path of the file or directory to inspect.\n            path: string,\n        ) -> result<metadata-hash-value, error-code>;\n    }\n\n    /// A stream of directory entries.\n    resource directory-entry-stream {\n        /// Read a single directory entry from a `directory-entry-stream`.\n        read-directory-entry: func() -> result<option<directory-entry>, error-code>;\n    }\n\n    /// Attempts to extract a filesystem-related `error-code` from the stream\n    /// `error` provided.\n    ///\n    /// Stream operations which return `stream-error::last-operation-failed`\n    /// have a payload with more information about the operation that failed.\n    /// This payload can be passed through to this function to see if there\'s\n    /// filesystem-related information about the error to return.\n    ///\n    /// Note that this function is fallible because not all stream-related\n    /// errors are filesystem-related errors.\n    filesystem-error-code: func(err: borrow<error>) -> option<error-code>;\n}\n";
        const _: &[u8] = b"\n/// This interface provides a value-export of the default network handle..\ninterface instance-network {\n    use network.{network};\n\n    /// Get a handle to the default network.\n    instance-network: func() -> network;\n\n}\n";
        const _: &[u8] = b"package wasi:random@0.2.0;\n/// WASI Random is a random data API.\n///\n/// It is intended to be portable at least between Unix-family platforms and\n/// Windows.\ninterface random {\n    /// Return `len` cryptographically-secure random or pseudo-random bytes.\n    ///\n    /// This function must produce data at least as cryptographically secure and\n    /// fast as an adequately seeded cryptographically-secure pseudo-random\n    /// number generator (CSPRNG). It must not block, from the perspective of\n    /// the calling program, under any circumstances, including on the first\n    /// request and on requests for numbers of bytes. The returned data must\n    /// always be unpredictable.\n    ///\n    /// This function must always return fresh data. Deterministic environments\n    /// must omit this function, rather than implementing it with deterministic\n    /// data.\n    get-random-bytes: func(len: u64) -> list<u8>;\n\n    /// Return a cryptographically-secure random or pseudo-random `u64` value.\n    ///\n    /// This function returns the same type of data as `get-random-bytes`,\n    /// represented as a `u64`.\n    get-random-u64: func() -> u64;\n}\n";
        const _: &[u8] = b"\ninterface ip-name-lookup {\n    use wasi:io/poll@0.2.0.{pollable};\n    use network.{network, error-code, ip-address};\n\n\n    /// Resolve an internet host name to a list of IP addresses.\n    ///\n    /// Unicode domain names are automatically converted to ASCII using IDNA encoding.\n    /// If the input is an IP address string, the address is parsed and returned\n    /// as-is without making any external requests.\n    ///\n    /// See the wasi-socket proposal README.md for a comparison with getaddrinfo.\n    ///\n    /// This function never blocks. It either immediately fails or immediately\n    /// returns successfully with a `resolve-address-stream` that can be used\n    /// to (asynchronously) fetch the results.\n    ///\n    /// # Typical errors\n    /// - `invalid-argument`: `name` is a syntactically invalid domain name or IP address.\n    ///\n    /// # References:\n    /// - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/getaddrinfo.html>\n    /// - <https://man7.org/linux/man-pages/man3/getaddrinfo.3.html>\n    /// - <https://learn.microsoft.com/en-us/windows/win32/api/ws2tcpip/nf-ws2tcpip-getaddrinfo>\n    /// - <https://man.freebsd.org/cgi/man.cgi?query=getaddrinfo&sektion=3>\n    resolve-addresses: func(network: borrow<network>, name: string) -> result<resolve-address-stream, error-code>;\n\n    resource resolve-address-stream {\n        /// Returns the next address from the resolver.\n        ///\n        /// This function should be called multiple times. On each call, it will\n        /// return the next address in connection order preference. If all\n        /// addresses have been exhausted, this function returns `none`.\n        ///\n        /// This function never returns IPv4-mapped IPv6 addresses.\n        ///\n        /// # Typical errors\n        /// - `name-unresolvable`:          Name does not exist or has no suitable associated IP addresses. (EAI_NONAME, EAI_NODATA, EAI_ADDRFAMILY)\n        /// - `temporary-resolver-failure`: A temporary failure in name resolution occurred. (EAI_AGAIN)\n        /// - `permanent-resolver-failure`: A permanent failure in name resolution occurred. (EAI_FAIL)\n        /// - `would-block`:                A result is not available yet. (EWOULDBLOCK, EAGAIN)\n        resolve-next-address: func() -> result<option<ip-address>, error-code>;\n\n        /// Create a `pollable` which will resolve once the stream is ready for I/O.\n        ///\n        /// Note: this function is here for WASI Preview2 only.\n        /// It\'s planned to be removed when `future` is natively supported in Preview3.\n        subscribe: func() -> pollable;\n    }\n}\n";
        const _: &[u8] = b"interface inbound-redis {\n  use redis-types.{payload, error};\n\n  // The entrypoint for a Redis handler.\n  handle-message: func(message: payload) -> result<_, error>;\n}\n";
        const _: &[u8] = b"/// Terminal input.\n///\n/// In the future, this may include functions for disabling echoing,\n/// disabling input buffering so that keyboard events are sent through\n/// immediately, querying supported features, and so on.\ninterface terminal-input {\n    /// The input side of a terminal.\n    resource terminal-input;\n}\n\n/// Terminal output.\n///\n/// In the future, this may include functions for querying the terminal\n/// size, being notified of terminal size changes, querying supported\n/// features, and so on.\ninterface terminal-output {\n    /// The output side of a terminal.\n    resource terminal-output;\n}\n\n/// An interface providing an optional `terminal-input` for stdin as a\n/// link-time authority.\ninterface terminal-stdin {\n    use terminal-input.{terminal-input};\n\n    /// If stdin is connected to a terminal, return a `terminal-input` handle\n    /// allowing further interaction with it.\n    get-terminal-stdin: func() -> option<terminal-input>;\n}\n\n/// An interface providing an optional `terminal-output` for stdout as a\n/// link-time authority.\ninterface terminal-stdout {\n    use terminal-output.{terminal-output};\n\n    /// If stdout is connected to a terminal, return a `terminal-output` handle\n    /// allowing further interaction with it.\n    get-terminal-stdout: func() -> option<terminal-output>;\n}\n\n/// An interface providing an optional `terminal-output` for stderr as a\n/// link-time authority.\ninterface terminal-stderr {\n    use terminal-output.{terminal-output};\n\n    /// If stderr is connected to a terminal, return a `terminal-output` handle\n    /// allowing further interaction with it.\n    get-terminal-stderr: func() -> option<terminal-output>;\n}\n";
        const _: &[u8] = b"package wasi:clocks@0.2.0;\n/// WASI Wall Clock is a clock API intended to let users query the current\n/// time. The name \"wall\" makes an analogy to a \"clock on the wall\", which\n/// is not necessarily monotonic as it may be reset.\n///\n/// It is intended to be portable at least between Unix-family platforms and\n/// Windows.\n///\n/// A wall clock is a clock which measures the date and time according to\n/// some external reference.\n///\n/// External references may be reset, so this clock is not necessarily\n/// monotonic, making it unsuitable for measuring elapsed time.\n///\n/// It is intended for reporting the current date and time for humans.\ninterface wall-clock {\n    /// A time and date in seconds plus nanoseconds.\n    record datetime {\n        seconds: u64,\n        nanoseconds: u32,\n    }\n\n    /// Read the current value of the clock.\n    ///\n    /// This clock is not monotonic, therefore calling this function repeatedly\n    /// will not necessarily produce a sequence of non-decreasing values.\n    ///\n    /// The returned timestamps represent the number of seconds since\n    /// 1970-01-01T00:00:00Z, also known as [POSIX\'s Seconds Since the Epoch],\n    /// also known as [Unix Time].\n    ///\n    /// The nanoseconds field of the output is always less than 1000000000.\n    ///\n    /// [POSIX\'s Seconds Since the Epoch]: https://pubs.opengroup.org/onlinepubs/9699919799/xrat/V4_xbd_chap04.html#tag_21_04_16\n    /// [Unix Time]: https://en.wikipedia.org/wiki/Unix_time\n    now: func() -> datetime;\n\n    /// Query the resolution of the clock.\n    ///\n    /// The nanoseconds field of the output is always less than 1000000000.\n    resolution: func() -> datetime;\n}\n";
        const _: &[u8] = b"/// This interface defines a handler of incoming HTTP Requests. It should\n/// be exported by components which can respond to HTTP Requests.\ninterface incoming-handler {\n  use types.{incoming-request, response-outparam};\n\n  /// This function is invoked with an incoming HTTP Request, and a resource\n  /// `response-outparam` which provides the capability to reply with an HTTP\n  /// Response. The response is sent by calling the `response-outparam.set`\n  /// method, which allows execution to continue after the response has been\n  /// sent. This enables both streaming to the response body, and performing other\n  /// work.\n  ///\n  /// The implementor of this function must write a response to the\n  /// `response-outparam` before returning, or else the caller will respond\n  /// with an error on its behalf.\n  handle: func(\n    request: incoming-request,\n    response-out: response-outparam\n  );\n}\n\n/// This interface defines a handler of outgoing HTTP Requests. It should be\n/// imported by components which wish to make HTTP Requests.\ninterface outgoing-handler {\n  use types.{\n    outgoing-request, request-options, future-incoming-response, error-code\n  };\n\n  /// This function is invoked with an outgoing HTTP Request, and it returns\n  /// a resource `future-incoming-response` which represents an HTTP Response\n  /// which may arrive in the future.\n  ///\n  /// The `options` argument accepts optional parameters for the HTTP\n  /// protocol\'s transport layer.\n  ///\n  /// This function may return an error if the `outgoing-request` is invalid\n  /// or not allowed to be made. Otherwise, protocol errors are reported\n  /// through the `future-incoming-response`.\n  handle: func(\n    request: outgoing-request,\n    options: option<request-options>\n  ) -> result<future-incoming-response, error-code>;\n}\n";
        const _: &[u8] = b"interface run {\n  /// Run the program.\n  run: func() -> result;\n}\n";
        const _: &[u8] = b"package wasi:clocks@0.2.0;\n\nworld imports {\n    import monotonic-clock;\n    import wall-clock;\n}\n";
        const _: &[u8] = b"package wasi:random@0.2.0;\n/// The insecure-seed interface for seeding hash-map DoS resistance.\n///\n/// It is intended to be portable at least between Unix-family platforms and\n/// Windows.\ninterface insecure-seed {\n    /// Return a 128-bit value that may contain a pseudo-random value.\n    ///\n    /// The returned value is not required to be computed from a CSPRNG, and may\n    /// even be entirely deterministic. Host implementations are encouraged to\n    /// provide pseudo-random values to any program exposed to\n    /// attacker-controlled content, to enable DoS protection built into many\n    /// languages\' hash-map implementations.\n    ///\n    /// This function is intended to only be called once, by a source language\n    /// to initialize Denial Of Service (DoS) protection in its hash-map\n    /// implementation.\n    ///\n    /// # Expected future evolution\n    ///\n    /// This will likely be changed to a value import, to prevent it from being\n    /// called multiple times and potentially used for purposes other than DoS\n    /// protection.\n    insecure-seed: func() -> tuple<u64, u64>;\n}\n";
        const _: &[u8] = b"\ninterface tcp-create-socket {\n    use network.{network, error-code, ip-address-family};\n    use tcp.{tcp-socket};\n\n    /// Create a new TCP socket.\n    ///\n    /// Similar to `socket(AF_INET or AF_INET6, SOCK_STREAM, IPPROTO_TCP)` in POSIX.\n    /// On IPv6 sockets, IPV6_V6ONLY is enabled by default and can\'t be configured otherwise.\n    ///\n    /// This function does not require a network capability handle. This is considered to be safe because\n    /// at time of creation, the socket is not bound to any `network` yet. Up to the moment `bind`/`connect`\n    /// is called, the socket is effectively an in-memory configuration object, unable to communicate with the outside world.\n    ///\n    /// All sockets are non-blocking. Use the wasi-poll interface to block on asynchronous operations.\n    ///\n    /// # Typical errors\n    /// - `not-supported`:     The specified `address-family` is not supported. (EAFNOSUPPORT)\n    /// - `new-socket-limit`:  The new socket resource could not be created because of a system limit. (EMFILE, ENFILE)\n    ///\n    /// # References\n    /// - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/socket.html>\n    /// - <https://man7.org/linux/man-pages/man2/socket.2.html>\n    /// - <https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-wsasocketw>\n    /// - <https://man.freebsd.org/cgi/man.cgi?query=socket&sektion=2>\n    create-tcp-socket: func(address-family: ip-address-family) -> result<tcp-socket, error-code>;\n}\n";
        const _: &[u8] = b"interface exit {\n  /// Exit the current instance and any linked instances.\n  exit: func(status: result);\n}\n";
        const _: &[u8] = b"package wasi:io@0.2.0;\n\n/// WASI I/O is an I/O abstraction API which is currently focused on providing\n/// stream types.\n///\n/// In the future, the component model is expected to add built-in stream types;\n/// when it does, they are expected to subsume this API.\ninterface streams {\n    use error.{error};\n    use poll.{pollable};\n\n    /// An error for input-stream and output-stream operations.\n    variant stream-error {\n        /// The last operation (a write or flush) failed before completion.\n        ///\n        /// More information is available in the `error` payload.\n        last-operation-failed(error),\n        /// The stream is closed: no more input will be accepted by the\n        /// stream. A closed output-stream will return this error on all\n        /// future operations.\n        closed\n    }\n\n    /// An input bytestream.\n    ///\n    /// `input-stream`s are *non-blocking* to the extent practical on underlying\n    /// platforms. I/O operations always return promptly; if fewer bytes are\n    /// promptly available than requested, they return the number of bytes promptly\n    /// available, which could even be zero. To wait for data to be available,\n    /// use the `subscribe` function to obtain a `pollable` which can be polled\n    /// for using `wasi:io/poll`.\n    resource input-stream {\n        /// Perform a non-blocking read from the stream.\n        ///\n        /// When the source of a `read` is binary data, the bytes from the source\n        /// are returned verbatim. When the source of a `read` is known to the\n        /// implementation to be text, bytes containing the UTF-8 encoding of the\n        /// text are returned.\n        ///\n        /// This function returns a list of bytes containing the read data,\n        /// when successful. The returned list will contain up to `len` bytes;\n        /// it may return fewer than requested, but not more. The list is\n        /// empty when no bytes are available for reading at this time. The\n        /// pollable given by `subscribe` will be ready when more bytes are\n        /// available.\n        ///\n        /// This function fails with a `stream-error` when the operation\n        /// encounters an error, giving `last-operation-failed`, or when the\n        /// stream is closed, giving `closed`.\n        ///\n        /// When the caller gives a `len` of 0, it represents a request to\n        /// read 0 bytes. If the stream is still open, this call should\n        /// succeed and return an empty list, or otherwise fail with `closed`.\n        ///\n        /// The `len` parameter is a `u64`, which could represent a list of u8 which\n        /// is not possible to allocate in wasm32, or not desirable to allocate as\n        /// as a return value by the callee. The callee may return a list of bytes\n        /// less than `len` in size while more bytes are available for reading.\n        read: func(\n            /// The maximum number of bytes to read\n            len: u64\n        ) -> result<list<u8>, stream-error>;\n\n        /// Read bytes from a stream, after blocking until at least one byte can\n        /// be read. Except for blocking, behavior is identical to `read`.\n        blocking-read: func(\n            /// The maximum number of bytes to read\n            len: u64\n        ) -> result<list<u8>, stream-error>;\n\n        /// Skip bytes from a stream. Returns number of bytes skipped.\n        ///\n        /// Behaves identical to `read`, except instead of returning a list\n        /// of bytes, returns the number of bytes consumed from the stream.\n        skip: func(\n            /// The maximum number of bytes to skip.\n            len: u64,\n        ) -> result<u64, stream-error>;\n\n        /// Skip bytes from a stream, after blocking until at least one byte\n        /// can be skipped. Except for blocking behavior, identical to `skip`.\n        blocking-skip: func(\n            /// The maximum number of bytes to skip.\n            len: u64,\n        ) -> result<u64, stream-error>;\n\n        /// Create a `pollable` which will resolve once either the specified stream\n        /// has bytes available to read or the other end of the stream has been\n        /// closed.\n        /// The created `pollable` is a child resource of the `input-stream`.\n        /// Implementations may trap if the `input-stream` is dropped before\n        /// all derived `pollable`s created with this function are dropped.\n        subscribe: func() -> pollable;\n    }\n\n\n    /// An output bytestream.\n    ///\n    /// `output-stream`s are *non-blocking* to the extent practical on\n    /// underlying platforms. Except where specified otherwise, I/O operations also\n    /// always return promptly, after the number of bytes that can be written\n    /// promptly, which could even be zero. To wait for the stream to be ready to\n    /// accept data, the `subscribe` function to obtain a `pollable` which can be\n    /// polled for using `wasi:io/poll`.\n    resource output-stream {\n        /// Check readiness for writing. This function never blocks.\n        ///\n        /// Returns the number of bytes permitted for the next call to `write`,\n        /// or an error. Calling `write` with more bytes than this function has\n        /// permitted will trap.\n        ///\n        /// When this function returns 0 bytes, the `subscribe` pollable will\n        /// become ready when this function will report at least 1 byte, or an\n        /// error.\n        check-write: func() -> result<u64, stream-error>;\n\n        /// Perform a write. This function never blocks.\n        ///\n        /// When the destination of a `write` is binary data, the bytes from\n        /// `contents` are written verbatim. When the destination of a `write` is\n        /// known to the implementation to be text, the bytes of `contents` are\n        /// transcoded from UTF-8 into the encoding of the destination and then\n        /// written.\n        ///\n        /// Precondition: check-write gave permit of Ok(n) and contents has a\n        /// length of less than or equal to n. Otherwise, this function will trap.\n        ///\n        /// returns Err(closed) without writing if the stream has closed since\n        /// the last call to check-write provided a permit.\n        write: func(\n            contents: list<u8>\n        ) -> result<_, stream-error>;\n\n        /// Perform a write of up to 4096 bytes, and then flush the stream. Block\n        /// until all of these operations are complete, or an error occurs.\n        ///\n        /// This is a convenience wrapper around the use of `check-write`,\n        /// `subscribe`, `write`, and `flush`, and is implemented with the\n        /// following pseudo-code:\n        ///\n        /// ```text\n        /// let pollable = this.subscribe();\n        /// while !contents.is_empty() {\n        ///     // Wait for the stream to become writable\n        ///     pollable.block();\n        ///     let Ok(n) = this.check-write(); // eliding error handling\n        ///     let len = min(n, contents.len());\n        ///     let (chunk, rest) = contents.split_at(len);\n        ///     this.write(chunk  );            // eliding error handling\n        ///     contents = rest;\n        /// }\n        /// this.flush();\n        /// // Wait for completion of `flush`\n        /// pollable.block();\n        /// // Check for any errors that arose during `flush`\n        /// let _ = this.check-write();         // eliding error handling\n        /// ```\n        blocking-write-and-flush: func(\n            contents: list<u8>\n        ) -> result<_, stream-error>;\n\n        /// Request to flush buffered output. This function never blocks.\n        ///\n        /// This tells the output-stream that the caller intends any buffered\n        /// output to be flushed. the output which is expected to be flushed\n        /// is all that has been passed to `write` prior to this call.\n        ///\n        /// Upon calling this function, the `output-stream` will not accept any\n        /// writes (`check-write` will return `ok(0)`) until the flush has\n        /// completed. The `subscribe` pollable will become ready when the\n        /// flush has completed and the stream can accept more writes.\n        flush: func() -> result<_, stream-error>;\n\n        /// Request to flush buffered output, and block until flush completes\n        /// and stream is ready for writing again.\n        blocking-flush: func() -> result<_, stream-error>;\n\n        /// Create a `pollable` which will resolve once the output-stream\n        /// is ready for more writing, or an error has occured. When this\n        /// pollable is ready, `check-write` will return `ok(n)` with n>0, or an\n        /// error.\n        ///\n        /// If the stream is closed, this pollable is always ready immediately.\n        ///\n        /// The created `pollable` is a child resource of the `output-stream`.\n        /// Implementations may trap if the `output-stream` is dropped before\n        /// all derived `pollable`s created with this function are dropped.\n        subscribe: func() -> pollable;\n\n        /// Write zeroes to a stream.\n        ///\n        /// This should be used precisely like `write` with the exact same\n        /// preconditions (must use check-write first), but instead of\n        /// passing a list of bytes, you simply pass the number of zero-bytes\n        /// that should be written.\n        write-zeroes: func(\n            /// The number of zero-bytes to write\n            len: u64\n        ) -> result<_, stream-error>;\n\n        /// Perform a write of up to 4096 zeroes, and then flush the stream.\n        /// Block until all of these operations are complete, or an error\n        /// occurs.\n        ///\n        /// This is a convenience wrapper around the use of `check-write`,\n        /// `subscribe`, `write-zeroes`, and `flush`, and is implemented with\n        /// the following pseudo-code:\n        ///\n        /// ```text\n        /// let pollable = this.subscribe();\n        /// while num_zeroes != 0 {\n        ///     // Wait for the stream to become writable\n        ///     pollable.block();\n        ///     let Ok(n) = this.check-write(); // eliding error handling\n        ///     let len = min(n, num_zeroes);\n        ///     this.write-zeroes(len);         // eliding error handling\n        ///     num_zeroes -= len;\n        /// }\n        /// this.flush();\n        /// // Wait for completion of `flush`\n        /// pollable.block();\n        /// // Check for any errors that arose during `flush`\n        /// let _ = this.check-write();         // eliding error handling\n        /// ```\n        blocking-write-zeroes-and-flush: func(\n            /// The number of zero-bytes to write\n            len: u64\n        ) -> result<_, stream-error>;\n\n        /// Read from one stream and write to another.\n        ///\n        /// The behavior of splice is equivelant to:\n        /// 1. calling `check-write` on the `output-stream`\n        /// 2. calling `read` on the `input-stream` with the smaller of the\n        /// `check-write` permitted length and the `len` provided to `splice`\n        /// 3. calling `write` on the `output-stream` with that read data.\n        ///\n        /// Any error reported by the call to `check-write`, `read`, or\n        /// `write` ends the splice and reports that error.\n        ///\n        /// This function returns the number of bytes transferred; it may be less\n        /// than `len`.\n        splice: func(\n            /// The stream to read from\n            src: borrow<input-stream>,\n            /// The number of bytes to splice\n            len: u64,\n        ) -> result<u64, stream-error>;\n\n        /// Read from one stream and write to another, with blocking.\n        ///\n        /// This is similar to `splice`, except that it blocks until the\n        /// `output-stream` is ready for writing, and the `input-stream`\n        /// is ready for reading, before performing the `splice`.\n        blocking-splice: func(\n            /// The stream to read from\n            src: borrow<input-stream>,\n            /// The number of bytes to splice\n            len: u64,\n        ) -> result<u64, stream-error>;\n    }\n}\n";
        const _: &[u8] = b"package wasi:clocks@0.2.0;\n/// WASI Monotonic Clock is a clock API intended to let users measure elapsed\n/// time.\n///\n/// It is intended to be portable at least between Unix-family platforms and\n/// Windows.\n///\n/// A monotonic clock is a clock which has an unspecified initial value, and\n/// successive reads of the clock will produce non-decreasing values.\n///\n/// It is intended for measuring elapsed time.\ninterface monotonic-clock {\n    use wasi:io/poll@0.2.0.{pollable};\n\n    /// An instant in time, in nanoseconds. An instant is relative to an\n    /// unspecified initial value, and can only be compared to instances from\n    /// the same monotonic-clock.\n    type instant = u64;\n\n    /// A duration of time, in nanoseconds.\n    type duration = u64;\n\n    /// Read the current value of the clock.\n    ///\n    /// The clock is monotonic, therefore calling this function repeatedly will\n    /// produce a sequence of non-decreasing values.\n    now: func() -> instant;\n\n    /// Query the resolution of the clock. Returns the duration of time\n    /// corresponding to a clock tick.\n    resolution: func() -> duration;\n\n    /// Create a `pollable` which will resolve once the specified instant\n    /// occured.\n    subscribe-instant: func(\n        when: instant,\n    ) -> pollable;\n\n    /// Create a `pollable` which will resolve once the given duration has\n    /// elapsed, starting at the time at which this function was called.\n    /// occured.\n    subscribe-duration: func(\n        when: duration,\n    ) -> pollable;\n}\n";
        const _: &[u8] = b"package wasi:io@0.2.0;\n\n\ninterface error {\n    /// A resource which represents some error information.\n    ///\n    /// The only method provided by this resource is `to-debug-string`,\n    /// which provides some human-readable information about the error.\n    ///\n    /// In the `wasi:io` package, this resource is returned through the\n    /// `wasi:io/streams/stream-error` type.\n    ///\n    /// To provide more specific error information, other interfaces may\n    /// provide functions to further \"downcast\" this error into more specific\n    /// error information. For example, `error`s returned in streams derived\n    /// from filesystem types to be described using the filesystem\'s own\n    /// error-code type, using the function\n    /// `wasi:filesystem/types/filesystem-error-code`, which takes a parameter\n    /// `borrow<error>` and returns\n    /// `option<wasi:filesystem/types/error-code>`.\n    ///\n    /// The set of functions which can \"downcast\" an `error` into a more\n    /// concrete type is open.\n    resource error {\n        /// Returns a string that is suitable to assist humans in debugging\n        /// this error.\n        ///\n        /// WARNING: The returned string should not be consumed mechanically!\n        /// It may change across platforms, hosts, or other implementation\n        /// details. Parsing this string is a major platform-compatibility\n        /// hazard.\n        to-debug-string: func() -> string;\n    }\n}\n";
        const _: &[u8] = b"package wasi:cli@0.2.0;\n\nworld imports {\n  include wasi:clocks/imports@0.2.0;\n  include wasi:filesystem/imports@0.2.0;\n  include wasi:sockets/imports@0.2.0;\n  include wasi:random/imports@0.2.0;\n  include wasi:io/imports@0.2.0;\n\n  import environment;\n  import exit;\n  import stdin;\n  import stdout;\n  import stderr;\n  import terminal-input;\n  import terminal-output;\n  import terminal-stdin;\n  import terminal-stdout;\n  import terminal-stderr;\n}\n";
        const _: &[u8] = b"\ninterface udp-create-socket {\n    use network.{network, error-code, ip-address-family};\n    use udp.{udp-socket};\n\n    /// Create a new UDP socket.\n    ///\n    /// Similar to `socket(AF_INET or AF_INET6, SOCK_DGRAM, IPPROTO_UDP)` in POSIX.\n    /// On IPv6 sockets, IPV6_V6ONLY is enabled by default and can\'t be configured otherwise.\n    ///\n    /// This function does not require a network capability handle. This is considered to be safe because\n    /// at time of creation, the socket is not bound to any `network` yet. Up to the moment `bind` is called,\n    /// the socket is effectively an in-memory configuration object, unable to communicate with the outside world.\n    ///\n    /// All sockets are non-blocking. Use the wasi-poll interface to block on asynchronous operations.\n    ///\n    /// # Typical errors\n    /// - `not-supported`:     The specified `address-family` is not supported. (EAFNOSUPPORT)\n    /// - `new-socket-limit`:  The new socket resource could not be created because of a system limit. (EMFILE, ENFILE)\n    ///\n    /// # References:\n    /// - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/socket.html>\n    /// - <https://man7.org/linux/man-pages/man2/socket.2.html>\n    /// - <https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-wsasocketw>\n    /// - <https://man.freebsd.org/cgi/man.cgi?query=socket&sektion=2>\n    create-udp-socket: func(address-family: ip-address-family) -> result<udp-socket, error-code>;\n}\n";
        const _: &[u8] = b"package wasi:cli@0.2.0;\n\nworld command {\n  include imports;\n\n  export run;\n}\n";
        const _: &[u8] = b"package wasi:random@0.2.0;\n/// The insecure interface for insecure pseudo-random numbers.\n///\n/// It is intended to be portable at least between Unix-family platforms and\n/// Windows.\ninterface insecure {\n    /// Return `len` insecure pseudo-random bytes.\n    ///\n    /// This function is not cryptographically secure. Do not use it for\n    /// anything related to security.\n    ///\n    /// There are no requirements on the values of the returned bytes, however\n    /// implementations are encouraged to return evenly distributed values with\n    /// a long period.\n    get-insecure-random-bytes: func(len: u64) -> list<u8>;\n\n    /// Return an insecure pseudo-random `u64` value.\n    ///\n    /// This function returns the same type of pseudo-random data as\n    /// `get-insecure-random-bytes`, represented as a `u64`.\n    get-insecure-random-u64: func() -> u64;\n}\n";
        const _: &[u8] = b"package wasi:filesystem@0.2.0;\n\nworld imports {\n    import types;\n    import preopens;\n}\n";
        const _: &[u8] = b"package wasi:sockets@0.2.0;\n\nworld imports {\n    import instance-network;\n    import network;\n    import udp;\n    import udp-create-socket;\n    import tcp;\n    import tcp-create-socket;\n    import ip-name-lookup;\n}\n";
        const _: &[u8] = b"package wasi:io@0.2.0;\n\n/// A poll API intended to let users wait for I/O events on multiple handles\n/// at once.\ninterface poll {\n    /// `pollable` represents a single I/O event which may be ready, or not.\n    resource pollable {\n\n      /// Return the readiness of a pollable. This function never blocks.\n      ///\n      /// Returns `true` when the pollable is ready, and `false` otherwise.\n      ready: func() -> bool;\n\n      /// `block` returns immediately if the pollable is ready, and otherwise\n      /// blocks until ready.\n      ///\n      /// This function is equivalent to calling `poll.poll` on a list\n      /// containing only this pollable.\n      block: func();\n    }\n\n    /// Poll for completion on a set of pollables.\n    ///\n    /// This function takes a list of pollables, which identify I/O sources of\n    /// interest, and waits until one or more of the events is ready for I/O.\n    ///\n    /// The result `list<u32>` contains one or more indices of handles in the\n    /// argument list that is ready for I/O.\n    ///\n    /// If the list contains more elements than can be indexed with a `u32`\n    /// value, this function traps.\n    ///\n    /// A timeout can be implemented by adding a pollable from the\n    /// wasi-clocks API to the list.\n    ///\n    /// This function does not return a `result`; polling in itself does not\n    /// do any I/O so it doesn\'t fail. If any of the I/O sources identified by\n    /// the pollables has an error, it is indicated by marking the source as\n    /// being reaedy for I/O.\n    poll: func(in: list<borrow<pollable>>) -> list<u32>;\n}\n";
        const _: &[u8] = b"package wasi:http@0.2.0;\n\n/// The `wasi:http/proxy` world captures a widely-implementable intersection of\n/// hosts that includes HTTP forward and reverse proxies. Components targeting\n/// this world may concurrently stream in and out any number of incoming and\n/// outgoing HTTP requests.\nworld proxy {\n  /// HTTP proxies have access to time and randomness.\n  include wasi:clocks/imports@0.2.0;\n  import wasi:random/random@0.2.0;\n\n  /// Proxies have standard output and error streams which are expected to\n  /// terminate in a developer-facing console provided by the host.\n  import wasi:cli/stdout@0.2.0;\n  import wasi:cli/stderr@0.2.0;\n\n  /// TODO: this is a temporary workaround until component tooling is able to\n  /// gracefully handle the absence of stdin. Hosts must return an eof stream\n  /// for this import, which is what wasi-libc + tooling will do automatically\n  /// when this import is properly removed.\n  import wasi:cli/stdin@0.2.0;\n\n  /// This is the default handler to use when user code simply wants to make an\n  /// HTTP request (e.g., via `fetch()`).\n  import outgoing-handler;\n\n  /// The host delivers incoming HTTP requests to a component by calling the\n  /// `handle` function of this exported interface. A host may arbitrarily reuse\n  /// or not reuse component instance when delivering incoming HTTP requests and\n  /// thus a component must be able to handle 0..N calls to `handle`.\n  export incoming-handler;\n}\n";
        const _: &[u8] = b"package wasi:random@0.2.0;\n\nworld imports {\n    import random;\n    import insecure;\n    import insecure-seed;\n}\n";
        const _: &[u8] = b"\ninterface udp {\n    use wasi:io/poll@0.2.0.{pollable};\n    use network.{network, error-code, ip-socket-address, ip-address-family};\n\n    /// A received datagram.\n    record incoming-datagram {\n        /// The payload.\n        /// \n        /// Theoretical max size: ~64 KiB. In practice, typically less than 1500 bytes.\n        data: list<u8>,\n\n        /// The source address.\n        ///\n        /// This field is guaranteed to match the remote address the stream was initialized with, if any.\n        ///\n        /// Equivalent to the `src_addr` out parameter of `recvfrom`.\n        remote-address: ip-socket-address,\n    }\n\n    /// A datagram to be sent out.\n    record outgoing-datagram {\n        /// The payload.\n        data: list<u8>,\n\n        /// The destination address.\n        ///\n        /// The requirements on this field depend on how the stream was initialized:\n        /// - with a remote address: this field must be None or match the stream\'s remote address exactly.\n        /// - without a remote address: this field is required.\n        ///\n        /// If this value is None, the send operation is equivalent to `send` in POSIX. Otherwise it is equivalent to `sendto`.\n        remote-address: option<ip-socket-address>,\n    }\n\n\n\n    /// A UDP socket handle.\n    resource udp-socket {\n        /// Bind the socket to a specific network on the provided IP address and port.\n        ///\n        /// If the IP address is zero (`0.0.0.0` in IPv4, `::` in IPv6), it is left to the implementation to decide which\n        /// network interface(s) to bind to.\n        /// If the port is zero, the socket will be bound to a random free port.\n        ///\n        /// # Typical errors\n        /// - `invalid-argument`:          The `local-address` has the wrong address family. (EAFNOSUPPORT, EFAULT on Windows)\n        /// - `invalid-state`:             The socket is already bound. (EINVAL)\n        /// - `address-in-use`:            No ephemeral ports available. (EADDRINUSE, ENOBUFS on Windows)\n        /// - `address-in-use`:            Address is already in use. (EADDRINUSE)\n        /// - `address-not-bindable`:      `local-address` is not an address that the `network` can bind to. (EADDRNOTAVAIL)\n        /// - `not-in-progress`:           A `bind` operation is not in progress.\n        /// - `would-block`:               Can\'t finish the operation, it is still in progress. (EWOULDBLOCK, EAGAIN)\n        ///\n        /// # Implementors note\n        /// Unlike in POSIX, in WASI the bind operation is async. This enables\n        /// interactive WASI hosts to inject permission prompts. Runtimes that\n        /// don\'t want to make use of this ability can simply call the native\n        /// `bind` as part of either `start-bind` or `finish-bind`.\n        ///\n        /// # References\n        /// - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/bind.html>\n        /// - <https://man7.org/linux/man-pages/man2/bind.2.html>\n        /// - <https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-bind>\n        /// - <https://man.freebsd.org/cgi/man.cgi?query=bind&sektion=2&format=html>\n        start-bind: func(network: borrow<network>, local-address: ip-socket-address) -> result<_, error-code>;\n        finish-bind: func() -> result<_, error-code>;\n\n        /// Set up inbound & outbound communication channels, optionally to a specific peer.\n        ///\n        /// This function only changes the local socket configuration and does not generate any network traffic.\n        /// On success, the `remote-address` of the socket is updated. The `local-address` may be updated as well,\n        /// based on the best network path to `remote-address`.\n        ///\n        /// When a `remote-address` is provided, the returned streams are limited to communicating with that specific peer:\n        /// - `send` can only be used to send to this destination.\n        /// - `receive` will only return datagrams sent from the provided `remote-address`.\n        ///\n        /// This method may be called multiple times on the same socket to change its association, but\n        /// only the most recently returned pair of streams will be operational. Implementations may trap if\n        /// the streams returned by a previous invocation haven\'t been dropped yet before calling `stream` again.\n        /// \n        /// The POSIX equivalent in pseudo-code is:\n        /// ```text\n        /// if (was previously connected) {\n        /// \tconnect(s, AF_UNSPEC)\n        /// }\n        /// if (remote_address is Some) {\n        /// \tconnect(s, remote_address)\n        /// }\n        /// ```\n        ///\n        /// Unlike in POSIX, the socket must already be explicitly bound.\n        /// \n        /// # Typical errors\n        /// - `invalid-argument`:          The `remote-address` has the wrong address family. (EAFNOSUPPORT)\n        /// - `invalid-argument`:          The IP address in `remote-address` is set to INADDR_ANY (`0.0.0.0` / `::`). (EDESTADDRREQ, EADDRNOTAVAIL)\n        /// - `invalid-argument`:          The port in `remote-address` is set to 0. (EDESTADDRREQ, EADDRNOTAVAIL)\n        /// - `invalid-state`:             The socket is not bound.\n        /// - `address-in-use`:            Tried to perform an implicit bind, but there were no ephemeral ports available. (EADDRINUSE, EADDRNOTAVAIL on Linux, EAGAIN on BSD)\n        /// - `remote-unreachable`:        The remote address is not reachable. (ECONNRESET, ENETRESET, EHOSTUNREACH, EHOSTDOWN, ENETUNREACH, ENETDOWN, ENONET)\n        /// - `connection-refused`:        The connection was refused. (ECONNREFUSED)\n        ///\n        /// # References\n        /// - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/connect.html>\n        /// - <https://man7.org/linux/man-pages/man2/connect.2.html>\n        /// - <https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-connect>\n        /// - <https://man.freebsd.org/cgi/man.cgi?connect>\n        %stream: func(remote-address: option<ip-socket-address>) -> result<tuple<incoming-datagram-stream, outgoing-datagram-stream>, error-code>;\n\n        /// Get the current bound address.\n        ///\n        /// POSIX mentions:\n        /// > If the socket has not been bound to a local name, the value\n        /// > stored in the object pointed to by `address` is unspecified.\n        ///\n        /// WASI is stricter and requires `local-address` to return `invalid-state` when the socket hasn\'t been bound yet.\n        /// \n        /// # Typical errors\n        /// - `invalid-state`: The socket is not bound to any local address.\n        ///\n        /// # References\n        /// - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/getsockname.html>\n        /// - <https://man7.org/linux/man-pages/man2/getsockname.2.html>\n        /// - <https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-getsockname>\n        /// - <https://man.freebsd.org/cgi/man.cgi?getsockname>\n        local-address: func() -> result<ip-socket-address, error-code>;\n\n        /// Get the address the socket is currently streaming to.\n        ///\n        /// # Typical errors\n        /// - `invalid-state`: The socket is not streaming to a specific remote address. (ENOTCONN)\n        ///\n        /// # References\n        /// - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/getpeername.html>\n        /// - <https://man7.org/linux/man-pages/man2/getpeername.2.html>\n        /// - <https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-getpeername>\n        /// - <https://man.freebsd.org/cgi/man.cgi?query=getpeername&sektion=2&n=1>\n        remote-address: func() -> result<ip-socket-address, error-code>;\n\n        /// Whether this is a IPv4 or IPv6 socket.\n        ///\n        /// Equivalent to the SO_DOMAIN socket option.\n        address-family: func() -> ip-address-family;\n\n        /// Equivalent to the IP_TTL & IPV6_UNICAST_HOPS socket options.\n        ///\n        /// If the provided value is 0, an `invalid-argument` error is returned.\n        ///\n        /// # Typical errors\n        /// - `invalid-argument`:     (set) The TTL value must be 1 or higher.\n        unicast-hop-limit: func() -> result<u8, error-code>;\n        set-unicast-hop-limit: func(value: u8) -> result<_, error-code>;\n\n        /// The kernel buffer space reserved for sends/receives on this socket.\n        ///\n        /// If the provided value is 0, an `invalid-argument` error is returned.\n        /// Any other value will never cause an error, but it might be silently clamped and/or rounded.\n        /// I.e. after setting a value, reading the same setting back may return a different value.\n        ///\n        /// Equivalent to the SO_RCVBUF and SO_SNDBUF socket options.\n        ///\n        /// # Typical errors\n        /// - `invalid-argument`:     (set) The provided value was 0.\n        receive-buffer-size: func() -> result<u64, error-code>;\n        set-receive-buffer-size: func(value: u64) -> result<_, error-code>;\n        send-buffer-size: func() -> result<u64, error-code>;\n        set-send-buffer-size: func(value: u64) -> result<_, error-code>;\n\n        /// Create a `pollable` which will resolve once the socket is ready for I/O.\n        ///\n        /// Note: this function is here for WASI Preview2 only.\n        /// It\'s planned to be removed when `future` is natively supported in Preview3.\n        subscribe: func() -> pollable;\n    }\n\n    resource incoming-datagram-stream {\n        /// Receive messages on the socket.\n        ///\n        /// This function attempts to receive up to `max-results` datagrams on the socket without blocking.\n        /// The returned list may contain fewer elements than requested, but never more.\n        ///\n        /// This function returns successfully with an empty list when either:\n        /// - `max-results` is 0, or:\n        /// - `max-results` is greater than 0, but no results are immediately available.\n        /// This function never returns `error(would-block)`.\n        ///\n        /// # Typical errors\n        /// - `remote-unreachable`: The remote address is not reachable. (ECONNRESET, ENETRESET on Windows, EHOSTUNREACH, EHOSTDOWN, ENETUNREACH, ENETDOWN, ENONET)\n        /// - `connection-refused`: The connection was refused. (ECONNREFUSED)\n        ///\n        /// # References\n        /// - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/recvfrom.html>\n        /// - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/recvmsg.html>\n        /// - <https://man7.org/linux/man-pages/man2/recv.2.html>\n        /// - <https://man7.org/linux/man-pages/man2/recvmmsg.2.html>\n        /// - <https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-recv>\n        /// - <https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-recvfrom>\n        /// - <https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms741687(v=vs.85)>\n        /// - <https://man.freebsd.org/cgi/man.cgi?query=recv&sektion=2>\n        receive: func(max-results: u64) -> result<list<incoming-datagram>, error-code>;\n\n        /// Create a `pollable` which will resolve once the stream is ready to receive again.\n        ///\n        /// Note: this function is here for WASI Preview2 only.\n        /// It\'s planned to be removed when `future` is natively supported in Preview3.\n        subscribe: func() -> pollable;\n    }\n\n    resource outgoing-datagram-stream {\n        /// Check readiness for sending. This function never blocks.\n        ///\n        /// Returns the number of datagrams permitted for the next call to `send`,\n        /// or an error. Calling `send` with more datagrams than this function has\n        /// permitted will trap.\n        ///\n        /// When this function returns ok(0), the `subscribe` pollable will\n        /// become ready when this function will report at least ok(1), or an\n        /// error.\n        /// \n        /// Never returns `would-block`.\n        check-send: func() -> result<u64, error-code>;\n\n        /// Send messages on the socket.\n        ///\n        /// This function attempts to send all provided `datagrams` on the socket without blocking and\n        /// returns how many messages were actually sent (or queued for sending). This function never\n        /// returns `error(would-block)`. If none of the datagrams were able to be sent, `ok(0)` is returned.\n        ///\n        /// This function semantically behaves the same as iterating the `datagrams` list and sequentially\n        /// sending each individual datagram until either the end of the list has been reached or the first error occurred.\n        /// If at least one datagram has been sent successfully, this function never returns an error.\n        ///\n        /// If the input list is empty, the function returns `ok(0)`.\n        ///\n        /// Each call to `send` must be permitted by a preceding `check-send`. Implementations must trap if\n        /// either `check-send` was not called or `datagrams` contains more items than `check-send` permitted.\n        ///\n        /// # Typical errors\n        /// - `invalid-argument`:        The `remote-address` has the wrong address family. (EAFNOSUPPORT)\n        /// - `invalid-argument`:        The IP address in `remote-address` is set to INADDR_ANY (`0.0.0.0` / `::`). (EDESTADDRREQ, EADDRNOTAVAIL)\n        /// - `invalid-argument`:        The port in `remote-address` is set to 0. (EDESTADDRREQ, EADDRNOTAVAIL)\n        /// - `invalid-argument`:        The socket is in \"connected\" mode and `remote-address` is `some` value that does not match the address passed to `stream`. (EISCONN)\n        /// - `invalid-argument`:        The socket is not \"connected\" and no value for `remote-address` was provided. (EDESTADDRREQ)\n        /// - `remote-unreachable`:      The remote address is not reachable. (ECONNRESET, ENETRESET on Windows, EHOSTUNREACH, EHOSTDOWN, ENETUNREACH, ENETDOWN, ENONET)\n        /// - `connection-refused`:      The connection was refused. (ECONNREFUSED)\n        /// - `datagram-too-large`:      The datagram is too large. (EMSGSIZE)\n        ///\n        /// # References\n        /// - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/sendto.html>\n        /// - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/sendmsg.html>\n        /// - <https://man7.org/linux/man-pages/man2/send.2.html>\n        /// - <https://man7.org/linux/man-pages/man2/sendmmsg.2.html>\n        /// - <https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-send>\n        /// - <https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-sendto>\n        /// - <https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-wsasendmsg>\n        /// - <https://man.freebsd.org/cgi/man.cgi?query=send&sektion=2>\n        send: func(datagrams: list<outgoing-datagram>) -> result<u64, error-code>;\n        \n        /// Create a `pollable` which will resolve once the stream is ready to send again.\n        ///\n        /// Note: this function is here for WASI Preview2 only.\n        /// It\'s planned to be removed when `future` is natively supported in Preview3.\n        subscribe: func() -> pollable;\n    }\n}\n";
        const _: &[u8] = b"interface environment {\n  /// Get the POSIX-style environment variables.\n  ///\n  /// Each environment variable is provided as a pair of string variable names\n  /// and string value.\n  ///\n  /// Morally, these are a value import, but until value imports are available\n  /// in the component model, this import function should return the same\n  /// values each time it is called.\n  get-environment: func() -> list<tuple<string, string>>;\n\n  /// Get the POSIX-style arguments to the program.\n  get-arguments: func() -> list<string>;\n\n  /// Return a path that programs should use as their initial current working\n  /// directory, interpreting `.` as shorthand for this.\n  initial-cwd: func() -> option<string>;\n}\n";
        const _: &[u8] = b"\ninterface tcp {\n    use wasi:io/streams@0.2.0.{input-stream, output-stream};\n    use wasi:io/poll@0.2.0.{pollable};\n    use wasi:clocks/monotonic-clock@0.2.0.{duration};\n    use network.{network, error-code, ip-socket-address, ip-address-family};\n\n    enum shutdown-type {\n        /// Similar to `SHUT_RD` in POSIX.\n        receive,\n\n        /// Similar to `SHUT_WR` in POSIX.\n        send,\n\n        /// Similar to `SHUT_RDWR` in POSIX.\n        both,\n    }\n    \n    /// A TCP socket resource.\n    ///\n    /// The socket can be in one of the following states:\n    /// - `unbound`\n    /// - `bind-in-progress`\n    /// - `bound` (See note below)\n    /// - `listen-in-progress`\n    /// - `listening`\n    /// - `connect-in-progress`\n    /// - `connected`\n    /// - `closed`\n    /// See <https://github.com/WebAssembly/wasi-sockets/TcpSocketOperationalSemantics.md>\n    /// for a more information.\n    ///\n    /// Note: Except where explicitly mentioned, whenever this documentation uses\n    /// the term \"bound\" without backticks it actually means: in the `bound` state *or higher*.\n    /// (i.e. `bound`, `listen-in-progress`, `listening`, `connect-in-progress` or `connected`)\n    ///\n    /// In addition to the general error codes documented on the\n    /// `network::error-code` type, TCP socket methods may always return\n    /// `error(invalid-state)` when in the `closed` state.\n    resource tcp-socket {\n        /// Bind the socket to a specific network on the provided IP address and port.\n        ///\n        /// If the IP address is zero (`0.0.0.0` in IPv4, `::` in IPv6), it is left to the implementation to decide which\n        /// network interface(s) to bind to.\n        /// If the TCP/UDP port is zero, the socket will be bound to a random free port.\n        ///\n        /// Bind can be attempted multiple times on the same socket, even with\n        /// different arguments on each iteration. But never concurrently and\n        /// only as long as the previous bind failed. Once a bind succeeds, the\n        /// binding can\'t be changed anymore.\n        ///\n        /// # Typical errors\n        /// - `invalid-argument`:          The `local-address` has the wrong address family. (EAFNOSUPPORT, EFAULT on Windows)\n        /// - `invalid-argument`:          `local-address` is not a unicast address. (EINVAL)\n        /// - `invalid-argument`:          `local-address` is an IPv4-mapped IPv6 address. (EINVAL)\n        /// - `invalid-state`:             The socket is already bound. (EINVAL)\n        /// - `address-in-use`:            No ephemeral ports available. (EADDRINUSE, ENOBUFS on Windows)\n        /// - `address-in-use`:            Address is already in use. (EADDRINUSE)\n        /// - `address-not-bindable`:      `local-address` is not an address that the `network` can bind to. (EADDRNOTAVAIL)\n        /// - `not-in-progress`:           A `bind` operation is not in progress.\n        /// - `would-block`:               Can\'t finish the operation, it is still in progress. (EWOULDBLOCK, EAGAIN)\n        /// \n        /// # Implementors note\n        /// When binding to a non-zero port, this bind operation shouldn\'t be affected by the TIME_WAIT\n        /// state of a recently closed socket on the same local address. In practice this means that the SO_REUSEADDR \n        /// socket option should be set implicitly on all platforms, except on Windows where this is the default behavior\n        /// and SO_REUSEADDR performs something different entirely.\n        ///\n        /// Unlike in POSIX, in WASI the bind operation is async. This enables\n        /// interactive WASI hosts to inject permission prompts. Runtimes that\n        /// don\'t want to make use of this ability can simply call the native\n        /// `bind` as part of either `start-bind` or `finish-bind`.\n        ///\n        /// # References\n        /// - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/bind.html>\n        /// - <https://man7.org/linux/man-pages/man2/bind.2.html>\n        /// - <https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-bind>\n        /// - <https://man.freebsd.org/cgi/man.cgi?query=bind&sektion=2&format=html>\n        start-bind: func(network: borrow<network>, local-address: ip-socket-address) -> result<_, error-code>;\n        finish-bind: func() -> result<_, error-code>;\n\n        /// Connect to a remote endpoint.\n        ///\n        /// On success:\n        /// - the socket is transitioned into the `connection` state.\n        /// - a pair of streams is returned that can be used to read & write to the connection\n        ///\n        /// After a failed connection attempt, the socket will be in the `closed`\n        /// state and the only valid action left is to `drop` the socket. A single\n        /// socket can not be used to connect more than once.\n        ///\n        /// # Typical errors\n        /// - `invalid-argument`:          The `remote-address` has the wrong address family. (EAFNOSUPPORT)\n        /// - `invalid-argument`:          `remote-address` is not a unicast address. (EINVAL, ENETUNREACH on Linux, EAFNOSUPPORT on MacOS)\n        /// - `invalid-argument`:          `remote-address` is an IPv4-mapped IPv6 address. (EINVAL, EADDRNOTAVAIL on Illumos)\n        /// - `invalid-argument`:          The IP address in `remote-address` is set to INADDR_ANY (`0.0.0.0` / `::`). (EADDRNOTAVAIL on Windows)\n        /// - `invalid-argument`:          The port in `remote-address` is set to 0. (EADDRNOTAVAIL on Windows)\n        /// - `invalid-argument`:          The socket is already attached to a different network. The `network` passed to `connect` must be identical to the one passed to `bind`.\n        /// - `invalid-state`:             The socket is already in the `connected` state. (EISCONN)\n        /// - `invalid-state`:             The socket is already in the `listening` state. (EOPNOTSUPP, EINVAL on Windows)\n        /// - `timeout`:                   Connection timed out. (ETIMEDOUT)\n        /// - `connection-refused`:        The connection was forcefully rejected. (ECONNREFUSED)\n        /// - `connection-reset`:          The connection was reset. (ECONNRESET)\n        /// - `connection-aborted`:        The connection was aborted. (ECONNABORTED)\n        /// - `remote-unreachable`:        The remote address is not reachable. (EHOSTUNREACH, EHOSTDOWN, ENETUNREACH, ENETDOWN, ENONET)\n        /// - `address-in-use`:            Tried to perform an implicit bind, but there were no ephemeral ports available. (EADDRINUSE, EADDRNOTAVAIL on Linux, EAGAIN on BSD)\n        /// - `not-in-progress`:           A connect operation is not in progress.\n        /// - `would-block`:               Can\'t finish the operation, it is still in progress. (EWOULDBLOCK, EAGAIN)\n        ///\n        /// # Implementors note\n        /// The POSIX equivalent of `start-connect` is the regular `connect` syscall.\n        /// Because all WASI sockets are non-blocking this is expected to return\n        /// EINPROGRESS, which should be translated to `ok()` in WASI.\n        ///\n        /// The POSIX equivalent of `finish-connect` is a `poll` for event `POLLOUT`\n        /// with a timeout of 0 on the socket descriptor. Followed by a check for\n        /// the `SO_ERROR` socket option, in case the poll signaled readiness.\n        ///\n        /// # References\n        /// - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/connect.html>\n        /// - <https://man7.org/linux/man-pages/man2/connect.2.html>\n        /// - <https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-connect>\n        /// - <https://man.freebsd.org/cgi/man.cgi?connect>\n        start-connect: func(network: borrow<network>, remote-address: ip-socket-address) -> result<_, error-code>;\n        finish-connect: func() -> result<tuple<input-stream, output-stream>, error-code>;\n\n        /// Start listening for new connections.\n        ///\n        /// Transitions the socket into the `listening` state.\n        ///\n        /// Unlike POSIX, the socket must already be explicitly bound.\n        ///\n        /// # Typical errors\n        /// - `invalid-state`:             The socket is not bound to any local address. (EDESTADDRREQ)\n        /// - `invalid-state`:             The socket is already in the `connected` state. (EISCONN, EINVAL on BSD)\n        /// - `invalid-state`:             The socket is already in the `listening` state.\n        /// - `address-in-use`:            Tried to perform an implicit bind, but there were no ephemeral ports available. (EADDRINUSE)\n        /// - `not-in-progress`:           A listen operation is not in progress.\n        /// - `would-block`:               Can\'t finish the operation, it is still in progress. (EWOULDBLOCK, EAGAIN)\n        ///\n        /// # Implementors note\n        /// Unlike in POSIX, in WASI the listen operation is async. This enables\n        /// interactive WASI hosts to inject permission prompts. Runtimes that\n        /// don\'t want to make use of this ability can simply call the native\n        /// `listen` as part of either `start-listen` or `finish-listen`.\n        ///\n        /// # References\n        /// - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/listen.html>\n        /// - <https://man7.org/linux/man-pages/man2/listen.2.html>\n        /// - <https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-listen>\n        /// - <https://man.freebsd.org/cgi/man.cgi?query=listen&sektion=2>\n        start-listen: func() -> result<_, error-code>;\n        finish-listen: func() -> result<_, error-code>;\n\n        /// Accept a new client socket.\n        ///\n        /// The returned socket is bound and in the `connected` state. The following properties are inherited from the listener socket:\n        /// - `address-family`\n        /// - `keep-alive-enabled`\n        /// - `keep-alive-idle-time`\n        /// - `keep-alive-interval`\n        /// - `keep-alive-count`\n        /// - `hop-limit`\n        /// - `receive-buffer-size`\n        /// - `send-buffer-size`\n        ///\n        /// On success, this function returns the newly accepted client socket along with\n        /// a pair of streams that can be used to read & write to the connection.\n        ///\n        /// # Typical errors\n        /// - `invalid-state`:      Socket is not in the `listening` state. (EINVAL)\n        /// - `would-block`:        No pending connections at the moment. (EWOULDBLOCK, EAGAIN)\n        /// - `connection-aborted`: An incoming connection was pending, but was terminated by the client before this listener could accept it. (ECONNABORTED)\n        /// - `new-socket-limit`:   The new socket resource could not be created because of a system limit. (EMFILE, ENFILE)\n        ///\n        /// # References\n        /// - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/accept.html>\n        /// - <https://man7.org/linux/man-pages/man2/accept.2.html>\n        /// - <https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-accept>\n        /// - <https://man.freebsd.org/cgi/man.cgi?query=accept&sektion=2>\n        accept: func() -> result<tuple<tcp-socket, input-stream, output-stream>, error-code>;\n\n        /// Get the bound local address.\n        ///\n        /// POSIX mentions:\n        /// > If the socket has not been bound to a local name, the value\n        /// > stored in the object pointed to by `address` is unspecified.\n        ///\n        /// WASI is stricter and requires `local-address` to return `invalid-state` when the socket hasn\'t been bound yet.\n        ///\n        /// # Typical errors\n        /// - `invalid-state`: The socket is not bound to any local address.\n        ///\n        /// # References\n        /// - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/getsockname.html>\n        /// - <https://man7.org/linux/man-pages/man2/getsockname.2.html>\n        /// - <https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-getsockname>\n        /// - <https://man.freebsd.org/cgi/man.cgi?getsockname>\n        local-address: func() -> result<ip-socket-address, error-code>;\n\n        /// Get the remote address.\n        ///\n        /// # Typical errors\n        /// - `invalid-state`: The socket is not connected to a remote address. (ENOTCONN)\n        ///\n        /// # References\n        /// - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/getpeername.html>\n        /// - <https://man7.org/linux/man-pages/man2/getpeername.2.html>\n        /// - <https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-getpeername>\n        /// - <https://man.freebsd.org/cgi/man.cgi?query=getpeername&sektion=2&n=1>\n        remote-address: func() -> result<ip-socket-address, error-code>;\n\n        /// Whether the socket is in the `listening` state.\n        ///\n        /// Equivalent to the SO_ACCEPTCONN socket option.\n        is-listening: func() -> bool;\n\n        /// Whether this is a IPv4 or IPv6 socket.\n        ///\n        /// Equivalent to the SO_DOMAIN socket option.\n        address-family: func() -> ip-address-family;\n\n        /// Hints the desired listen queue size. Implementations are free to ignore this.\n        ///\n        /// If the provided value is 0, an `invalid-argument` error is returned.\n        /// Any other value will never cause an error, but it might be silently clamped and/or rounded.\n        ///\n        /// # Typical errors\n        /// - `not-supported`:        (set) The platform does not support changing the backlog size after the initial listen.\n        /// - `invalid-argument`:     (set) The provided value was 0.\n        /// - `invalid-state`:        (set) The socket is in the `connect-in-progress` or `connected` state.\n        set-listen-backlog-size: func(value: u64) -> result<_, error-code>;\n\n        /// Enables or disables keepalive.\n        ///\n        /// The keepalive behavior can be adjusted using:\n        /// - `keep-alive-idle-time`\n        /// - `keep-alive-interval`\n        /// - `keep-alive-count`\n        /// These properties can be configured while `keep-alive-enabled` is false, but only come into effect when `keep-alive-enabled` is true.\n        ///\n        /// Equivalent to the SO_KEEPALIVE socket option.\n        keep-alive-enabled: func() -> result<bool, error-code>;\n        set-keep-alive-enabled: func(value: bool) -> result<_, error-code>;\n\n        /// Amount of time the connection has to be idle before TCP starts sending keepalive packets.\n        ///\n        /// If the provided value is 0, an `invalid-argument` error is returned.\n        /// Any other value will never cause an error, but it might be silently clamped and/or rounded.\n        /// I.e. after setting a value, reading the same setting back may return a different value.\n        ///\n        /// Equivalent to the TCP_KEEPIDLE socket option. (TCP_KEEPALIVE on MacOS)\n        ///\n        /// # Typical errors\n        /// - `invalid-argument`:     (set) The provided value was 0.\n        keep-alive-idle-time: func() -> result<duration, error-code>;\n        set-keep-alive-idle-time: func(value: duration) -> result<_, error-code>;\n\n        /// The time between keepalive packets.\n        ///\n        /// If the provided value is 0, an `invalid-argument` error is returned.\n        /// Any other value will never cause an error, but it might be silently clamped and/or rounded.\n        /// I.e. after setting a value, reading the same setting back may return a different value.\n        ///\n        /// Equivalent to the TCP_KEEPINTVL socket option.\n        ///\n        /// # Typical errors\n        /// - `invalid-argument`:     (set) The provided value was 0.\n        keep-alive-interval: func() -> result<duration, error-code>;\n        set-keep-alive-interval: func(value: duration) -> result<_, error-code>;\n\n        /// The maximum amount of keepalive packets TCP should send before aborting the connection.\n        ///\n        /// If the provided value is 0, an `invalid-argument` error is returned.\n        /// Any other value will never cause an error, but it might be silently clamped and/or rounded.\n        /// I.e. after setting a value, reading the same setting back may return a different value.\n        ///\n        /// Equivalent to the TCP_KEEPCNT socket option.\n        ///\n        /// # Typical errors\n        /// - `invalid-argument`:     (set) The provided value was 0.\n        keep-alive-count: func() -> result<u32, error-code>;\n        set-keep-alive-count: func(value: u32) -> result<_, error-code>;\n\n        /// Equivalent to the IP_TTL & IPV6_UNICAST_HOPS socket options.\n        ///\n        /// If the provided value is 0, an `invalid-argument` error is returned.\n        ///\n        /// # Typical errors\n        /// - `invalid-argument`:     (set) The TTL value must be 1 or higher.\n        hop-limit: func() -> result<u8, error-code>;\n        set-hop-limit: func(value: u8) -> result<_, error-code>;\n\n        /// The kernel buffer space reserved for sends/receives on this socket.\n        ///\n        /// If the provided value is 0, an `invalid-argument` error is returned.\n        /// Any other value will never cause an error, but it might be silently clamped and/or rounded.\n        /// I.e. after setting a value, reading the same setting back may return a different value.\n        ///\n        /// Equivalent to the SO_RCVBUF and SO_SNDBUF socket options.\n        ///\n        /// # Typical errors\n        /// - `invalid-argument`:     (set) The provided value was 0.\n        receive-buffer-size: func() -> result<u64, error-code>;\n        set-receive-buffer-size: func(value: u64) -> result<_, error-code>;\n        send-buffer-size: func() -> result<u64, error-code>;\n        set-send-buffer-size: func(value: u64) -> result<_, error-code>;\n\n        /// Create a `pollable` which can be used to poll for, or block on,\n        /// completion of any of the asynchronous operations of this socket.\n        ///\n        /// When `finish-bind`, `finish-listen`, `finish-connect` or `accept`\n        /// return `error(would-block)`, this pollable can be used to wait for\n        /// their success or failure, after which the method can be retried.\n        ///\n        /// The pollable is not limited to the async operation that happens to be\n        /// in progress at the time of calling `subscribe` (if any). Theoretically,\n        /// `subscribe` only has to be called once per socket and can then be\n        /// (re)used for the remainder of the socket\'s lifetime.\n        ///\n        /// See <https://github.com/WebAssembly/wasi-sockets/TcpSocketOperationalSemantics.md#Pollable-readiness>\n        /// for a more information.\n        ///\n        /// Note: this function is here for WASI Preview2 only.\n        /// It\'s planned to be removed when `future` is natively supported in Preview3.\n        subscribe: func() -> pollable;\n\n        /// Initiate a graceful shutdown.\n        ///\n        /// - `receive`: The socket is not expecting to receive any data from\n        ///   the peer. The `input-stream` associated with this socket will be\n        ///   closed. Any data still in the receive queue at time of calling\n        ///   this method will be discarded.\n        /// - `send`: The socket has no more data to send to the peer. The `output-stream`\n        ///   associated with this socket will be closed and a FIN packet will be sent.\n        /// - `both`: Same effect as `receive` & `send` combined.\n        ///\n        /// This function is idempotent. Shutting a down a direction more than once\n        /// has no effect and returns `ok`.\n        ///\n        /// The shutdown function does not close (drop) the socket.\n        ///\n        /// # Typical errors\n        /// - `invalid-state`: The socket is not in the `connected` state. (ENOTCONN)\n        ///\n        /// # References\n        /// - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/shutdown.html>\n        /// - <https://man7.org/linux/man-pages/man2/shutdown.2.html>\n        /// - <https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-shutdown>\n        /// - <https://man.freebsd.org/cgi/man.cgi?query=shutdown&sektion=2>\n        shutdown: func(shutdown-type: shutdown-type) -> result<_, error-code>;\n    }\n}\n";
        const _: &[u8] = b"\ninterface network {\n    /// An opaque resource that represents access to (a subset of) the network.\n    /// This enables context-based security for networking.\n    /// There is no need for this to map 1:1 to a physical network interface.\n    resource network;\n\n    /// Error codes.\n    ///\n    /// In theory, every API can return any error code.\n    /// In practice, API\'s typically only return the errors documented per API\n    /// combined with a couple of errors that are always possible:\n    /// - `unknown`\n    /// - `access-denied`\n    /// - `not-supported`\n    /// - `out-of-memory`\n    /// - `concurrency-conflict`\n    ///\n    /// See each individual API for what the POSIX equivalents are. They sometimes differ per API.\n    enum error-code {\n        /// Unknown error\n        unknown,\n\n        /// Access denied.\n        ///\n        /// POSIX equivalent: EACCES, EPERM\n        access-denied,\n\n        /// The operation is not supported.\n        ///\n        /// POSIX equivalent: EOPNOTSUPP\n        not-supported,\n\n        /// One of the arguments is invalid.\n        ///\n        /// POSIX equivalent: EINVAL\n        invalid-argument,\n\n        /// Not enough memory to complete the operation.\n        ///\n        /// POSIX equivalent: ENOMEM, ENOBUFS, EAI_MEMORY\n        out-of-memory,\n\n        /// The operation timed out before it could finish completely.\n        timeout,\n\n        /// This operation is incompatible with another asynchronous operation that is already in progress.\n        ///\n        /// POSIX equivalent: EALREADY\n        concurrency-conflict,\n\n        /// Trying to finish an asynchronous operation that:\n        /// - has not been started yet, or:\n        /// - was already finished by a previous `finish-*` call.\n        ///\n        /// Note: this is scheduled to be removed when `future`s are natively supported.\n        not-in-progress,\n\n        /// The operation has been aborted because it could not be completed immediately.\n        ///\n        /// Note: this is scheduled to be removed when `future`s are natively supported.\n        would-block,\n\n\n        /// The operation is not valid in the socket\'s current state.\n        invalid-state,\n\n        /// A new socket resource could not be created because of a system limit.\n        new-socket-limit,\n\n        /// A bind operation failed because the provided address is not an address that the `network` can bind to.\n        address-not-bindable,\n\n        /// A bind operation failed because the provided address is already in use or because there are no ephemeral ports available.\n        address-in-use,\n\n        /// The remote address is not reachable\n        remote-unreachable,\n\n\n        /// The TCP connection was forcefully rejected\n        connection-refused,\n\n        /// The TCP connection was reset.\n        connection-reset,\n\n        /// A TCP connection was aborted.\n        connection-aborted,\n\n\n        /// The size of a datagram sent to a UDP socket exceeded the maximum\n        /// supported size.\n        datagram-too-large,\n\n\n        /// Name does not exist or has no suitable associated IP addresses.\n        name-unresolvable,\n\n        /// A temporary failure in name resolution occurred.\n        temporary-resolver-failure,\n\n        /// A permanent failure in name resolution occurred.\n        permanent-resolver-failure,\n    }\n\n    enum ip-address-family {\n        /// Similar to `AF_INET` in POSIX.\n        ipv4,\n\n        /// Similar to `AF_INET6` in POSIX.\n        ipv6,\n    }\n\n    type ipv4-address = tuple<u8, u8, u8, u8>;\n    type ipv6-address = tuple<u16, u16, u16, u16, u16, u16, u16, u16>;\n\n    variant ip-address {\n        ipv4(ipv4-address),\n        ipv6(ipv6-address),\n    }\n\n    record ipv4-socket-address {\n        /// sin_port\n        port: u16,\n        /// sin_addr\n        address: ipv4-address,\n    }\n\n    record ipv6-socket-address {\n        /// sin6_port\n        port: u16,\n        /// sin6_flowinfo\n        flow-info: u32,\n        /// sin6_addr\n        address: ipv6-address,\n        /// sin6_scope_id\n        scope-id: u32,\n    }\n\n    variant ip-socket-address {\n        ipv4(ipv4-socket-address),\n        ipv6(ipv6-socket-address),\n    }\n\n}\n";
        const _: &[u8] = b"interface redis-types {\n  // General purpose error.\n  enum error {\n      success,\n      error,\n  }\n\n  // The message payload.\n  type payload = list<u8>;\n\n  // A parameter type for the general-purpose `execute` function.\n  variant redis-parameter {\n      int64(s64),\n      binary(payload)\n  }\n\n  // A return type for the general-purpose `execute` function.\n  variant redis-result {\n      nil,\n      status(string),\n      int64(s64),\n      binary(payload)\n  }\n}\n";
        const _: &[u8] = b"interface stdin {\n  use wasi:io/streams@0.2.0.{input-stream};\n\n  get-stdin: func() -> input-stream;\n}\n\ninterface stdout {\n  use wasi:io/streams@0.2.0.{output-stream};\n\n  get-stdout: func() -> output-stream;\n}\n\ninterface stderr {\n  use wasi:io/streams@0.2.0.{output-stream};\n\n  get-stderr: func() -> output-stream;\n}\n";
        const _: &[u8] = b"package wasi:filesystem@0.2.0;\n\ninterface preopens {\n    use types.{descriptor};\n\n    /// Return the set of preopened directories, and their path.\n    get-directories: func() -> list<tuple<descriptor, string>>;\n}\n";
        pub struct Spin;
        const _: () = {
            #[unsafe(export_name = "wasi:http/incoming-handler@0.2.0#handle")]
            unsafe extern "C" fn export_handle(arg0: i32, arg1: i32) {
                unsafe {
                    self::exports::wasi::http::incoming_handler::_export_handle_cabi::<
                        Spin,
                    >(arg0, arg1)
                }
            }
        };
    }
    impl self::preamble::exports::wasi::http::incoming_handler::Guest
    for self::preamble::Spin {
        fn handle(
            request: self::preamble::wasi::http::types::IncomingRequest,
            response_out: self::preamble::wasi::http::types::ResponseOutparam,
        ) {
            let request: ::spin_sdk::http::IncomingRequest = ::std::convert::Into::into(
                request,
            );
            let response_out: ::spin_sdk::http::ResponseOutparam = ::std::convert::Into::into(
                response_out,
            );
            ::spin_sdk::http::run(async move {
                match ::spin_sdk::http::conversions::TryFromIncomingRequest::try_from_incoming_request(
                        request,
                    )
                    .await
                {
                    ::std::result::Result::Ok(req) => {
                        handle_response(response_out, super::handle_request(req).await)
                            .await
                    }
                    ::std::result::Result::Err(e) => {
                        handle_response(response_out, e).await
                    }
                }
            });
        }
    }
    async fn handle_response<R: ::spin_sdk::http::IntoResponse>(
        response_out: ::spin_sdk::http::ResponseOutparam,
        resp: R,
    ) {
        let mut response = ::spin_sdk::http::IntoResponse::into_response(resp);
        let body = ::std::mem::take(response.body_mut());
        match ::std::convert::TryInto::try_into(response) {
            ::std::result::Result::Ok(response) => {
                if let Err(e) = ::spin_sdk::http::ResponseOutparam::set_with_body(
                        response_out,
                        response,
                        body,
                    )
                    .await
                {
                    {
                        ::std::io::_eprint(
                            format_args!("Could not set `ResponseOutparam`: {0}\n", e),
                        );
                    };
                }
            }
            ::std::result::Result::Err(e) => {
                {
                    ::std::io::_eprint(
                        format_args!("Could not convert response: {0}\n", e),
                    );
                };
            }
        }
    }
    impl From<self::preamble::wasi::http::types::IncomingRequest>
    for ::spin_sdk::http::IncomingRequest {
        fn from(req: self::preamble::wasi::http::types::IncomingRequest) -> Self {
            unsafe { Self::from_handle(req.take_handle()) }
        }
    }
    impl From<::spin_sdk::http::OutgoingResponse>
    for self::preamble::wasi::http::types::OutgoingResponse {
        fn from(resp: ::spin_sdk::http::OutgoingResponse) -> Self {
            unsafe { Self::from_handle(resp.take_handle()) }
        }
    }
    impl From<self::preamble::wasi::http::types::ResponseOutparam>
    for ::spin_sdk::http::ResponseOutparam {
        fn from(resp: self::preamble::wasi::http::types::ResponseOutparam) -> Self {
            unsafe { Self::from_handle(resp.take_handle()) }
        }
    }
}
/// ヘルスチェックをコア層にプロキシする
///
/// 認証不要でコア層の /health エンドポイントにリクエストを転送。
/// ロードバランサーや Kubernetes のヘルスプローブ用。
///
/// # 戻り値
/// * `Response` - コア層からのヘルスチェックレスポンス
async fn proxy_health_check() -> Response {
    let url = ::alloc::__export::must_use({
        ::alloc::fmt::format(format_args!("{0}/health", CORE_URL))
    });
    {
        ::std::io::_print(format_args!("[Gateway] Health check -> {0}\n", url));
    };
    let outbound_req = Request::builder().method(Method::Get).uri(&url).build();
    match spin_sdk::http::send::<_, Response>(outbound_req).await {
        Ok(response) => {
            let status = *response.status();
            let body = response.into_body();
            Response::builder()
                .status(status)
                .header("Content-Type", "application/json")
                .body(body)
                .build()
        }
        Err(e) => {
            let body = serde_json::to_string(
                    &ErrorResponse {
                        error: ::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!("Health check failed: {0}", e),
                            )
                        }),
                    },
                )
                .unwrap();
            Response::builder()
                .status(503)
                .header("Content-Type", "application/json")
                .body(body)
                .build()
        }
    }
}
/// Authorization ヘッダーから Bearer トークンを抽出する
///
/// HTTP Authorization ヘッダーの形式: "Bearer <token>"
/// この関数は "Bearer " プレフィックスを除去し、トークン部分のみを返します。
///
/// # 引数
/// * `req` - HTTP リクエスト
///
/// # 戻り値
/// * `String` - トークン文字列（見つからない場合は空文字列）
///
/// # 例
/// ```text
/// Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
/// → "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
/// ```
fn extract_bearer_token(req: &Request) -> String {
    req.header("Authorization")
        .and_then(|h| h.as_str())
        .and_then(|auth| {
            if auth.starts_with("Bearer ") { Some(auth[7..].to_string()) } else { None }
        })
        .unwrap_or_default()
}
/// コア層（axum サーバー）にリクエストをプロキシする
///
/// 認証成功後、元のリクエストをコア層に転送します。
/// 転送時に保持される情報：
/// - HTTP メソッド（GET, POST, PATCH, DELETE）
/// - リクエストボディ
/// - クエリパラメータ（例: `?completed=true`）
/// - Content-Type ヘッダー
///
/// 追加されるヘッダー：
/// - X-User-Id: 認証済みユーザーID
/// - X-Request-Id: リクエスト追跡用 UUID
/// - X-Edge-Verified: Edge 検証用シークレット（Defense in Depth）
///
/// # 引数
/// * `req` - 元の HTTP リクエスト
/// * `user_id` - 認証で取得したユーザーID
///
/// # 戻り値
/// * `Response` - コア層からのレスポンス、またはエラーレスポンス
///
/// # エラー処理
/// コア層への接続に失敗した場合、502 Bad Gateway を返します。
async fn proxy_to_core(req: &Request, user_id: &str) -> Response {
    let path = req.path();
    let query = req.query();
    let url = if query.is_empty() {
        ::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}{1}", CORE_URL, path))
        })
    } else {
        ::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}{1}?{2}", CORE_URL, path, query))
        })
    };
    let request_id = Uuid::new_v4().to_string();
    {
        ::std::io::_print(
            format_args!(
                "[Gateway] Proxying {0} {1} -> {2} (request_id={3})\n",
                req.method(),
                path,
                url,
                request_id,
            ),
        );
    };
    let content_type = req
        .header("Content-Type")
        .and_then(|h| h.as_str())
        .unwrap_or("application/json");
    let body = req.body().to_vec();
    let outbound_req = Request::builder()
        .method(req.method().clone())
        .uri(&url)
        .header("Content-Type", content_type)
        .header("X-User-Id", user_id)
        .header("X-Request-Id", &request_id)
        .header("X-Edge-Verified", EDGE_SECRET)
        .body(body)
        .build();
    match spin_sdk::http::send::<_, Response>(outbound_req).await {
        Ok(response) => {
            let status = *response.status();
            let body = response.into_body();
            Response::builder()
                .status(status)
                .header("Content-Type", "application/json")
                .header("X-Request-Id", &request_id)
                .body(body)
                .build()
        }
        Err(e) => {
            {
                ::std::io::_print(
                    format_args!(
                        "[Gateway] Proxy error (request_id={0}): {1}\n",
                        request_id,
                        e,
                    ),
                );
            };
            let body = serde_json::to_string(
                    &ErrorResponse {
                        error: ::alloc::__export::must_use({
                            ::alloc::fmt::format(format_args!("Proxy error: {0}", e))
                        }),
                    },
                )
                .unwrap();
            Response::builder()
                .status(502)
                .header("Content-Type", "application/json")
                .header("X-Request-Id", &request_id)
                .body(body)
                .build()
        }
    }
}
/// パブリックパス（認証不要）かどうかを判定
///
/// # 引数
/// * `path` - リクエストパス
///
/// # 戻り値
/// * `bool` - パブリックパスの場合 true
fn is_public_path(path: &str) -> bool {
    PUBLIC_PATHS.iter().any(|&p| path == p)
}
/// パブリックパス用のコア層プロキシ
///
/// 認証不要のリクエストをコア層に転送します。
/// X-User-Id ヘッダーは付与しません。
///
/// # 引数
/// * `req` - 元の HTTP リクエスト
///
/// # 戻り値
/// * `Response` - コア層からのレスポンス、またはエラーレスポンス
async fn proxy_to_core_public(req: &Request) -> Response {
    let path = req.path();
    let query = req.query();
    let url = if query.is_empty() {
        ::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}{1}", CORE_URL, path))
        })
    } else {
        ::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("{0}{1}?{2}", CORE_URL, path, query))
        })
    };
    let request_id = Uuid::new_v4().to_string();
    {
        ::std::io::_print(
            format_args!(
                "[Gateway] Proxying public {0} {1} -> {2} (request_id={3})\n",
                req.method(),
                path,
                url,
                request_id,
            ),
        );
    };
    let content_type = req
        .header("Content-Type")
        .and_then(|h| h.as_str())
        .unwrap_or("application/json");
    let body = req.body().to_vec();
    let outbound_req = Request::builder()
        .method(req.method().clone())
        .uri(&url)
        .header("Content-Type", content_type)
        .header("X-Request-Id", &request_id)
        .body(body)
        .build();
    match spin_sdk::http::send::<_, Response>(outbound_req).await {
        Ok(response) => {
            let status = *response.status();
            let body = response.into_body();
            Response::builder()
                .status(status)
                .header("Content-Type", "application/json")
                .header("X-Request-Id", &request_id)
                .body(body)
                .build()
        }
        Err(e) => {
            {
                ::std::io::_print(
                    format_args!(
                        "[Gateway] Proxy error (request_id={0}): {1}\n",
                        request_id,
                        e,
                    ),
                );
            };
            let body = serde_json::to_string(
                    &ErrorResponse {
                        error: ::alloc::__export::must_use({
                            ::alloc::fmt::format(format_args!("Proxy error: {0}", e))
                        }),
                    },
                )
                .unwrap();
            Response::builder()
                .status(502)
                .header("Content-Type", "application/json")
                .header("X-Request-Id", &request_id)
                .body(body)
                .build()
        }
    }
}
