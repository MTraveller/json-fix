// src/utils/json_utils.rs

use serde::de::DeserializeOwned;

/// Checks if the input string looks like it starts with a JSON object or array.
pub fn looks_like_json(input: &str) -> bool {
    let trimmed = input.trim_start();
    trimmed.starts_with('{') || trimmed.starts_with('[')
}

/// Attempts to deserialize a JSON string into any type, returning None on failure.
pub fn safe_parse_json<T: DeserializeOwned>(input: &str) -> Option<T> {
    serde_json::from_str::<T>(input).ok()
}

/// Quickly checks if the input is likely a minified JSON (no newlines or spacing).
pub fn is_minified_json(input: &str) -> bool {
    !input.contains('\n') && input.len() > 20
}
