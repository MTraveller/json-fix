// src/diagnostics/misc.rs

use crate::diagnostics::Diagnoser;
use crate::types::diagnostic_core::{DiagnosticSeverity, FixDiagnostic, FixDiagnosticKind};

include!("../../generated_patterns/misc.rs");

pub struct MiscDiagnoser;

impl Diagnoser for MiscDiagnoser {
    fn analyze(&self, input: &str) -> Vec<FixDiagnostic> {
        let mut diagnostics = Vec::new();

        let pattern_map = [
            ("RE_NULL_SLOTS", &RE_NULL_SLOTS),
            ("RE_FALLBACK_ARTIFACTS", &RE_FALLBACK_ARTIFACTS),
        ];

        for (key, regex) in pattern_map {
            if let Some(mat) = regex.find(input) {
                diagnostics.push(FixDiagnostic {
                    kind: FixDiagnosticKind::Misc,
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
