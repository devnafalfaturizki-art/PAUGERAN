//! Content extractor that combines the HTTP client, robots checker,
//! and HTML parser into a single research helper.

use serde::{Deserialize, Serialize};

use super::html_parser::HtmlParser;
use super::http_client::ResearchHttpClient;
use super::robots_checker::RobotsChecker;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedContent {
    pub url: String,
    pub title: Option<String>,
    pub text: String,
    pub length: usize,
}

pub struct ContentExtractor {
    pub http: ResearchHttpClient,
    pub robots: RobotsChecker,
}

impl Default for ContentExtractor {
    fn default() -> Self {
        Self {
            http: ResearchHttpClient::default(),
            robots: RobotsChecker::default(),
        }
    }
}

impl ContentExtractor {
    pub fn fetch_and_extract(&self, url: &str) -> Result<ExtractedContent, String> {
        if !self.robots.is_allowed(url) {
            return Err(format!("robots.txt melarang akses ke {url}"));
        }
        let html = self.http.fetch(url)?;
        let title = HtmlParser::extract_title(&html);
        let text = HtmlParser::extract_text(&html);
        Ok(ExtractedContent {
            url: url.to_string(),
            title,
            length: text.len(),
            text,
        })
    }
}