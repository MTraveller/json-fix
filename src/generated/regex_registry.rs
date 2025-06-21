// 💡 AUTO-GENERATED FILE — DO NOT EDIT
// ⚙️  To regenerate: cargo run --bin regex_manifest_codegen

#![allow(clippy::all)]

// -----------------------------------------------------------------------------

use once_cell::sync::Lazy as LazyRegex;
use regex::Regex;
use std::collections::HashMap;
use crate::generated_patterns::regex::*;
use crate::types::fix_step::FixStep;

pub struct RegexReg {
    pub regex: &'static Regex,
    pub fix_step: FixStep,
    pub reason: &'static str,
}

pub static REGEX_REGISTRY: LazyRegex<HashMap<&'static str, RegexReg>> = LazyRegex::new(|| {
    HashMap::from([
        ("RE_DOUBLE_COLON", RegexReg { regex: &RE_DOUBLE_COLON, fix_step: FixStep::new("RE_DOUBLE_COLON"), reason: "Double colon in key-value pair" }),
        ("RE_MARKDOWN_WRAPPER", RegexReg { regex: &RE_MARKDOWN_WRAPPER, fix_step: FixStep::new("RE_MARKDOWN_WRAPPER"), reason: "Markdown-wrapped JSON blocks" }),
        ("RE_ADJACENT_STRINGS", RegexReg { regex: &RE_ADJACENT_STRINGS, fix_step: FixStep::new("RE_ADJACENT_STRINGS"), reason: "Adjacent strings with no comma" }),
        ("RE_DOUBLE_CLOSING_BRACE", RegexReg { regex: &RE_DOUBLE_CLOSING_BRACE, fix_step: FixStep::new("RE_DOUBLE_CLOSING_BRACE"), reason: "Extra closing braces" }),
        ("RE_SMART_QUOTES", RegexReg { regex: &RE_SMART_QUOTES, fix_step: FixStep::new("RE_SMART_QUOTES"), reason: "Smart quotes (double curly)" }),
        ("RE_FLOAT_NO_ZERO", RegexReg { regex: &RE_FLOAT_NO_ZERO, fix_step: FixStep::new("RE_FLOAT_NO_ZERO"), reason: "Float without leading zero" }),
        ("RE_EMPTY_ARRAY_SLOTS", RegexReg { regex: &RE_EMPTY_ARRAY_SLOTS, fix_step: FixStep::new("RE_EMPTY_ARRAY_SLOTS"), reason: "Empty array slots" }),
        ("RE_UNQUOTED_KEYS", RegexReg { regex: &RE_UNQUOTED_KEYS, fix_step: FixStep::new("RE_UNQUOTED_KEYS"), reason: "Unquoted keys followed by colon" }),
        ("RE_JS_COMMENTS", RegexReg { regex: &RE_JS_COMMENTS, fix_step: FixStep::new("RE_JS_COMMENTS"), reason: "JavaScript-style comments" }),
        ("RE_STRAY_COMMA_AFTER_OPENING", RegexReg { regex: &RE_STRAY_COMMA_AFTER_OPENING, fix_step: FixStep::new("RE_STRAY_COMMA_AFTER_OPENING"), reason: "Stray comma after opening brace or bracket" }),
        ("RE_TRUNCATED_BLOCK", RegexReg { regex: &RE_TRUNCATED_BLOCK, fix_step: FixStep::new("RE_TRUNCATED_BLOCK"), reason: "Likely truncated JSON block" }),
        ("RE_JSON_BLOCK_QUOTED", RegexReg { regex: &RE_JSON_BLOCK_QUOTED, fix_step: FixStep::new("RE_JSON_BLOCK_QUOTED"), reason: "JSON block inside string quotes" }),
        ("RE_TRAILING_DOT", RegexReg { regex: &RE_TRAILING_DOT, fix_step: FixStep::new("RE_TRAILING_DOT"), reason: "Number with trailing dot" }),
        ("RE_UNQUOTED_STRING", RegexReg { regex: &RE_UNQUOTED_STRING, fix_step: FixStep::new("RE_UNQUOTED_STRING"), reason: "Unquoted string used as value" }),
        ("RE_MULTILINE_STRING", RegexReg { regex: &RE_MULTILINE_STRING, fix_step: FixStep::new("RE_MULTILINE_STRING"), reason: "Multiline string without escaping" }),
        ("RE_UNCLOSED_STRING", RegexReg { regex: &RE_UNCLOSED_STRING, fix_step: FixStep::new("RE_UNCLOSED_STRING"), reason: "Unclosed string value" }),
        ("RE_INVALID_ESCAPES", RegexReg { regex: &RE_INVALID_ESCAPES, fix_step: FixStep::new("RE_INVALID_ESCAPES"), reason: "Invalid escape sequences" }),
        ("RE_CHAINED_STRING_VALUES", RegexReg { regex: &RE_CHAINED_STRING_VALUES, fix_step: FixStep::new("RE_CHAINED_STRING_VALUES"), reason: "Chained string values without commas" }),
        ("RE_KEY_TRAPS", RegexReg { regex: &RE_KEY_TRAPS, fix_step: FixStep::new("RE_KEY_TRAPS"), reason: "Key traps with invalid characters" }),
        ("RE_MISSING_COLON", RegexReg { regex: &RE_MISSING_COLON, fix_step: FixStep::new("RE_MISSING_COLON"), reason: "Missing colon between key and value" }),
        ("RE_LEADING_COMMA_IN_ARRAY", RegexReg { regex: &RE_LEADING_COMMA_IN_ARRAY, fix_step: FixStep::new("RE_LEADING_COMMA_IN_ARRAY"), reason: "Leading comma in array" }),
        ("RE_MISSING_COMMAS_BETWEEN_PAIRS", RegexReg { regex: &RE_MISSING_COMMAS_BETWEEN_PAIRS, fix_step: FixStep::new("RE_MISSING_COMMAS_BETWEEN_PAIRS"), reason: "Missing commas between key-value pairs" }),
        ("RE_ORPHANED_CLOSE_BRACE", RegexReg { regex: &RE_ORPHANED_CLOSE_BRACE, fix_step: FixStep::new("RE_ORPHANED_CLOSE_BRACE"), reason: "Orphaned closing brace at start" }),
        ("RE_MULTI_ROOT_OBJECTS", RegexReg { regex: &RE_MULTI_ROOT_OBJECTS, fix_step: FixStep::new("RE_MULTI_ROOT_OBJECTS"), reason: "Multiple root objects without separator" }),
        ("RE_ORPHANED_STRING_VALUE", RegexReg { regex: &RE_ORPHANED_STRING_VALUE, fix_step: FixStep::new("RE_ORPHANED_STRING_VALUE"), reason: "Orphaned string value after null" }),
        ("RE_KEY_VALUE_MISALIGNED", RegexReg { regex: &RE_KEY_VALUE_MISALIGNED, fix_step: FixStep::new("RE_KEY_VALUE_MISALIGNED"), reason: "Key-value misalignment" }),
        ("RE_MARKDOWN_JSON_BLOCK", RegexReg { regex: &RE_MARKDOWN_JSON_BLOCK, fix_step: FixStep::new("RE_MARKDOWN_JSON_BLOCK"), reason: "Fenced markdown JSON blocks" }),
        ("RE_NULL_SLOTS", RegexReg { regex: &RE_NULL_SLOTS, fix_step: FixStep::new("RE_NULL_SLOTS"), reason: "Null slots like 'key': ," }),
        ("RE_INVALID_ESCAPE_SEQUENCE", RegexReg { regex: &RE_INVALID_ESCAPE_SEQUENCE, fix_step: FixStep::new("RE_INVALID_ESCAPE_SEQUENCE"), reason: "Invalid escape character" }),
        ("RE_UNFINISHED_VALUE", RegexReg { regex: &RE_UNFINISHED_VALUE, fix_step: FixStep::new("RE_UNFINISHED_VALUE"), reason: "Key with unfinished value" }),
        ("RE_ORPHANED_OPEN_BRACE", RegexReg { regex: &RE_ORPHANED_OPEN_BRACE, fix_step: FixStep::new("RE_ORPHANED_OPEN_BRACE"), reason: "Orphaned opening brace at end" }),
        ("RE_BROKEN_UNICODE_ESCAPES", RegexReg { regex: &RE_BROKEN_UNICODE_ESCAPES, fix_step: FixStep::new("RE_BROKEN_UNICODE_ESCAPES"), reason: "Broken unicode escapes" }),
        ("RE_UNDEFINED", RegexReg { regex: &RE_UNDEFINED, fix_step: FixStep::new("RE_UNDEFINED"), reason: "JavaScript-style 'undefined' value" }),
        ("RE_TRAILING_COMMA_IN_ARRAY", RegexReg { regex: &RE_TRAILING_COMMA_IN_ARRAY, fix_step: FixStep::new("RE_TRAILING_COMMA_IN_ARRAY"), reason: "Trailing commas in arrays" }),
        ("RE_FALLBACK_ARTIFACTS", RegexReg { regex: &RE_FALLBACK_ARTIFACTS, fix_step: FixStep::new("RE_FALLBACK_ARTIFACTS"), reason: "Fallback artifacts between values" }),
        ("RE_CURLY_QUOTES", RegexReg { regex: &RE_CURLY_QUOTES, fix_step: FixStep::new("RE_CURLY_QUOTES"), reason: "Smart quotes (single curly)" }),
        ("RE_SINGLE_QUOTES", RegexReg { regex: &RE_SINGLE_QUOTES, fix_step: FixStep::new("RE_SINGLE_QUOTES"), reason: "Single quotes used instead of double quotes" }),
        ("RE_MALFORMED_NESTED_ARRAYS", RegexReg { regex: &RE_MALFORMED_NESTED_ARRAYS, fix_step: FixStep::new("RE_MALFORMED_NESTED_ARRAYS"), reason: "Malformed nested arrays" }),
        ("RE_COLON_UNPAIRED_KEYVALUE", RegexReg { regex: &RE_COLON_UNPAIRED_KEYVALUE, fix_step: FixStep::new("RE_COLON_UNPAIRED_KEYVALUE"), reason: "Unpaired key-value without colon" }),
        ("RE_BACKTICKED_KEYS", RegexReg { regex: &RE_BACKTICKED_KEYS, fix_step: FixStep::new("RE_BACKTICKED_KEYS"), reason: "Backticked keys used" }),
        ("RE_REPEATED_KEYS", RegexReg { regex: &RE_REPEATED_KEYS, fix_step: FixStep::new("RE_REPEATED_KEYS"), reason: "Repeated keys in object" }),
        ("RE_STRINGIFIED_BOOLEAN", RegexReg { regex: &RE_STRINGIFIED_BOOLEAN, fix_step: FixStep::new("RE_STRINGIFIED_BOOLEAN"), reason: "Stringified boolean or null" }),
        ("RE_NEGATIVE_INVALID", RegexReg { regex: &RE_NEGATIVE_INVALID, fix_step: FixStep::new("RE_NEGATIVE_INVALID"), reason: "Invalid negative number" }),
        ("RE_DOUBLE_COMMAS", RegexReg { regex: &RE_DOUBLE_COMMAS, fix_step: FixStep::new("RE_DOUBLE_COMMAS"), reason: "Double commas in JSON" }),
        ("RE_NAN_INFINITY", RegexReg { regex: &RE_NAN_INFINITY, fix_step: FixStep::new("RE_NAN_INFINITY"), reason: "JavaScript-style NaN or Infinity" }),
    ])
});
