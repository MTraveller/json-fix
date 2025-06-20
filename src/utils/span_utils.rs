// src/utils/span_utils.rs

/// Estimate the span of a problematic bracket region.
/// If `missing` is true, guess where the bracket is missing (typically near the end).
/// If `missing` is false, guess where there's an extra closing bracket (early in the input).
pub fn estimate_bracket_span(input: &str, missing: bool) -> Option<(usize, usize)> {
    if missing {
        // Guess near the end where something might be missing
        let start = input.len().saturating_sub(5);
        Some((start, input.len()))
    } else {
        // Guess early for extra closing bracket
        let end = input.find(|c| c == '}' || c == ']').unwrap_or(5);
        Some((end.saturating_sub(1), end + 1))
    }
}
