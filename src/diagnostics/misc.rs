// src/diagnostics/misc.rs

use crate::types::diagnostic_meta::{DiagnosticCategory, DiagnosticSeverity};
use regex::Regex;

#[derive(Debug, Default)]
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
    let re_null_slots = Regex::new(r#""\s*:\s*(,|\])"#).unwrap();
    if re_null_slots.is_match(json) {
        diag.has_null_slots = true;
    }

    // Detect fallback artifact patterns such as `, "something", ,`
    let re_fallbacks = Regex::new(r#"\s*,\s*("[^"]*")\s*,\s*"#).unwrap();
    if re_fallbacks.is_match(json) {
        diag.has_fallbacks = true;
    }

    diag
}
