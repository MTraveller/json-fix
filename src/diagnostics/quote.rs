// src/diagnostics/quote.rs

use crate::diagnostics::Diagnoser;
use crate::types::diagnostic_core::{DiagnosticSeverity, FixDiagnostic, FixDiagnosticKind};

include!("../../generated_patterns/quote.rs");

pub struct QuoteDiagnoser;

impl Diagnoser for QuoteDiagnoser {
    fn analyze(&self, input: &str) -> Vec<FixDiagnostic> {
        let mut diagnostics = Vec::new();

        let pattern_map = [
            ("RE_SINGLE_QUOTES", &RE_SINGLE_QUOTES),
            ("RE_CURLY_QUOTES", &RE_CURLY_QUOTES),
            ("RE_SMART_QUOTES", &RE_SMART_QUOTES),
        ];

        for (key, regex) in pattern_map {
            if let Some(mat) = regex.find(input) {
                diagnostics.push(FixDiagnostic {
                    kind: FixDiagnosticKind::Quote,
                    severity: DiagnosticSeverity::Warning,
                    message: REGEX_DESCRIPTIONS.get(key).unwrap().to_string(),
                    span: Some((mat.start(), mat.end())),
                    ..Default::default()
                });
            }
        }

        diagnostics
    }
}
