use crate::{
    database::Database,
    error::AppError,
    export::{formatter, types::ExportFormat, pdf_generator::generate_pdf, docx_generator::generate_docx},
    http::state::AppState,
};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

#[derive(serde::Deserialize)]
pub struct ExportRequest {
    pub format: String,
}

pub async fn export_case(
    State(state): State<Arc<AppState>>,
    Path(case_id): Path<String>,
    Json(payload): Json<ExportRequest>,
) -> Result<impl IntoResponse, AppError> {
    let record = state
        .database
        .find_case(&case_id)
        .await?
        .ok_or_else(|| AppError::NotFound("perkara tidak ditemukan".into()))?;

    let mode = crate::engine::mode_router::ReasoningMode::parse(&record.mode).unwrap_or(crate::engine::mode_router::ReasoningMode::Exploration);

    let document = crate::export::ExportDocument {
        title: record.title.clone(),
        sections: vec![
            crate::export::ExportSection {
                heading: "Ringkasan Perkara".into(),
                content: format!("Perkara: {}\nState: {:?}\nMode: {}", record.title, record.state, record.mode),
            },
            crate::export::ExportSection {
                heading: "Analisis Hukum".into(),
                content: "Analisis lengkap akan dihasilkan setelah penalaran mode aktif selesai.".into(),
            },
        ],
        footer: "Dibuat dengan bantuan PAUGERAN AI.".into(),
    };

    let format = match payload.format.as_str() {
        "docx" => ExportFormat::Docx,
        _ => ExportFormat::Pdf,
    };

    match format {
        ExportFormat::Pdf => {
            let bytes = generate_pdf(&document)?;
            Ok((
                [("content-type", "application/pdf")],
                bytes,
            ))
        }
        ExportFormat::Docx => {
            let bytes = generate_docx(&document)?;
            Ok((
                [("content-type", "application/vnd.openxmlformats-officedocument.wordprocessingml.document")],
                bytes,
            ))
        }
    }
}
