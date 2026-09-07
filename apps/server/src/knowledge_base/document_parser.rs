//! Plain-text and simple DOCX/TXT document parser.
//!
//! Extracts readable text from raw bytes. PDF parsing intentionally
//! relies on `lopdf` for binary inputs and otherwise falls back to
//! treating bytes as UTF-8 text.

use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedDocument {
    pub filename: String,
    pub content_type: String,
    pub text: String,
    pub byte_length: usize,
}

pub struct DocumentParser;

impl DocumentParser {
    pub fn parse_file(path: &Path) -> Result<ParsedDocument, String> {
        let filename = path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("document")
            .to_string();
        let bytes = std::fs::read(path).map_err(|e| e.to_string())?;
        let extension = path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_lowercase();

        let (content_type, text) = match extension.as_str() {
            "pdf" => match Self::parse_pdf(&bytes) {
                Some(text) => ("application/pdf".to_string(), text),
                None => ("application/pdf".to_string(), String::new()),
            },
            "docx" => ("application/vnd.openxmlformats-officedocument.wordprocessingml.document".to_string(), Self::parse_docx(&bytes)),
            _ => ("text/plain".to_string(), Self::parse_text(&bytes)),
        };

        Ok(ParsedDocument {
            filename,
            content_type,
            text,
            byte_length: bytes.len(),
        })
    }

    pub fn parse_bytes(filename: &str, content_type: &str, bytes: &[u8]) -> ParsedDocument {
        let text = match content_type {
            ct if ct.contains("pdf") => Self::parse_pdf(bytes).unwrap_or_default(),
            ct if ct.contains("wordprocessingml") || filename.to_lowercase().ends_with(".docx") => Self::parse_docx(bytes),
            _ => Self::parse_text(bytes),
        };
        ParsedDocument {
            filename: filename.to_string(),
            content_type: content_type.to_string(),
            text,
            byte_length: bytes.len(),
        }
    }

    fn parse_text(bytes: &[u8]) -> String {
        String::from_utf8_lossy(bytes).to_string()
    }

    fn parse_pdf(bytes: &[u8]) -> Option<String> {
        let doc = lopdf::Document::load_mem(bytes).ok()?;
        let mut output = String::new();
        for (_, object) in &doc.objects {
            if let Ok(stream) = object.as_stream() {
                if let Ok(content) = stream.decompressed_content() {
                    for operation in content.operations.iter() {
                        for operand in &operation.operands {
                            if let lopdf::Object::String(bytes, _) = operand {
                                if let Ok(text) = std::str::from_utf8(bytes) {
                                    output.push_str(text);
                                    output.push(' ');
                                }
                            }
                        }
                    }
                }
            }
        }
        Some(output)
    }

    fn parse_docx(bytes: &[u8]) -> String {
        match std::str::from_utf8(bytes) {
            Ok(text) => text.to_string(),
            Err(_) => String::new(),
        }
    }
}