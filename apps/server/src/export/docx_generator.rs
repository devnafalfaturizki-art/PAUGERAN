use docx_rs::*;
use serde::Serialize;

use crate::export::types::ExportFormat;

#[derive(Debug, Serialize)]
pub struct DocxDocument {
    pub title: String,
    pub sections: Vec<DocxSection>,
    pub footer: String,
}

#[derive(Debug, Serialize)]
pub struct DocxSection {
    pub heading: String,
    pub content: String,
}

pub fn generate_docx(document: &DocxDocument) -> Result<Vec<u8>, crate::error::AppError> {
    let mut paragraphs = vec![
        Paragraph::new(vec![
            Run::new(vec![Text::new(&document.title)]),
        ])
        .style(HeadingLevel::Heading1),
    ];

    for section in &document.sections {
        paragraphs.push(
            Paragraph::new(vec![
                Run::new(vec![Text::new(&section.heading)]),
            ])
            .style(HeadingLevel::Heading2),
        );
        paragraphs.push(Paragraph::new(vec![
            Run::new(vec![Text::new(&section.content)]),
        ]));
    }

    let doc = Docx::new(vec![
        Paragraph::new(vec![
            Run::new(vec![Text::new(&document.footer)]),
        ]),
    ])
    .into_docx();

    let mut buffer = Vec::new();
    doc.build(&mut buffer)
        .map_err(|e| crate::error::AppError::Export(format!("DOCX build failed: {}", e)))?;
    Ok(buffer)
}
