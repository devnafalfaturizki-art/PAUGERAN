use crate::export::types::{ExportDocument, ExportSection};

pub fn format_document(document: &ExportDocument, mode: &str) -> Result<String, crate::error::AppError> {
    match mode {
        "exploration" => Ok(crate::export::templates::exploration::render_exploration(document)),
        "preventive" => Ok(crate::export::templates::preventive::render_preventive(document)),
        "dispute" => Ok(crate::export::templates::dispute::render_dispute(document)),
        "litigation_prep" => Ok(crate::export::templates::litigation::render_litigation(document)),
        "adversarial" => Ok(crate::export::templates::adversarial::render_adversarial(document)),
        "neutral" => Ok(crate::export::templates::neutral::render_neutral(document)),
        _ => Ok(crate::export::templates::exploration::render_exploration(document)),
    }
}
