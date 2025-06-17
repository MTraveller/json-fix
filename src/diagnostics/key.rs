// src/diagnostics/key.rs

use crate::types::diagnostic_meta::{DiagnosticCategory, DiagnosticSeverity};
use regex::Regex;

#[derive(Debug, Default)]
pub struct KeyDiagnostics {
    pub has_unquoted_keys: bool,
    pub has_key_traps: bool,
    pub category: DiagnosticCategory,
    pub severity: DiagnosticSeverity,
}

pub fn analyze_keys(json: &str) -> KeyDiagnostics {
    let mut diag = KeyDiagnostics::default();

    diag.category = DiagnosticCategory::Syntax;
    diag.severity = DiagnosticSeverity::Warning;

    // Detect unquoted keys (e.g., {key: "value"})
    let re_unquoted_keys = Regex::new(r#"(?m)(?:\{|\s|,)([a-zA-Z_][a-zA-Z0-9_-]*)\s*:"#).unwrap();
    if re_unquoted_keys.is_match(json) {
        diag.has_unquoted_keys = true;
    }

    // Detect key traps (e.g., "emotion": , "hopeful")
    let re_key_trap = Regex::new(r#""\s*:\s*,\s*""#).unwrap();
    if re_key_trap.is_match(json) {
        diag.has_key_traps = true;
    }

    diag
}
