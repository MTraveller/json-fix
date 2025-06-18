// src/diagnostics/colon.rs

use crate::types::diagnostic_meta::{DiagnosticCategory, DiagnosticSeverity};
use crate::utils::regex_utils::{RE_DOUBLE_COLON, RE_MISSING_COLON};

#[derive(Debug, Default, Clone)]
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
    if RE_MISSING_COLON.is_match(input) {
        diag.has_missing_colons = true;
    }

    // Detect invalid colon usage like "::"
    if RE_DOUBLE_COLON.is_match(input) {
        diag.has_colon_misuse = true;
    }

    diag
}
