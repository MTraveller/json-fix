// src/fixers/brackets/mod.rs

pub mod fixer;
pub mod subfixes;

use crate::fixers::bracket::fixer::BracketFixer;
use crate::types::fixer_context::FixContext;

/// Entry point for bracket-related fixes.
/// Called by the orchestrator when bracket issues are detected.
pub fn fix_brackets<'a>(ctx: &mut FixContext) -> String {
    let mut fixer = BracketFixer { ctx };
    fixer.apply_all()
}
