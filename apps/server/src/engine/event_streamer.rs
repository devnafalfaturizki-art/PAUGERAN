use serde::{Deserialize, Serialize};
use tracing::info;
use crate::engine::context::ExecutionContext;
use crate::engine::mode_router::ReasoningMode;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReasoningEvent {
    PhaseStarted { phase: String, description: String },
    PhaseCompleted { phase: String, result_summary: String },
    NodeExecuted { node_type: String, node_id: String, outcome: String },
    LayerCompleted { layer: u8, layer_name: String, key_finding: String },
    Warning { message: String },
    Error { message: String },
    ClarificationNeeded { question: String },
    OutputChunk { text: String },
}

pub struct EventStreamer {
    sender: Option<tokio::sync::mpsc::Sender<ReasoningEvent>>,
}

impl EventStreamer {
    pub fn new(sender: Option<tokio::sync::mpsc::Sender<ReasoningEvent>>) -> Self {
        Self { sender }
    }

    pub fn emit_phase_start(&self, phase: &str, description: &str) {
        let event = ReasoningEvent::PhaseStarted {
            phase: phase.to_string(),
            description: description.to_string(),
        };
        self.emit(event);
        info!(phase=%phase, "phase started");
    }

    pub fn emit_phase_complete(&self, phase: &str, summary: &str) {
        let event = ReasoningEvent::PhaseCompleted {
            phase: phase.to_string(),
            result_summary: summary.to_string(),
        };
        self.emit(event);
        info!(phase=%phase, "phase completed");
    }

    pub fn emit_node(&self, node_type: &str, node_id: &str, outcome: &str) {
        let event = ReasoningEvent::NodeExecuted {
            node_type: node_type.to_string(),
            node_id: node_id.to_string(),
            outcome: outcome.to_string(),
        };
        self.emit(event);
        tracing::debug!(node_type=%node_type, node_id=%node_id, "node executed");
    }

    pub fn emit_layer(&self, layer: u8, name: &str, finding: &str) {
        let event = ReasoningEvent::LayerCompleted {
            layer,
            layer_name: name.to_string(),
            key_finding: finding.to_string(),
        };
        self.emit(event);
        info!(layer=%layer, layer_name=%name, "layer completed");
    }

    pub fn emit_warning(&self, message: &str) {
        let event = ReasoningEvent::Warning {
            message: message.to_string(),
        };
        self.emit(event);
        tracing::warn!(message=%message, "engine warning");
    }

    pub fn emit_error(&self, message: &str) {
        let event = ReasoningEvent::Error {
            message: message.to_string(),
        };
        self.emit(event);
        tracing::error!(message=%message, "engine error");
    }

    pub fn emit_clarification(&self, question: &str) {
        let event = ReasoningEvent::ClarificationNeeded {
            question: question.to_string(),
        };
        self.emit(event);
        info!(question=%question, "clarification needed");
    }

    pub fn emit_output(&self, text: &str) {
        let event = ReasoningEvent::OutputChunk {
            text: text.to_string(),
        };
        self.emit(event);
    }

    pub fn emit_reasoning_start(&self, mode: ReasoningMode) {
        let desc = format!("mode: {:?}", mode);
        self.emit_phase_start("reasoning", &desc);
    }

    pub fn emit_reasoning_complete(&self, mode: ReasoningMode) {
        let summary = format!("mode: {:?} completed", mode);
        self.emit_phase_complete("reasoning", &summary);
    }

    fn emit(&self, event: ReasoningEvent) {
        if let Some(sender) = &self.sender {
            let _ = sender.try_send(event);
        }
    }
}
