// src/diagnostics/colon.rs

use crate::diagnostics::Diagnoser;
use crate::generated_patterns::regex::*;
use crate::types::diagnostic_core::{DiagnosticSeverity, FixDiagnostic, FixDiagnosticKind};

pub struct ColonDiagnoser;

impl Diagnoser for ColonDiagnoser {
    fn analyze(&self, input: &str) -> Vec<FixDiagnostic> {
        let mut diagnostics = Vec::new();

        let pattern_map = [
            ("RE_MISSING_COLON", &RE_MISSING_COLON),
            ("RE_DOUBLE_COLON", &RE_DOUBLE_COLON),
        ];

        for (key, regex) in pattern_map {
            if let Some(mat) = regex.find(input) {
                diagnostics.push(FixDiagnostic {
                    kind: FixDiagnosticKind::Colon,
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
