//! Input validation helpers used by HTTP handlers.

use super::id_generator::is_valid_uuid;

pub fn require_non_empty(field: &str, value: &str) -> Result<(), String> {
    if value.trim().is_empty() {
        Err(format!("{field} tidak boleh kosong"))
    } else {
        Ok(())
    }
}

pub fn require_uuid(field: &str, value: &str) -> Result<(), String> {
    if is_valid_uuid(value) {
        Ok(())
    } else {
        Err(format!("{field} harus berupa UUID"))
    }
}

pub fn clamp_length(field: &str, value: &str, max: usize) -> Result<String, String> {
    if value.chars().count() > max {
        Err(format!("{field} melebihi panjang maksimum {max}"))
    } else {
        Ok(value.to_string())
    }
}

pub fn require_max_length(field: &str, value: &str, max: usize) -> Result<(), String> {
    if value.chars().count() > max {
        Err(format!("{field} melebihi panjang maksimum {max}"))
    } else {
        Ok(())
    }
}