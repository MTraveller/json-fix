// src/fixers/comma/mod.rs

pub mod fixer;
pub mod subfixes;

use crate::fixers::comma::fixer::CommaFixer;
use crate::types::fix_scope::{FixScope, ScopeCategory};
use crate::types::fixer_context::FixContext;

/// Entry point for comma-related fixes.
/// Called by the orchestrator when comma issues are detected.
pub fn fix_commas<'a>(ctx: &mut FixContext) -> String {
    let scope = FixScope::new(&ctx.input, &[ScopeCategory::Comma], None);
    let mut fixer = CommaFixer { ctx, scope };
    fixer.apply_all()
}

crate::comma_fixer!(CommaDoubleFixer, comma_double);
crate::comma_fixer!(CommaMisalignedFixer, comma_misaligned);
crate::comma_fixer!(CommaOrphanedValueFixer, comma_orphaned_value);
crate::comma_fixer!(CommaChainFixer, comma_chain);
crate::comma_fixer!(CommaStrayFixer, comma_stray);
crate::comma_fixer!(CommaMissingFixer, comma_missing);
