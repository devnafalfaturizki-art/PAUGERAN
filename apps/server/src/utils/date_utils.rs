//! Date and time utilities used by the engine for temporal law
//! application and audit logging.

use chrono::{DateTime, Utc};

pub fn now() -> DateTime<Utc> {
    Utc::now()
}

pub fn parse_iso8601(value: &str) -> Option<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(value)
        .ok()
        .map(|dt| dt.with_timezone(&Utc))
}

pub fn days_between(start: DateTime<Utc>, end: DateTime<Utc>) -> i64 {
    (end - start).num_days()
}

pub fn years_between(start: DateTime<Utc>, end: DateTime<Utc>) -> i64 {
    days_between(start, end) / 365
}

pub fn format_iso(date: DateTime<Utc>) -> String {
    date.to_rfc3339()
}