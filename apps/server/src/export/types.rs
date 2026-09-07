use serde::Serialize;

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
