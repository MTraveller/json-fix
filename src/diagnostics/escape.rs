// src/diagnostics/escape.rs

use crate::types::diagnostic_meta::{DiagnosticCategory, DiagnosticSeverity};
use regex::Regex;

#[derive(Debug, Default)]
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
    let re_invalid_escape = Regex::new(r#"\\[^btnfru"\\]"#).unwrap();
    if re_invalid_escape.is_match(json) {
        diag.has_invalid_escape = true;
    }

    // Detect broken Unicode sequences (e.g., \u12, \uGHIJ)
    let re_broken_unicode =
        Regex::new(r#"\\u(?:[0-9a-fA-F]{1,3}(?![0-9a-fA-F])|[^0-9a-fA-F]{1,4})"#).unwrap();
    if re_broken_unicode.is_match(json) {
        diag.has_broken_unicode = true;
    }

    diag
}
