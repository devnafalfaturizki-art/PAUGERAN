//! Token budget boundary.
//!
//! [CB §26] — Multi-Provider LLM token management.

pub struct TokenCounter;

impl TokenCounter {
    pub fn count(text: &str) -> usize {
        Self::estimate_tokens(text)
    }

    pub fn count_messages(messages: &[super::provider_trait::ChatMessage]) -> usize {
        messages.iter().map(|m| Self::estimate_tokens(&m.content)).sum()
    }

    pub fn estimate_tokens(text: &str) -> usize {
        if text.is_empty() {
            return 0;
        }

        let char_count = text.chars().count();
        let whitespace_count = text.chars().filter(|c| c.is_whitespace()).count();
        let non_whitespace = char_count.saturating_sub(whitespace_count);

        let punctuation_count = text.chars().filter(|c| matches!(c, '.' | ',' | '!' | '?' | ';' | ':' | '(' | ')' | '[' | ']' | '{' | '}' | '"' | '\'')).count();

        let estimated = (non_whitespace / 4) + (punctuation_count / 2) + (whitespace_count / 5);

        estimated.max(1)
    }

    pub fn truncate_to_budget(text: &str, max_tokens: usize) -> String {
        let estimated_tokens = Self::estimate_tokens(text);
        if estimated_tokens <= max_tokens {
            return text.to_string();
        }

        let ratio = max_tokens as f32 / estimated_tokens as f32;
        let char_limit = (text.chars().count() as f32 * ratio) as usize;

        let mut truncated = text.chars().take(char_limit).collect::<String>();
        truncated.push_str("\n\n[... truncated to fit token budget ...]");
        truncated
    }

    pub fn truncate_messages_to_budget(
        messages: &mut [super::provider_trait::ChatMessage],
        max_tokens: usize,
    ) {
        let total: usize = messages.iter().map(|m| Self::estimate_tokens(&m.content)).sum();

        if total <= max_tokens {
            return;
        }

        let ratio = max_tokens as f32 / total as f32;
        let token_budget_per_message = ((max_tokens as f32 * ratio) / messages.len() as f32) as usize;

        for msg in messages.iter_mut() {
            msg.content = Self::truncate_to_budget(&msg.content, token_budget_per_message.max(50));
        }
    }
}
