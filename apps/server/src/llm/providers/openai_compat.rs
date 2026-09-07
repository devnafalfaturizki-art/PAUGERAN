//! OpenAI-compatible provider boundary.
//!
//! Supports Groq, Together AI, Fireworks, OpenRouter, DeepSeek,
//! and any other provider compatible with the OpenAI chat API format.
//!
//! [CB §26] — Multi-Provider LLM

use super::super::provider_trait::{LlmProvider, ModelInfo, ChatMessage, ChatOptions, LlmResponse, LlmError};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::instrument;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompatProviderKind {
    Groq,
    Together,
    Fireworks,
    OpenRouter,
    DeepSeek,
    Generic,
}

impl CompatProviderKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Groq => "groq",
            Self::Together => "together",
            Self::Fireworks => "fireworks",
            Self::OpenRouter => "openrouter",
            Self::DeepSeek => "deepseek",
            Self::Generic => "generic",
        }
    }

    pub fn default_base_url(self) -> &'static str {
        match self {
            Self::Groq => "https://api.groq.com/openai/v1",
            Self::Together => "https://api.together.xyz/v1",
            Self::Fireworks => "https://api.fireworks.ai/inference/v1",
            Self::OpenRouter => "https://openrouter.ai/api/v1",
            Self::DeepSeek => "https://api.deepseek.com",
            Self::Generic => "https://api.openai.com",
        }
    }
}

#[derive(Debug, Serialize)]
struct CompatRequest {
    model: String,
    messages: Vec<CompatMessage>,
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
struct CompatMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct CompatResponse {
    id: String,
    model: String,
    choices: Vec<CompatChoice>,
    usage: CompatUsage,
}

#[derive(Debug, Deserialize)]
struct CompatChoice {
    message: CompatMessage,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CompatUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

pub struct OpenAICompatibleProvider {
    client: Client,
    api_key: String,
    base_url: String,
    kind: CompatProviderKind,
}

impl OpenAICompatibleProvider {
    pub fn new(kind: CompatProviderKind, api_key: impl Into<String>) -> Self {
        Self::with_base_url(kind, api_key, kind.default_base_url().to_string())
    }

    pub fn with_base_url(
        kind: CompatProviderKind,
        api_key: impl Into<String>,
        base_url: String,
    ) -> Self {
        Self {
            client: Client::builder()
                .user_agent("PAUGERAN/1.0")
                .build()
                .expect("failed to build reqwest client"),
            api_key: api_key.into(),
            base_url,
            kind,
        }
    }
}

#[async_trait]
impl LlmProvider for OpenAICompatibleProvider {
    fn provider_name(&self) -> &'static str {
        self.kind.as_str()
    }

    #[instrument(skip(self, messages, options))]
    async fn chat(
        &self,
        model: &str,
        messages: &[ChatMessage],
        options: &ChatOptions,
    ) -> Result<LlmResponse, LlmError> {
        let request = CompatRequest {
            model: model.to_string(),
            messages: messages.iter().map(|m| CompatMessage { role: m.role.as_str().to_string(), content: m.content.clone() }).collect(),
            temperature: options.temperature,
            max_tokens: options.max_tokens,
            top_p: options.top_p,
            stop: options.stop_sequences.clone(),
            stream: options.stream,
        };

        let url = format!("{}/chat/completions", self.base_url.trim_end_matches('/'));

        let mut req_builder = self.client
            .post(&url)
            .header("content-type", "application/json")
            .json(&request);

        req_builder = match self.kind {
            CompatProviderKind::OpenRouter => req_builder.header("authorization", format!("Bearer {}", self.api_key)),
            CompatProviderKind::DeepSeek => req_builder.header("authorization", format!("Bearer {}", self.api_key)),
            _ => req_builder.header("authorization", format!("Bearer {}", self.api_key)),
        };

        let response = req_builder.send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_body = response.text().await.unwrap_or_default();
            let error_msg = error_body.lines().next().unwrap_or(&error_body).to_string();

            if status == 429 {
                return Err(LlmError::RateLimited(format!("{} rate limited: {}", self.kind.as_str(), error_msg)));
            }
            if status == 401 {
                return Err(LlmError::AuthFailed(format!("{} auth failed: {}", self.kind.as_str(), error_msg)));
            }
            return Err(LlmError::Provider(format!("{} API error {}: {}", self.kind.as_str(), status, error_msg)));
        }

        let compat_response: CompatResponse = response.json().await?;

        let choice = compat_response.choices.into_iter().next().ok_or_else(|| {
            LlmError::Provider(format!("{} returned empty choices", self.kind.as_str()))
        })?;

        Ok(LlmResponse {
            content: choice.message.content,
            model: compat_response.model,
            provider: self.kind.as_str().to_string(),
            finish_reason: choice.finish_reason,
            usage: super::super::provider_trait::TokenUsage {
                prompt_tokens: compat_response.usage.prompt_tokens,
                completion_tokens: compat_response.usage.completion_tokens,
                total_tokens: compat_response.usage.total_tokens,
            },
        })
    }

    fn models(&self) -> Vec<ModelInfo> {
        match self.kind {
            CompatProviderKind::Groq => vec![
                ModelInfo { id: "llama-3.3-70b-versatile".to_string(), name: "LLaMA 3.3 70B (Groq)".to_string(), provider: "groq".to_string(), max_tokens: Some(4096), supports_streaming: true, cost_per_1k_input: Some(0.0), cost_per_1k_output: Some(0.0) },
                ModelInfo { id: "mixtral-8x7b-32768".to_string(), name: "Mixtral 8x7B (Groq)".to_string(), provider: "groq".to_string(), max_tokens: Some(32768), supports_streaming: true, cost_per_1k_input: Some(0.0), cost_per_1k_output: Some(0.0) },
            ],
            CompatProviderKind::Together => vec![
                ModelInfo { id: "meta-llama/Llama-3.3-70B-Instruct-Turbo".to_string(), name: "LLaMA 3.3 70B (Together)".to_string(), provider: "together".to_string(), max_tokens: Some(4096), supports_streaming: true, cost_per_1k_input: Some(0.88), cost_per_1k_output: Some(0.88) },
            ],
            CompatProviderKind::Fireworks => vec![
                ModelInfo { id: "accounts/fireworks/models/llama-v3p1-405b-instruct".to_string(), name: "LLaMA 3.1 405B (Fireworks)".to_string(), provider: "fireworks".to_string(), max_tokens: Some(4096), supports_streaming: true, cost_per_1k_input: Some(2.0), cost_per_1k_output: Some(2.0) },
            ],
            CompatProviderKind::OpenRouter => vec![
                ModelInfo { id: "openai/gpt-4o".to_string(), name: "GPT-4o (OpenRouter)".to_string(), provider: "openrouter".to_string(), max_tokens: Some(4096), supports_streaming: true, cost_per_1k_input: Some(2.5), cost_per_1k_output: Some(10.0) },
                ModelInfo { id: "anthropic/claude-3.5-sonnet".to_string(), name: "Claude 3.5 Sonnet (OpenRouter)".to_string(), provider: "openrouter".to_string(), max_tokens: Some(8192), supports_streaming: true, cost_per_1k_input: Some(3.0), cost_per_1k_output: Some(15.0) },
            ],
            CompatProviderKind::DeepSeek => vec![
                ModelInfo { id: "deepseek-chat".to_string(), name: "DeepSeek V3".to_string(), provider: "deepseek".to_string(), max_tokens: Some(4096), supports_streaming: true, cost_per_1k_input: Some(0.14), cost_per_1k_output: Some(0.28) },
            ],
            CompatProviderKind::Generic => vec![
                ModelInfo { id: "generic-model".to_string(), name: "Generic OpenAI-compatible Model".to_string(), provider: "generic".to_string(), max_tokens: Some(4096), supports_streaming: true, cost_per_1k_input: None, cost_per_1k_output: None },
            ],
        }
    }
}
