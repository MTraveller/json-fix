// src/utils/diagnostic_utils.rs

use crate::types::diagnostic_meta::{DiagnosticCategory, DiagnosticSeverity};

/// Utility to convert category to a human-readable label.
pub fn category_label(category: &DiagnosticCategory) -> &'static str {
    match category {
        DiagnosticCategory::Syntax => "Syntax",
        DiagnosticCategory::Structural => "Structural",
        DiagnosticCategory::JSStyle => "JSStyle",
        DiagnosticCategory::Compatibility => "Compatibility",
        DiagnosticCategory::Wrapper => "Wrapper",
    }
}

/// Utility to convert severity to a human-readable label.
pub fn severity_label(severity: &DiagnosticSeverity) -> &'static str {
    match severity {
        DiagnosticSeverity::Info => "Info",
        DiagnosticSeverity::Warning => "Warning",
        DiagnosticSeverity::Error => "Error",
        DiagnosticSeverity::Critical => "Critical",
    }
}

/// Pretty print for a (category, severity) diagnostic label.
pub fn format_diagnostic_label(
    category: &DiagnosticCategory,
    severity: &DiagnosticSeverity,
) -> String {
    format!(
        "[{}][{}]",
        severity_label(severity),
        category_label(category)
    )
}
