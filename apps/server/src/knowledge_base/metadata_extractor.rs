//! Extracts structured metadata (number, year, title, status) from
//! Indonesian legislation citations found in free text or filenames.

use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegulationMetadata {
    pub kind: Option<String>,
    pub number: Option<String>,
    pub year: Option<i32>,
    pub title: Option<String>,
    pub status: String,
    pub hierarchy_level: u8,
}

impl Default for RegulationMetadata {
    fn default() -> Self {
        Self {
            kind: None,
            number: None,
            year: None,
            title: None,
            status: "aktif".to_string(),
            hierarchy_level: 0,
        }
    }
}

pub struct MetadataExtractor {
    pattern: Regex,
}

impl Default for MetadataExtractor {
    fn default() -> Self {
        let pattern = Regex::new(
            r"(?i)\b(UU|Perpu|PP|Perpres|Permen|Perda|TAP MPR|UUD|KUHPerdata|KUHAP|KUH Dagang)\s*(?:No(?:mor)?\.?)?\s*([0-9A-Z\.]+)?\s*(?:Tahun\s*)?([0-9]{4})?",
        )
        .expect("valid regex");
        Self { pattern }
    }
}

impl MetadataExtractor {
    pub fn extract(&self, text: &str) -> RegulationMetadata {
        let mut meta = RegulationMetadata::default();
        if let Some(captures) = self.pattern.captures(text) {
            if let Some(kind) = captures.get(1) {
                let kind_upper = kind.as_str().to_uppercase();
                meta.kind = Some(kind_upper.clone());
                meta.hierarchy_level = match kind_upper.as_str() {
                    "UUD" => 1,
                    "TAP MPR" => 2,
                    "UU" | "PERPU" => 3,
                    "PP" => 4,
                    "PERPRES" => 5,
                    "PERMEN" => 6,
                    "PERDA" => 6,
                    _ => 0,
                };
            }
            if let Some(number) = captures.get(2) {
                meta.number = Some(number.as_str().to_string());
            }
            if let Some(year) = captures.get(3) {
                if let Ok(value) = year.as_str().parse::<i32>() {
                    meta.year = Some(value);
                }
            }
        }

        if let Some(title) = Self::extract_title(text) {
            meta.title = Some(title);
        }
        meta
    }

    pub fn hierarchy_for(kind: &str) -> u8 {
        match kind.to_uppercase().as_str() {
            "UUD" => 1,
            "TAP MPR" | "TAP_MPR" => 2,
            "UU" | "PERPU" => 3,
            "PP" => 4,
            "PERPRES" => 5,
            "PERMEN" | "PERDA" => 6,
            _ => 0,
        }
    }

    fn extract_title(text: &str) -> Option<String> {
        let keyword_markers = ["tentang", "Tentang", "TENTANG"];
        for marker in keyword_markers {
            if let Some(index) = text.find(marker) {
                let rest = &text[index + marker.len()..];
                let candidate: String = rest
                    .chars()
                    .take_while(|ch| *ch != '\n' && *ch != '.')
                    .collect();
                let candidate = candidate.trim().to_string();
                if !candidate.is_empty() {
                    return Some(candidate);
                }
            }
        }
        None
    }
}