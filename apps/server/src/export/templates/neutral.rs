use crate::export::{ExportDocument, ExportSection};

pub fn render_neutral(document: &ExportDocument) -> String {
    let mut output = String::new();
    output.push_str(&format!("{}\n", document.title));
    output.push_str(&"=".repeat(document.title.len()));
    output.push_str("\n\n");
    for section in &document.sections {
        output.push_str(&format!("{}\n", section.heading));
        output.push_str(&"-".repeat(section.heading.len()));
        output.push_str("\n");
        output.push_str(&format!("{}\n\n", section.content));
    }
    output.push_str(&format!("{}\n", document.footer));
    output
}
