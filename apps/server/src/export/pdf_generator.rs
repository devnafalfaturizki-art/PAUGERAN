use std::io::Write;

use serde::Serialize;

use crate::export::types::ExportFormat;

pub fn generate_pdf(document: &crate::export::ExportDocument) -> Result<Vec<u8>, crate::error::AppError> {
    let mut buffer = Vec::new();
    writeln!(buffer, "PAUGERAN — {}", document.title).map_err(|e| crate::error::AppError::Export(format!("write failed: {}", e)))?;
    writeln!(buffer, "{}", "=".repeat(60)).map_err(|e| crate::error::AppError::Export(format!("write failed: {}", e)))?;
    writeln!(buffer).map_err(|e| crate::error::AppError::Export(format!("write failed: {}", e)))?;

    for section in &document.sections {
        writeln!(buffer, "{}", section.heading).map_err(|e| crate::error::AppError::Export(format!("write failed: {}", e)))?;
        writeln!(buffer, "{}", "-".repeat(40)).map_err(|e| crate::error::AppError::Export(format!("write failed: {}", e)))?;
        writeln!(buffer, "{}", section.content).map_err(|e| crate::error::AppError::Export(format!("write failed: {}", e)))?;
        writeln!(buffer).map_err(|e| crate::error::AppError::Export(format!("write failed: {}", e)))?;
    }

    writeln!(buffer, "{}", "-".repeat(60)).map_err(|e| crate::error::AppError::Export(format!("write failed: {}", e)))?;
    writeln!(buffer, "{}", document.footer).map_err(|e| crate::error::AppError::Export(format!("write failed: {}", e)))?;
    writeln!(
        buffer,
        "Dibuat dengan bantuan PAUGERAN AI. Analisis ini bukan nasihat hukum final dan wajib diverifikasi oleh Advokat berlisensi. Tanggal generate: {}",
        chrono::Utc::now().to_rfc3339()
    ).map_err(|e| crate::error::AppError::Export(format!("write failed: {}", e)))?;

    Ok(buffer)
}
