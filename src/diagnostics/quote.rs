// src/diagnostics/quote.rs

use crate::types::diagnostic_meta::{DiagnosticCategory, DiagnosticSeverity};
use crate::utils::regex_utils::{RE_CURLY_QUOTES, RE_SINGLE_QUOTES, RE_SMART_QUOTES};

#[derive(Debug, Default, Clone)]
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
    if RE_SINGLE_QUOTES.is_match(json) {
        diag.has_single_quotes = true;
    }

    // Detect curly quotes: ‘ ’
    if RE_CURLY_QUOTES.is_match(json) {
        diag.has_curly_quotes = true;
    }

    // Detect smart quotes: “ ”
    if RE_SMART_QUOTES.is_match(json) {
        diag.has_smart_quotes = true;
    }

    diag
}
