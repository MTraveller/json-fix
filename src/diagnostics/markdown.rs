// src/diagnostics/markdown.rs

use crate::diagnostics::Diagnoser;
use crate::generated_patterns::regex::*;
use crate::types::diagnostic_core::{DiagnosticSeverity, FixDiagnostic, FixDiagnosticKind};

pub struct MarkdownDiagnoser;

impl Diagnoser for MarkdownDiagnoser {
    fn analyze(&self, input: &str) -> Vec<FixDiagnostic> {
        let mut diagnostics = Vec::new();

        let pattern_map = [
            ("RE_MARKDOWN_WRAPPER", &RE_MARKDOWN_WRAPPER),
            ("RE_MARKDOWN_JSON_BLOCK", &RE_MARKDOWN_JSON_BLOCK),
        ];

        for (key, regex) in pattern_map {
            if let Some(mat) = regex.find(input) {
                diagnostics.push(FixDiagnostic {
                    kind: FixDiagnosticKind::Markdown,
                    severity: DiagnosticSeverity::Info,
                    message: REGEX_DESCRIPTIONS.get(key).unwrap().to_string(),
                    span: Some((mat.start(), mat.end())),
                    ..Default::default()
                });
            }
        }

        diagnostics
    }
}
