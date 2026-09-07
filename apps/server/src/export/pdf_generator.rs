use std::io::Write;

use serde::Serialize;

use crate::export::types::ExportFormat;

#[derive(Debug, Serialize)]
pub struct PdfDocument {
    pub title: String,
    pub sections: Vec<PdfSection>,
    pub footer: String,
}

#[derive(Debug, Serialize)]
pub struct PdfSection {
    pub heading: String,
    pub content: String,
}

pub fn generate_pdf(document: &PdfDocument) -> Result<Vec<u8>, crate::error::AppError> {
    let mut buffer = Vec::new();
    writeln!(buffer, "PAUGERAN — {}", document.title)?;
    writeln!(buffer, "{}", "=".repeat(60))?;
    writeln!(buffer)?;

    for section in &document.sections {
        writeln!(buffer, "{}", section.heading)?;
        writeln!(buffer, "{}", "-".repeat(40))?;
        writeln!(buffer, "{}", section.content)?;
        writeln!(buffer)?;
    }

    writeln!(buffer, "{}", "-".repeat(60))?;
    writeln!(buffer, "{}", document.footer)?;
    writeln!(
        buffer,
        "Dibuat dengan bantuan PAUGERAN AI. Analisis ini bukan nasihat hukum final dan wajib diverifikasi oleh Advokat berlisensi. Tanggal generate: {}",
        chrono::Utc::now().to_rfc3339()
    )?;

    Ok(buffer)
}
