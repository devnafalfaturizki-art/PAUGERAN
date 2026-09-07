//! Research coordinator. Combines the whitelist, content extractor, and
//! metadata parser into a single end-to-end fetcher used by the engine.

use chrono::Utc;
use serde::{Deserialize, Serialize};

use super::content_extractor::{ContentExtractor, ExtractedContent};
use super::whitelist::Whitelist;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchResult {
    pub url: String,
    pub title: Option<String>,
    pub fetched_at: String,
    pub length: usize,
    pub text_preview: String,
    pub error: Option<String>,
}

pub struct WebResearcher {
    pub whitelist: Whitelist,
    pub extractor: ContentExtractor,
}

impl Default for WebResearcher {
    fn default() -> Self {
        Self {
            whitelist: Whitelist::default(),
            extractor: ContentExtractor::default(),
        }
    }
}

impl WebResearcher {
    pub fn fetch(&self, url: &str) -> ResearchResult {
        let timestamp = Utc::now().to_rfc3339();
        if !self.whitelist.allows(url) {
            return ResearchResult {
                url: url.to_string(),
                title: None,
                fetched_at: timestamp,
                length: 0,
                text_preview: String::new(),
                error: Some("domain tidak termasuk whitelist".to_string()),
            };
        }

        match self.extractor.fetch_and_extract(url) {
            Ok(content) => self.to_result(content, timestamp, None),
            Err(error) => ResearchResult {
                url: url.to_string(),
                title: None,
                fetched_at: timestamp,
                length: 0,
                text_preview: String::new(),
                error: Some(error),
            },
        }
    }

    fn to_result(&self, content: ExtractedContent, fetched_at: String, error: Option<String>) -> ResearchResult {
        let preview: String = content.text.chars().take(400).collect();
        ResearchResult {
            url: content.url,
            title: content.title,
            fetched_at,
            length: content.length,
            text_preview: preview,
            error,
        }
    }
}