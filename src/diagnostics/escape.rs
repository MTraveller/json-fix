// src/diagnostics/escape.rs

use crate::types::diagnostic_meta::{DiagnosticCategory, DiagnosticSeverity};
use crate::utils::regex_utils::{RE_BROKEN_UNICODE_ESCAPES, RE_INVALID_ESCAPES};

#[derive(Debug, Default, Clone)]
pub struct EscapeDiagnostics {
    pub has_invalid_escape: bool,
    pub has_broken_unicode: bool,
    pub category: DiagnosticCategory,
    pub severity: DiagnosticSeverity,
}

pub fn analyze_escapes(json: &str) -> EscapeDiagnostics {
    let mut diag = EscapeDiagnostics::default();

    diag.category = DiagnosticCategory::Syntax;
    diag.severity = DiagnosticSeverity::Error;

    // Detect invalid escape sequences (e.g., \q, \x, etc.)
    // let re_invalid_escape = Regex::new(r#"\\[^btnfru"\\]"#).unwrap();
    if RE_INVALID_ESCAPES.is_match(json) {
        diag.has_invalid_escape = true;
    }

    // Detect broken Unicode sequences (e.g., \u12, \uGHIJ)
    if RE_BROKEN_UNICODE_ESCAPES.is_match(json) {
        diag.has_broken_unicode = true;
    }

    diag
}
