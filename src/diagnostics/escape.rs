// src/diagnostics/escape.rs

use crate::diagnostics::Diagnoser;
use crate::types::diagnostic_core::{DiagnosticSeverity, FixDiagnostic, FixDiagnosticKind};

include!("../../generated_patterns/escape.rs");

pub struct EscapeDiagnoser;

impl Diagnoser for EscapeDiagnoser {
    fn analyze(&self, input: &str) -> Vec<FixDiagnostic> {
        let mut diagnostics = Vec::new();

        let pattern_map = [
            ("RE_INVALID_ESCAPES", &RE_INVALID_ESCAPES),
            ("RE_BROKEN_UNICODE_ESCAPES", &RE_BROKEN_UNICODE_ESCAPES),
        ];

        for (key, regex) in pattern_map {
            if let Some(mat) = regex.find(input) {
                diagnostics.push(FixDiagnostic {
                    kind: FixDiagnosticKind::Escape,
                    severity: DiagnosticSeverity::Error,
                    message: REGEX_DESCRIPTIONS.get(key).unwrap().to_string(),
                    span: Some((mat.start(), mat.end())),
                    ..Default::default()
                });
            }
        }

        diagnostics
    }
}
