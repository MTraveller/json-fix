// src/fixers/quotes/mod.rs

pub mod fixer;
pub mod subfixes;

use crate::fixers::quote::fixer::QuoteFixer;
use crate::types::fixer_context::FixContext;

/// Entry point for quote-related fixes.
/// Called by the orchestrator when quote issues are detected.
pub fn fix_quotes<'a>(ctx: &mut FixContext) -> String {
    let mut fixer = QuoteFixer { ctx };
    fixer.apply_all()
}
