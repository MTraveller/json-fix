// src/fixers/quotes/mod.rs

pub mod fixer;
pub mod subfixes;

use crate::fixers::quotes::fixer::QuoteFixer;
use crate::types::fix_step::FixStep;

/// Entry point for quote-related fixes.
/// This is what the orchestrator calls.
pub fn fix_quotes(input: &str, steps: &mut Vec<FixStep>) -> String {
    let mut fixer = QuoteFixer { input, steps };
    fixer.apply_all()
}
