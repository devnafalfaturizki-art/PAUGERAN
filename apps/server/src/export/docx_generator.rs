use crate::export::{ExportDocument, ExportSection, ExportFormat};

pub fn generate_docx(document: &ExportDocument) -> Result<Vec<u8>, crate::error::AppError> {
    let mut buffer = Vec::new();
    buffer.extend_from_slice(b"<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n");
    buffer.extend_from_slice(b"<w:document xmlns:w=\"http://schemas.openxmlformats.org/wordprocessingml/2006/main\">\n");
    buffer.extend_from_slice(b"  <w:body>\n");

    buffer.extend_from_slice(b"    <w:p><w:r><w:t>");
    buffer.extend_from_slice(document.title.as_bytes());
    buffer.extend_from_slice(b"</w:t></w:r></w:p>\n");

    for section in &document.sections {
        buffer.extend_from_slice(b"    <w:p><w:r><w:t>");
        buffer.extend_from_slice(section.heading.as_bytes());
        buffer.extend_from_slice(b"</w:t></w:r></w:p>\n");
        buffer.extend_from_slice(b"    <w:p><w:r><w:t>");
        buffer.extend_from_slice(section.content.as_bytes());
        buffer.extend_from_slice(b"</w:t></w:r></w:p>\n");
    }

    buffer.extend_from_slice(b"    <w:p><w:r><w:t>");
    buffer.extend_from_slice(document.footer.as_bytes());
    buffer.extend_from_slice(b"</w:t></w:r></w:p>\n");

    buffer.extend_from_slice(b"  </w:body>\n");
    buffer.extend_from_slice(b"</w:document>\n");

    Ok(buffer)
}
