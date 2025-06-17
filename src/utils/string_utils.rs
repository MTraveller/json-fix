// src/utils/string_utils.rs

/// Collapse repeated spaces/tabs into a single space.
/// Also trims leading/trailing space.
pub fn normalize_whitespace(input: &str) -> String {
    let collapsed = input.split_whitespace().collect::<Vec<_>>().join(" ");
    collapsed.trim().to_string()
}

/// Ensures input ends with a closing brace or bracket (if it appears truncated).
pub fn maybe_complete_bracket(input: &str) -> String {
    let mut result = input.trim().to_string();

    let open_braces = result.chars().filter(|&c| c == '{').count();
    let close_braces = result.chars().filter(|&c| c == '}').count();
    if open_braces > close_braces {
        result.push('}');
    }

    let open_brackets = result.chars().filter(|&c| c == '[').count();
    let close_brackets = result.chars().filter(|&c| c == ']').count();
    if open_brackets > close_brackets {
        result.push(']');
    }

    result
}
