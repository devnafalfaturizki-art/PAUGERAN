//! Local Ollama provider boundary.
//!
//! [CB §26] — Multi-Provider LLM

use super::super::provider_trait::{LlmProvider, ModelInfo, ChatMessage, ChatOptions, LlmResponse, LlmError};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::instrument;

#[derive(Debug, Serialize)]
struct OllamaRequest {
    model: String,
    messages: Vec<OllamaMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<OllamaOptions>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OllamaMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize, Default)]
struct OllamaOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    num_predict: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
}

#[derive(Debug, Deserialize)]
struct OllamaResponse {
    model: String,
    created_at: String,
    message: OllamaResponseMessage,
    done: bool,
}

#[derive(Debug, Deserialize)]
struct OllamaResponseMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct OllamaTagsResponse {
    models: Vec<OllamaModelInfo>,
}

#[derive(Debug, Deserialize)]
struct OllamaModelInfo {
    name: String,
    modified_at: String,
    size: u64,
}

#[derive(Debug)]
pub struct OllamaProvider {
    client: Client,
    base_url: String,
}

impl OllamaProvider {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            client: Client::builder()
                .user_agent("PAUGERAN/1.0")
                .build()
                .expect("failed to build reqwest client"),
            base_url: base_url.into(),
        }
    }

    pub fn local() -> Self {
        Self::new("http://localhost:11434")
    }
}

#[async_trait]
impl LlmProvider for OllamaProvider {
    fn provider_name(&self) -> &'static str {
        "ollama"
    }

    #[instrument(skip(self, messages, options))]
    async fn chat(
        &self,
        model: &str,
        messages: &[ChatMessage],
        options: &ChatOptions,
    ) -> Result<LlmResponse, LlmError> {
        let request = OllamaRequest {
            model: model.to_string(),
            messages: messages.iter().map(|m| OllamaMessage { role: m.role.as_str().to_string(), content: m.content.clone() }).collect(),
            stream: Some(false),
            options: Some(OllamaOptions {
                temperature: options.temperature,
                num_predict: options.max_tokens,
                top_p: options.top_p,
            }),
        };

        let url = format!("{}/api/chat", self.base_url.trim_end_matches('/'));

        let response = self.client
            .post(&url)
            .header("content-type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_body = response.text().await.unwrap_or_default();
            let error_msg = error_body.lines().next().unwrap_or(&error_body).to_string();
            return Err(LlmError::Provider(format!("ollama API error {}: {}", status, error_msg)));
        }

        let ollama_response: OllamaResponse = response.json().await?;

        Ok(LlmResponse {
            content: ollama_response.message.content,
            model: ollama_response.model,
            provider: "ollama".to_string(),
            finish_reason: if ollama_response.done { Some("stop".to_string()) } else { None },
            usage: super::super::provider_trait::TokenUsage {
                prompt_tokens: 0,
                completion_tokens: 0,
                total_tokens: 0,
            },
        })
    }

    fn models(&self) -> Vec<ModelInfo> {
        vec![
            ModelInfo { id: "llama3.3".to_string(), name: "LLaMA 3.3 (Ollama)".to_string(), provider: "ollama".to_string(), max_tokens: Some(4096), supports_streaming: false, cost_per_1k_input: Some(0.0), cost_per_1k_output: Some(0.0) },
            ModelInfo { id: "llama3.1".to_string(), name: "LLaMA 3.1 (Ollama)".to_string(), provider: "ollama".to_string(), max_tokens: Some(4096), supports_streaming: false, cost_per_1k_input: Some(0.0), cost_per_1k_output: Some(0.0) },
            ModelInfo { id: "mistral".to_string(), name: "Mistral (Ollama)".to_string(), provider: "ollama".to_string(), max_tokens: Some(4096), supports_streaming: false, cost_per_1k_input: Some(0.0), cost_per_1k_output: Some(0.0) },
            ModelInfo { id: "gemma2".to_string(), name: "Gemma 2 (Ollama)".to_string(), provider: "ollama".to_string(), max_tokens: Some(4096), supports_streaming: false, cost_per_1k_input: Some(0.0), cost_per_1k_output: Some(0.0) },
        ]
    }
}
