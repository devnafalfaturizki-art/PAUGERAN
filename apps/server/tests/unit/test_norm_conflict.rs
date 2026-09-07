//! Unit tests for Norm Conflict Resolution.
//! 
//! [CB §10] — Testing Strategy

#[cfg(test)]
mod tests {
    use paugeran::engine::modules::norm_conflict::NormConflictResolver;

    #[test]
    fn test_lex_specialis() {
        let resolver = NormConflictResolver::new();
        let result = resolver.resolve_special_general(
            "UU No. 13 Tahun 2003 Pasal 158",
            "KUHPerdata Pasal 1243",
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_lex_posterior() {
        let resolver = NormConflictResolver::new();
        let result = resolver.resolve_posterior(
            "UU No. 13 Tahun 2003 Pasal 158",
            "UU Cipta Kerja Pasal 81",
        );
        assert!(result.is_ok());
    }
}
