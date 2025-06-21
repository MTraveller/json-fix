// src/fixers/arrays/mod.rs

pub mod fixer;
pub mod subfixes;

use crate::fixers::array::fixer::ArrayFixer;
use crate::types::fix_scope::{FixScope, ScopeCategory};
use crate::types::fixer_context::FixContext;

/// Entry point for array-related fixes.
/// Called by the orchestrator when array issues are detected.
pub fn fix_arrays<'a>(ctx: &mut FixContext) -> String {
    let scope = FixScope::new(&ctx.input, &[ScopeCategory::Array], None);
    let mut fixer = ArrayFixer { ctx, scope };
    fixer.apply_all()
}

crate::array_fixer!(ArrayTrailingCommaFixer, array_trailing_comma);
crate::array_fixer!(ArrayLeadingCommaFixer, array_leading_comma);
crate::array_fixer!(ArrayEmptySlotFixer, array_empty_slot);
crate::array_fixer!(ArrayStructureMalformedFixer, array_structure_malformed);
