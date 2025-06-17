// src/diagnostics/comma.rs

use crate::types::diagnostic_meta::{DiagnosticCategory, DiagnosticSeverity};
use regex::Regex;

#[derive(Debug, Default)]
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

    if json.contains(",,") {
        diag.has_double_commas = true;
    }

    let re_key_val_misaligned = Regex::new(r#"\s*:\s*,\s*""#).unwrap();
    if re_key_val_misaligned.is_match(json) {
        diag.has_key_value_misalignment = true;
    }

    let re_orphaned_string = Regex::new(r#"\bnull\s*,\s*"[^"]+""#).unwrap();
    if re_orphaned_string.is_match(json) {
        diag.has_orphaned_values = true;
    }

    let re_stray_commas = Regex::new(r#"(\{|\[)\s*,"#).unwrap();
    if re_stray_commas.is_match(json) {
        diag.has_stray_commas = true;
    }

    let re_comma_chaining = Regex::new(r#""[^"]+"\s*,\s*"[^"]+""#).unwrap();
    if re_comma_chaining.is_match(json) {
        diag.has_comma_value_chaining = true;
    }

    diag
}
