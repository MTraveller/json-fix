// src/diagnostics/markdown.rs

use crate::types::diagnostic_meta::{DiagnosticCategory, DiagnosticSeverity};
use crate::utils::regex_utils::{RE_MARKDOWN_JSON_BLOCK, RE_MARKDOWN_WRAPPER};

#[derive(Debug, Default, Clone)]
pub struct MarkdownDiagnostics {
    pub has_markdown_wrappers: bool,
    pub has_embedded_json: bool,
    pub category: DiagnosticCategory,
    pub severity: DiagnosticSeverity,
}

pub fn analyze_markdown(json: &str) -> MarkdownDiagnostics {
    let mut diag = MarkdownDiagnostics::default();

    diag.category = DiagnosticCategory::Wrapper;
    diag.severity = DiagnosticSeverity::Info;

    // Detect markdown code block wrappers (```json ... ```)
    if RE_MARKDOWN_WRAPPER.is_match(json) {
        diag.has_markdown_wrappers = true;
    }

    // Detect embedded JSON inside larger markdown or text (heuristic)
    if RE_MARKDOWN_JSON_BLOCK.is_match(json) && json.len() > 300 {
        diag.has_embedded_json = true;
    }

    diag
}
