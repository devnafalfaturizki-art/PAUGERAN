//! Redacts common Indonesian PII before text is persisted or sent to a provider.

use regex::Regex;

#[derive(Clone)]
pub struct PiiRedactor {
    nik: Regex,
    phone: Regex,
    email: Regex,
}

impl Default for PiiRedactor {
    fn default() -> Self {
        Self {
            nik: Regex::new(r"\b\d{16}\b").expect("valid NIK pattern"),
            phone: Regex::new(r"\b(?:\+62|62|0)8\d{8,12}\b").expect("valid phone pattern"),
            email: Regex::new(r"\b[\w.+-]+@[\w.-]+\.[A-Za-z]{2,}\b").expect("valid email pattern"),
        }
    }
}

impl PiiRedactor {
    pub fn redact(&self, input: &str) -> String {
        let output = self.nik.replace_all(input, "[NIK_REDACTED]");
        let output = self.phone.replace_all(&output, "[PHONE_REDACTED]");
        self.email
            .replace_all(&output, "[EMAIL_REDACTED]")
            .into_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::PiiRedactor;

    #[test]
    fn redacts_indonesian_pii() {
        let redacted =
            PiiRedactor::default().redact("NIK 3201010101010001, email a@contoh.id, 081234567890");
        assert!(!redacted.contains("3201010101010001"));
        assert!(!redacted.contains("a@contoh.id"));
        assert!(!redacted.contains("081234567890"));
    }
}
