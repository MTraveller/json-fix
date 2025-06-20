use fancy_regex::Regex;
use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static FANCY_REGEX_MAP: Lazy<HashMap<&'static str, Regex>> = Lazy::new(|| {
    let mut map = HashMap::new();

    // Matches misaligned array strings like: ["item1"item2", , "item3"]
    map.insert(
        "RE_ARRAY_STRING_MISALIGN",
        Regex::new(r#""[^"]+"\s*(?=")"#).expect("Failed to compile RE_ARRAY_STRING_MISALIGN"),
    );

    map.insert(
        "RE_DOUBLE_CLOSING_BRACE",
        Regex::new(r#"\}\s*\}"#).expect("Failed to compile RE_DOUBLE_CLOSING_BRACE"),
    );
    map.insert(
        "RE_BRACKETS_BEFORE_COLON",
        Regex::new(r#"\}\s*:"#).expect("Failed to compile RE_BRACKETS_BEFORE_COLON"),
    );
    map.insert(
        "RE_QUOTES_IN_WRONG_PLACE",
        Regex::new(r#"[A-Za-z0-9]"\s*:"#).expect("Failed to compile RE_QUOTES_IN_WRONG_PLACE"),
    );
    map.insert(
        "RE_QUOTES_BEFORE_BRACE",
        Regex::new(r#""\s*\}"#).expect("Failed to compile RE_QUOTES_BEFORE_BRACE"),
    );
    map.insert(
        "RE_EXTRA_BACKSLASH_IN_QUOTES",
        Regex::new(r#""[^"]*\\\\+[^"]*""#).expect("Failed to compile RE_EXTRA_BACKSLASH_IN_QUOTES"),
    );
    map.insert(
        "RE_EMPTY_QUOTES_BEFORE_KEY",
        Regex::new(r#"""\s*,\s*""#).expect("Failed to compile RE_EMPTY_QUOTES_BEFORE_KEY"),
    );

    map
});

pub fn fancy_is_match(key: &str, input: &str) -> bool {
    match FANCY_REGEX_MAP.get(key) {
        Some(regex) => regex.is_match(input).unwrap_or(false),
        None => false,
    }
}
