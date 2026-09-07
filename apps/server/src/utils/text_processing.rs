//! Text processing helpers used by the citation renderer and exporter.

pub fn normalize_whitespace(input: &str) -> String {
    input.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub fn truncate(input: &str, max_chars: usize) -> String {
    if input.chars().count() <= max_chars {
        return input.to_string();
    }
    let truncated: String = input.chars().take(max_chars).collect();
    format!("{truncated}…")
}

pub fn contains_any(haystack: &str, needles: &[&str]) -> bool {
    let lower = haystack.to_lowercase();
    needles.iter().any(|needle| lower.contains(&needle.to_lowercase()))
}

pub fn word_count(input: &str) -> usize {
    input.split_whitespace().count()
}