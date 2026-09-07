//! Unit tests for LLM Router.
//! 
//! [CB §26] — Multi-Provider LLM

#[cfg(test)]
mod tests {
    use paugeran::llm::router::LlmRouter;

    #[test]
    fn test_route_exploration_mode() {
        let router = LlmRouter::new();
        let model = router.select_model("exploration");
        assert!(model.is_some());
        assert!(model.unwrap().contains("haiku") || model.unwrap().contains("mini"));
    }

    #[test]
    fn test_route_adversarial_mode() {
        let router = LlmRouter::new();
        let model = router.select_model("adversarial");
        assert!(model.is_some());
        assert!(model.unwrap().contains("sonnet") || model.unwrap().contains("opus") || model.unwrap().contains("gpt-4"));
    }
}
