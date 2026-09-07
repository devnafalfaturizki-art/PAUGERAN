//! LLM fallback strategy boundary.
//!
//! [CB §26] — Multi-Provider LLM fallback chain.

use super::provider_trait::{ChatMessage, ChatOptions, LlmError, LlmProvider, LlmResponse};

pub struct FallbackChain {
    providers: Vec<Box<dyn LlmProvider>>,
    max_attempts: usize,
}

impl FallbackChain {
    pub fn new(providers: Vec<Box<dyn LlmProvider>>) -> Self {
        Self {
            providers,
            max_attempts: providers.len(),
        }
    }

    pub fn with_max_attempts(mut self, max_attempts: usize) -> Self {
        self.max_attempts = max_attempts.min(self.providers.len());
        self
    }

    pub async fn chat(
        &self,
        model: &str,
        messages: &[ChatMessage],
        options: &ChatOptions,
    ) -> Result<LlmResponse, LlmError> {
        let mut last_error = None;

        for (attempt, provider) in self.providers.iter().enumerate().take(self.max_attempts) {
            if !provider.is_available() {
                tracing::warn!(
                    "fallback attempt {}: provider {} unavailable",
                    attempt + 1,
                    provider.provider_name()
                );
                continue;
            }

            tracing::info!(
                "fallback attempt {}: trying provider {} with model {}",
                attempt + 1,
                provider.provider_name(),
                model
            );

            match provider.chat(model, messages, options).await {
                Ok(response) => {
                    if attempt > 0 {
                        tracing::info!(
                            "succeeded on fallback attempt {} via {}",
                            attempt + 1,
                            provider.provider_name()
                        );
                    }
                    return Ok(response);
                }
                Err(e) => {
                    tracing::warn!(
                        "fallback attempt {} failed on {}: {}",
                        attempt + 1,
                        provider.provider_name(),
                        e
                    );
                    last_error = Some(e);
                }
            }
        }

        Err(last_error.unwrap_or_else(|| LlmError::Unavailable("all fallback providers exhausted".to_string())))
    }

    pub fn provider_names(&self) -> Vec<&'static str> {
        self.providers.iter().map(|p| p.provider_name()).collect()
    }
}
