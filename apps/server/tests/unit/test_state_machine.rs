//! Unit tests for Case State Machine.
//! 
//! [CB §10] — Testing Strategy

#[cfg(test)]
mod tests {
    use paugeran::engine::state_machine::{StateMachine, CaseState};

    #[test]
    fn test_initial_state() {
        let sm = StateMachine::new();
        assert_eq!(sm.current_state(), CaseState::Unknown);
    }

    #[test]
    fn test_state_transition() {
        let mut sm = StateMachine::new();
        let result = sm.transition(CaseState::Exploration);
        assert!(result.is_ok());
        assert_eq!(sm.current_state(), CaseState::Exploration);
    }

    #[test]
    fn test_invalid_transition() {
        let mut sm = StateMachine::new();
        let result = sm.transition(CaseState::Resolved);
        // Unknown -> Resolved should fail
        assert!(result.is_err());
    }
}
