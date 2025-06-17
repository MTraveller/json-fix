// src/diagnostics/markdown.rs

use fancy_regex::Regex as FanRegex;
use regex::Regex;

#[derive(Debug, Default)]
pub struct MarkdownDiagnostics {
    pub has_markdown_wrappers: bool,
    pub has_embedded_json: bool,
}

pub fn analyze_markdown(json: &str) -> MarkdownDiagnostics {
    let mut diag = MarkdownDiagnostics::default();

    // Detect markdown code block wrappers (```json ... ```)
    let re_md_wrapper = FanRegex::new(r"(?s)```(?:json)?\s*.*?```").unwrap();
    if re_md_wrapper.is_match(json).unwrap_or(false) {
        diag.has_markdown_wrappers = true;
    }

    // Detect embedded JSON inside larger markdown or text (heuristic)
    let re_json_like = Regex::new(r#"\{[^{}]+\:\s*[^{}]+\}"#).unwrap();
    if re_json_like.is_match(json) && json.len() > 300 {
        diag.has_embedded_json = true;
    }

    diag
}
