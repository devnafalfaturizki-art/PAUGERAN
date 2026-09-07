//! Unit tests for Citation Validator.
//! 
//! [CB §30] — Mandatory Citation Format

#[cfg(test)]
mod tests {
    use paugeran::engine::nodes::citation_validator::CitationValidator;

    #[test]
    fn test_valid_citation() {
        let validator = CitationValidator::new();
        let citation = r#"[Sumber: Undang-Undang Nomor 13 Tahun 2003 tentang Ketenagakerjaan, Pasal 158, diundangkan pada 2003-03-25. Status: Aktif. Teks: "Pihak yang karena kesalahannya mengakibatkan tidak dilaksanakannya perjanjian, wajib mengganti kerugian."]"#;
        let result = validator.validate(citation);
        assert!(result.is_valid);
    }

    #[test]
    fn test_incomplete_citation() {
        let validator = CitationValidator::new();
        let citation = "Pasal 158 UU 13/2003";
        let result = validator.validate(citation);
        assert!(!result.is_valid);
        assert!(!result.errors.is_empty());
    }
}
