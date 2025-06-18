// src/diagnostics/comma.rs

use crate::types::diagnostic_meta::{DiagnosticCategory, DiagnosticSeverity};
use crate::utils::regex_utils::{
    RE_DOUBLE_COMMAS, RE_FALLBACK_ARTIFACTS, RE_KEY_VALUE_MISALIGNED, RE_ORPHANED_STRING_VALUE,
    RE_STRAY_COMMA_AFTER_OPENING,
};

#[derive(Debug, Default, Clone)]
pub struct CommaDiagnostics {
    pub has_double_commas: bool,
    pub has_orphaned_values: bool,
    pub has_key_value_misalignment: bool,
    pub has_stray_commas: bool,
    pub has_comma_value_chaining: bool,
    pub category: DiagnosticCategory,
    pub severity: DiagnosticSeverity,
}

pub fn analyze_commas(json: &str) -> CommaDiagnostics {
    let mut diag = CommaDiagnostics::default();

    diag.category = DiagnosticCategory::Syntax;
    diag.severity = DiagnosticSeverity::Warning;

    if RE_DOUBLE_COMMAS.is_match(json) {
        diag.has_double_commas = true;
    }

    if RE_KEY_VALUE_MISALIGNED.is_match(json) {
        diag.has_key_value_misalignment = true;
    }

    if RE_ORPHANED_STRING_VALUE.is_match(json) {
        diag.has_orphaned_values = true;
    }

    if RE_STRAY_COMMA_AFTER_OPENING.is_match(json) {
        diag.has_stray_commas = true;
    }

    if RE_FALLBACK_ARTIFACTS.is_match(json) {
        diag.has_comma_value_chaining = true;
    }

    diag
}
