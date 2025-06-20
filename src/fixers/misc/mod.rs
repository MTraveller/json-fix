// src/fixers/misc/mod.rs

pub mod fixer;
pub mod subfixes;

use crate::fixers::misc::fixer::MiscFixer;
use crate::types::fix_scope::{FixScope, ScopeCategory};
use crate::types::fixer_context::FixContext;

/// Entry point for miscellaneous fixes.
/// Called by the orchestrator when miscellaneous issues are detected.
pub fn fix_misc<'a>(ctx: &mut FixContext) -> String {
    let scope = FixScope::new(&ctx.input, &[ScopeCategory::Misc], None);
    let mut fixer = MiscFixer { ctx, scope };
    fixer.apply_all()
}

crate::misc_fixer!(MiscNullSlotsFixer, misc_null_slots);
crate::misc_fixer!(MiscFallbackFixer, misc_fallback);
