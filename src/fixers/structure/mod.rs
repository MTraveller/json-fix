pub mod fixer;
pub mod subfixes;

use crate::fixers::structure::fixer::StructureFixer;
use crate::types::fixer_context::FixContext;

/// Entry point for structure-related fixes.
/// Called by the orchestrator when structure issues are detected.
pub fn fix_structure<'a>(ctx: &mut FixContext) -> String {
    let mut fixer = StructureFixer { ctx };
    fixer.apply_all()
}
