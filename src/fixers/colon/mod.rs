pub mod fixer;
pub mod subfixes;

use crate::fixers::colon::fixer::ColonFixer;
use crate::types::fixer_context::FixContext;

/// Entry point for colon-related fixes.
/// Called by the orchestrator when colon issues are detected.
pub fn fix_colons<'a>(ctx: &mut FixContext) -> String {
    let mut fixer = ColonFixer { ctx };
    fixer.apply_all()
}
