pub mod fixer;
pub mod subfixes;

use crate::fixers::colon::fixer::ColonFixer;
use crate::types::fix_scope::{FixScope, ScopeCategory};
use crate::types::fixer_context::FixContext;

/// Entry point for colon-related fixes.
/// Called by the orchestrator when colon issues are detected.
pub fn fix_colons<'a>(ctx: &mut FixContext) -> String {
    let scope = FixScope::new(&ctx.input, &[ScopeCategory::Colon], None);
    let mut fixer = ColonFixer { ctx, scope };
    fixer.apply_all()
}

crate::colon_fixer!(ColonMissingFixer, colon_missing);
crate::colon_fixer!(ColonMisuseFixer, colon_misuse);
