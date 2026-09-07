use tracing::info;
use crate::engine::context::ExecutionContext;
use crate::engine::nodes::{NodeExecutor, NodeOutput, NodeResult, IssueEntry, IssuePriority};
use crate::engine::EngineError;

pub struct IssueIdentifier;

impl IssueIdentifier {
    pub fn new() -> Self {
        Self
    }

    fn identify_potential_issues(facts: &[String], mode: &crate::engine::mode_router::ReasoningMode) -> Vec<IssueEntry> {
        let mut issues = Vec::new();

        let combined = facts.join(" ").to_lowercase();

        if combined.contains("perjanjian") || combined.contains("kontrak") {
            issues.push(IssueEntry {
                id: format!("issue_{}", uuid::Uuid::new_v4()),
                description: "Kelengkapan dan keabsahan perjanjian/kontrak".to_string(),
                legal_basis: vec!["Pasal 1320 KUHPerdata".to_string(), "Pasal 1338 KUHPerdata".to_string()],
                priority: IssuePriority::High,
                certainty: 0.7,
            });
        }

        if combined.contains("kerugian") || combined.contains("rugi") || combined.contains("ganti rugi") {
            issues.push(IssueEntry {
                id: format!("issue_{}", uuid::Uuid::new_v4()),
                description: "Pertanggungjawaban dan ganti rugi".to_string(),
                legal_basis: vec!["Pasal 1246 KUHPerdata".to_string()],
                priority: IssuePriority::High,
                certainty: 0.65,
            });
        }

        if combined.contains("karyawan") || combined.contains("pekerja") || combined.contains("phk") {
            issues.push(IssueEntry {
                id: format!("issue_{}", uuid::Uuid::new_v4()),
                description: "Hubungan kerja dan pemutusan hubungan kerja".to_string(),
                legal_basis: vec!["UU No. 13 Tahun 2003".to_string()],
                priority: IssuePriority::High,
                certainty: 0.7,
            });
        }

        if combined.contains("tanah") || combined.contains("properti") || combined.contains("sertifikat") {
            issues.push(IssueEntry {
                id: format!("issue_{}", uuid::Uuid::new_v4()),
                description: "Status hukum dan peralihan tanah/properti".to_string(),
                legal_basis: vec!["UU No. 5 Tahun 1960 (UUPA)".to_string()],
                priority: IssuePriority::High,
                certainty: 0.65,
            });
        }

        if combined.contains("warisan") || combined.contains("waris") {
            issues.push(IssueEntry {
                id: format!("issue_{}", uuid::Uuid::new_v4()),
                description: "Pembagian harta waris".to_string(),
                legal_basis: vec!["KUHPerdata (Bagian Waris)".to_string(), "UU No. 1 Tahun 1974".to_string()],
                priority: IssuePriority::High,
                certainty: 0.6,
            });
        }

        if combined.contains("wanprestasi") || combined.contains("ingkar janji") || combined.contains("tidak bayar") {
            issues.push(IssueEntry {
                id: format!("issue_{}", uuid::Uuid::new_v4()),
                description: "Wanprestasi dan pemenuhan kewajiban kontraktual".to_string(),
                legal_basis: vec!["Pasal 1236 KUHPerdata".to_string(), "Pasal 1243 KUHPerdata".to_string()],
                priority: IssuePriority::High,
                certainty: 0.75,
            });
        }

        if combined.contains("kegagalan") || combined.contains("gagal") || combined.contains("force majeure") {
            issues.push(IssueEntry {
                id: format!("issue_{}", uuid::Uuid::new_v4()),
                description: "Pembebasan tanggung jawab karena force majeure".to_string(),
                legal_basis: vec!["Pasal 1244 KUHPerdata".to_string(), "Pasal 1245 KUHPerdata".to_string()],
                priority: IssuePriority::Medium,
                certainty: 0.5,
            });
        }

        if issues.is_empty() {
            issues.push(IssueEntry {
                id: format!("issue_{}", uuid::Uuid::new_v4()),
                description: format!("Isu hukum umum terkait: {}", facts.join(", ")),
                legal_basis: Vec::new(),
                priority: IssuePriority::Medium,
                certainty: 0.3,
            });
        }

        match mode {
            crate::engine::mode_router::ReasoningMode::Preventive => {
                issues.iter_mut().for_each(|i| {
                    i.priority = IssuePriority::High;
                    i.certainty = (i.certainty + 0.1).min(1.0);
                });
            }
            crate::engine::mode_router::ReasoningMode::Exploration => {
                issues.iter_mut().for_each(|i| {
                    i.certainty = (i.certainty * 0.7).max(0.2);
                });
            }
            _ => {}
        }

        issues
    }
}

#[async_trait::async_trait]
impl NodeExecutor for IssueIdentifier {
    async fn execute(&self, context: &mut ExecutionContext) -> Result<NodeResult, EngineError> {
        context.check_cancelled()?;
        info!(case_id=%context.case_id, "identifying legal issues");

        let facts: Vec<String> = context.case_graph.facts.iter().map(|f| f.content.clone()).collect();
        let issues = Self::identify_potential_issues(&facts, &context.reasoning_mode);

        for issue in &issues {
            context.case_graph.issues.push(crate::engine::context::GraphNode {
                id: issue.id.clone(),
                node_type: "issue".to_string(),
                content: issue.description.clone(),
                metadata: serde_json::json!({"priority": format!("{:?}", issue.priority).to_lowercase(), "certainty": issue.certainty}),
                certainty: issue.certainty,
            });
        }

        Ok(NodeResult {
            node_id: format!("issue_id_{}", uuid::Uuid::new_v4()),
            node_type: "issue_identifier".to_string(),
            success: true,
            output: NodeOutput::Issues { issues },
            confidence: 0.7,
            warnings: Vec::new(),
            metadata: serde_json::json!({"total_issues": context.case_graph.issues.len()}),
        })
    }

    fn node_type(&self) -> &'static str {
        "issue_identifier"
    }
}
