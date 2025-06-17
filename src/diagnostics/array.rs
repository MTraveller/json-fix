// src/diagnostics/array.rs

use crate::types::diagnostic_meta::{DiagnosticCategory, DiagnosticSeverity};
use regex::Regex;

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

    // Detect trailing commas before closing bracket
    let re_trailing = Regex::new(r#",\s*\]"#).unwrap();
    if re_trailing.is_match(json) {
        diag.has_trailing_commas = true;
    }

    // Detect malformed nested arrays like [][], [[] or []]
    let re_malformed = Regex::new(r"\[\s*\[\s*\[|\[\]\[\]|\[\]\]").unwrap();
    if re_malformed.is_match(json) {
        diag.has_malformed_nesting = true;
    }

    // Detect empty array slots: [1, , 2] or [null, , 3]
    let re_empty = Regex::new(r#"\[\s*(null\s*,)?\s*,\s*"#).unwrap();
    if re_empty.is_match(json) {
        diag.has_empty_array_slots = true;
    }

    diag
}
