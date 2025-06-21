// 💡 AUTO-GENERATED FILE — DO NOT EDIT
// ⚙️  To regenerate: cargo run --bin regex_manifest_codegen

#![allow(clippy::all)]

// -----------------------------------------------------------------------------
// ── Regex Constants ─────────────────────────────

use once_cell::sync::Lazy as LazyRegex;
use regex::Regex;
use std::collections::HashMap;

/// Double colon in key-value pair
pub static RE_DOUBLE_COLON: LazyRegex<Regex> = LazyRegex::new(|| Regex::new(r#"\"[^\"]+\"\s*::"#).unwrap());

/// Markdown-wrapped JSON blocks
pub static RE_MARKDOWN_WRAPPER: LazyRegex<Regex> = LazyRegex::new(|| Regex::new(r#"(?s)```(?:json)?\s*(.*?)\s*```"#).unwrap());

/// Adjacent strings with no comma
pub static RE_ADJACENT_STRINGS: LazyRegex<Regex> = LazyRegex::new(|| Regex::new(r#"s+\""#).unwrap());

/// Extra closing braces
pub static RE_DOUBLE_CLOSING_BRACE: LazyRegex<Regex> = LazyRegex::new(|| Regex::new(r#"\}\s*\}+"#).unwrap());

/// Smart quotes (double curly)
pub static RE_SMART_QUOTES: LazyRegex<Regex> = LazyRegex::new(|| Regex::new(r#"[“”]"#).unwrap());

/// Float without leading zero
pub static RE_FLOAT_NO_ZERO: LazyRegex<Regex> = LazyRegex::new(|| Regex::new(r#"(?<!\d)\.\d+"#).unwrap());

/// Empty array slots
pub static RE_EMPTY_ARRAY_SLOTS: LazyRegex<Regex> = LazyRegex::new(|| Regex::new(r#",\s*(,)+"#).unwrap());

/// Unquoted keys followed by colon
pub static RE_UNQUOTED_KEYS: LazyRegex<Regex> = LazyRegex::new(|| Regex::new(r#"\b[a-zA-Z_][a-zA-Z0-9_]*\s*:"#).unwrap());

/// JavaScript-style comments
pub static RE_JS_COMMENTS: LazyRegex<Regex> = LazyRegex::new(|| Regex::new(r#"(//[^\n]*|/\*[\s\S]*?\*/)"#).unwrap());

/// Stray comma after opening brace or bracket
pub static RE_STRAY_COMMA_AFTER_OPENING: LazyRegex<Regex> = LazyRegex::new(|| Regex::new(r#"(?:\{|\[)\s*,"#).unwrap());

/// Likely truncated JSON block
pub static RE_TRUNCATED_BLOCK: LazyRegex<Regex> = LazyRegex::new(|| Regex::new(r#"[\{\[][^]*$"#).unwrap());

/// JSON block inside string quotes
pub static RE_JSON_BLOCK_QUOTED: LazyRegex<Regex> = LazyRegex::new(|| Regex::new(r#"\"\\s*\\{[^}]+\\}\""#).unwrap());

/// Number with trailing dot
pub static RE_TRAILING_DOT: LazyRegex<Regex> = LazyRegex::new(|| Regex::new(r#"\d+\."#).unwrap());

/// Unquoted string used as value
pub static RE_UNQUOTED_STRING: LazyRegex<Regex> = LazyRegex::new(|| Regex::new(r#"(?<=[:,\{]\s*)[a-zA-Z_]\w*(?=\s*[:,\}])"#).unwrap());

/// Multiline string without escaping
pub static RE_MULTILINE_STRING: LazyRegex<Regex> = LazyRegex::new(|| Regex::new(r#""([^"]*\n[^"]*)+""#).unwrap());

/// Unclosed string value
pub static RE_UNCLOSED_STRING: LazyRegex<Regex> = LazyRegex::new(|| Regex::new(r#"\"[^\\\"\n]*$"#).unwrap());

/// Invalid escape sequences
pub static RE_INVALID_ESCAPES: LazyRegex<Regex> = LazyRegex::new(|| Regex::new(r#"[^btnfru\"]"#).unwrap());

/// Chained string values without commas
pub static RE_CHAINED_STRING_VALUES: LazyRegex<Regex> = LazyRegex::new(|| Regex::new(r#"\"[^\"]+\"\s+[^\"]+\""#).unwrap());

/// Key traps with invalid characters
pub static RE_KEY_TRAPS: LazyRegex<Regex> = LazyRegex::new(|| Regex::new(r#"\"[^\"]*\"\s*:\s*(,|])"#).unwrap());

/// Missing colon between key and value
pub static RE_MISSING_COLON: LazyRegex<Regex> = LazyRegex::new(|| Regex::new(r#"\"[^\"]+\"\s+\"[^\"]+\""#).unwrap());

/// Leading comma in array
pub static RE_LEADING_COMMA_IN_ARRAY: LazyRegex<Regex> = LazyRegex::new(|| Regex::new(r#"\[\s*,\s*"#).unwrap());

/// Missing commas between key-value pairs
pub static RE_MISSING_COMMAS_BETWEEN_PAIRS: LazyRegex<Regex> = LazyRegex::new(|| Regex::new(r#"\"[^\"]+\"\s*:\s*\"[^\"]+\"\s+\"[^\"]+\"\s*:"#).unwrap());

/// Orphaned closing brace at start
pub static RE_ORPHANED_CLOSE_BRACE: LazyRegex<Regex> = LazyRegex::new(|| Regex::new(r#"^[^\{\}]*\}"#).unwrap());

/// Multiple root objects without separator
pub static RE_MULTI_ROOT_OBJECTS: LazyRegex<Regex> = LazyRegex::new(|| Regex::new(r#"\}\s*\{"#).unwrap());

/// Orphaned string value after null
pub static RE_ORPHANED_STRING_VALUE: LazyRegex<Regex> = LazyRegex::new(|| Regex::new(r#"\bnull\s*,\s*\"[^\"]+\""#).unwrap());

/// Key-value misalignment
pub static RE_KEY_VALUE_MISALIGNED: LazyRegex<Regex> = LazyRegex::new(|| Regex::new(r#"\"\s*:\s*,\s*\""#).unwrap());

/// Fenced markdown JSON blocks
pub static RE_MARKDOWN_JSON_BLOCK: LazyRegex<Regex> = LazyRegex::new(|| Regex::new(r#"(?s)(?:^|\n)\s*```(?:json)?\s*(.*?)\s*```(?:\n|$)"#).unwrap());

/// Null slots like 'key': ,
pub static RE_NULL_SLOTS: LazyRegex<Regex> = LazyRegex::new(|| Regex::new(r#"(\":\s*)(,|\])"#).unwrap());

/// Invalid escape character
pub static RE_INVALID_ESCAPE_SEQUENCE: LazyRegex<Regex> = LazyRegex::new(|| Regex::new(r#"\\\\[^btnfr\\\"\\\\/u]"#).unwrap());

/// Key with unfinished value
pub static RE_UNFINISHED_VALUE: LazyRegex<Regex> = LazyRegex::new(|| Regex::new(r#"\"\\w+\"\\s*:\\s*$"#).unwrap());

/// Orphaned opening brace at end
pub static RE_ORPHANED_OPEN_BRACE: LazyRegex<Regex> = LazyRegex::new(|| Regex::new(r#"\{[^\{\}]*$"#).unwrap());

/// Broken unicode escapes
pub static RE_BROKEN_UNICODE_ESCAPES: LazyRegex<Regex> = LazyRegex::new(|| Regex::new(r#"u[0-9a-fA-F]{1,3}[^0-9a-fA-F]"#).unwrap());

/// JavaScript-style 'undefined' value
pub static RE_UNDEFINED: LazyRegex<Regex> = LazyRegex::new(|| Regex::new(r#"\bundefined\b"#).unwrap());

/// Trailing commas in arrays
pub static RE_TRAILING_COMMA_IN_ARRAY: LazyRegex<Regex> = LazyRegex::new(|| Regex::new(r#",\s*(\])"#).unwrap());

/// Fallback artifacts between values
pub static RE_FALLBACK_ARTIFACTS: LazyRegex<Regex> = LazyRegex::new(|| Regex::new(r#"\s*,\s*(\"[^\"]*\")\s*,\s*"#).unwrap());

/// Smart quotes (single curly)
pub static RE_CURLY_QUOTES: LazyRegex<Regex> = LazyRegex::new(|| Regex::new(r#"[‘’]"#).unwrap());

/// Single quotes used instead of double quotes
pub static RE_SINGLE_QUOTES: LazyRegex<Regex> = LazyRegex::new(|| Regex::new(r#"'([^']*)'"#).unwrap());

/// Malformed nested arrays
pub static RE_MALFORMED_NESTED_ARRAYS: LazyRegex<Regex> = LazyRegex::new(|| Regex::new(r#"\[\s*\[|\]\s*\]|\[\s*\]"#).unwrap());

/// Unpaired key-value without colon
pub static RE_COLON_UNPAIRED_KEYVALUE: LazyRegex<Regex> = LazyRegex::new(|| Regex::new(r#"\"[^\\\"]+\"\\s+\"[^\\\"]+\""#).unwrap());

/// Backticked keys used
pub static RE_BACKTICKED_KEYS: LazyRegex<Regex> = LazyRegex::new(|| Regex::new(r#"`[^`]+`\s*:"#).unwrap());

/// Repeated keys in object
pub static RE_REPEATED_KEYS: LazyRegex<Regex> = LazyRegex::new(|| Regex::new(r#"\"(\\w+)\"\\s*:"#).unwrap());

/// Stringified boolean or null
pub static RE_STRINGIFIED_BOOLEAN: LazyRegex<Regex> = LazyRegex::new(|| Regex::new(r#"\"(true|false|null)\""#).unwrap());

/// Invalid negative number
pub static RE_NEGATIVE_INVALID: LazyRegex<Regex> = LazyRegex::new(|| Regex::new(r#"-\s*\d+"#).unwrap());

/// Double commas in JSON
pub static RE_DOUBLE_COMMAS: LazyRegex<Regex> = LazyRegex::new(|| Regex::new(r#",\s*,+"#).unwrap());

/// JavaScript-style NaN or Infinity
pub static RE_NAN_INFINITY: LazyRegex<Regex> = LazyRegex::new(|| Regex::new(r#"\b(NaN|Infinity|-Infinity)\b"#).unwrap());

// ── Descriptions Map ───────────────────────────

pub static REGEX_DESCRIPTIONS: LazyRegex<HashMap<&'static str, &'static str>> = LazyRegex::new(|| {
    HashMap::from([
        ("RE_DOUBLE_COLON", "Double colon in key-value pair"),
        ("RE_MARKDOWN_WRAPPER", "Markdown-wrapped JSON blocks"),
        ("RE_ADJACENT_STRINGS", "Adjacent strings with no comma"),
        ("RE_DOUBLE_CLOSING_BRACE", "Extra closing braces"),
        ("RE_SMART_QUOTES", "Smart quotes (double curly)"),
        ("RE_FLOAT_NO_ZERO", "Float without leading zero"),
        ("RE_EMPTY_ARRAY_SLOTS", "Empty array slots"),
        ("RE_UNQUOTED_KEYS", "Unquoted keys followed by colon"),
        ("RE_JS_COMMENTS", "JavaScript-style comments"),
        ("RE_STRAY_COMMA_AFTER_OPENING", "Stray comma after opening brace or bracket"),
        ("RE_TRUNCATED_BLOCK", "Likely truncated JSON block"),
        ("RE_JSON_BLOCK_QUOTED", "JSON block inside string quotes"),
        ("RE_TRAILING_DOT", "Number with trailing dot"),
        ("RE_UNQUOTED_STRING", "Unquoted string used as value"),
        ("RE_MULTILINE_STRING", "Multiline string without escaping"),
        ("RE_UNCLOSED_STRING", "Unclosed string value"),
        ("RE_INVALID_ESCAPES", "Invalid escape sequences"),
        ("RE_CHAINED_STRING_VALUES", "Chained string values without commas"),
        ("RE_KEY_TRAPS", "Key traps with invalid characters"),
        ("RE_MISSING_COLON", "Missing colon between key and value"),
        ("RE_LEADING_COMMA_IN_ARRAY", "Leading comma in array"),
        ("RE_MISSING_COMMAS_BETWEEN_PAIRS", "Missing commas between key-value pairs"),
        ("RE_ORPHANED_CLOSE_BRACE", "Orphaned closing brace at start"),
        ("RE_MULTI_ROOT_OBJECTS", "Multiple root objects without separator"),
        ("RE_ORPHANED_STRING_VALUE", "Orphaned string value after null"),
        ("RE_KEY_VALUE_MISALIGNED", "Key-value misalignment"),
        ("RE_MARKDOWN_JSON_BLOCK", "Fenced markdown JSON blocks"),
        ("RE_NULL_SLOTS", "Null slots like 'key': ,"),
        ("RE_INVALID_ESCAPE_SEQUENCE", "Invalid escape character"),
        ("RE_UNFINISHED_VALUE", "Key with unfinished value"),
        ("RE_ORPHANED_OPEN_BRACE", "Orphaned opening brace at end"),
        ("RE_BROKEN_UNICODE_ESCAPES", "Broken unicode escapes"),
        ("RE_UNDEFINED", "JavaScript-style 'undefined' value"),
        ("RE_TRAILING_COMMA_IN_ARRAY", "Trailing commas in arrays"),
        ("RE_FALLBACK_ARTIFACTS", "Fallback artifacts between values"),
        ("RE_CURLY_QUOTES", "Smart quotes (single curly)"),
        ("RE_SINGLE_QUOTES", "Single quotes used instead of double quotes"),
        ("RE_MALFORMED_NESTED_ARRAYS", "Malformed nested arrays"),
        ("RE_COLON_UNPAIRED_KEYVALUE", "Unpaired key-value without colon"),
        ("RE_BACKTICKED_KEYS", "Backticked keys used"),
        ("RE_REPEATED_KEYS", "Repeated keys in object"),
        ("RE_STRINGIFIED_BOOLEAN", "Stringified boolean or null"),
        ("RE_NEGATIVE_INVALID", "Invalid negative number"),
        ("RE_DOUBLE_COMMAS", "Double commas in JSON"),
        ("RE_NAN_INFINITY", "JavaScript-style NaN or Infinity"),
    ])
});
