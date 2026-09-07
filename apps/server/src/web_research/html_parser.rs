//! HTML parser for Indonesian legal documents.
//!
//! Uses the `scraper` crate to strip navigation and surface only the
//! article-like content from a fetched page.

use scraper::{Html, Selector};

pub struct HtmlParser;

impl HtmlParser {
    pub fn extract_text(html: &str) -> String {
        let document = Html::parse_document(html);
        let body_selector = Selector::parse("body").expect("valid selector");
        let mut text = String::new();
        if let Some(body) = document.select(&body_selector).next() {
            Self::collect_text(body, &mut text);
        }
        text
    }

    pub fn extract_title(html: &str) -> Option<String> {
        let document = Html::parse_document(html);
        let selector = Selector::parse("title").ok()?;
        document
            .select(&selector)
            .next()
            .map(|el| el.text().collect::<String>().trim().to_string())
            .filter(|s| !s.is_empty())
    }

    fn collect_text(node: scraper::element_ref::ElementRef, output: &mut String) {
        for child in node.children() {
            if let Some(text) = child.value().as_text() {
                let trimmed = text.trim();
                if !trimmed.is_empty() {
                    output.push_str(trimmed);
                    output.push(' ');
                }
            }
            if let Some(element) = child.value().as_element() {
                if matches!(element.name(), "script" | "style" | "noscript") {
                    continue;
                }
                if let Some(child_ref) = scraper::element_ref::ElementRef::wrap(child) {
                    Self::collect_text(child_ref, output);
                }
            }
        }
    }
}