// src/diagnostics/structure.rs

use crate::types::diagnostic_meta::{DiagnosticCategory, DiagnosticSeverity};
use crate::utils::regex_utils::{
    RE_CONCATENATED_JSON_OBJECTS, RE_ORPHANED_CLOSE_BRACE, RE_ORPHANED_OPEN_BRACE,
};

#[derive(Debug, Default, Clone)]
pub struct StructureDiagnostics {
    pub has_concatenated_json: bool,
    pub has_orphaned_braces: bool,
    pub category: DiagnosticCategory,
    pub severity: DiagnosticSeverity,
}

pub fn analyze_structure(input: &str) -> StructureDiagnostics {
    let mut diag = StructureDiagnostics::default();

    diag.category = DiagnosticCategory::Structural;
    diag.severity = DiagnosticSeverity::Error;

    // Detects if multiple root-level JSON objects are concatenated without a comma or array
    if RE_CONCATENATED_JSON_OBJECTS.is_match(input) {
        diag.has_concatenated_json = true;
    }

    // Detect potential brace imbalance via regex match rather than just count
    if RE_ORPHANED_OPEN_BRACE.is_match(input) || RE_ORPHANED_CLOSE_BRACE.is_match(input) {
        diag.has_orphaned_braces = true;
    }

    diag
}
