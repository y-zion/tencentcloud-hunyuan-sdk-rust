# TencentCloud Hunyuan SDK for Rust

[![Crates.io](https://img.shields.io/crates/v/tencentcloud-hunyuan-sdk.svg)](https://crates.io/crates/tencentcloud-hunyuan-sdk)
[![Documentation](https://docs.rs/tencentcloud-hunyuan-sdk/badge.svg)](https://docs.rs/tencentcloud-hunyuan-sdk)
[![License](https://img.shields.io/crates/l/tencentcloud-hunyuan-sdk.svg)](LICENSE)

A Rust SDK for TencentCloud Hunyuan, offering a full-featured, async-first interface to the Hunyuan API (v2023-09-01).

- Async-first (tokio)
- TC3-HMAC-SHA256 request signing
- Rustls TLS by default
- Includes a typed helper for `ChatCompletions` and a generic `call_action`

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
tencentcloud-hunyuan-sdk = "0.1.4"
```

## Quick Start

```rust
use tencentcloud_hunyuan_sdk::{Client, ClientBuilder, Credential, Region};
use tencentcloud_hunyuan_sdk::models::{ChatCompletionsRequest, Message};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Set credentials via env or read from a config vault
    let secret_id = std::env::var("TENCENTCLOUD_SECRET_ID")?;
    let secret_key = std::env::var("TENCENTCLOUD_SECRET_KEY")?;

    let client: Client = ClientBuilder::new()
        .credential(Credential { secret_id, secret_key, token: None })
        .region(Region::ApGuangzhou)
        .debug(true) // or set env var TENCENTCLOUD_SDK_DEBUG=true
        .build();

    let req = ChatCompletionsRequest {
        model: Some("hunyuan-lite".to_string()),
        messages: vec![
            Message { role: "user".into(), content: "Hello, Hunyuan!".into() },
        ],
        temperature: Some(0.7),
        top_p: Some(0.95),
        // Add more fields as needed per API
        stream: Some(false),
    };

    let resp = client.chat_completions(&req).await?;
    println!("{:?}", resp);
    Ok(())
}
```

Environment variables used in the example:

- `TENCENTCLOUD_SECRET_ID`
- `TENCENTCLOUD_SECRET_KEY`

Optionally, if you use temporary credentials, provide session token through `Credential { token: Some("...".into()), .. }` which is sent as `X-TC-Token`.

## Features

- **Client builder**: configure region and custom endpoint
- **TC3 signing**: compliant with TencentCloud TC3-HMAC-SHA256
- **Async HTTP**: `reqwest` with configurable TLS backends
- **Models**: request/response structs for `ChatCompletions` plus standard response envelope

## TLS Backends

This SDK provides two TLS backends for HTTP requests:

- **`rustls-tls`** (default): Uses the `rustls` TLS implementation. This is the default and recommended for most use cases.
- **`native-tls`**: Uses the system's native TLS implementation (OpenSSL on Linux/macOS, SChannel on Windows).

## Usage

### Default (rustls-tls)

```toml
[dependencies]
tencentcloud-hunyuan-sdk = "0.1.4"
```

### Using native-tls

```toml
[dependencies]
tencentcloud-hunyuan-sdk = { version = "0.1.4", default-features = false, features = ["native-tls"] }
```

## Docker Deployment

### Important: CA Certificates Required

You **must** ensure the container has proper CA certificates installed. The `rustls` or `native-tls` backend requires these to verify SSL certificates.

#### Install CA certificates in your Docker image

```dockerfile
# Debian/Ubuntu
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Alpine
RUN apk add --no-cache ca-certificates

# Copy from host (if using minimal images)
COPY /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
```

## Regions and Endpoints

The default endpoint is `hunyuan.tencentcloudapi.com` and the default region is `ap-guangzhou`.

```rust
use tencentcloud_hunyuan_sdk::{ClientBuilder, Credential, Region};

let client = ClientBuilder::new()
    .credential(Credential { secret_id, secret_key, token: None })
    .region(Region::ApBeijing) // or Region::ApGuangzhou, or Region::Custom("ap-shanghai".into())
    .build();
```

You can also override the endpoint:

```rust
let client = ClientBuilder::new()
    .credential(cred)
    .endpoint("hunyuan.tencentcloudapi.com")
    .build();
```

## Error Handling

Errors are returned as `SdkError` and include:
- HTTP/transport errors
- JSON serialization errors
- Service errors mapped from Tencent Cloud error payloads (`code`, `message`, `request_id`)

Example pattern:

```rust
match client.chat_completions(&req).await {
    Ok(resp) => println!("ok: {:?}", resp),
    Err(e) => eprintln!("error: {}", e),
}
```

## Debug Logging

You can enable SDK debug logs to print key request/response information with sensitive fields masked.

- Enable via builder: `ClientBuilder::new().debug(true)`
- Or via environment variable: set `TENCENTCLOUD_SDK_DEBUG=true` (also accepts `1`/`on`)

When enabled, the SDK prints:
- tc3_sign details: credential scope, hashes, and a masked signature
- Request summary: action, URL, region, presence of token
- Selected headers with masked `Authorization`
- Request body JSON
- Response status and body, and parsed error payloads if any

Note: Do not post debug logs publicly; while signatures and secrets are masked, request/response bodies may contain sensitive data.

## Generic Actions

Beyond the provided `chat_completions` helper, you can call any action supported by the Hunyuan API via the generic caller (exposed internally by the client). To add new typed actions, define the request/response models in `models.rs` and forward to `call_action("ActionName", &req)`.

Refer to the Go SDK models for exact shapes to mirror.

## Examples

Run the included example after exporting credentials:

```bash
export TENCENTCLOUD_SECRET_ID=... \
export TENCENTCLOUD_SECRET_KEY=...
cargo run --example chat
```

## Development

- Format and lint with your usual Rust toolchain
- Extend models under `src/models.rs`
- Add new helpers under `src/client.rs`

## License

This project uses the license provided in `LICENSE`.

## Acknowledgments

This SDK is based on the [TencentCloud Go SDK (Hunyuan v20230901)](https://github.com/TencentCloud/tencentcloud-sdk-go/tree/master/tencentcloud/hunyuan/v20230901) implementation by [Cursor](https://cursor.com) and follows the same API patterns and structure for consistency across different language SDKs.
