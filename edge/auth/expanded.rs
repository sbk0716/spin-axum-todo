#![feature(prelude_import)]
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
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2021::*;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use hmac::{Hmac, Mac};
use serde::Deserialize;
use sha2::Sha256;
#[allow(dead_code, clippy::all)]
pub mod exports {
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
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
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
                            authenticated: ::core::clone::Clone::clone(
                                &self.authenticated,
                            ),
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
                #[doc(hidden)]
                #[allow(non_snake_case, unused_unsafe)]
                pub unsafe fn _export_verify_token_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                ) -> *mut u8 {
                    unsafe {
                        _rt::run_ctors_once();
                        let result1 = {
                            let len0 = arg1;
                            let bytes0 = _rt::Vec::from_raw_parts(
                                arg0.cast(),
                                len0,
                                len0,
                            );
                            T::verify_token(_rt::string_lift(bytes0))
                        };
                        let ptr2 = (&raw mut _RET_AREA.0).cast::<u8>();
                        let AuthResult {
                            authenticated: authenticated3,
                            user_id: user_id3,
                            error: error3,
                        } = result1;
                        *ptr2.add(0).cast::<u8>() = (match authenticated3 {
                            true => 1,
                            false => 0,
                        }) as u8;
                        match user_id3 {
                            Some(e) => {
                                *ptr2
                                    .add(::core::mem::size_of::<*const u8>())
                                    .cast::<u8>() = (1i32) as u8;
                                let vec4 = (e.into_bytes()).into_boxed_slice();
                                let ptr4 = vec4.as_ptr().cast::<u8>();
                                let len4 = vec4.len();
                                ::core::mem::forget(vec4);
                                *ptr2
                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                    .cast::<usize>() = len4;
                                *ptr2
                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                    .cast::<*mut u8>() = ptr4.cast_mut();
                            }
                            None => {
                                *ptr2
                                    .add(::core::mem::size_of::<*const u8>())
                                    .cast::<u8>() = (0i32) as u8;
                            }
                        };
                        match error3 {
                            Some(e) => {
                                *ptr2
                                    .add(4 * ::core::mem::size_of::<*const u8>())
                                    .cast::<u8>() = (1i32) as u8;
                                let vec5 = (e.into_bytes()).into_boxed_slice();
                                let ptr5 = vec5.as_ptr().cast::<u8>();
                                let len5 = vec5.len();
                                ::core::mem::forget(vec5);
                                *ptr2
                                    .add(6 * ::core::mem::size_of::<*const u8>())
                                    .cast::<usize>() = len5;
                                *ptr2
                                    .add(5 * ::core::mem::size_of::<*const u8>())
                                    .cast::<*mut u8>() = ptr5.cast_mut();
                            }
                            None => {
                                *ptr2
                                    .add(4 * ::core::mem::size_of::<*const u8>())
                                    .cast::<u8>() = (0i32) as u8;
                            }
                        };
                        ptr2
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_verify_token<T: Guest>(arg0: *mut u8) {
                    unsafe {
                        let l0 = i32::from(
                            *arg0.add(::core::mem::size_of::<*const u8>()).cast::<u8>(),
                        );
                        match l0 {
                            0 => {}
                            _ => {
                                let l1 = *arg0
                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                    .cast::<*mut u8>();
                                let l2 = *arg0
                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                    .cast::<usize>();
                                _rt::cabi_dealloc(l1, l2, 1);
                            }
                        }
                        let l3 = i32::from(
                            *arg0
                                .add(4 * ::core::mem::size_of::<*const u8>())
                                .cast::<u8>(),
                        );
                        match l3 {
                            0 => {}
                            _ => {
                                let l4 = *arg0
                                    .add(5 * ::core::mem::size_of::<*const u8>())
                                    .cast::<*mut u8>();
                                let l5 = *arg0
                                    .add(6 * ::core::mem::size_of::<*const u8>())
                                    .cast::<usize>();
                                _rt::cabi_dealloc(l4, l5, 1);
                            }
                        }
                    }
                }
                pub trait Guest {
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
                    fn verify_token(token: _rt::String) -> AuthResult;
                }
                #[doc(hidden)]
                pub(crate) use __export_demo_auth_authenticator_cabi;
                #[repr(align(4))]
                struct _RetArea(
                    [::core::mem::MaybeUninit<
                        u8,
                    >; 7 * ::core::mem::size_of::<*const u8>()],
                );
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 7
                        * ::core::mem::size_of::<*const u8>()],
                );
            }
        }
    }
}
mod _rt {
    #![allow(dead_code, unused_imports, clippy::all)]
    pub use alloc_crate::string::String;
    pub fn run_ctors_once() {
        wit_bindgen::rt::run_ctors_once();
    }
    pub use alloc_crate::vec::Vec;
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if true {
            String::from_utf8(bytes).unwrap()
        } else {
            unsafe { String::from_utf8_unchecked(bytes) }
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
    extern crate alloc as alloc_crate;
    pub use alloc_crate::alloc;
}
#[doc(inline)]
pub(crate) use __export_auth_world_impl as export;
#[unsafe(
    link_section = "component-type:wit-bindgen:0.51.0:demo:auth:auth-world:encoded world"
)]
#[doc(hidden)]
#[allow(clippy::octal_escapes)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 274] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\x91\x01\x01A\x02\x01\
A\x02\x01B\x05\x01ks\x01r\x03\x0dauthenticated\x7f\x07user-id\0\x05error\0\x04\0\
\x0bauth-result\x03\0\x01\x01@\x01\x05tokens\0\x02\x04\0\x0cverify-token\x01\x03\
\x04\0\x17demo:auth/authenticator\x05\0\x04\0\x14demo:auth/auth-world\x04\0\x0b\x10\
\x01\0\x0aauth-world\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit-comp\
onent\x070.244.0\x10wit-bindgen-rust\x060.51.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen::rt::maybe_link_cabi_realloc();
}
const _: &[u8] = b"// =============================================================================\n// WIT (WebAssembly Interface Types) \xe3\x82\xa4\xe3\x83\xb3\xe3\x82\xbf\xe3\x83\xbc\xe3\x83\x95\xe3\x82\xa7\xe3\x83\xbc\xe3\x82\xb9\xe5\xae\x9a\xe7\xbe\xa9\n// =============================================================================\n//\n// \xe3\x81\x93\xe3\x81\xae\xe3\x83\x95\xe3\x82\xa1\xe3\x82\xa4\xe3\x83\xab\xe3\x81\xaf Wasm \xe3\x82\xb3\xe3\x83\xb3\xe3\x83\x9d\xe3\x83\xbc\xe3\x83\x8d\xe3\x83\xb3\xe3\x83\x88\xe9\x96\x93\xe3\x81\xae\xe3\x82\xa4\xe3\x83\xb3\xe3\x82\xbf\xe3\x83\xbc\xe3\x83\x95\xe3\x82\xa7\xe3\x83\xbc\xe3\x82\xb9\xe3\x82\x92\xe5\xae\x9a\xe7\xbe\xa9\xe3\x81\x97\xe3\x81\xbe\xe3\x81\x99\xe3\x80\x82\n// WIT \xe3\x81\xaf Wasm \xe3\x82\xb3\xe3\x83\xb3\xe3\x83\x9d\xe3\x83\xbc\xe3\x83\x8d\xe3\x83\xb3\xe3\x83\x88\xe3\x83\xa2\xe3\x83\x87\xe3\x83\xab\xe3\x81\xae\xe4\xb8\x80\xe9\x83\xa8\xe3\x81\xa7\xe3\x80\x81\xe5\x9e\x8b\xe5\xae\x89\xe5\x85\xa8\xe3\x81\xaa\xe3\x82\xb3\xe3\x83\xb3\xe3\x83\x9d\xe3\x83\xbc\xe3\x83\x8d\xe3\x83\xb3\xe3\x83\x88\xe9\x96\x93\xe9\x80\x9a\xe4\xbf\xa1\xe3\x82\x92\xe5\x8f\xaf\xe8\x83\xbd\xe3\x81\xab\xe3\x81\x97\xe3\x81\xbe\xe3\x81\x99\xe3\x80\x82\n//\n// \xe5\x8f\x82\xe7\x85\xa7:\n// - WIT \xe4\xbb\x95\xe6\xa7\x98: https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md\n// - \xe3\x82\xb3\xe3\x83\xb3\xe3\x83\x9d\xe3\x83\xbc\xe3\x83\x8d\xe3\x83\xb3\xe3\x83\x88\xe3\x83\xa2\xe3\x83\x87\xe3\x83\xab: https://component-model.bytecodealliance.org/\n//\n// =============================================================================\n\n// -----------------------------------------------------------------------------\n// \xe3\x83\x91\xe3\x83\x83\xe3\x82\xb1\xe3\x83\xbc\xe3\x82\xb8\xe5\xae\xa3\xe8\xa8\x80\n// -----------------------------------------------------------------------------\n// \xe3\x83\x91\xe3\x83\x83\xe3\x82\xb1\xe3\x83\xbc\xe3\x82\xb8\xe5\x90\x8d\xe3\x81\xaf \"namespace:package\" \xe3\x81\xae\xe5\xbd\xa2\xe5\xbc\x8f\xe3\x81\xa7\xe6\x8c\x87\xe5\xae\x9a\n// - namespace: \xe7\xb5\x84\xe7\xb9\x94\xe3\x82\x84\xe3\x83\x97\xe3\x83\xad\xe3\x82\xb8\xe3\x82\xa7\xe3\x82\xaf\xe3\x83\x88\xe3\x82\x92\xe8\xad\x98\xe5\x88\xa5\xef\xbc\x88\xe4\xbe\x8b: wasi, demo\xef\xbc\x89\n// - package: \xe3\x83\x91\xe3\x83\x83\xe3\x82\xb1\xe3\x83\xbc\xe3\x82\xb8\xe5\x90\x8d\xef\xbc\x88\xe4\xbe\x8b: auth, http\xef\xbc\x89\npackage demo:auth;\n\n// =============================================================================\n// authenticator \xe3\x82\xa4\xe3\x83\xb3\xe3\x82\xbf\xe3\x83\xbc\xe3\x83\x95\xe3\x82\xa7\xe3\x83\xbc\xe3\x82\xb9\n// =============================================================================\n//\n// JWT \xe8\xaa\x8d\xe8\xa8\xbc\xe6\xa9\x9f\xe8\x83\xbd\xe3\x82\x92\xe6\x8f\x90\xe4\xbe\x9b\xe3\x81\x99\xe3\x82\x8b\xe3\x82\xa4\xe3\x83\xb3\xe3\x82\xbf\xe3\x83\xbc\xe3\x83\x95\xe3\x82\xa7\xe3\x83\xbc\xe3\x82\xb9\xe3\x81\xa7\xe3\x81\x99\xe3\x80\x82\n// auth \xe3\x82\xb3\xe3\x83\xb3\xe3\x83\x9d\xe3\x83\xbc\xe3\x83\x8d\xe3\x83\xb3\xe3\x83\x88\xe3\x81\x8c\xe3\x81\x93\xe3\x81\xae\xe3\x82\xa4\xe3\x83\xb3\xe3\x82\xbf\xe3\x83\xbc\xe3\x83\x95\xe3\x82\xa7\xe3\x83\xbc\xe3\x82\xb9\xe3\x82\x92\xe3\x82\xa8\xe3\x82\xaf\xe3\x82\xb9\xe3\x83\x9d\xe3\x83\xbc\xe3\x83\x88\xef\xbc\x88\xe5\xae\x9f\xe8\xa3\x85\xef\xbc\x89\xe3\x81\x97\xe3\x80\x81\n// gateway \xe3\x82\xb3\xe3\x83\xb3\xe3\x83\x9d\xe3\x83\xbc\xe3\x83\x8d\xe3\x83\xb3\xe3\x83\x88\xe3\x81\x8c\xe3\x82\xa4\xe3\x83\xb3\xe3\x83\x9d\xe3\x83\xbc\xe3\x83\x88\xef\xbc\x88\xe4\xbd\xbf\xe7\x94\xa8\xef\xbc\x89\xe3\x81\x97\xe3\x81\xbe\xe3\x81\x99\xe3\x80\x82\n\n/// \xe8\xaa\x8d\xe8\xa8\xbc\xe6\xa9\x9f\xe8\x83\xbd\xe3\x82\x92\xe6\x8f\x90\xe4\xbe\x9b\xe3\x81\x99\xe3\x82\x8b\xe3\x82\xa4\xe3\x83\xb3\xe3\x82\xbf\xe3\x83\xbc\xe3\x83\x95\xe3\x82\xa7\xe3\x83\xbc\xe3\x82\xb9\ninterface authenticator {\n    // -------------------------------------------------------------------------\n    // \xe3\x83\xac\xe3\x82\xb3\xe3\x83\xbc\xe3\x83\x89\xe5\x9e\x8b\xe3\x81\xae\xe5\xae\x9a\xe7\xbe\xa9\n    // -------------------------------------------------------------------------\n    //\n    // record \xe3\x81\xaf Rust \xe3\x81\xae struct\xe3\x80\x81TypeScript \xe3\x81\xae interface \xe3\x81\xab\xe7\x9b\xb8\xe5\xbd\x93\xe3\x81\x97\xe3\x81\xbe\xe3\x81\x99\xe3\x80\x82\n    // \xe3\x83\x95\xe3\x82\xa3\xe3\x83\xbc\xe3\x83\xab\xe3\x83\x89\xe3\x81\xaf\xe3\x82\xb1\xe3\x83\x90\xe3\x83\x96\xe3\x82\xb1\xe3\x83\xbc\xe3\x82\xb9\xef\xbc\x88kebab-case\xef\xbc\x89\xe3\x81\xa7\xe8\xa8\x98\xe8\xbf\xb0\xe3\x81\x97\xe3\x80\x81\n    // \xe7\x94\x9f\xe6\x88\x90\xe3\x81\x95\xe3\x82\x8c\xe3\x82\x8b\xe3\x82\xb3\xe3\x83\xbc\xe3\x83\x89\xe3\x81\xa7\xe3\x81\xaf\xe3\x82\xb9\xe3\x83\x8d\xe3\x83\xbc\xe3\x82\xaf\xe3\x82\xb1\xe3\x83\xbc\xe3\x82\xb9\xef\xbc\x88snake_case\xef\xbc\x89\xe3\x81\xab\xe5\xa4\x89\xe6\x8f\x9b\xe3\x81\x95\xe3\x82\x8c\xe3\x81\xbe\xe3\x81\x99\xe3\x80\x82\n\n    /// JWT \xe6\xa4\x9c\xe8\xa8\xbc\xe3\x81\xae\xe7\xb5\x90\xe6\x9e\x9c\xe3\x82\x92\xe8\xa1\xa8\xe3\x81\x99\xe3\x83\xac\xe3\x82\xb3\xe3\x83\xbc\xe3\x83\x89\xe5\x9e\x8b\n    ///\n    /// \xe8\xaa\x8d\xe8\xa8\xbc\xe3\x81\xae\xe6\x88\x90\xe5\x8a\x9f/\xe5\xa4\xb1\xe6\x95\x97\xe3\x82\x92\xe7\xa4\xba\xe3\x81\x97\xe3\x80\x81\xe6\x88\x90\xe5\x8a\x9f\xe6\x99\x82\xe3\x81\xaf\xe3\x83\xa6\xe3\x83\xbc\xe3\x82\xb6\xe3\x83\xbcID\xe3\x80\x81\xe5\xa4\xb1\xe6\x95\x97\xe6\x99\x82\xe3\x81\xaf\xe3\x82\xa8\xe3\x83\xa9\xe3\x83\xbc\xe3\x83\xa1\xe3\x83\x83\xe3\x82\xbb\xe3\x83\xbc\xe3\x82\xb8\xe3\x82\x92\xe5\x90\xab\xe3\x81\xbf\xe3\x81\xbe\xe3\x81\x99\xe3\x80\x82\n    record auth-result {\n        /// \xe8\xaa\x8d\xe8\xa8\xbc\xe3\x81\x8c\xe6\x88\x90\xe5\x8a\x9f\xe3\x81\x97\xe3\x81\x9f\xe3\x81\x8b\xe3\x81\xa9\xe3\x81\x86\xe3\x81\x8b\n        /// - true: JWT \xe3\x81\x8c\xe6\x9c\x89\xe5\x8a\xb9\xe3\x81\xa7\xe8\xaa\x8d\xe8\xa8\xbc\xe6\x88\x90\xe5\x8a\x9f\n        /// - false: JWT \xe3\x81\x8c\xe7\x84\xa1\xe5\x8a\xb9\xe3\x81\xbe\xe3\x81\x9f\xe3\x81\xaf\xe6\x9c\x9f\xe9\x99\x90\xe5\x88\x87\xe3\x82\x8c\xe3\x81\xa7\xe8\xaa\x8d\xe8\xa8\xbc\xe5\xa4\xb1\xe6\x95\x97\n        authenticated: bool,\n\n        /// \xe8\xaa\x8d\xe8\xa8\xbc\xe6\x88\x90\xe5\x8a\x9f\xe6\x99\x82\xe3\x81\xae\xe3\x83\xa6\xe3\x83\xbc\xe3\x82\xb6\xe3\x83\xbcID\n        /// JWT \xe3\x81\xae sub (Subject) \xe3\x82\xaf\xe3\x83\xac\xe3\x83\xbc\xe3\x83\xa0\xe3\x81\x8b\xe3\x82\x89\xe6\x8a\xbd\xe5\x87\xba\xe3\x81\x97\xe3\x81\x9f\xe5\x80\xa4\n        /// \xe8\xaa\x8d\xe8\xa8\xbc\xe5\xa4\xb1\xe6\x95\x97\xe6\x99\x82\xe3\x81\xaf None (null)\n        user-id: option<string>,\n\n        /// \xe8\xaa\x8d\xe8\xa8\xbc\xe5\xa4\xb1\xe6\x95\x97\xe6\x99\x82\xe3\x81\xae\xe3\x82\xa8\xe3\x83\xa9\xe3\x83\xbc\xe3\x83\xa1\xe3\x83\x83\xe3\x82\xbb\xe3\x83\xbc\xe3\x82\xb8\n        /// \xe4\xbe\x8b: \"Missing token\", \"Invalid signature\", \"Token expired\"\n        /// \xe8\xaa\x8d\xe8\xa8\xbc\xe6\x88\x90\xe5\x8a\x9f\xe6\x99\x82\xe3\x81\xaf None (null)\n        error: option<string>,\n    }\n\n    // -------------------------------------------------------------------------\n    // \xe9\x96\xa2\xe6\x95\xb0\xe3\x81\xae\xe5\xae\x9a\xe7\xbe\xa9\n    // -------------------------------------------------------------------------\n    //\n    // func \xe3\x82\xad\xe3\x83\xbc\xe3\x83\xaf\xe3\x83\xbc\xe3\x83\x89\xe3\x81\xa7\xe9\x96\xa2\xe6\x95\xb0\xe3\x82\x92\xe5\xae\x9a\xe7\xbe\xa9\xe3\x81\x97\xe3\x81\xbe\xe3\x81\x99\xe3\x80\x82\n    // \xe5\xbc\x95\xe6\x95\xb0\xe3\x81\xa8\xe6\x88\xbb\xe3\x82\x8a\xe5\x80\xa4\xe3\x81\xae\xe5\x9e\x8b\xe3\x82\x92\xe6\x8c\x87\xe5\xae\x9a\xe3\x81\x97\xe3\x80\x81\xe5\xae\x9f\xe8\xa3\x85\xe3\x81\xaf\xe5\x90\x84\xe3\x82\xb3\xe3\x83\xb3\xe3\x83\x9d\xe3\x83\xbc\xe3\x83\x8d\xe3\x83\xb3\xe3\x83\x88\xe3\x81\xa7\xe8\xa1\x8c\xe3\x81\x84\xe3\x81\xbe\xe3\x81\x99\xe3\x80\x82\n\n    /// JWT \xe3\x83\x88\xe3\x83\xbc\xe3\x82\xaf\xe3\x83\xb3\xe3\x82\x92\xe6\xa4\x9c\xe8\xa8\xbc\xe3\x81\x99\xe3\x82\x8b\n    ///\n    /// Authorization \xe3\x83\x98\xe3\x83\x83\xe3\x83\x80\xe3\x83\xbc\xe3\x81\x8b\xe3\x82\x89\xe5\x8f\x96\xe5\xbe\x97\xe3\x81\x97\xe3\x81\x9f Bearer \xe3\x83\x88\xe3\x83\xbc\xe3\x82\xaf\xe3\x83\xb3\xe3\x82\x92\xe6\xa4\x9c\xe8\xa8\xbc\xe3\x81\x97\xe3\x80\x81\n    /// \xe8\xaa\x8d\xe8\xa8\xbc\xe7\xb5\x90\xe6\x9e\x9c\xe3\x82\x92\xe8\xbf\x94\xe3\x81\x97\xe3\x81\xbe\xe3\x81\x99\xe3\x80\x82\n    ///\n    /// # \xe5\xbc\x95\xe6\x95\xb0\n    /// * `token` - \xe6\xa4\x9c\xe8\xa8\xbc\xe5\xaf\xbe\xe8\xb1\xa1\xe3\x81\xae JWT \xe3\x83\x88\xe3\x83\xbc\xe3\x82\xaf\xe3\x83\xb3\xe6\x96\x87\xe5\xad\x97\xe5\x88\x97\n    ///   - \xe7\xa9\xba\xe6\x96\x87\xe5\xad\x97\xe5\x88\x97\xe3\x81\xae\xe5\xa0\xb4\xe5\x90\x88\xe3\x80\x81\"Missing token\" \xe3\x82\xa8\xe3\x83\xa9\xe3\x83\xbc\n    ///   - \xe4\xb8\x8d\xe6\xad\xa3\xe3\x81\xaa\xe5\xbd\xa2\xe5\xbc\x8f\xe3\x81\xae\xe5\xa0\xb4\xe5\x90\x88\xe3\x80\x81\"Invalid token format\" \xe3\x82\xa8\xe3\x83\xa9\xe3\x83\xbc\n    ///\n    /// # \xe6\x88\xbb\xe3\x82\x8a\xe5\x80\xa4\n    /// * `auth-result` - \xe8\xaa\x8d\xe8\xa8\xbc\xe7\xb5\x90\xe6\x9e\x9c\n    ///   - \xe6\x88\x90\xe5\x8a\x9f\xe6\x99\x82: authenticated=true, user-id=Some(\xe3\x83\xa6\xe3\x83\xbc\xe3\x82\xb6\xe3\x83\xbcID)\n    ///   - \xe5\xa4\xb1\xe6\x95\x97\xe6\x99\x82: authenticated=false, error=Some(\xe3\x82\xa8\xe3\x83\xa9\xe3\x83\xbc\xe3\x83\xa1\xe3\x83\x83\xe3\x82\xbb\xe3\x83\xbc\xe3\x82\xb8)\n    ///\n    /// # \xe6\xa4\x9c\xe8\xa8\xbc\xe5\x86\x85\xe5\xae\xb9\n    /// 1. \xe3\x83\x88\xe3\x83\xbc\xe3\x82\xaf\xe3\x83\xb3\xe3\x81\x8c\xe7\xa9\xba\xe3\x81\xa7\xe3\x81\xaa\xe3\x81\x84\xe3\x81\x93\xe3\x81\xa8\n    /// 2. JWT \xe3\x83\x95\xe3\x82\xa9\xe3\x83\xbc\xe3\x83\x9e\xe3\x83\x83\xe3\x83\x88\xef\xbc\x88header.payload.signature\xef\xbc\x89\xe3\x81\xa7\xe3\x81\x82\xe3\x82\x8b\xe3\x81\x93\xe3\x81\xa8\n    /// 3. \xe3\x82\xa2\xe3\x83\xab\xe3\x82\xb4\xe3\x83\xaa\xe3\x82\xba\xe3\x83\xa0\xe3\x81\x8c HS256 \xe3\x81\xa7\xe3\x81\x82\xe3\x82\x8b\xe3\x81\x93\xe3\x81\xa8\n    /// 4. HMAC-SHA256 \xe7\xbd\xb2\xe5\x90\x8d\xe3\x81\x8c\xe6\xad\xa3\xe3\x81\x97\xe3\x81\x84\xe3\x81\x93\xe3\x81\xa8\n    /// 5. sub \xe3\x82\xaf\xe3\x83\xac\xe3\x83\xbc\xe3\x83\xa0\xe3\x81\x8c\xe5\xad\x98\xe5\x9c\xa8\xe3\x81\x99\xe3\x82\x8b\xe3\x81\x93\xe3\x81\xa8\n    verify-token: func(token: string) -> auth-result;\n}\n\n// =============================================================================\n// World \xe5\xae\x9a\xe7\xbe\xa9\n// =============================================================================\n//\n// World \xe3\x81\xaf\xe3\x82\xb3\xe3\x83\xb3\xe3\x83\x9d\xe3\x83\xbc\xe3\x83\x8d\xe3\x83\xb3\xe3\x83\x88\xe3\x81\xae\xe3\x80\x8c\xe9\xa1\x94\xe3\x80\x8d\xe3\x82\x92\xe5\xae\x9a\xe7\xbe\xa9\xe3\x81\x97\xe3\x81\xbe\xe3\x81\x99\xe3\x80\x82\n// - export: \xe3\x82\xb3\xe3\x83\xb3\xe3\x83\x9d\xe3\x83\xbc\xe3\x83\x8d\xe3\x83\xb3\xe3\x83\x88\xe3\x81\x8c\xe5\xa4\x96\xe9\x83\xa8\xe3\x81\xab\xe6\x8f\x90\xe4\xbe\x9b\xe3\x81\x99\xe3\x82\x8b\xe3\x82\xa4\xe3\x83\xb3\xe3\x82\xbf\xe3\x83\xbc\xe3\x83\x95\xe3\x82\xa7\xe3\x83\xbc\xe3\x82\xb9\n// - import: \xe3\x82\xb3\xe3\x83\xb3\xe3\x83\x9d\xe3\x83\xbc\xe3\x83\x8d\xe3\x83\xb3\xe3\x83\x88\xe3\x81\x8c\xe5\xa4\x96\xe9\x83\xa8\xe3\x81\x8b\xe3\x82\x89\xe4\xbd\xbf\xe7\x94\xa8\xe3\x81\x99\xe3\x82\x8b\xe3\x82\xa4\xe3\x83\xb3\xe3\x82\xbf\xe3\x83\xbc\xe3\x83\x95\xe3\x82\xa7\xe3\x83\xbc\xe3\x82\xb9\n//\n// \xe5\x90\x84 Wasm \xe3\x82\xb3\xe3\x83\xb3\xe3\x83\x9d\xe3\x83\xbc\xe3\x83\x8d\xe3\x83\xb3\xe3\x83\x88\xe3\x81\xaf 1 \xe3\x81\xa4\xe3\x81\xae world \xe3\x82\x92\xe5\xae\x9f\xe8\xa3\x85\xe3\x81\x97\xe3\x81\xbe\xe3\x81\x99\xe3\x80\x82\n\n/// auth \xe3\x82\xb3\xe3\x83\xb3\xe3\x83\x9d\xe3\x83\xbc\xe3\x83\x8d\xe3\x83\xb3\xe3\x83\x88\xe3\x81\x8c\xe5\xae\x9f\xe8\xa3\x85\xe3\x81\x99\xe3\x82\x8b world\n///\n/// \xe3\x81\x93\xe3\x81\xae\xe3\x82\xb3\xe3\x83\xb3\xe3\x83\x9d\xe3\x83\xbc\xe3\x83\x8d\xe3\x83\xb3\xe3\x83\x88\xe3\x81\xaf authenticator \xe3\x82\xa4\xe3\x83\xb3\xe3\x82\xbf\xe3\x83\xbc\xe3\x83\x95\xe3\x82\xa7\xe3\x83\xbc\xe3\x82\xb9\xe3\x82\x92\xe3\x82\xa8\xe3\x82\xaf\xe3\x82\xb9\xe3\x83\x9d\xe3\x83\xbc\xe3\x83\x88\xe3\x81\x97\xe3\x80\x81\n/// JWT \xe6\xa4\x9c\xe8\xa8\xbc\xe6\xa9\x9f\xe8\x83\xbd\xe3\x82\x92\xe5\xa4\x96\xe9\x83\xa8\xe3\x81\xab\xe6\x8f\x90\xe4\xbe\x9b\xe3\x81\x97\xe3\x81\xbe\xe3\x81\x99\xe3\x80\x82\n///\n/// \xe4\xbd\xbf\xe7\x94\xa8\xe4\xbe\x8b:\n/// - gateway \xe3\x82\xb3\xe3\x83\xb3\xe3\x83\x9d\xe3\x83\xbc\xe3\x83\x8d\xe3\x83\xb3\xe3\x83\x88\xe3\x81\x8b\xe3\x82\x89 verify_token \xe9\x96\xa2\xe6\x95\xb0\xe3\x81\x8c\xe5\x91\xbc\xe3\x81\xb3\xe5\x87\xba\xe3\x81\x95\xe3\x82\x8c\xe3\x82\x8b\nworld auth-world {\n    // authenticator \xe3\x82\xa4\xe3\x83\xb3\xe3\x82\xbf\xe3\x83\xbc\xe3\x83\x95\xe3\x82\xa7\xe3\x83\xbc\xe3\x82\xb9\xe3\x82\x92\xe5\xa4\x96\xe9\x83\xa8\xe3\x81\xab\xe5\x85\xac\xe9\x96\x8b\n    // \xe3\x81\x93\xe3\x82\x8c\xe3\x81\xab\xe3\x82\x88\xe3\x82\x8a\xe3\x80\x81\xe4\xbb\x96\xe3\x81\xae\xe3\x82\xb3\xe3\x83\xb3\xe3\x83\x9d\xe3\x83\xbc\xe3\x83\x8d\xe3\x83\xb3\xe3\x83\x88\xe3\x81\x8c\xe3\x81\x93\xe3\x81\xae\xe3\x82\xa4\xe3\x83\xb3\xe3\x82\xbf\xe3\x83\xbc\xe3\x83\x95\xe3\x82\xa7\xe3\x83\xbc\xe3\x82\xb9\xe3\x82\x92\xe4\xbd\xbf\xe7\x94\xa8\xe5\x8f\xaf\xe8\x83\xbd\n    export authenticator;\n}\n\n/// gateway \xe3\x82\xb3\xe3\x83\xb3\xe3\x83\x9d\xe3\x83\xbc\xe3\x83\x8d\xe3\x83\xb3\xe3\x83\x88\xe3\x81\x8c\xe5\xae\x9f\xe8\xa3\x85\xe3\x81\x99\xe3\x82\x8b world\n///\n/// \xe3\x81\x93\xe3\x81\xae\xe3\x82\xb3\xe3\x83\xb3\xe3\x83\x9d\xe3\x83\xbc\xe3\x83\x8d\xe3\x83\xb3\xe3\x83\x88\xe3\x81\xaf authenticator \xe3\x82\xa4\xe3\x83\xb3\xe3\x82\xbf\xe3\x83\xbc\xe3\x83\x95\xe3\x82\xa7\xe3\x83\xbc\xe3\x82\xb9\xe3\x82\x92\xe3\x82\xa4\xe3\x83\xb3\xe3\x83\x9d\xe3\x83\xbc\xe3\x83\x88\xe3\x81\x97\xe3\x80\x81\n/// auth \xe3\x82\xb3\xe3\x83\xb3\xe3\x83\x9d\xe3\x83\xbc\xe3\x83\x8d\xe3\x83\xb3\xe3\x83\x88\xe3\x81\xae\xe6\xa9\x9f\xe8\x83\xbd\xe3\x82\x92\xe5\x88\xa9\xe7\x94\xa8\xe3\x81\x97\xe3\x81\xbe\xe3\x81\x99\xe3\x80\x82\n///\n/// \xe4\xbd\xbf\xe7\x94\xa8\xe4\xbe\x8b:\n/// - HTTP \xe3\x83\xaa\xe3\x82\xaf\xe3\x82\xa8\xe3\x82\xb9\xe3\x83\x88\xe3\x82\x92\xe5\x8f\x97\xe4\xbf\xa1\n/// - auth \xe3\x82\xb3\xe3\x83\xb3\xe3\x83\x9d\xe3\x83\xbc\xe3\x83\x8d\xe3\x83\xb3\xe3\x83\x88\xe3\x81\xae verify_token \xe3\x82\x92\xe5\x91\xbc\xe3\x81\xb3\xe5\x87\xba\xe3\x81\x97\xe3\x81\xa6\xe8\xaa\x8d\xe8\xa8\xbc\n/// - \xe8\xaa\x8d\xe8\xa8\xbc\xe6\x88\x90\xe5\x8a\x9f\xe6\x99\x82\xe3\x80\x81\xe3\x82\xb3\xe3\x82\xa2\xe5\xb1\xa4\xe3\x81\xab\xe3\x83\x97\xe3\x83\xad\xe3\x82\xad\xe3\x82\xb7\nworld gateway-world {\n    // authenticator \xe3\x82\xa4\xe3\x83\xb3\xe3\x82\xbf\xe3\x83\xbc\xe3\x83\x95\xe3\x82\xa7\xe3\x83\xbc\xe3\x82\xb9\xe3\x82\x92\xe4\xbd\xbf\xe7\x94\xa8\n    // Spin \xe3\x81\xae\xe3\x82\xb3\xe3\x83\xb3\xe3\x83\x9d\xe3\x83\xbc\xe3\x83\x8d\xe3\x83\xb3\xe3\x83\x88\xe5\x90\x88\xe6\x88\x90\xe3\x81\xab\xe3\x82\x88\xe3\x82\x8a\xe3\x80\x81auth \xe3\x82\xb3\xe3\x83\xb3\xe3\x83\x9d\xe3\x83\xbc\xe3\x83\x8d\xe3\x83\xb3\xe3\x83\x88\xe3\x81\xab\xe6\x8e\xa5\xe7\xb6\x9a\xe3\x81\x95\xe3\x82\x8c\xe3\x82\x8b\n    import authenticator;\n}\n";
use exports::demo::auth::authenticator::{AuthResult, Guest};
/// JWT 署名検証用の秘密鍵
///
/// 注意: これはデモ用のハードコードされた秘密鍵です。
/// 本番環境では以下の方法で安全に管理してください：
/// - 環境変数から取得
/// - シークレット管理サービス（HashiCorp Vault など）を使用
/// - Spin の変数機能を使用
const SECRET_KEY: &[u8] = b"super-secret-key";
/// JWT ヘッダーを表す構造体
///
/// JWT のヘッダー部分には、トークンのタイプと署名アルゴリズムが含まれます。
/// 例: {"alg":"HS256","typ":"JWT"}
struct JwtHeader {
    /// 署名アルゴリズム（例: "HS256", "RS256"）
    /// このデモでは HS256 のみサポート
    alg: String,
    /// トークンタイプ（通常は "JWT"）
    /// 検証には使用しないため、dead_code 警告を抑制
    #[allow(dead_code)]
    typ: Option<String>,
}
#[automatically_derived]
impl ::core::fmt::Debug for JwtHeader {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "JwtHeader",
            "alg",
            &self.alg,
            "typ",
            &&self.typ,
        )
    }
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
    impl<'de> _serde::Deserialize<'de> for JwtHeader {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private228::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __ignore,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private228::Formatter,
                ) -> _serde::__private228::fmt::Result {
                    _serde::__private228::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private228::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private228::Ok(__Field::__field0),
                        1u64 => _serde::__private228::Ok(__Field::__field1),
                        _ => _serde::__private228::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private228::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "alg" => _serde::__private228::Ok(__Field::__field0),
                        "typ" => _serde::__private228::Ok(__Field::__field1),
                        _ => _serde::__private228::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private228::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"alg" => _serde::__private228::Ok(__Field::__field0),
                        b"typ" => _serde::__private228::Ok(__Field::__field1),
                        _ => _serde::__private228::Ok(__Field::__ignore),
                    }
                }
            }
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private228::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private228::PhantomData<JwtHeader>,
                lifetime: _serde::__private228::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = JwtHeader;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private228::Formatter,
                ) -> _serde::__private228::fmt::Result {
                    _serde::__private228::Formatter::write_str(
                        __formatter,
                        "struct JwtHeader",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private228::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<
                        String,
                    >(&mut __seq)? {
                        _serde::__private228::Some(__value) => __value,
                        _serde::__private228::None => {
                            return _serde::__private228::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct JwtHeader with 2 elements",
                                ),
                            );
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<
                        Option<String>,
                    >(&mut __seq)? {
                        _serde::__private228::Some(__value) => __value,
                        _serde::__private228::None => {
                            return _serde::__private228::Err(
                                _serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct JwtHeader with 2 elements",
                                ),
                            );
                        }
                    };
                    _serde::__private228::Ok(JwtHeader {
                        alg: __field0,
                        typ: __field1,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private228::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private228::Option<String> = _serde::__private228::None;
                    let mut __field1: _serde::__private228::Option<Option<String>> = _serde::__private228::None;
                    while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                        __Field,
                    >(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private228::Option::is_some(&__field0) {
                                    return _serde::__private228::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("alg"),
                                    );
                                }
                                __field0 = _serde::__private228::Some(
                                    _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private228::Option::is_some(&__field1) {
                                    return _serde::__private228::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("typ"),
                                    );
                                }
                                __field1 = _serde::__private228::Some(
                                    _serde::de::MapAccess::next_value::<
                                        Option<String>,
                                    >(&mut __map)?,
                                );
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private228::Some(__field0) => __field0,
                        _serde::__private228::None => {
                            _serde::__private228::de::missing_field("alg")?
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private228::Some(__field1) => __field1,
                        _serde::__private228::None => {
                            _serde::__private228::de::missing_field("typ")?
                        }
                    };
                    _serde::__private228::Ok(JwtHeader {
                        alg: __field0,
                        typ: __field1,
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["alg", "typ"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "JwtHeader",
                FIELDS,
                __Visitor {
                    marker: _serde::__private228::PhantomData::<JwtHeader>,
                    lifetime: _serde::__private228::PhantomData,
                },
            )
        }
    }
};
/// JWT ペイロード（クレーム）を表す構造体
///
/// JWT のペイロード部分には、トークンに関する情報（クレーム）が含まれます。
/// 標準クレーム（RFC 7519）の一部を定義しています。
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
#[automatically_derived]
impl ::core::fmt::Debug for JwtPayload {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "JwtPayload",
            "sub",
            &self.sub,
            "exp",
            &self.exp,
            "iat",
            &&self.iat,
        )
    }
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
    impl<'de> _serde::Deserialize<'de> for JwtPayload {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private228::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __ignore,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private228::Formatter,
                ) -> _serde::__private228::fmt::Result {
                    _serde::__private228::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private228::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private228::Ok(__Field::__field0),
                        1u64 => _serde::__private228::Ok(__Field::__field1),
                        2u64 => _serde::__private228::Ok(__Field::__field2),
                        _ => _serde::__private228::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private228::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "sub" => _serde::__private228::Ok(__Field::__field0),
                        "exp" => _serde::__private228::Ok(__Field::__field1),
                        "iat" => _serde::__private228::Ok(__Field::__field2),
                        _ => _serde::__private228::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private228::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"sub" => _serde::__private228::Ok(__Field::__field0),
                        b"exp" => _serde::__private228::Ok(__Field::__field1),
                        b"iat" => _serde::__private228::Ok(__Field::__field2),
                        _ => _serde::__private228::Ok(__Field::__ignore),
                    }
                }
            }
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private228::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private228::PhantomData<JwtPayload>,
                lifetime: _serde::__private228::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = JwtPayload;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private228::Formatter,
                ) -> _serde::__private228::fmt::Result {
                    _serde::__private228::Formatter::write_str(
                        __formatter,
                        "struct JwtPayload",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private228::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<
                        Option<String>,
                    >(&mut __seq)? {
                        _serde::__private228::Some(__value) => __value,
                        _serde::__private228::None => {
                            return _serde::__private228::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct JwtPayload with 3 elements",
                                ),
                            );
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<
                        Option<u64>,
                    >(&mut __seq)? {
                        _serde::__private228::Some(__value) => __value,
                        _serde::__private228::None => {
                            return _serde::__private228::Err(
                                _serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct JwtPayload with 3 elements",
                                ),
                            );
                        }
                    };
                    let __field2 = match _serde::de::SeqAccess::next_element::<
                        Option<u64>,
                    >(&mut __seq)? {
                        _serde::__private228::Some(__value) => __value,
                        _serde::__private228::None => {
                            return _serde::__private228::Err(
                                _serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct JwtPayload with 3 elements",
                                ),
                            );
                        }
                    };
                    _serde::__private228::Ok(JwtPayload {
                        sub: __field0,
                        exp: __field1,
                        iat: __field2,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private228::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private228::Option<Option<String>> = _serde::__private228::None;
                    let mut __field1: _serde::__private228::Option<Option<u64>> = _serde::__private228::None;
                    let mut __field2: _serde::__private228::Option<Option<u64>> = _serde::__private228::None;
                    while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                        __Field,
                    >(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private228::Option::is_some(&__field0) {
                                    return _serde::__private228::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("sub"),
                                    );
                                }
                                __field0 = _serde::__private228::Some(
                                    _serde::de::MapAccess::next_value::<
                                        Option<String>,
                                    >(&mut __map)?,
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private228::Option::is_some(&__field1) {
                                    return _serde::__private228::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("exp"),
                                    );
                                }
                                __field1 = _serde::__private228::Some(
                                    _serde::de::MapAccess::next_value::<
                                        Option<u64>,
                                    >(&mut __map)?,
                                );
                            }
                            __Field::__field2 => {
                                if _serde::__private228::Option::is_some(&__field2) {
                                    return _serde::__private228::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("iat"),
                                    );
                                }
                                __field2 = _serde::__private228::Some(
                                    _serde::de::MapAccess::next_value::<
                                        Option<u64>,
                                    >(&mut __map)?,
                                );
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private228::Some(__field0) => __field0,
                        _serde::__private228::None => {
                            _serde::__private228::de::missing_field("sub")?
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private228::Some(__field1) => __field1,
                        _serde::__private228::None => {
                            _serde::__private228::de::missing_field("exp")?
                        }
                    };
                    let __field2 = match __field2 {
                        _serde::__private228::Some(__field2) => __field2,
                        _serde::__private228::None => {
                            _serde::__private228::de::missing_field("iat")?
                        }
                    };
                    _serde::__private228::Ok(JwtPayload {
                        sub: __field0,
                        exp: __field1,
                        iat: __field2,
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["sub", "exp", "iat"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "JwtPayload",
                FIELDS,
                __Visitor {
                    marker: _serde::__private228::PhantomData::<JwtPayload>,
                    lifetime: _serde::__private228::PhantomData,
                },
            )
        }
    }
};
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
        match verify_jwt(&token) {
            Ok(user_id) => {
                AuthResult {
                    authenticated: true,
                    user_id: Some(user_id),
                    error: None,
                }
            }
            Err(e) => {
                AuthResult {
                    authenticated: false,
                    user_id: None,
                    error: Some(e),
                }
            }
        }
    }
}
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
    if token.is_empty() {
        return Err("Missing token".to_string());
    }
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return Err("Invalid token format".to_string());
    }
    let header_b64 = parts[0];
    let payload_b64 = parts[1];
    let signature_b64 = parts[2];
    let header_json = URL_SAFE_NO_PAD
        .decode(header_b64)
        .map_err(|_| "Invalid header encoding".to_string())?;
    let header: JwtHeader = serde_json::from_slice(&header_json)
        .map_err(|_| "Invalid header JSON".to_string())?;
    if header.alg != "HS256" {
        return Err(
            ::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!("Unsupported algorithm: {0}", header.alg),
                )
            }),
        );
    }
    let signature = URL_SAFE_NO_PAD
        .decode(signature_b64)
        .map_err(|_| "Invalid signature encoding".to_string())?;
    let message = ::alloc::__export::must_use({
        ::alloc::fmt::format(format_args!("{0}.{1}", header_b64, payload_b64))
    });
    verify_signature(&message, &signature)?;
    let payload_json = URL_SAFE_NO_PAD
        .decode(payload_b64)
        .map_err(|_| "Invalid payload encoding".to_string())?;
    let payload: JwtPayload = serde_json::from_slice(&payload_json)
        .map_err(|_| "Invalid payload JSON".to_string())?;
    if let Some(exp) = payload.exp {
        if exp < 1577836800 {
            return Err("Token expired".to_string());
        }
    }
    payload.sub.ok_or_else(|| "Missing subject claim".to_string())
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
    type HmacSha256 = Hmac<Sha256>;
    let mut mac = HmacSha256::new_from_slice(SECRET_KEY)
        .map_err(|_| "Invalid key length".to_string())?;
    mac.update(message.as_bytes());
    mac.verify_slice(signature).map_err(|_| "Invalid signature".to_string())
}
const _: () = {
    #[unsafe(export_name = "demo:auth/authenticator#verify-token")]
    unsafe extern "C" fn export_verify_token(arg0: *mut u8, arg1: usize) -> *mut u8 {
        unsafe {
            self::exports::demo::auth::authenticator::_export_verify_token_cabi::<
                AuthComponent,
            >(arg0, arg1)
        }
    }
    #[unsafe(export_name = "cabi_post_demo:auth/authenticator#verify-token")]
    unsafe extern "C" fn _post_return_verify_token(arg0: *mut u8) {
        unsafe {
            self::exports::demo::auth::authenticator::__post_return_verify_token::<
                AuthComponent,
            >(arg0)
        }
    }
};
