// src/diagnostics/bracket.rs

use crate::types::diagnostic_meta::{DiagnosticCategory, DiagnosticSeverity};

#[derive(Debug, Default, Clone)]
pub struct BracketDiagnostics {
    pub has_missing_closing: bool,
    pub has_extra_closing: bool,
    pub has_unbalanced_pairs: bool,
    pub category: DiagnosticCategory,
    pub severity: DiagnosticSeverity,
}

pub fn analyze_brackets(json: &str) -> BracketDiagnostics {
    let mut diag = BracketDiagnostics::default();

    diag.category = DiagnosticCategory::Structural;
    diag.severity = DiagnosticSeverity::Error;

    let mut curly = 0;
    let mut square = 0;

    for ch in json.chars() {
        match ch {
            '{' => curly += 1,
            '}' => {
                if curly == 0 {
                    diag.has_extra_closing = true;
                } else {
                    curly -= 1;
                }
            }
            '[' => square += 1,
            ']' => {
                if square == 0 {
                    diag.has_extra_closing = true;
                } else {
                    square -= 1;
                }
            }
            _ => {}
        }
    }

    if curly > 0 || square > 0 {
        diag.has_missing_closing = true;
    }

    diag.has_unbalanced_pairs = diag.has_missing_closing || diag.has_extra_closing;

    diag
}
