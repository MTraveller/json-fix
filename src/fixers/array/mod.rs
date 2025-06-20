// src/fixers/arrays/mod.rs

pub mod fixer;
pub mod subfixes;

use crate::fixers::array::fixer::ArrayFixer;
use crate::types::fix_scope::{FixScope, ScopeCategory};
use crate::types::fixer_context::FixContext;

/// Entry point for array-related fixes.
/// Called by the orchestrator when array issues are detected.
pub fn fix_arrays<'a>(ctx: &mut FixContext) -> String {
    let scope = FixScope::new(&ctx.input, &[ScopeCategory::Array]);
    let mut fixer = ArrayFixer { ctx, scope };
    fixer.apply_all()
}
