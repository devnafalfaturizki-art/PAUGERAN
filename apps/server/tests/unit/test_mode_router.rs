//! Unit tests for Mode Router.
//! 
//! [CB §10] — Testing Strategy

#[cfg(test)]
mod tests {
    use paugeran::engine::mode_router::{ModeRouter, ReasoningMode};

    #[test]
    fn test_mode_selection_exploration() {
        let router = ModeRouter::new();
        let mode = router.select_mode("exploration");
        assert_eq!(mode, ReasoningMode::Exploration);
    }

    #[test]
    fn test_mode_selection_preventive() {
        let router = ModeRouter::new();
        let mode = router.select_mode("preventive");
        assert_eq!(mode, ReasoningMode::Preventive);
    }

    #[test]
    fn test_mode_selection_adversarial() {
        let router = ModeRouter::new();
        let mode = router.select_mode("adversarial");
        assert_eq!(mode, ReasoningMode::Adversarial);
    }
}
