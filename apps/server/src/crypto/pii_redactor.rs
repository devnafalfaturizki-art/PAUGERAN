//! Redacts common Indonesian PII before text is persisted or sent to a provider.
//!
//! [CB §63] — PII Redaction

use regex::Regex;
use std::collections::HashMap;

#[derive(Clone)]
pub struct PiiRedactor {
    nik: Regex,
    bank: Regex,
    email: Regex,
}

impl Default for PiiRedactor {
    fn default() -> Self {
        Self {
            nik: Regex::new(r"\b\d{16}\b").expect("valid NIK pattern"),
            bank: Regex::new(r"\b\d{10,15}\b").expect("valid bank account pattern"),
            email: Regex::new(r"\b[\w.+-]+@[\w.-]+\.[A-Za-z]{2,}\b").expect("valid email pattern"),
        }
    }
}

impl PiiRedactor {
    pub fn redact(&self, input: &str) -> (String, HashMap<String, String>) {
        let mut mapping = HashMap::new();
        let mut nik_counter = 0u32;
        let mut bank_counter = 0u32;
        let mut email_counter = 0u32;

        let output = self
            .nik
            .replace_all(input, |caps: &regex::Captures| {
                let original = caps[0].to_string();
                nik_counter += 1;
                let placeholder = format!("[NIK_{}]", nik_counter);
                mapping.insert(placeholder.clone(), original);
                placeholder
            })
            .into_owned();

        let output = self
            .bank
            .replace_all(&output, |caps: &regex::Captures| {
                let original = caps[0].to_string();
                bank_counter += 1;
                let placeholder = format!("[BANK_{}]", bank_counter);
                mapping.insert(placeholder.clone(), original);
                placeholder
            })
            .into_owned();

        let output = self
            .email
            .replace_all(&output, |caps: &regex::Captures| {
                let original = caps[0].to_string();
                email_counter += 1;
                let placeholder = format!("[EMAIL_{}]", email_counter);
                mapping.insert(placeholder.clone(), original);
                placeholder
            })
            .into_owned();

        (output, mapping)
    }

    pub fn restore(&self, text: &str, mapping: &HashMap<String, String>) -> String {
        let mut output = text.to_string();
        for (placeholder, original) in mapping {
            output = output.replace(placeholder, original);
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::PiiRedactor;

    #[test]
    fn redacts_indonesian_pii() {
        let (redacted, mapping) =
            PiiRedactor::default().redact("NIK 3201010101010001, email a@contoh.id, 081234567890, 1234567890");
        assert!(!redacted.contains("3201010101010001"));
        assert!(!redacted.contains("a@contoh.id"));
        assert!(!redacted.contains("081234567890"));
        assert!(redacted.contains("[NIK_1]"));
        assert!(redacted.contains("[EMAIL_1]"));
        // 081234567890 is phone, not 10-15 digit bank account
        // 1234567890 is 10 digits, matches bank regex
        assert!(redacted.contains("[BANK_1]"));
        assert_eq!(mapping.get("[NIK_1]"), Some(&"3201010101010001".to_string()));
        assert_eq!(mapping.get("[EMAIL_1]"), Some(&"a@contoh.id".to_string()));
    }

    #[test]
    fn restore_roundtrip() {
        let redactor = PiiRedactor::default();
        let original = "NIK 3201010101010001, email a@contoh.id, 1234567890123456";
        let (redacted, mapping) = redactor.redact(original);
        let restored = redactor.restore(&redacted, &mapping);
        assert_eq!(restored, original);
    }
}
