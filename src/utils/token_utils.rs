/// Estimate token count based on simple heuristic (word count * 1.3).
/// Not as accurate as tokenizer-based but good for rough logging.
pub fn estimate_token_count(text: &str) -> usize {
    let word_count = text.split_whitespace().count();
    ((word_count as f64) * 1.3).ceil() as usize
}

/// Estimate token cost based on assumed average cost per token.
/// Used for budgeting or audit trails in GPT workflows.
pub fn estimate_token_cost(tokens: usize, cost_per_1k: f64) -> f64 {
    (tokens as f64 / 1000.0) * cost_per_1k
}
