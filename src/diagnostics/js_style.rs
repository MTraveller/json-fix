use crate::types::diagnostic_meta::{DiagnosticCategory, DiagnosticSeverity};
use crate::utils::regex_utils::{RE_JS_COMMENTS, RE_NAN_INFINITY, RE_UNDEFINED};

#[derive(Debug, Default, Clone)]
pub struct JSStyleDiagnostics {
    pub has_undefined: bool,
    pub has_nan: bool,
    pub has_comments: bool,
    pub category: DiagnosticCategory,
    pub severity: DiagnosticSeverity,
}

pub fn analyze_js_styles(input: &str) -> JSStyleDiagnostics {
    let mut diag = JSStyleDiagnostics::default();

    diag.category = DiagnosticCategory::JSStyle;
    diag.severity = DiagnosticSeverity::Warning;

    if RE_UNDEFINED.is_match(input) {
        diag.has_undefined = true;
    }

    if RE_NAN_INFINITY.is_match(input) {
        diag.has_nan = true;
    }

    if RE_JS_COMMENTS.is_match(input) {
        diag.has_comments = true;
    }

    diag
}
