//! OpenAI GPT provider boundary.
//!
//! [CB §26] — Multi-Provider LLM

use super::super::provider_trait::{LlmProvider, ModelInfo, ChatMessage, ChatOptions, LlmResponse, LlmError};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::instrument;

#[derive(Debug, Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<OpenAIMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop: Option<Vec<String>>,
    stream: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenAIMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct OpenAIResponse {
    id: String,
    model: String,
    choices: Vec<OpenAIChoice>,
    usage: OpenAIUsage,
}

#[derive(Debug, Deserialize)]
struct OpenAIChoice {
    message: OpenAIMessage,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OpenAIUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

pub struct OpenAIProvider {
    client: Client,
    api_key: String,
    base_url: String,
}

impl OpenAIProvider {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self::with_base_url(api_key, "https://api.openai.com".to_string())
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
impl LlmProvider for OpenAIProvider {
    fn provider_name(&self) -> &'static str {
        "openai"
    }

    #[instrument(skip(self, messages, options))]
    async fn chat(
        &self,
        model: &str,
        messages: &[ChatMessage],
        options: &ChatOptions,
    ) -> Result<LlmResponse, LlmError> {
        let request = OpenAIRequest {
            model: model.to_string(),
            messages: messages.iter().map(|m| OpenAIMessage { role: m.role.as_str().to_string(), content: m.content.clone() }).collect(),
            temperature: options.temperature,
            max_tokens: options.max_tokens,
            top_p: options.top_p,
            stop: options.stop_sequences.clone(),
            stream: options.stream,
        };

        let url = format!("{}/v1/chat/completions", self.base_url.trim_end_matches('/'));

        let response = self.client
            .post(&url)
            .header("authorization", format!("Bearer {}", self.api_key))
            .header("content-type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_body = response.text().await.unwrap_or_default();
            let error_msg = error_body.lines().next().unwrap_or(&error_body).to_string();

            if status == 429 {
                return Err(LlmError::RateLimited(format!("openai rate limited: {}", error_msg)));
            }
            if status == 401 {
                return Err(LlmError::AuthFailed(format!("openai auth failed: {}", error_msg)));
            }
            return Err(LlmError::Provider(format!("openai API error {}: {}", status, error_msg)));
        }

        let openai_response: OpenAIResponse = response.json().await?;

        let choice = openai_response.choices.into_iter().next().ok_or_else(|| {
            LlmError::Provider("openai returned empty choices".to_string())
        })?;

        Ok(LlmResponse {
            content: choice.message.content,
            model: openai_response.model,
            provider: "openai".to_string(),
            finish_reason: choice.finish_reason,
            usage: super::super::provider_trait::TokenUsage {
                prompt_tokens: openai_response.usage.prompt_tokens,
                completion_tokens: openai_response.usage.completion_tokens,
                total_tokens: openai_response.usage.total_tokens,
            },
        })
    }

    fn models(&self) -> Vec<ModelInfo> {
        vec![
            ModelInfo { id: "gpt-4o".to_string(), name: "GPT-4o".to_string(), provider: "openai".to_string(), max_tokens: Some(4096), supports_streaming: true, cost_per_1k_input: Some(2.5), cost_per_1k_output: Some(10.0) },
            ModelInfo { id: "gpt-4o-mini".to_string(), name: "GPT-4o Mini".to_string(), provider: "openai".to_string(), max_tokens: Some(4096), supports_streaming: true, cost_per_1k_input: Some(0.15), cost_per_1k_output: Some(0.6) },
            ModelInfo { id: "o3".to_string(), name: "o3".to_string(), provider: "openai".to_string(), max_tokens: Some(4096), supports_streaming: true, cost_per_1k_input: Some(10.0), cost_per_1k_output: Some(40.0) },
        ]
    }
}
