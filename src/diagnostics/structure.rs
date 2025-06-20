// src/diagnostics/structure.rs

use crate::diagnostics::Diagnoser;
use crate::types::diagnostic_core::{DiagnosticSeverity, FixDiagnostic, FixDiagnosticKind};

include!("../../generated_patterns/structure.rs");

pub struct StructureDiagnoser;

impl Diagnoser for StructureDiagnoser {
    fn analyze(&self, input: &str) -> Vec<FixDiagnostic> {
        let mut diagnostics = Vec::new();

        let pattern_map = [
            (
                "RE_CONCATENATED_JSON_OBJECTS",
                &RE_CONCATENATED_JSON_OBJECTS,
            ),
            ("RE_ORPHANED_OPEN_BRACE", &RE_ORPHANED_OPEN_BRACE),
            ("RE_ORPHANED_CLOSE_BRACE", &RE_ORPHANED_CLOSE_BRACE),
        ];

        for (key, regex) in pattern_map {
            for mat in regex.find_iter(input) {
                diagnostics.push(FixDiagnostic {
                    kind: FixDiagnosticKind::Structure,
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
