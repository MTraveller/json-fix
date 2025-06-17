// src/diagnostics/escape.rs

use fancy_regex::Regex as FanRegex;
use regex::Regex;

#[derive(Debug, Default)]
pub struct EscapeDiagnostics {
    pub has_invalid_escape: bool,
    pub has_broken_unicode: bool,
}

pub fn analyze_escapes(json: &str) -> EscapeDiagnostics {
    let mut diag = EscapeDiagnostics::default();

    // Detect invalid escape sequences (e.g., \q, \x, etc.)
    let re_invalid_escape = Regex::new(r#"\\[^btnfru"\\]"#).unwrap();
    if re_invalid_escape.is_match(json) {
        diag.has_invalid_escape = true;
    }

    // Detect broken Unicode sequences (e.g., \u12, \uGHIJ)
    let re_broken_unicode = FanRegex::new(r#"\\u[0-9a-fA-F]{0,3}(?![0-9a-fA-F])"#).unwrap();
    if re_broken_unicode.is_match(json).unwrap_or(false) {
        diag.has_broken_unicode = true;
    }

    diag
}
