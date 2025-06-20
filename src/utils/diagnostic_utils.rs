// src/utils/diagnostic_utils.rs

use crate::types::diagnostic_core::{DiagnosticSeverity, FixDiagnosticKind};
use crate::types::fix_scope::ScopeCategory;
use std::str::FromStr;

use std::collections::HashMap;

/// Utility to convert category to a human-readable label.
pub fn category_label(category: &FixDiagnosticKind) -> &'static str {
    match category {
        FixDiagnosticKind::Array => "Array",
        FixDiagnosticKind::Bracket => "Bracket",
        FixDiagnosticKind::Colon => "Colon",
        FixDiagnosticKind::Comma => "Comma",
        FixDiagnosticKind::Escape => "Escape",
        FixDiagnosticKind::Key => "Key",
        FixDiagnosticKind::Markdown => "Markdown",
        FixDiagnosticKind::Misc => "Misc",
        FixDiagnosticKind::Quote => "Quote",
        FixDiagnosticKind::Structure => "Structure",
        FixDiagnosticKind::JS => "JS",
    }
}

/// Internal mapping of ScopeCategory labels.
fn get_scope_label_map() -> HashMap<ScopeCategory, &'static str> {
    use ScopeCategory::*;
    HashMap::from([
        (Array, "Array"),
        (Bracket, "Bracket"),
        (Colon, "Colon"),
        (Comma, "Comma"),
        (Escape, "Escape"),
        (Key, "Key"),
        (Markdown, "Markdown"),
        (Misc, "Misc"),
        (Quote, "Quotes"),
        (Structure, "Structure"),
        (JS, "JavaScript"),
    ])
}

/// Utility to convert ScopeCategory to a human-readable label.
pub fn scope_label(scope: &ScopeCategory) -> &'static str {
    get_scope_label_map()
        .get(scope)
        .copied()
        .unwrap_or("Unknown")
}

impl FromStr for ScopeCategory {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        get_scope_label_map()
            .into_iter()
            .find_map(|(k, v)| if v == s { Some(k) } else { None })
            .ok_or_else(|| format!("Unknown ScopeCategory: {}", s))
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
    category: &FixDiagnosticKind,
    severity: &DiagnosticSeverity,
) -> String {
    format!(
        "[{}][{}]",
        severity_label(severity),
        category_label(category)
    )
}
