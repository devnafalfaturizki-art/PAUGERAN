//! Unit tests for Temporal Law Application.
//! 
//! [CB §10] — Testing Strategy

#[cfg(test)]
mod tests {
    use paugeran::engine::modules::temporal_law::TemporalLawApplier;

    #[test]
    fn test_applicable_law_2018() {
        let applier = TemporalLawApplier::new();
        let result = applier.apply_for_date("2018-03-15");
        assert!(result.is_ok());
        let applicable = result.unwrap();
        assert!(applicable.contains("UU No. 13 Tahun 2003"));
    }

    #[test]
    fn test_non_retroactive() {
        let applier = TemporalLawApplier::new();
        let result = applier.apply_for_date("2018-03-15");
        assert!(result.is_ok());
        let applicable = result.unwrap();
        assert!(!applicable.contains("UU Cipta Kerja"));
    }
}
