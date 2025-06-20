// src/diagnostics/comma.rs

use crate::diagnostics::Diagnoser;
use crate::types::diagnostic_core::{DiagnosticSeverity, FixDiagnostic, FixDiagnosticKind};

include!("../../generated_patterns/comma.rs");

pub struct CommaDiagnoser;

impl Diagnoser for CommaDiagnoser {
    fn analyze(&self, input: &str) -> Vec<FixDiagnostic> {
        let mut diagnostics = Vec::new();

        let pattern_map = [
            ("RE_DOUBLE_COMMAS", &RE_DOUBLE_COMMAS),
            ("RE_KEY_VALUE_MISALIGNED", &RE_KEY_VALUE_MISALIGNED),
            ("RE_ORPHANED_STRING_VALUE", &RE_ORPHANED_STRING_VALUE),
            (
                "RE_STRAY_COMMA_AFTER_OPENING",
                &RE_STRAY_COMMA_AFTER_OPENING,
            ),
            ("RE_FALLBACK_ARTIFACTS", &RE_FALLBACK_ARTIFACTS),
        ];

        for (key, regex) in pattern_map {
            if let Some(mat) = regex.find(input) {
                diagnostics.push(FixDiagnostic {
                    kind: FixDiagnosticKind::Comma,
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
