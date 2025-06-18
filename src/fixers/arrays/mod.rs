// src/fixers/arrays/mod.rs

pub mod fixer;
pub mod subfixes;

use crate::fixers::arrays::fixer::ArrayFixer;
use crate::types::fixer_context::FixContext;

/// Entry point for array-related fixes.
/// Called by the orchestrator when array issues are detected.
pub fn fix_arrays<'a>(ctx: &mut FixContext) -> String {
    let mut fixer = ArrayFixer { ctx };
    fixer.apply_all()
}
