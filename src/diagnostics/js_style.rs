// src/diagnostics/js_style.rs

use crate::diagnostics::Diagnoser;
use crate::generated_patterns::regex::*;
use crate::types::diagnostic_core::{DiagnosticSeverity, FixDiagnostic, FixDiagnosticKind};

pub struct JSStyleDiagnoser;

impl Diagnoser for JSStyleDiagnoser {
    fn analyze(&self, input: &str) -> Vec<FixDiagnostic> {
        let mut diagnostics = Vec::new();

        let pattern_map = [
            ("RE_UNDEFINED", &RE_UNDEFINED),
            ("RE_NAN_INFINITY", &RE_NAN_INFINITY),
            ("RE_JS_COMMENTS", &RE_JS_COMMENTS),
        ];

        for (key, regex) in pattern_map {
            if let Some(mat) = regex.find(input) {
                diagnostics.push(FixDiagnostic {
                    kind: FixDiagnosticKind::JS,
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
