//! Splits raw regulation text into canonical article (pasal) entries.
//!
//! Recognises common Indonesian legislative markers such as "Pasal 1",
//! "Pasal 2", "Ayat (1)", "Huruf a", etc., and emits structured
//! records that downstream modules can persist or render.

use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleChunk {
    pub article_number: String,
    pub heading: String,
    pub body: String,
    pub order_index: usize,
}

pub struct ArticleSplitter {
    pattern: Regex,
}

impl Default for ArticleSplitter {
    fn default() -> Self {
        Self {
            pattern: Regex::new(r"(?i)\bpasal\s+(?:[0-9]+[a-z]?(?:\s*ayat\s*\([0-9]+\))?|[xivlcdm]+)\b")
                .expect("valid regex"),
        }
    }
}

impl ArticleSplitter {
    pub fn split(&self, text: &str) -> Vec<ArticleChunk> {
        let mut chunks = Vec::new();
        let mut current: Option<(String, String, String, usize)> = None;
        let mut order = 0usize;

        for line in text.lines() {
            let trimmed = line.trim();
            if let Some(caps) = self.pattern.find(trimmed) {
                if let Some((article, _heading, body, idx)) = current.take() {
                    chunks.push(ArticleChunk {
                        article_number: article,
                        heading: _heading,
                        body: body.trim().to_string(),
                        order_index: idx,
                    });
                }
                let label = caps.as_str().trim().to_string();
                let heading_end = trimmed[caps.end()..].trim_start();
                let heading = heading_end
                    .splitn(2, '\n')
                    .next()
                    .unwrap_or("")
                    .trim()
                    .to_string();
                current = Some((label, heading, String::new(), order));
                order += 1;
            } else if let Some((_, _, ref mut body, _)) = current {
                if !trimmed.is_empty() {
                    body.push_str(trimmed);
                    body.push('\n');
                }
            }
        }

        if let Some((article, heading, body, idx)) = current {
            chunks.push(ArticleChunk {
                article_number: article,
                heading,
                body: body.trim().to_string(),
                order_index: idx,
            });
        }

        if chunks.is_empty() {
            chunks.push(ArticleChunk {
                article_number: "0".to_string(),
                heading: String::new(),
                body: text.trim().to_string(),
                order_index: 0,
            });
        }

        chunks
    }
}