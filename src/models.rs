use serde::{Deserialize, Serialize};

/// Generic Tencent Cloud success response envelope.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TencentCloudResponse<T> {
    #[serde(rename = "Response")]
    pub response: T,
}

/// Generic Tencent Cloud error response envelope.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TencentCloudErrorResponse {
    #[serde(rename = "RequestId")]
    pub request_id: Option<String>,
    #[serde(rename = "Error")]
    pub error: Option<ErrorContent>,
}

/// Error content as returned by Tencent Cloud.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorContent {
    #[serde(rename = "Code")]
    pub code: String,
    #[serde(rename = "Message")]
    pub message: String,
}

// Minimal ChatCompletions models based on common TencentCloud LLM APIs.
// Reference: Go SDK hunyuan/v20230901 (actions like ChatCompletions)

/// Chat message used in `ChatCompletionsRequest`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    #[serde(rename = "Role")]
    pub role: String,
    #[serde(rename = "Content")]
    pub content: String,
}

/// Request for the `ChatCompletions` action.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionsRequest {
    #[serde(rename = "Model")]
    pub model: Option<String>,
    #[serde(rename = "Messages")]
    pub messages: Vec<Message>,
    #[serde(rename = "Temperature")]
    pub temperature: Option<f32>,
    #[serde(rename = "TopP")]
    pub top_p: Option<f32>,
    #[serde(rename = "Stream")]
    pub stream: Option<bool>,
    // Add other fields as needed per upstream API
}

/// Message inside a choice in `ChatCompletionsResponse`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatChoiceMessage {
    #[serde(rename = "Role")]
    pub role: Option<String>,
    #[serde(rename = "Content")]
    pub content: Option<String>,
}

/// Single choice in `ChatCompletionsResponse`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatChoice {
    #[serde(rename = "Index")]
    pub index: Option<u32>,
    #[serde(rename = "Message")]
    pub message: Option<ChatChoiceMessage>,
    #[serde(rename = "FinishReason")]
    pub finish_reason: Option<String>,
}

/// Token usage statistics returned by the service.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    #[serde(rename = "PromptTokens")]
    pub prompt_tokens: Option<u32>,
    #[serde(rename = "CompletionTokens")]
    pub completion_tokens: Option<u32>,
    #[serde(rename = "TotalTokens")]
    pub total_tokens: Option<u32>,
}

/// Inner payload for `ChatCompletionsResponse`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionsResponseInner {
    #[serde(rename = "RequestId")]
    pub request_id: Option<String>,
    #[serde(rename = "Id")]
    pub id: Option<String>,
    #[serde(rename = "Choices")]
    pub choices: Option<Vec<ChatChoice>>,
    #[serde(rename = "Usage")]
    pub usage: Option<Usage>,
}

/// Type alias for the full `ChatCompletions` response envelope.
pub type ChatCompletionsResponse = TencentCloudResponse<ChatCompletionsResponseInner>;
