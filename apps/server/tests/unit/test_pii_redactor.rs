//! Unit tests for PII Redaction.
//! 
//! [CB §62] — Data Security & PII Redaction

#[cfg(test)]
mod tests {
    use paugeran::crypto::pii_redactor::PiiRedactor;

    #[test]
    fn test_redact_nik() {
        let redactor = PiiRedactor::new();
        let text = "NIK saya adalah 3173016501810001";
        let redacted = redactor.redact(text);
        assert!(!redacted.contains("3173016501810001"));
        assert!(redacted.contains("[PERSON_1]"));
    }

    #[test]
    fn test_redact_phone() {
        let redactor = PiiRedactor::new();
        let text = "Hubungi saya di 08123456789";
        let redacted = redactor.redact(text);
        assert!(!redacted.contains("08123456789"));
        assert!(redacted.contains("[PHONE_1]"));
    }

    #[test]
    fn test_redact_email() {
        let redactor = PiiRedactor::new();
        let text = "Email: john@example.com";
        let redacted = redactor.redact(text);
        assert!(!redacted.contains("john@example.com"));
        assert!(redacted.contains("[EMAIL_1]"));
    }
}
