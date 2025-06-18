// src/diagnostics/key.rs

use crate::types::diagnostic_meta::{DiagnosticCategory, DiagnosticSeverity};
use crate::utils::regex_utils::{RE_KEY_TRAPS, RE_UNQUOTED_KEYS};

#[derive(Debug, Default, Clone)]
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
    if RE_UNQUOTED_KEYS.is_match(json) {
        diag.has_unquoted_keys = true;
    }

    // Detect key traps (e.g., "emotion": , "hopeful")
    if RE_KEY_TRAPS.is_match(json) {
        diag.has_key_traps = true;
    }

    diag
}
