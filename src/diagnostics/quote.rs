// src/diagnostics/quote.rs

use crate::types::diagnostic_meta::{DiagnosticCategory, DiagnosticSeverity};
use regex::Regex;

#[derive(Debug, Default)]
pub struct QuoteDiagnostics {
    pub has_single_quotes: bool,
    pub has_curly_quotes: bool,
    pub has_smart_quotes: bool,
    pub category: DiagnosticCategory,
    pub severity: DiagnosticSeverity,
}

pub fn analyze_quotes(json: &str) -> QuoteDiagnostics {
    let mut diag = QuoteDiagnostics::default();

    diag.category = DiagnosticCategory::Syntax;
    diag.severity = DiagnosticSeverity::Warning;

    // Match single-quoted keys or values
    let re_single = Regex::new(r#"'[^']*'"#).unwrap();
    if re_single.is_match(json) {
        diag.has_single_quotes = true;
    }

    // Detect curly quotes: “ ” ‘ ’
    if json.contains('“') || json.contains('”') || json.contains('‘') || json.contains('’')
    {
        diag.has_curly_quotes = true;
    }

    // Detect smart quotes (often copy-pasted from documents)
    let re_smart = Regex::new(r"[“”‘’]").unwrap();
    if re_smart.is_match(json) {
        diag.has_smart_quotes = true;
    }

    diag
}
