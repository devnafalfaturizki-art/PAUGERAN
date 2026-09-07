use tracing::info;
use crate::engine::context::ExecutionContext;
use crate::engine::nodes::{NodeExecutor, NodeOutput, NodeResult};
use crate::engine::EngineError;

pub struct CitationValidator;

impl CitationValidator {
    pub fn new() -> Self {
        Self
    }

    fn validate_citations(&self, context: &ExecutionContext) -> (bool, Vec<String>) {
        let mut issues = Vec::new();

        for rule in &context.case_graph.rules {
            if rule.content.is_empty() {
                issues.push(format!("Rule node {} has empty content", rule.id));
            }
            if rule.metadata.get("hierarchy_level").is_none() {
                issues.push(format!("Rule node {} missing hierarchy_level", rule.id));
            }
            if rule.certainty < 0.1 {
                issues.push(format!("Rule node {} has very low certainty", rule.id));
            }
        }

        for conclusion in &context.case_graph.conclusions {
            let has_support = conclusion.metadata.get("conditions").is_some();
            if !has_support {
                issues.push(format!("Conclusion {} lacks supporting conditions", conclusion.id));
            }
        }

        (issues.is_empty(), issues)
    }
}

#[async_trait::async_trait]
impl NodeExecutor for CitationValidator {
    async fn execute(&self, context: &mut ExecutionContext) -> Result<NodeResult, EngineError> {
        context.check_cancelled()?;
        info!(case_id=%context.case_id, "validating citations");

        let (valid, issues) = self.validate_citations(context);

        Ok(NodeResult {
            node_id: format!("citation_val_{}", uuid::Uuid::new_v4()),
            node_type: "citation_validator".to_string(),
            success: valid,
            output: NodeOutput::CitationValidation { valid, issues: issues.clone() },
            confidence: if valid { 0.9 } else { 0.4 },
            warnings: if !valid { issues.clone() } else { Vec::new() },
            metadata: serde_json::json!({"valid": valid, "issue_count": issues.len()}),
        })
    }

    fn node_type(&self) -> &'static str {
        "citation_validator"
    }
}
