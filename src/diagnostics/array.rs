// src/diagnostics/array.rs

use crate::types::diagnostic_meta::{DiagnosticCategory, DiagnosticSeverity};

#[derive(Debug, Default)]
pub struct ArrayDiagnostics {
    pub has_trailing_commas: bool,
    pub has_malformed_nesting: bool,
    pub has_empty_array_slots: bool,
    pub category: DiagnosticCategory,
    pub severity: DiagnosticSeverity,
}

pub fn analyze_arrays(json: &str) -> ArrayDiagnostics {
    let mut diag = ArrayDiagnostics::default();

    diag.category = DiagnosticCategory::Syntax;
    diag.severity = DiagnosticSeverity::Warning;

    if json.contains("[,") || json.contains(",]") {
        diag.has_trailing_commas = true;
    }

    if json.contains("[][]") || json.contains("[[]") || json.contains("[]]") {
        diag.has_malformed_nesting = true;
    }

    if json.contains("[, ,") || json.contains("[null, ,") {
        diag.has_empty_array_slots = true;
    }

    diag
}
