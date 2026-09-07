use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CaseState {
    Unknown,
    Exploration,
    Preventive,
    Dispute,
    Litigation,
    Resolved,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum TransitionError {
    #[error("transisi dari {from:?} ke {to:?} tidak diizinkan")]
    NotAllowed { from: CaseState, to: CaseState },
    #[error("perkara yang sudah selesai tidak dapat diaktifkan kembali")]
    ResolvedCase,
}

impl CaseState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Unknown => "unknown",
            Self::Exploration => "exploration",
            Self::Preventive => "preventive",
            Self::Dispute => "dispute",
            Self::Litigation => "litigation",
            Self::Resolved => "resolved",
        }
    }
}

impl CaseState {
    pub fn can_transition_to(self, next: Self) -> bool {
        matches!(
            (self, next),
            (Self::Unknown, Self::Exploration)
                | (Self::Exploration, Self::Preventive)
                | (Self::Exploration, Self::Dispute)
                | (Self::Preventive, Self::Dispute)
                | (Self::Preventive, Self::Litigation)
                | (Self::Dispute, Self::Preventive)
                | (Self::Dispute, Self::Litigation)
                | (Self::Litigation, Self::Resolved)
                | (Self::Dispute, Self::Resolved)
                | (Self::Preventive, Self::Resolved)
        )
    }

    pub fn transition_to(self, next: Self) -> Result<Self, TransitionError> {
        if self == Self::Resolved {
            return Err(TransitionError::ResolvedCase);
        }
        if self.can_transition_to(next) {
            Ok(next)
        } else {
            Err(TransitionError::NotAllowed {
                from: self,
                to: next,
            })
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateTransition {
    pub from: CaseState,
    pub to: CaseState,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaseStateMachine {
    current: CaseState,
    history: Vec<StateTransition>,
}

impl CaseStateMachine {
    pub fn new(initial: CaseState) -> Self {
        Self {
            current: initial,
            history: Vec::new(),
        }
    }

    pub fn current(&self) -> CaseState {
        self.current
    }

    pub fn history(&self) -> &[StateTransition] {
        &self.history
    }

    pub fn transition(
        &mut self,
        next: CaseState,
        reason: impl Into<String>,
    ) -> Result<(), TransitionError> {
        let previous = self.current;
        self.current = previous.transition_to(next)?;
        self.history.push(StateTransition {
            from: previous,
            to: next,
            reason: reason.into(),
        });
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{CaseState, CaseStateMachine, TransitionError};

    #[test]
    fn follows_a_valid_case_lifecycle() {
        let mut machine = CaseStateMachine::new(CaseState::Unknown);
        machine
            .transition(CaseState::Exploration, "fakta awal diterima")
            .unwrap();
        machine
            .transition(CaseState::Dispute, "posisi para pihak berlawanan")
            .unwrap();
        machine
            .transition(CaseState::Litigation, "negosiasi tidak berhasil")
            .unwrap();
        machine
            .transition(CaseState::Resolved, "putusan telah berkekuatan tetap")
            .unwrap();

        assert_eq!(machine.current(), CaseState::Resolved);
        assert_eq!(machine.history().len(), 4);
    }

    #[test]
    fn rejects_skipping_from_unknown_to_litigation() {
        let mut machine = CaseStateMachine::new(CaseState::Unknown);
        let error = machine
            .transition(CaseState::Litigation, "langsung mengajukan gugatan")
            .unwrap_err();

        assert_eq!(
            error,
            TransitionError::NotAllowed {
                from: CaseState::Unknown,
                to: CaseState::Litigation
            }
        );
    }

    #[test]
    fn resolved_case_cannot_be_reopened() {
        let mut machine = CaseStateMachine::new(CaseState::Resolved);
        assert_eq!(
            machine
                .transition(CaseState::Dispute, "fakta baru")
                .unwrap_err(),
            TransitionError::ResolvedCase
        );
    }
}
