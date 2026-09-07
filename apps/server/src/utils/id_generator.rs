//! UUID generation utility.
//!
//! Centralised so that handlers, repositories, and the engine all
//! emit IDs in the same canonical `String` form.

use uuid::Uuid;

/// Generate a fresh v4 UUID as a hyphenated string.
pub fn new_uuid() -> String {
    Uuid::new_v4().to_string()
}

/// Validate that the supplied string is a well-formed UUID.
///
/// Returns `true` if the value parses, `false` otherwise. Used by
/// route handlers that take user-supplied IDs in path parameters
/// and want to short-circuit before hitting the database.
pub fn is_valid_uuid(value: &str) -> bool {
    Uuid::parse_str(value).is_ok()
}