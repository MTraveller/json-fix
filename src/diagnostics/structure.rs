// src/diagnostics/structure.rs

use crate::types::diagnostic_meta::{DiagnosticCategory, DiagnosticSeverity};
use regex::Regex;

#[derive(Debug, Default)]
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
    let concat_pattern = Regex::new(r"\}\s*\{").unwrap();
    if concat_pattern.is_match(input) {
        diag.has_concatenated_json = true;
    }

    // Detect potential brace imbalance via regex match rather than just count
    let re_orphaned_open = Regex::new(r#"\{[^\{\}]*$"#).unwrap();
    let re_orphaned_close = Regex::new(r#"^[^\{\}]*\}"#).unwrap();
    if re_orphaned_open.is_match(input) || re_orphaned_close.is_match(input) {
        diag.has_orphaned_braces = true;
    }

    diag
}
