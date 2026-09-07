use serde::{Deserialize, Serialize};
use tracing::info;
use crate::engine::context::ExecutionContext;
use crate::engine::nodes::{NodeExecutor, NodeOutput, NodeResult};
use crate::engine::EngineError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayerOutput {
    pub layer_number: u8,
    pub layer_name: String,
    pub interpretation: String,
    pub key_findings: Vec<String>,
    pub ambiguities: Vec<String>,
    pub certainty: f32,
    pub supporting_sources: Vec<String>,
}

impl LayerOutput {
    pub fn new(layer_number: u8, layer_name: impl Into<String>) -> Self {
        Self {
            layer_number,
            layer_name: layer_name.into(),
            interpretation: String::new(),
            key_findings: Vec::new(),
            ambiguities: Vec::new(),
            certainty: 0.0,
            supporting_sources: Vec::new(),
        }
    }
}

#[async_trait::async_trait]
pub trait LegalLayer: Send + Sync {
    async fn analyze(&self, context: &ExecutionContext) -> Result<LayerOutput, EngineError>;
    fn layer_number(&self) -> u8;
    fn layer_name(&self) -> &'static str;
}
