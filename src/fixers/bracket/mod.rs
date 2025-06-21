// src/fixers/brackets/mod.rs

pub mod fixer;
pub mod subfixes;

use crate::fixers::bracket::fixer::BracketFixer;
use crate::types::fix_scope::{FixScope, ScopeCategory};
use crate::types::fixer_context::FixContext;

/// Entry point for bracket-related fixes.
/// Called by the orchestrator when bracket issues are detected.
pub fn fix_arrays<'a>(ctx: &mut FixContext) -> String {
    let scope = FixScope::new(&ctx.input, &[ScopeCategory::Bracket], None);
    let mut fixer = BracketFixer { ctx, scope };
    fixer.apply_all()
}

crate::bracket_fixer!(BracketsUnbalancedFixer, brackets_unbalanced);
crate::bracket_fixer!(BracketsExtraFixer, brackets_extra);
crate::bracket_fixer!(BracketsMissingFixer, brackets_missing);
