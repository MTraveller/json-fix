// src/utils/regex_utils.rs
use once_cell::sync::Lazy;
use regex::Regex;

// ── Syntax Cleanup ────────────────────────────────
/// Extra closing braces, e.g., `}}`
pub static RE_DOUBLE_CLOSING_BRACE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"\}\s*\}"#).expect("Failed to compile RE_DOUBLE_CLOSING_BRACE"));
/// Double commas, e.g., `,,`
pub static RE_DOUBLE_COMMAS: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#",\s*,+"#).expect("Failed to compile RE_DOUBLE_COMMAS"));

/// Trailing commas in arrays, e.g., [1, 2, 3, ]
pub static RE_TRAILING_COMMA_IN_ARRAY: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#",\s*(\])"#).expect("Failed to compile RE_TRAILING_COMMA_IN_ARRAY"));

/// Malformed nested arrays, e.g., `[[]`, `[]]`, `[][]`
pub static RE_MALFORMED_NESTED_ARRAYS: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"\[\[\]|\[\[|\]\]|\]\["#).expect("Failed to compile RE_MALFORMED_NESTED_ARRAYS")
});

/// Empty array slots, e.g., `[null, ,]`, `[ , ,]`
pub static RE_EMPTY_ARRAY_SLOTS: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"\[\s*(null\s*,\s*)?,\s*,+"#).expect("Failed to compile RE_EMPTY_ARRAY_SLOTS")
});

/// Misaligned array strings (e.g., "a" "b" without comma)
pub static RE_ARRAY_STRING_MISALIGN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#""\s*("[^"]*")"#).expect("Failed to compile RE_ARRAY_STRING_MISALIGN")
});

/// Adjacent strings with no comma: `"hello" "world"`
pub static RE_ADJACENT_STRINGS: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#""\s+""#).expect("Failed to compile RE_ADJACENT_STRINGS"));

/// Missing colon between key and value, e.g., `"key" "value"`
pub static RE_MISSING_COLON: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#""[^"]+"\s+"[^"]+""#).expect("Failed to compile RE_MISSING_COLON"));

/// Double colon, e.g., `"key"::`
pub static RE_DOUBLE_COLON: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#""[^"]+"\s*::"#).expect("Failed to compile RE_DOUBLE_COLON"));

/// Chained string values without commas: `"one" "two"`
pub static RE_CHAINED_STRING_VALUES: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#""[^"]+"\s+"[^"]+""#).expect("Failed to compile RE_CHAINED_STRING_VALUES")
});

/// Missing commas between key-value pairs: `"key1": "value1" "key2": "value2"`
pub static RE_MISSING_COMMAS_BETWEEN_PAIRS: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#""[^"]+"\s*:\s*"[^"]+"\s+"[^"]+"\s*:"#)
        .expect("Failed to compile RE_MISSING_COMMAS_BETWEEN_PAIRS")
});

/// Stray comma after opening brace/bracket: `{,` or `[,`
pub static RE_STRAY_COMMA_AFTER_OPENING: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"[\{\[]\s*,"#).expect("Failed to compile RE_STRAY_COMMA_AFTER_OPENING")
});

/// Invalid escape sequences, e.g., `\x`, `\q`, etc.
pub static RE_INVALID_ESCAPES: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"\\[^btnfru"\\]"#).expect("Failed to compile RE_INVALID_ESCAPES"));

/// Broken unicode escapes like `\u12`, `\u1`, `\u`
pub static RE_BROKEN_UNICODE_ESCAPES: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"\\u[0-9a-fA-F]{1,3}[^0-9a-fA-F]"#)
        .expect("Failed to compile RE_BROKEN_UNICODE_ESCAPES")
});

// ── Semantic Violations ───────────────────────────
/// Orphaned value like: `"key": null, "value"`
pub static RE_ORPHANED_STRING_VALUE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"\bnull\s*,\s*"[^"]+""#).expect("Failed to compile RE_ORPHANED_STRING_VALUE")
});

/// Key-value misalignment like: `"emotion": , "hopeful"`
pub static RE_KEY_VALUE_MISALIGNED: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#""\s*:\s*,\s*""#).expect("Failed to compile RE_KEY_VALUE_MISALIGNED")
});

/// Utility to safely run a regex match
pub fn has_match(regex: &Regex, input: &str) -> bool {
    regex.is_match(input)
}

/// Counts how many matches a regex finds in the input.
pub fn count_matches(regex: &Regex, input: &str) -> usize {
    regex.find_iter(input).count()
}

/// Matches standalone 'undefined' values
pub static RE_UNDEFINED: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\bundefined\b").expect("Failed to compile RE_UNDEFINED"));

/// Matches JavaScript-style `NaN`, `Infinity`, `-Infinity`
pub static RE_NAN_INFINITY: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b(NaN|Infinity|-Infinity)\b").expect("Failed to compile RE_NAN_INFINITY")
});

/// Matches JavaScript-style comments: `// comment` or `/* comment */`
pub static RE_JS_COMMENTS: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(//[^\n]*|/\*[\s\S]*?\*/)").expect("Failed to compile RE_JS_COMMENTS")
});

/// Matches unquoted keys followed by colon, e.g., {key: "value"}
pub static RE_UNQUOTED_KEYS: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"(?m)(?P<pre>[^"\s{,])(?P<key>\w+)\s*:"#)
        .expect("Failed to compile RE_UNQUOTED_KEYS")
});

/// Matches empty keys or invalid key traps like: "" : , or "" : ]
pub static RE_KEY_TRAPS: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#""[^"]*"\s*:\s*(,|\])"#).expect("Failed to compile RE_KEY_TRAPS"));

/// Matches fenced markdown blocks that may contain JSON content
pub static RE_MARKDOWN_WRAPPER: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"(?s)```(?:json)?\s*(.*?)\s*```"#).expect("Failed to compile RE_MARKDOWN_WRAPPER")
});

/// Matches and extracts content from fenced markdown JSON blocks line-wise
pub static RE_MARKDOWN_JSON_BLOCK: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"(?s)(?:^|\n)\s*```(?:json)?\s*(.*?)\s*```(?:\n|$)"#)
        .expect("Failed to compile RE_MARKDOWN_JSON_BLOCK")
});

/// Matches null slot issues, e.g., `"key": ,` or `"key": ]`
pub static RE_NULL_SLOTS: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"(":\s*)(,|\])"#).expect("Failed to compile RE_NULL_SLOTS"));

/// Matches fallback JSON artifacts like `"value", , "next"`
pub static RE_FALLBACK_ARTIFACTS: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"\s*,\s*("[^"]*")\s*,\s*"#).expect("Failed to compile RE_FALLBACK_ARTIFACTS")
});

/// Matches single-quoted strings to convert to double quotes
pub static RE_SINGLE_QUOTES: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"'([^']*)'"#).expect("Failed to compile RE_SINGLE_QUOTES"));

/// Matches curly quotes: ‘ ’
pub static RE_CURLY_QUOTES: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"[‘’]").expect("Failed to compile RE_CURLY_QUOTES"));

/// Matches smart quotes: “ ”
pub static RE_SMART_QUOTES: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"[“”]").expect("Failed to compile RE_SMART_QUOTES"));

/// Matches concatenated JSON objects without comma: `}{`
pub static RE_CONCATENATED_JSON_OBJECTS: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"\}\s*\{"#).expect("Failed to compile RE_CONCATENATED_JSON_OBJECTS"));

// ── Optional Additional Patterns ─────────────────────────────────

/// Matches orphaned opening brace `{` at the end of string (unclosed)
pub static RE_ORPHANED_OPEN_BRACE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"\{[^\{\}]*$"#).expect("Failed to compile RE_ORPHANED_OPEN_BRACE"));

/// Matches orphaned closing brace `}` at the start of string (unopened)
pub static RE_ORPHANED_CLOSE_BRACE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"^[^\{\}]*\}"#).expect("Failed to compile RE_ORPHANED_CLOSE_BRACE"));

// Matches unquoted string values like: "key": hello
// pub static RE_UNQUOTED_STRING_VALUES: Lazy<Regex> = Lazy::new(|| {
//     Regex::new(r#""[^"]+"\s*:\s*[\w\d]+"#).expect("Failed to compile RE_UNQUOTED_STRING_VALUES")
// });

// Matches quoted true/false/null: "key": "true"
// pub static RE_TRUE_FALSE_NULL_AS_STRINGS: Lazy<Regex> = Lazy::new(|| {
//     Regex::new(r#""[^"]+"\s*:\s*"(?i)(true|false|null)""#).expect("Failed to compile RE_TRUE_FALSE_NULL_AS_STRINGS")
// });

// Matches trailing null values like: "value", null
// pub static RE_TRAILING_NULL: Lazy<Regex> = Lazy::new(|| {
//     Regex::new(r#",\s*null\s*(}|])"#).expect("Failed to compile RE_TRAILING_NULL")
// });

// Matches mixed quote styles: "key": 'value'
// pub static RE_MIXED_QUOTES: Lazy<Regex> = Lazy::new(|| {
//     Regex::new(r#""[^"]+"\s*:\s*'[^']*'"#).expect("Failed to compile RE_MIXED_QUOTES")
// });
