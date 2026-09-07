//! Unit tests for Causation Analysis.
//! 
//! [CB §10] — Testing Strategy

#[cfg(test)]
mod tests {
    use paugeran::engine::modules::causation::CausationAnalyzer;

    #[test]
    fn test_but_for_test() {
        let analyzer = CausationAnalyzer::new();
        let result = analyzer.but_for_test(
            "Keterlambatan pengiriman",
            "Proyek terlambat 1 minggu",
        );
        assert!(result.is_causal);
    }

    #[test]
    fn test_foreseeability() {
        let analyzer = CausationAnalyzer::new();
        let result = analyzer.foreseeability(
            "Keterlambatan pengiriman",
            "Kerugian Rp 10 M",
        );
        assert!(!result.is_foreseeable);
    }
}
