use tracing::info;
use crate::engine::context::ExecutionContext;
use crate::engine::nodes::{NodeExecutor, NodeOutput, NodeResult, RuleEntry, RuleStatus};
use crate::engine::EngineError;

pub struct RuleRetriever;

impl RuleRetriever {
    pub fn new() -> Self {
        Self
    }

    fn search_knowledge_base(&self, context: &ExecutionContext, query: &str) -> Vec<RuleEntry> {
        let mut results = Vec::new();
        let lower_query = query.to_lowercase();

        if lower_query.contains("perjanjian") || lower_query.contains("kontrak") {
            results.push(RuleEntry {
                id: format!("rule_{}", uuid::Uuid::new_v4()),
                regulation: "Kitab Undang-Undang Hukum Perdata".to_string(),
                article: "Pasal 1320".to_string(),
                full_text: "Suatu perjanjian adalah perikatan antara dua orang atau lebih yang sesuai dengan undang-undang untuk mengikat diri mereka sendiri. Perikatan itu lahir dari suatu perjanjian atau suatu undang-undang.".to_string(),
                hierarchy_level: 2,
                status: RuleStatus::Active,
                effective_date: "1847-01-01".to_string(),
                source_url: None,
                access_date: chrono::Utc::now().format("%Y-%m-%d").to_string(),
                precedential_weight: None,
            });
            results.push(RuleEntry {
                id: format!("rule_{}", uuid::Uuid::new_v4()),
                regulation: "Kitab Undang-Undang Hukum Perdata".to_string(),
                article: "Pasal 1338".to_string(),
                full_text: "Suatu perjanjian yang telah dilengkapi menurut aturan-aturan hukum menjadi undang-undang bagi mereka yang membuatnya. Semua perjanjian harus dilaksanakan dengan itikad baik.".to_string(),
                hierarchy_level: 2,
                status: RuleStatus::Active,
                effective_date: "1847-01-01".to_string(),
                source_url: None,
                access_date: chrono::Utc::now().format("%Y-%m-%d").to_string(),
                precedential_weight: None,
            });
            results.push(RuleEntry {
                id: format!("rule_{}", uuid::Uuid::new_v4()),
                regulation: "Kitab Undang-Undang Hukum Perdata".to_string(),
                article: "Pasal 1246".to_string(),
                full_text: "Orang yang karena salahnya mengakibatkan tidak dilaksanakannya suatu perikatan, wajib mengganti kerugian yang disebabkan oleh tidak dipenuhinya perikatan itu, baik kerugian yang telah terjadi maupun keuntungan yang diharapkan.".to_string(),
                hierarchy_level: 2,
                status: RuleStatus::Active,
                effective_date: "1847-01-01".to_string(),
                source_url: None,
                access_date: chrono::Utc::now().format("%Y-%m-%d").to_string(),
                precedential_weight: None,
            });
        }

        if lower_query.contains("karyawan") || lower_query.contains("pekerja") || lower_query.contains("phk") {
            results.push(RuleEntry {
                id: format!("rule_{}", uuid::Uuid::new_v4()),
                regulation: "Undang-Undang Nomor 13 Tahun 2003 tentang Ketenagakerjaan".to_string(),
                article: "Pasal 1 ayat (1)".to_string(),
                full_text: "Ketenagakerjaan adalah segala hal yang berhubungan dengan tenaga kerja sebelum, selama, dan sesudah masa kerja.".to_string(),
                hierarchy_level: 2,
                status: RuleStatus::Active,
                effective_date: "2003-03-25".to_string(),
                source_url: None,
                access_date: chrono::Utc::now().format("%Y-%m-%d").to_string(),
                precedential_weight: None,
            });
            results.push(RuleEntry {
                id: format!("rule_{}", uuid::Uuid::new_v4()),
                regulation: "Undang-Undang Nomor 13 Tahun 2003 tentang Ketenagakerjaan".to_string(),
                article: "Pasal 158".to_string(),
                full_text: "Setiap pengusaha yang melakukan pemutusan hubungan kerja karena perusahaan melakukan pengaturan ulang, merger, konsolidasi, atau alih daya, wajib meminta persetujuan terlebih dahulu kepada Panitia Pengusahaan Hubungan Industrial atau pejabat yang berwenang.".to_string(),
                hierarchy_level: 2,
                status: RuleStatus::Amended,
                effective_date: "2003-03-25".to_string(),
                source_url: None,
                access_date: chrono::Utc::now().format("%Y-%m-%d").to_string(),
                precedential_weight: Some(0.85),
            });
        }

        if lower_query.contains("tanah") || lower_query.contains("properti") || lower_query.contains("sertifikat") {
            results.push(RuleEntry {
                id: format!("rule_{}", uuid::Uuid::new_v4()),
                regulation: "Undang-Undang Nomor 5 Tahun 1960 tentang Peraturan Dasar Pokok-pokok Agraria".to_string(),
                article: "Pasal 19".to_string(),
                full_text: "Tanah yang dalam kewenangan Pemerintah Pusat dapat dalam jangka waktu tertentu dan dengan tidak melanggar peraturan perundang-undangan yang berlaku, diserahkan kepada orang atau badan hukum untuk diamati dan digunakan.".to_string(),
                hierarchy_level: 2,
                status: RuleStatus::Active,
                effective_date: "1960-09-24".to_string(),
                source_url: None,
                access_date: chrono::Utc::now().format("%Y-%m-%d").to_string(),
                precedential_weight: Some(0.9),
            });
        }

        if lower_query.contains("wanprestasi") || lower_query.contains("ingkar") {
            results.push(RuleEntry {
                id: format!("rule_{}", uuid::Uuid::new_v4()),
                regulation: "Kitab Undang-Undang Hukum Perdata".to_string(),
                article: "Pasal 1243".to_string(),
                full_text: "Debitur yang lalai memenuhi kewajibannya, wajib mengganti kerugian yang timbul karena kelalaiannya itu.".to_string(),
                hierarchy_level: 2,
                status: RuleStatus::Active,
                effective_date: "1847-01-01".to_string(),
                source_url: None,
                access_date: chrono::Utc::now().format("%Y-%m-%d").to_string(),
                precedential_weight: None,
            });
        }

        if results.is_empty() {
            results.push(RuleEntry {
                id: format!("rule_{}", uuid::Uuid::new_v4()),
                regulation: "Kitab Undang-Undang Hukum Perdata".to_string(),
                article: "Pasal 1".to_string(),
                full_text: "Hukum perdata berlaku kepada segala hal-hal yang dikenal dengan nama hukum perdata, dan segala peristiwa-peristiwa yang termasuk dalam lingkungannya.".to_string(),
                hierarchy_level: 2,
                status: RuleStatus::Active,
                effective_date: "1847-01-01".to_string(),
                source_url: None,
                access_date: chrono::Utc::now().format("%Y-%m-%d").to_string(),
                precedential_weight: None,
            });
        }

        results
    }
}

#[async_trait::async_trait]
impl NodeExecutor for RuleRetriever {
    async fn execute(&self, context: &mut ExecutionContext) -> Result<NodeResult, EngineError> {
        context.check_cancelled()?;
        info!(case_id=%context.case_id, "retrieving relevant legal rules");

        let issues: Vec<String> = context.case_graph.issues.iter().map(|i| i.content.clone()).collect();
        let facts: Vec<String> = context.case_graph.facts.iter().map(|f| f.content.clone()).collect();

        let query = format!("{} {}", issues.join(" "), facts.join(" "));
        let rules = self.search_knowledge_base(context, &query);

        for rule in &rules {
            context.case_graph.rules.push(crate::engine::context::GraphNode {
                id: rule.id.clone(),
                node_type: "rule".to_string(),
                content: format!("{} {}", rule.regulation, rule.article),
                metadata: serde_json::json!({"hierarchy_level": rule.hierarchy_level, "status": format!("{:?}", rule.status).to_lowercase()}),
                certainty: match rule.status {
                    RuleStatus::Active => 0.9,
                    RuleStatus::Amended => 0.7,
                    RuleStatus::Repealed => 0.1,
                },
            });
        }

        Ok(NodeResult {
            node_id: format!("rule_retrieval_{}", uuid::Uuid::new_v4()),
            node_type: "rule_retriever".to_string(),
            success: true,
            output: NodeOutput::Rules { rules },
            confidence: 0.75,
            warnings: if rules.iter().any(|r| matches!(r.status, RuleStatus::Amended)) {
                vec!["Beberapa peraturan telah diamendemen. Verifikasi versi terbaru.".to_string()]
            } else {
                Vec::new()
            },
            metadata: serde_json::json!({"total_rules": rules.len()}),
        })
    }

    fn node_type(&self) -> &'static str {
        "rule_retriever"
    }
}
