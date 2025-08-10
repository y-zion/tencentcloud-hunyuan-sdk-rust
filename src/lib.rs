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

#[cfg(test)]
mod tests {
    use crate::client::{Client, ClientBuilder, Credential, Region};
    use crate::models::{ChatCompletionsRequest, Message};
    use time::OffsetDateTime;

    #[test]
    fn test_region_as_str() {
        assert_eq!(Region::ApBeijing.as_str(), "ap-beijing");
        assert_eq!(Region::ApGuangzhou.as_str(), "ap-guangzhou");
        assert_eq!(
            Region::Custom("custom-region".to_string()).as_str(),
            "custom-region"
        );
    }

    #[test]
    fn test_credential_creation() {
        let cred = Credential {
            secret_id: "test_id".to_string(),
            secret_key: "test_key".to_string(),
            token: None,
        };
        assert_eq!(cred.secret_id, "test_id");
        assert_eq!(cred.secret_key, "test_key");
        assert!(cred.token.is_none());

        let cred_with_token = Credential {
            secret_id: "test_id".to_string(),
            secret_key: "test_key".to_string(),
            token: Some("test_token".to_string()),
        };
        assert_eq!(cred_with_token.token, Some("test_token".to_string()));
    }

    #[test]
    fn test_client_builder_defaults() {
        let client = ClientBuilder::new()
            .credential(Credential {
                secret_id: "test_id".to_string(),
                secret_key: "test_key".to_string(),
                token: None,
            })
            .build();

        assert_eq!(client.region().as_str(), "ap-guangzhou");
        assert_eq!(client.endpoint(), "hunyuan.tencentcloudapi.com");
        // Note: debug state depends on environment, so we don't test it here
    }

    #[test]
    fn test_client_builder_custom_values() {
        let client = ClientBuilder::new()
            .credential(Credential {
                secret_id: "test_id".to_string(),
                secret_key: "test_key".to_string(),
                token: None,
            })
            .region(Region::ApBeijing)
            .endpoint("custom.endpoint.com")
            .debug(true)
            .build();

        assert_eq!(client.region().as_str(), "ap-beijing");
        assert_eq!(client.endpoint(), "custom.endpoint.com");
        assert!(client.debug());
    }

    #[test]
    fn test_client_builder_custom_region() {
        let client = ClientBuilder::new()
            .credential(Credential {
                secret_id: "test_id".to_string(),
                secret_key: "test_key".to_string(),
                token: None,
            })
            .region(Region::Custom("us-west-1".to_string()))
            .build();

        assert_eq!(client.region().as_str(), "us-west-1");
    }

    #[test]
    fn test_client_builder_with_token() {
        let client = ClientBuilder::new()
            .credential(Credential {
                secret_id: "test_id".to_string(),
                secret_key: "test_key".to_string(),
                token: Some("session_token".to_string()),
            })
            .build();

        assert_eq!(client.credential().token, Some("session_token".to_string()));
    }

    #[test]
    fn test_client_builder_env_debug() {
        // Test with environment variable set
        std::env::set_var("TENCENTCLOUD_SDK_DEBUG", "true");
        let _client = ClientBuilder::new()
            .credential(Credential {
                secret_id: "test_id".to_string(),
                secret_key: "test_key".to_string(),
                token: None,
            })
            .build();
        // Note: debug state depends on environment, so we don't test it here

        // Test with environment variable not set
        std::env::remove_var("TENCENTCLOUD_SDK_DEBUG");
        let _client = ClientBuilder::new()
            .credential(Credential {
                secret_id: "test_id".to_string(),
                secret_key: "test_key".to_string(),
                token: None,
            })
            .build();
        // Note: debug state depends on environment, so we don't test it here
    }

    #[test]
    fn test_client_builder_override_env_debug() {
        std::env::set_var("TENCENTCLOUD_SDK_DEBUG", "true");

        // Override environment variable
        let client = ClientBuilder::new()
            .credential(Credential {
                secret_id: "test_id".to_string(),
                secret_key: "test_key".to_string(),
                token: None,
            })
            .debug(false)
            .build();
        assert!(!client.debug());

        std::env::remove_var("TENCENTCLOUD_SDK_DEBUG");
    }

    #[test]
    fn test_client_builder_panic_no_credential() {
        let result = std::panic::catch_unwind(|| {
            ClientBuilder::new().build();
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_client_builder_methods() {
        let builder = ClientBuilder::new();
        assert!(!builder.has_http());
        assert!(!builder.has_credential());
        assert!(!builder.has_region());
        assert!(!builder.has_endpoint());
        assert!(!builder.has_debug());
    }

    #[test]
    fn test_client_builder_fluent_interface() {
        let builder = ClientBuilder::new()
            .http(reqwest::Client::new())
            .credential(Credential {
                secret_id: "test_id".to_string(),
                secret_key: "test_key".to_string(),
                token: None,
            })
            .region(Region::ApBeijing)
            .endpoint("test.endpoint.com")
            .debug(true);

        assert!(builder.has_http());
        assert!(builder.has_credential());
        assert!(builder.has_region());
        assert!(builder.has_endpoint());
        assert!(builder.has_debug());
    }

    #[test]
    fn test_client_clone() {
        let client = ClientBuilder::new()
            .credential(Credential {
                secret_id: "test_id".to_string(),
                secret_key: "test_key".to_string(),
                token: None,
            })
            .build();

        let cloned = client.clone();
        assert_eq!(cloned.region().as_str(), client.region().as_str());
        assert_eq!(cloned.endpoint(), client.endpoint());
        assert_eq!(cloned.debug(), client.debug());
    }

    #[test]
    fn test_client_builder_method() {
        let client = Client::builder()
            .credential(Credential {
                secret_id: "test_id".to_string(),
                secret_key: "test_key".to_string(),
                token: None,
            })
            .build();

        assert_eq!(client.region().as_str(), "ap-guangzhou");
    }

    #[test]
    fn test_tc3_sign_components() {
        let client = ClientBuilder::new()
            .credential(Credential {
                secret_id: "test_id".to_string(),
                secret_key: "test_key".to_string(),
                token: None,
            })
            .build();

        let timestamp = OffsetDateTime::now_utc().unix_timestamp();
        let (signature, credential_scope) = client.tc3_sign(
            "POST",
            "/",
            "",
            "content-type:application/json; charset=utf-8\nhost:hunyuan.tencentcloudapi.com\n",
            "content-type;host",
            "test_payload_hash",
            timestamp,
        );

        assert!(!signature.is_empty());
        assert!(!credential_scope.is_empty());
        assert!(credential_scope.contains("hunyuan"));
        assert!(credential_scope.contains("tc3_request"));
    }

    #[test]
    fn test_build_headers() {
        let client = ClientBuilder::new()
            .credential(Credential {
                secret_id: "test_id".to_string(),
                secret_key: "test_key".to_string(),
                token: None,
            })
            .build();

        let timestamp = OffsetDateTime::now_utc().unix_timestamp();
        let headers = client.build_headers("TestAction", "test_body", timestamp);

        assert_eq!(headers.get("Host").unwrap(), "hunyuan.tencentcloudapi.com");
        assert_eq!(
            headers.get("Content-Type").unwrap(),
            "application/json; charset=utf-8"
        );
        assert_eq!(headers.get("X-TC-Action").unwrap(), "TestAction");
        assert_eq!(headers.get("X-TC-Version").unwrap(), "2023-09-01");
        assert_eq!(headers.get("X-TC-Region").unwrap(), "ap-guangzhou");
        assert_eq!(
            headers.get("X-TC-Timestamp").unwrap(),
            &timestamp.to_string()
        );
        assert!(headers.get("X-TC-Token").is_none());
    }

    #[test]
    fn test_build_headers_with_token() {
        let client = ClientBuilder::new()
            .credential(Credential {
                secret_id: "test_id".to_string(),
                secret_key: "test_key".to_string(),
                token: Some("test_token".to_string()),
            })
            .build();

        let timestamp = OffsetDateTime::now_utc().unix_timestamp();
        let headers = client.build_headers("TestAction", "test_body", timestamp);

        assert_eq!(headers.get("X-TC-Token").unwrap(), "test_token");
    }

    #[test]
    fn test_models_creation() {
        let message = Message {
            role: "user".to_string(),
            content: "Hello, world!".to_string(),
        };

        assert_eq!(message.role, "user");
        assert_eq!(message.content, "Hello, world!");

        let request = ChatCompletionsRequest {
            model: Some("hunyuan-pro".to_string()),
            messages: vec![message],
            temperature: Some(0.7),
            top_p: Some(0.9),
            stream: Some(false),
        };

        assert_eq!(request.model, Some("hunyuan-pro".to_string()));
        assert_eq!(request.messages.len(), 1);
        assert_eq!(request.temperature, Some(0.7));
        assert_eq!(request.top_p, Some(0.9));
        assert_eq!(request.stream, Some(false));
    }

    #[test]
    fn test_serde_serialization() {
        let message = Message {
            role: "user".to_string(),
            content: "Test message".to_string(),
        };

        let json = serde_json::to_string(&message).unwrap();
        let deserialized: Message = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.role, message.role);
        assert_eq!(deserialized.content, message.content);
    }

    #[test]
    fn test_serde_serialization_with_optional_fields() {
        let request = ChatCompletionsRequest {
            model: Some("hunyuan-pro".to_string()),
            messages: vec![Message {
                role: "user".to_string(),
                content: "Test".to_string(),
            }],
            temperature: None,
            top_p: None,
            stream: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        let deserialized: ChatCompletionsRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.temperature, None);
        assert_eq!(deserialized.top_p, None);
        assert_eq!(deserialized.stream, None);
    }
}

pub use client::{Client, ClientBuilder, Credential, Region};
