//! Mode-aware LLM router boundary.
//!
//! [CB §26] — Multi-Provider LLM routing based on Reasoning Mode.

use super::provider_trait::{LlmProvider, ModelInfo, ChatMessage, ChatOptions, LlmResponse, LlmError};
use crate::engine::mode_router::ReasoningMode;

#[derive(Debug, Clone)]
pub struct ModelPreference {
    pub primary: String,
    pub fallback: Option<String>,
    pub provider: String,
}

#[derive(Debug)]
pub struct LlmRouter {
    providers: Vec<Box<dyn LlmProvider>>,
    mode_preferences: std::collections::HashMap<ReasoningMode, Vec<ModelPreference>>,
}

impl LlmRouter {
    pub fn new(providers: Vec<Box<dyn LlmProvider>>) -> Self {
        let mut router = Self {
            providers,
            mode_preferences: std::collections::HashMap::new(),
        };
        router.init_default_preferences();
        router
    }

    pub fn with_preferences(
        providers: Vec<Box<dyn LlmProvider>>,
        preferences: std::collections::HashMap<ReasoningMode, Vec<ModelPreference>>,
    ) -> Self {
        let mut router = Self {
            providers,
            mode_preferences: preferences,
        };
        router.init_default_preferences();
        router
    }

    fn init_default_preferences(&mut self) {
        self.mode_preferences.entry(ReasoningMode::Exploration).or_insert_with(|| vec![
            ModelPreference { primary: "claude-3-5-haiku-20241022".to_string(), fallback: Some("gpt-4o-mini".to_string()), provider: "anthropic".to_string() },
        ]);
        self.mode_preferences.entry(ReasoningMode::Preventive).or_insert_with(|| vec![
            ModelPreference { primary: "claude-3-5-haiku-20241022".to_string(), fallback: Some("gpt-4o-mini".to_string()), provider: "anthropic".to_string() },
        ]);
        self.mode_preferences.entry(ReasoningMode::Dispute).or_insert_with(|| vec![
            ModelPreference { primary: "claude-sonnet-4-20250514".to_string(), fallback: Some("gpt-4o".to_string()), provider: "anthropic".to_string() },
        ]);
        self.mode_preferences.entry(ReasoningMode::LitigationPrep).or_insert_with(|| vec![
            ModelPreference { primary: "claude-sonnet-4-20250514".to_string(), fallback: Some("gpt-4o".to_string()), provider: "anthropic".to_string() },
        ]);
        self.mode_preferences.entry(ReasoningMode::Adversarial).or_insert_with(|| vec![
            ModelPreference { primary: "claude-3-5-sonnet-20241022".to_string(), fallback: Some("gpt-4o".to_string()), provider: "anthropic".to_string() },
        ]);
        self.mode_preferences.entry(ReasoningMode::Neutral).or_insert_with(|| vec![
            ModelPreference { primary: "claude-sonnet-4-20250514".to_string(), fallback: Some("gpt-4o".to_string()), provider: "anthropic".to_string() },
        ]);
    }

    pub fn select_model(&self, mode: ReasoningMode) -> Option<&ModelPreference> {
        self.mode_preferences.get(&mode).and_then(|prefs| prefs.first())
    }

    pub fn set_mode_preference(
        &mut self,
        mode: ReasoningMode,
        preferences: Vec<ModelPreference>,
    ) {
        self.mode_preferences.insert(mode, preferences);
    }

    pub async fn chat(
        &self,
        mode: ReasoningMode,
        messages: &[ChatMessage],
        options: &ChatOptions,
    ) -> Result<LlmResponse, LlmError> {
        let prefs = self.mode_preferences.get(&mode).ok_or_else(|| {
            LlmError::Provider(format!("no model preference configured for mode {:?}", mode))
        })?;

        let mut last_error = None;

        for pref in prefs {
            let provider = self.providers.iter().find(|p| p.provider_name() == pref.provider);
            let provider = match provider {
                Some(p) => p,
                None => continue,
            };

            if !provider.is_available() {
                tracing::warn!("provider {} unavailable, skipping", pref.provider);
                continue;
            }

            match provider.chat(&pref.primary, messages, options).await {
                Ok(response) => return Ok(response),
                Err(e) => {
                    tracing::warn!("primary model {} failed: {}", pref.primary, e);
                    last_error = Some(e);
                }
            }

            if let Some(ref fallback_model) = pref.fallback {
                match provider.chat(fallback_model, messages, options).await {
                    Ok(response) => return Ok(response),
                    Err(e) => {
                        tracing::warn!("fallback model {} failed: {}", fallback_model, e);
                        last_error = Some(e);
                    }
                }
            }
        }

        Err(last_error.unwrap_or_else(|| LlmError::Provider("no providers available".to_string())))
    }

    pub fn list_all_models(&self) -> Vec<ModelInfo> {
        self.providers.iter().flat_map(|p| p.models()).collect()
    }

    pub fn provider_names(&self) -> Vec<&'static str> {
        self.providers.iter().map(|p| p.provider_name()).collect()
    }
}
