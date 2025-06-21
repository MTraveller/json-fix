// src/diagnostics/bracket.rs

use crate::diagnostics::Diagnoser;
use crate::generated_patterns::regex::*;
use crate::types::diagnostic_core::{DiagnosticSeverity, FixDiagnostic, FixDiagnosticKind};

pub struct BracketDiagnoser;

impl Diagnoser for BracketDiagnoser {
    fn analyze(&self, input: &str) -> Vec<FixDiagnostic> {
        let mut diagnostics = Vec::new();
        let mut curly = 0;
        let mut square = 0;
        let mut has_extra = false;
        let mut has_missing = false;

        for ch in input.chars() {
            match ch {
                '{' => curly += 1,
                '}' => {
                    if curly == 0 {
                        has_extra = true;
                    } else {
                        curly -= 1;
                    }
                }
                '[' => square += 1,
                ']' => {
                    if square == 0 {
                        has_extra = true;
                    } else {
                        square -= 1;
                    }
                }
                _ => {}
            }
        }

        if curly > 0 || square > 0 {
            has_missing = true;
        }

        if has_extra {
            diagnostics.push(FixDiagnostic {
                kind: FixDiagnosticKind::Bracket,
                severity: DiagnosticSeverity::Error,
                message: "Extra closing bracket found".to_string(),
                span: estimate_bracket_span(input, false),
                ..Default::default()
            });
        }

        if has_missing {
            diagnostics.push(FixDiagnostic {
                kind: FixDiagnosticKind::Bracket,
                severity: DiagnosticSeverity::Error,
                message: "Missing closing bracket detected".to_string(),
                span: estimate_bracket_span(input, true),
                ..Default::default()
            });
        }

        diagnostics
    }
}
