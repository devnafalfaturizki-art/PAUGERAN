use serde::{Deserialize, Serialize};
use tracing::info;
use crate::engine::context::ExecutionContext;
use crate::engine::mode_router::ReasoningMode;
use crate::engine::state_machine::{CaseState, CaseStateMachine};
use crate::engine::EngineError;
use crate::engine::event_streamer::EventStreamer;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisOutput {
    pub case_id: String,
    pub reasoning_mode: ReasoningMode,
    pub case_state: CaseState,
    pub certainty: f32,
    pub interpretation: String,
    pub case_graph: crate::engine::context::CaseGraphSnapshot,
    pub warnings: Vec<String>,
    pub metadata: serde_json::Value,
}

pub struct AdaptiveEngine {
    streamer: Option<EventStreamer>,
}

impl AdaptiveEngine {
    pub fn new(streamer: Option<EventStreamer>) -> Self {
        Self { streamer }
    }

    pub async fn run(
        &self,
        mut context: ExecutionContext,
        state_machine: &mut CaseStateMachine,
    ) -> Result<AnalysisOutput, EngineError> {
        info!(case_id=%context.case_id, mode=?context.reasoning_mode, state=?context.case_state, "starting adaptive engine");

        let streamer = self.streamer.as_ref().unwrap();
        streamer.emit_reasoning_start(context.reasoning_mode);

        streamer.emit_phase_start("understanding", "mengumpulkan dan memahami fakta kasus");

        let fact_extractor = crate::engine::nodes::fact_extractor::FactExtractor::new();
        let fact_result = fact_extractor.execute(&mut context).await?;
        streamer.emit_node("fact_extractor", &fact_result.node_id, "facts extracted");

        streamer.emit_phase_complete("understanding", "fakta dikumpulkan");

        streamer.emit_phase_start("issue_identification", "mengidentifikasi isu hukum");

        let issue_identifier = crate::engine::nodes::issue_identifier::IssueIdentifier::new();
        let issue_result = issue_identifier.execute(&mut context).await?;
        streamer.emit_node("issue_identifier", &issue_result.node_id, "issues identified");

        streamer.emit_phase_complete("issue_identification", "isu hukum teridentifikasi");

        streamer.emit_phase_start("rule_retrieval", "mencari peraturan yang relevan");

        let rule_retriever = crate::engine::nodes::rule_retriever::RuleRetriever::new();
        let rule_result = rule_retriever.execute(&mut context).await?;
        streamer.emit_node("rule_retriever", &rule_result.node_id, "rules retrieved");

        streamer.emit_phase_complete("rule_retrieval", "peraturan ditemukan");

        streamer.emit_phase_start("argument_construction", "membangun argumen hukum");

        let argument_builder = crate::engine::nodes::argument_builder::ArgumentBuilder::new();
        let arg_result = argument_builder.execute(&mut context).await?;
        streamer.emit_node("argument_builder", &arg_result.node_id, "arguments built");

        let counterargument = crate::engine::nodes::counterargument::CounterargumentGenerator::new();
        let counter_result = counterargument.execute(&mut context).await?;
        streamer.emit_node("counterargument", &counter_result.node_id, "counterarguments generated");

        streamer.emit_phase_complete("argument_construction", "argumen dibangun");

        streamer.emit_phase_start("risk_assessment", "menilai risiko hukum");

        let risk_assessor = crate::engine::nodes::risk_assessor::RiskAssessor::new();
        let risk_result = risk_assessor.execute(&mut context).await?;
        streamer.emit_node("risk_assessor", &risk_result.node_id, "risks assessed");

        streamer.emit_phase_complete("risk_assessment", "risiko dinilai");

        streamer.emit_phase_start("citation_validation", "memvalidasi sitasi");

        let citation_validator = crate::engine::nodes::citation_validator::CitationValidator::new();
        let citation_result = citation_validator.execute(&mut context).await?;
        streamer.emit_node("citation_validator", &citation_result.node_id, "citations validated");

        if let crate::engine::nodes::NodeOutput::CitationValidation { valid: false, issues } = &citation_result.output {
            for issue in issues {
                streamer.emit_warning(issue);
            }
        }

        streamer.emit_phase_complete("citation_validation", "sitasi divalidasi");

        streamer.emit_phase_start("mode_specific_analysis", "analisis spesifik mode");

        let mode_result = self.execute_mode(&mut context).await?;
        streamer.emit_node("mode_execution", &mode_result.node_id, "mode analysis complete");

        streamer.emit_phase_complete("mode_specific_analysis", "analisis mode selesai");

        streamer.emit_phase_start("requalification", "fact re-qualification challenge");

        let requalification = crate::engine::nodes::requalification::FactRequalifier::new();
        let requal_result = requalification.execute(&mut context).await?;
        streamer.emit_node("requalification", &requal_result.node_id, "re-qualification complete");

        streamer.emit_phase_complete("requalification", "re-qualification selesai");

        streamer.emit_phase_start("conclusion", "menarik kesimpulan akhir");

        let conclusion_maker = crate::engine::nodes::conclusion_maker::ConclusionMaker::new();
        let conclusion_result = conclusion_maker.execute(&mut context).await?;
        streamer.emit_node("conclusion_maker", &conclusion_result.node_id, "conclusions drawn");

        streamer.emit_phase_complete("conclusion", "kesimpulan ditarik");

        streamer.emit_phase_start("layer_analysis", "7 lapisan penafsiran hukum");

        let layers = self.run_all_layers(&mut context).await?;
        for layer in &layers {
            streamer.emit_layer(layer.layer_number, &layer.layer_name, &layer.key_findings.first().unwrap_or(&"No key findings".to_string()));
        }

        streamer.emit_phase_complete("layer_analysis", "7 lapisan penafsiran selesai");

        streamer.emit_phase_start("synthesis", "mensintesis hasil analisis");

        let synthesis = crate::engine::layers::synthesis::SynthesisEngine::new()
            .synthesize(&layers, &context)?;

        streamer.emit_phase_complete("synthesis", "sintesis selesai");

        let interpretation = if !mode_result.metadata.get("interpretation").and_then(|v| v.as_str()).is_some() {
            synthesis.final_interpretation.clone()
        } else {
            mode_result.metadata.get("interpretation").and_then(|v| v.as_str()).unwrap_or(&synthesis.final_interpretation).to_string()
        };

        let certainty = synthesis.certainty.max(mode_result.confidence);

        streamer.emit_output(&interpretation);
        streamer.emit_reasoning_complete(context.reasoning_mode);

        info!(case_id=%context.case_id, certainty=certainty, "adaptive engine completed");

        Ok(AnalysisOutput {
            case_id: context.case_id,
            reasoning_mode: context.reasoning_mode,
            case_state: state_machine.current(),
            certainty,
            interpretation,
            case_graph: context.case_graph,
            warnings: mode_result.warnings,
            metadata: serde_json::json!({"synthesis": synthesis, "mode_result": mode_result.metadata}),
        })
    }

    async fn execute_mode(&self, context: &mut ExecutionContext) -> Result<crate::engine::nodes::NodeResult, EngineError> {
        match context.reasoning_mode {
            ReasoningMode::Exploration => {
                let mode = crate::engine::modes::exploration::ExplorationMode::new();
                mode.execute(context).await
            }
            ReasoningMode::Preventive => {
                let mode = crate::engine::modes::preventive::PreventiveMode::new();
                mode.execute(context).await
            }
            ReasoningMode::Dispute => {
                let mode = crate::engine::modes::dispute::DisputeMode::new();
                mode.execute(context).await
            }
            ReasoningMode::LitigationPrep => {
                let mode = crate::engine::modes::litigation_prep::LitigationPrepMode::new();
                mode.execute(context).await
            }
            ReasoningMode::Adversarial => {
                let mode = crate::engine::modes::adversarial::AdversarialMode::new();
                mode.execute(context).await
            }
            ReasoningMode::Neutral => {
                let mode = crate::engine::modes::neutral::NeutralMode::new();
                mode.execute(context).await
            }
        }
    }

    async fn run_all_layers(&self, context: &mut ExecutionContext) -> Result<Vec<crate::engine::layers::base::LayerOutput>, EngineError> {
        let mut layers = Vec::new();

        let grammatical = crate::engine::layers::grammatical::GrammaticalInterpreter::new();
        layers.push(grammatical.analyze(context).await?);

        let systematic = crate::engine::layers::systematic::SystematicInterpreter::new();
        layers.push(systematic.analyze(context).await?);

        let teleological = crate::engine::layers::teleological::TeleologicalInterpreter::new();
        layers.push(teleological.analyze(context).await?);

        let sociological = crate::engine::layers::sociological::SociologicalInterpreter::new();
        layers.push(sociological.analyze(context).await?);

        let historical = crate::engine::layers::historical::HistoricalInterpreter::new();
        layers.push(historical.analyze(context).await?);

        let comparative = crate::engine::layers::comparative::ComparativeInterpreter::new();
        layers.push(comparative.analyze(context).await?);

        let critical = crate::engine::layers::critical::CriticalInterpreter::new();
        layers.push(critical.analyze(context).await?);

        Ok(layers)
    }
}
