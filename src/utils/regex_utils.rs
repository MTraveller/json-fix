// src/utils/regex_utils.rs
use once_cell::sync::Lazy;
use regex::Regex;

/// Double commas, e.g., `,,`
pub static RE_DOUBLE_COMMAS: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#",\s*,+"#).expect("Failed to compile RE_DOUBLE_COMMAS"));

/// Orphaned value like: `"key": null, "value"`
pub static RE_ORPHANED_STRING_VALUE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"\bnull\s*,\s*"[^"]+""#).expect("Failed to compile RE_ORPHANED_STRING_VALUE")
});

/// Key-value misalignment like: `"emotion": , "hopeful"`
pub static RE_KEY_VALUE_MISALIGNED: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#""\s*:\s*,\s*""#).expect("Failed to compile RE_KEY_VALUE_MISALIGNED")
});

/// Adjacent strings with no comma: `"hello" "world"`
pub static RE_ADJACENT_STRINGS: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#""\s+""#).expect("Failed to compile RE_ADJACENT_STRINGS"));

/// Utility to safely run a regex match
pub fn has_match(regex: &Regex, input: &str) -> bool {
    regex.is_match(input)
}
