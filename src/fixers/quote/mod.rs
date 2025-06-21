// src/fixers/quotes/mod.rs

pub mod fixer;
pub mod subfixes;

use crate::fixers::quote::fixer::QuoteFixer;
use crate::types::fix_scope::{FixScope, ScopeCategory};
use crate::types::fixer_context::FixContext;

/// Entry point for quote-related fixes.
/// Called by the orchestrator when quote issues are detected.
pub fn fix_quotes<'a>(ctx: &mut FixContext) -> String {
    let scope = FixScope::new(&ctx.input, &[ScopeCategory::Quote], None);
    let mut fixer = QuoteFixer { ctx, scope };
    fixer.apply_all()
}

crate::quote_fixer!(QuotesSingleFixer, quotes_single);
crate::quote_fixer!(QuotesCurlyFixer, quotes_curly);
