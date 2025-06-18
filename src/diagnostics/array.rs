// src/diagnostics/array.rs

use crate::types::diagnostic_meta::{DiagnosticCategory, DiagnosticSeverity};
use crate::utils::regex_utils::{
    RE_ADJACENT_STRINGS, RE_ARRAY_STRING_MISALIGN, RE_EMPTY_ARRAY_SLOTS, RE_FALLBACK_ARTIFACTS,
    RE_MALFORMED_NESTED_ARRAYS, RE_TRAILING_COMMA_IN_ARRAY,
};

#[derive(Debug, Default, Clone)]
pub struct ArrayDiagnostics {
    pub has_trailing_commas: bool,
    pub has_malformed_nesting: bool,
    pub has_empty_array_slots: bool,
    pub has_misaligned_strings: bool,
    pub has_adjacent_strings: bool,
    pub has_fallback_artifacts: bool,
    pub category: DiagnosticCategory,
    pub severity: DiagnosticSeverity,
}

pub fn analyze_arrays(json: &str) -> ArrayDiagnostics {
    let mut diag = ArrayDiagnostics::default();

    diag.category = DiagnosticCategory::Syntax;
    diag.severity = DiagnosticSeverity::Warning;

    if RE_TRAILING_COMMA_IN_ARRAY.is_match(json) {
        diag.has_trailing_commas = true;
    }

    if RE_MALFORMED_NESTED_ARRAYS.is_match(json) {
        diag.has_malformed_nesting = true;
    }

    if RE_EMPTY_ARRAY_SLOTS.is_match(json) {
        diag.has_empty_array_slots = true;
    }

    if RE_ARRAY_STRING_MISALIGN.is_match(json) {
        diag.has_misaligned_strings = true;
    }

    if RE_ADJACENT_STRINGS.is_match(json) {
        diag.has_adjacent_strings = true;
    }

    if RE_FALLBACK_ARTIFACTS.is_match(json) {
        diag.has_fallback_artifacts = true;
    }

    diag
}
