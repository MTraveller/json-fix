// src/fixers/comma/mod.rs

pub mod fixer;
pub mod subfixes;

use crate::fixers::comma::fixer::CommaFixer;
use crate::types::fixer_context::FixContext;

/// Entry point for comma-related fixes.
/// Called by the orchestrator when comma issues are detected.
pub fn fix_commas<'a>(ctx: &mut FixContext) -> String {
    let mut fixer = CommaFixer { ctx };
    fixer.apply_all()
}
