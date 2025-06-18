// src/diagnostics/misc.rs

use crate::types::diagnostic_meta::{DiagnosticCategory, DiagnosticSeverity};
use crate::utils::regex_utils::{RE_FALLBACK_ARTIFACTS, RE_NULL_SLOTS};

#[derive(Debug, Default, Clone)]
pub struct MiscDiagnostics {
    pub has_null_slots: bool,
    pub has_fallbacks: bool,
    pub category: DiagnosticCategory,
    pub severity: DiagnosticSeverity,
}

pub fn analyze_misc(json: &str) -> MiscDiagnostics {
    let mut diag = MiscDiagnostics::default();

    diag.category = DiagnosticCategory::Syntax;
    diag.severity = DiagnosticSeverity::Info;

    // Detect null slots like `"key": ,` or `"key": ]` needing filling
    if RE_NULL_SLOTS.is_match(json) {
        diag.has_null_slots = true;
    }

    // Detect fallback artifact patterns such as `, "something", ,`
    if RE_FALLBACK_ARTIFACTS.is_match(json) {
        diag.has_fallbacks = true;
    }

    diag
}
