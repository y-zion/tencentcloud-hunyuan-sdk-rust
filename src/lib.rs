//! TencentCloud Hunyuan SDK for Rust
//!
//! Async client for Hunyuan (v2023-09-01) with TC3-HMAC-SHA256 signing.
//!
//! Features:
//! - Async HTTP via `reqwest`
//! - TC3 signing
//! - Typed helper for `ChatCompletions`
//!
//! Debug logging can be enabled with `ClientBuilder::debug(true)` or by setting the
//! environment variable `TENCENTCLOUD_SDK_DEBUG=true`. Sensitive values are masked
//! in logs, but request/response bodies may still contain sensitive data.
//!
//! Quick start example is available in the README and under `examples/chat.rs`.
pub mod client;
pub mod models;
pub mod signing;

pub use client::{Client, ClientBuilder, Credential, Region};
