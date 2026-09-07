use crate::error::AppError;

pub fn render_hierarchy(hierarchy: &[(i32, String)]) -> String {
    let mut output = String::new();
    for (level, name) in hierarchy {
        output.push_str(&format!("[{}] {}\n", level, name));
    }
    output
}
