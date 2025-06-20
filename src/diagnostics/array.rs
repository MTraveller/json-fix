// src/diagnostics/array.rs

use crate::diagnostics::Diagnoser;
use crate::types::diagnostic_core::{DiagnosticSeverity, FixDiagnostic, FixDiagnosticKind};
// use crate::types::emotion_phase::EmotionPhase;
// use crate::types::fixer_context::FixContext;
// use crate::utils::souldiag_utils::apply_diagnosis;

include!("../../generated_patterns/array.rs");

/// Modular diagnoser for array-related syntax issues.
/// This is the Fitrah-aligned version using the Diagnoser trait.
pub struct ArrayDiagnoser;

impl Diagnoser for ArrayDiagnoser {
    fn analyze(&self, input: &str) -> Vec<FixDiagnostic> {
        let mut diagnostics = Vec::new();

        let pattern_map = [
            ("RE_TRAILING_COMMA_IN_ARRAY", &RE_TRAILING_COMMA_IN_ARRAY),
            ("RE_MALFORMED_NESTED_ARRAYS", &RE_MALFORMED_NESTED_ARRAYS),
            ("RE_EMPTY_ARRAY_SLOTS", &RE_EMPTY_ARRAY_SLOTS),
            ("RE_ARRAY_STRING_MISALIGN", &RE_ARRAY_STRING_MISALIGN),
            ("RE_ADJACENT_STRINGS", &RE_ADJACENT_STRINGS),
            ("RE_FALLBACK_ARTIFACTS", &RE_FALLBACK_ARTIFACTS),
            ("RE_LEADING_COMMA_IN_ARRAY", &RE_LEADING_COMMA_IN_ARRAY),
        ];

        for (key, regex) in pattern_map {
            if let Some(mat) = regex.find(input) {
                diagnostics.push(FixDiagnostic {
                    kind: FixDiagnosticKind::Array,
                    severity: DiagnosticSeverity::Warning,
                    message: REGEX_DESCRIPTIONS.get(key).unwrap().to_string(),
                    regex_key: key.to_string(),
                    span: Some((mat.start(), mat.end())),
                });
            }
        }

        diagnostics
    }
}
