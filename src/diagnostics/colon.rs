// src/diagnostics/colon.rs

use crate::types::diagnostic_meta::{DiagnosticCategory, DiagnosticSeverity};
use regex::Regex;

#[derive(Debug, Default)]
pub struct ColonDiagnostics {
    pub has_missing_colons: bool,
    pub has_colon_misuse: bool,
    pub category: DiagnosticCategory,
    pub severity: DiagnosticSeverity,
}

pub fn analyze_colons(input: &str) -> ColonDiagnostics {
    let mut diag = ColonDiagnostics::default();

    diag.category = DiagnosticCategory::Syntax;
    diag.severity = DiagnosticSeverity::Error;

    // Detect missing colons between key-value pairs
    // e.g. "key" "value" or "key" 123
    let missing_colon_pattern = Regex::new(r#""[^"]+"\s+(?="[^"]+"|\d+|true|false|null)"#).unwrap();
    if missing_colon_pattern.is_match(input) {
        diag.has_missing_colons = true;
    }

    // Detect invalid colon usage like "::"
    let colon_misuse_pattern = Regex::new(r"::+").unwrap();
    if colon_misuse_pattern.is_match(input) {
        diag.has_colon_misuse = true;
    }

    diag
}
