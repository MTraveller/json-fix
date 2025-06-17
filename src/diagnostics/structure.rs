// src/diagnostics/structure.rs

use regex::Regex;

#[derive(Debug, Default)]
pub struct StructureDiagnostics {
    pub has_concatenated_json: bool,
    pub has_orphaned_braces: bool,
}

pub fn analyze_structure(input: &str) -> StructureDiagnostics {
    let mut diag = StructureDiagnostics::default();

    // Detects if multiple root-level JSON objects are concatenated without a comma or array
    let concat_pattern = Regex::new(r"\}\s*\{").unwrap();
    if concat_pattern.is_match(input) {
        diag.has_concatenated_json = true;
    }

    // Counts opening and closing braces to check for imbalance
    let open_count = input.matches('{').count();
    let close_count = input.matches('}').count();
    if open_count != close_count {
        diag.has_orphaned_braces = true;
    }

    diag
}
