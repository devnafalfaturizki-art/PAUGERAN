use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum ExportFormat {
    Pdf,
    Docx,
}

#[derive(Debug, Serialize)]
pub struct ExportDocument {
    pub title: String,
    pub sections: Vec<ExportSection>,
    pub footer: String,
}

#[derive(Debug, Serialize)]
pub struct ExportSection {
    pub heading: String,
    pub content: String,
}
