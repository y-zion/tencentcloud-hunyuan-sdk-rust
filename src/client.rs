use crate::models::{ChatCompletionsRequest, ChatCompletionsResponse, TencentCloudErrorResponse};
use crate::signing::{hmac_sha256, sha256_hex};
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client as HttpClient;
use serde::de::DeserializeOwned;
use serde::Serialize;
use thiserror::Error;
use time::OffsetDateTime;

const SERVICE: &str = "hunyuan";
const VERSION: &str = "2023-09-01";
const ACTION_CHAT_COMPLETIONS: &str = "ChatCompletions";

#[derive(Debug, Clone)]
pub struct Credential {
    pub secret_id: String,
    pub secret_key: String,
    pub token: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Region {
    ApBeijing,
    ApGuangzhou,
    Custom(String),
}

impl Region {
    pub fn as_str(&self) -> &str {
        match self {
            Region::ApBeijing => "ap-beijing",
            Region::ApGuangzhou => "ap-guangzhou",
            Region::Custom(s) => s.as_str(),
        }
    }
}

#[derive(Debug, Error)]
pub enum SdkError {
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("serialization error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("service error: {code}: {message} (request_id={request_id:?})")]
    Service {
        code: String,
        message: String,
        request_id: Option<String>,
    },
}

#[derive(Clone)]
pub struct Client {
    http: HttpClient,
    credential: Credential,
    region: Region,
    endpoint: String,
}

pub struct ClientBuilder {
    http: Option<HttpClient>,
    credential: Option<Credential>,
    region: Option<Region>,
    endpoint: Option<String>,
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self {
            http: None,
            credential: None,
            region: None,
            endpoint: None,
        }
    }
}

impl ClientBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn http(mut self, http: HttpClient) -> Self {
        self.http = Some(http);
        self
    }
    pub fn credential(mut self, credential: Credential) -> Self {
        self.credential = Some(credential);
        self
    }
    pub fn region(mut self, region: Region) -> Self {
        self.region = Some(region);
        self
    }
    pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = Some(endpoint.into());
        self
    }

    pub fn build(self) -> Client {
        let http = self
            .http
            .unwrap_or_else(|| HttpClient::builder().build().expect("reqwest client"));
        let region = self.region.unwrap_or(Region::ApGuangzhou);
        let endpoint = self
            .endpoint
            .unwrap_or_else(|| format!("{}.tencentcloudapi.com", SERVICE));
        let credential = self.credential.expect("credential is required");
        Client {
            http,
            credential,
            region,
            endpoint,
        }
    }
}

impl Client {
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    fn tc3_sign(
        &self,
        method: &str,
        canonical_uri: &str,
        canonical_querystring: &str,
        canonical_headers: &str,
        signed_headers: &str,
        hashed_payload: &str,
        timestamp: i64,
    ) -> (String, String) {
        // 1. Canonical request
        let canonical_request = format!(
            "{method}\n{uri}\n{query}\n{headers}\n{signed}\n{payload}",
            method = method,
            uri = canonical_uri,
            query = canonical_querystring,
            headers = canonical_headers,
            signed = signed_headers,
            payload = hashed_payload
        );
        let hashed_canonical_request = sha256_hex(&canonical_request);

        // 2. String to sign
        let date = OffsetDateTime::from_unix_timestamp(timestamp)
            .unwrap()
            .format(&time::format_description::parse("%Y-%m-%d").unwrap())
            .unwrap();
        let credential_scope = format!("{}/{}/tc3_request", date, SERVICE);
        let string_to_sign = format!(
            "TC3-HMAC-SHA256\n{}\n{}\n{}",
            timestamp, credential_scope, hashed_canonical_request
        );

        // 3. Signature
        let secret_key = format!("TC3{}", self.credential.secret_key);
        let secret_date = hmac_sha256(secret_key.as_bytes(), &date);
        let secret_service = hmac_sha256(&secret_date, SERVICE);
        let secret_signing = hmac_sha256(&secret_service, "tc3_request");
        let signature = crate::signing::hmac_sha256_hex(&secret_signing, &string_to_sign);

        (signature, credential_scope)
    }

    fn build_headers(&self, action: &str, _json_body: &str, timestamp: i64) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert("Host", HeaderValue::from_str(&self.endpoint).unwrap());
        headers.insert(
            "Content-Type",
            HeaderValue::from_static("application/json; charset=utf-8"),
        );
        headers.insert("X-TC-Action", HeaderValue::from_str(action).unwrap());
        headers.insert("X-TC-Version", HeaderValue::from_static(VERSION));
        headers.insert(
            "X-TC-Region",
            HeaderValue::from_str(self.region.as_str()).unwrap(),
        );
        headers.insert(
            "X-TC-Timestamp",
            HeaderValue::from_str(&timestamp.to_string()).unwrap(),
        );
        if let Some(token) = &self.credential.token {
            headers.insert("X-TC-Token", HeaderValue::from_str(token).unwrap());
        }
        headers
    }

    async fn call_action<TReq: Serialize, TResp: DeserializeOwned>(
        &self,
        action: &str,
        req: &TReq,
    ) -> Result<TResp, SdkError> {
        let method = "POST";
        let canonical_uri = "/";
        let canonical_querystring = "";

        let body = serde_json::to_string(req)?;
        let timestamp = OffsetDateTime::now_utc().unix_timestamp();

        let mut headers = self.build_headers(action, &body, timestamp);

        // Headers for signing
        let host = self.endpoint.clone();
        let canonical_headers = format!(
            "content-type:application/json; charset=utf-8\nhost:{}\n",
            host
        );
        let signed_headers = "content-type;host";
        let hashed_payload = sha256_hex(&body);
        let (signature, credential_scope) = self.tc3_sign(
            method,
            canonical_uri,
            canonical_querystring,
            &canonical_headers,
            signed_headers,
            &hashed_payload,
            timestamp,
        );

        let authorization = format!(
            "TC3-HMAC-SHA256 Credential={}/{}, SignedHeaders={}, Signature={}",
            self.credential.secret_id, credential_scope, signed_headers, signature
        );
        headers.insert(
            "Authorization",
            HeaderValue::from_str(&authorization).unwrap(),
        );

        let url = format!("https://{}/", self.endpoint);
        let resp = self
            .http
            .post(url)
            .headers(headers)
            .body(body)
            .send()
            .await?;
        let status = resp.status();
        let text = resp.text().await?;

        if !status.is_success() {
            // Try to decode TencentCloud style error
            let err: Result<TencentCloudErrorResponse, _> = serde_json::from_str(&text);
            if let Ok(err) = err {
                if let Some(e) = err.error {
                    return Err(SdkError::Service {
                        code: e.code,
                        message: e.message,
                        request_id: err.request_id,
                    });
                }
            }
            return Err(SdkError::Service {
                code: format!("HTTP_{}", status.as_u16()),
                message: text,
                request_id: None,
            });
        }

        let parsed: TResp = serde_json::from_str(&text)?;
        Ok(parsed)
    }

    pub async fn chat_completions(
        &self,
        req: &ChatCompletionsRequest,
    ) -> Result<ChatCompletionsResponse, SdkError> {
        self.call_action(ACTION_CHAT_COMPLETIONS, req).await
    }
}
