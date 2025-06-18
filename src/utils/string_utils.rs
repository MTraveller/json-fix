// src/utils/string_utils.rs

/// Collapse repeated spaces/tabs into a single space.
/// Also trims leading/trailing space.
pub fn normalize_whitespace(input: &str) -> String {
    let collapsed = input.split_whitespace().collect::<Vec<_>>().join(" ");
    collapsed.trim().to_string()
}

/// Removes excessive newlines and trims whitespace on each line.
pub fn normalize_newlines(input: &str) -> String {
    input
        .replace("\r\n", "\n")
        .lines()
        .map(str::trim)
        .collect::<Vec<_>>()
        .join("\n")
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

/// Checks if the number of opening and closing brackets is balanced.
pub fn is_bracket_balanced(input: &str) -> bool {
    let open = input.chars().filter(|&c| c == '[').count();
    let close = input.chars().filter(|&c| c == ']').count();
    open == close
}

/// Checks if the number of opening and closing braces is balanced.
pub fn is_brace_balanced(input: &str) -> bool {
    let open = input.chars().filter(|&c| c == '{').count();
    let close = input.chars().filter(|&c| c == '}').count();
    open == close
}

/// Ensures the input ends with both a closing brace and bracket, if they are imbalanced.
pub fn maybe_complete_json_wrappers(input: &str) -> String {
    let mut result = input.trim().to_string();

    let open_braces = result.matches('{').count();
    let close_braces = result.matches('}').count();
    if open_braces > close_braces {
        result.push('}');
    }

    let open_brackets = result.matches('[').count();
    let close_brackets = result.matches(']').count();
    if open_brackets > close_brackets {
        result.push(']');
    }

    result
}
