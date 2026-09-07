//! Anthropic Claude provider boundary.
//!
//! [CB §26] — Multi-Provider LLM

use super::super::provider_trait::{LlmProvider, ModelInfo, ChatMessage, ChatOptions, LlmResponse, LlmError};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::instrument;

#[derive(Debug, Serialize)]
struct AnthropicRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<AnthropicMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop_sequences: Option<Vec<String>>,
    stream: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct AnthropicMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct AnthropicResponse {
    id: String,
    model: String,
    content: Vec<AnthropicContentBlock>,
    usage: AnthropicUsage,
    stop_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct AnthropicContentBlock {
    #[serde(rename = "type")]
    content_type: String,
    text: String,
}

#[derive(Debug, Deserialize)]
struct AnthropicUsage {
    input_tokens: u32,
    output_tokens: u32,
}

#[derive(Debug, Deserialize)]
struct AnthropicErrorResponse {
    #[serde(rename = "type")]
    error_type: String,
    error: AnthropicErrorDetail,
}

#[derive(Debug, Deserialize)]
struct AnthropicErrorDetail {
    message: String,
    #[serde(rename = "type")]
    error_type: String,
}

#[derive(Debug)]
pub struct AnthropicProvider {
    client: Client,
    api_key: String,
    base_url: String,
}

impl AnthropicProvider {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self::with_base_url(api_key, "https://api.anthropic.com".to_string())
    }

    pub fn with_base_url(api_key: impl Into<String>, base_url: String) -> Self {
        Self {
            client: Client::builder()
                .user_agent("PAUGERAN/1.0")
                .build()
                .expect("failed to build reqwest client"),
            api_key: api_key.into(),
            base_url,
        }
    }
}

#[async_trait]
impl LlmProvider for AnthropicProvider {
    fn provider_name(&self) -> &'static str {
        "anthropic"
    }

    #[instrument(skip(self, messages, options))]
    async fn chat(
        &self,
        model: &str,
        messages: &[ChatMessage],
        options: &ChatOptions,
    ) -> Result<LlmResponse, LlmError> {
        let (system_prompt, chat_messages): (Option<String>, Vec<_>) = match messages.split_first() {
            Some((first, rest)) if first.role == super::super::provider_trait::MessageRole::System => {
                (Some(first.content.clone()), rest.iter().map(|m| AnthropicMessage { role: m.role.as_str().to_string(), content: m.content.clone() }).collect())
            }
            _ => (None, messages.iter().map(|m| AnthropicMessage { role: m.role.as_str().to_string(), content: m.content.clone() }).collect()),
        };

        let request = AnthropicRequest {
            model: model.to_string(),
            max_tokens: options.max_tokens.unwrap_or(4096),
            messages: chat_messages,
            system: system_prompt,
            temperature: options.temperature,
            top_p: options.top_p,
            stop_sequences: options.stop_sequences.clone(),
            stream: options.stream,
        };

        let url = format!("{}/v1/messages", self.base_url.trim_end_matches('/'));

        let response = self.client
            .post(&url)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_body = response.text().await.unwrap_or_default();
            let error_msg = error_body.lines().next().unwrap_or(&error_body).to_string();

            if status == 429 {
                return Err(LlmError::RateLimited(format!("anthropic rate limited: {}", error_msg)));
            }
            if status == 401 {
                return Err(LlmError::AuthFailed(format!("anthropic auth failed: {}", error_msg)));
            }
            return Err(LlmError::Provider(format!("anthropic API error {}: {}", status, error_msg)));
        }

        let anthropic_response: AnthropicResponse = response.json().await?;

        let content = anthropic_response.content
            .iter()
            .filter(|b| b.content_type == "text")
            .map(|b| b.text.clone())
            .collect::<Vec<_>>()
            .join("\n");

        Ok(LlmResponse {
            content,
            model: anthropic_response.model,
            provider: "anthropic".to_string(),
            finish_reason: anthropic_response.stop_reason,
            usage: super::super::provider_trait::TokenUsage {
                prompt_tokens: anthropic_response.usage.input_tokens,
                completion_tokens: anthropic_response.usage.output_tokens,
                total_tokens: anthropic_response.usage.input_tokens + anthropic_response.usage.output_tokens,
            },
        })
    }

    fn models(&self) -> Vec<ModelInfo> {
        vec![
            ModelInfo { id: "claude-3-5-sonnet-20241022".to_string(), name: "Claude 3.5 Sonnet".to_string(), provider: "anthropic".to_string(), max_tokens: Some(8192), supports_streaming: true, cost_per_1k_input: Some(3.0), cost_per_1k_output: Some(15.0) },
            ModelInfo { id: "claude-3-5-haiku-20241022".to_string(), name: "Claude 3.5 Haiku".to_string(), provider: "anthropic".to_string(), max_tokens: Some(8192), supports_streaming: true, cost_per_1k_input: Some(0.80), cost_per_1k_output: Some(4.0) },
            ModelInfo { id: "claude-sonnet-4-20250514".to_string(), name: "Claude Sonnet 4".to_string(), provider: "anthropic".to_string(), max_tokens: Some(16384), supports_streaming: true, cost_per_1k_input: Some(3.0), cost_per_1k_output: Some(15.0) },
            ModelInfo { id: "claude-opus-4-20250514".to_string(), name: "Claude Opus 4".to_string(), provider: "anthropic".to_string(), max_tokens: Some(16384), supports_streaming: true, cost_per_1k_input: Some(15.0), cost_per_1k_output: Some(75.0) },
        ]
    }
}
