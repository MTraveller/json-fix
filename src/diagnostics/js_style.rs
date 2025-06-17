use crate::types::diagnostic_meta::{DiagnosticCategory, DiagnosticSeverity};
use regex::Regex;

#[derive(Debug, Default)]
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

    let re_undefined = Regex::new(r#"(?i)\bundefined\b"#).unwrap();
    if re_undefined.is_match(input) {
        diag.has_undefined = true;
    }

    let re_nan_inf = Regex::new(r#"(?i)\b(NaN|Infinity|-Infinity)\b"#).unwrap();
    if re_nan_inf.is_match(input) {
        diag.has_nan = true;
    }

    let re_comments = Regex::new(r#"(?m)//.*?$|/\*[\s\S]*?\*/"#).unwrap();
    if re_comments.is_match(input) {
        diag.has_comments = true;
    }

    diag
}
