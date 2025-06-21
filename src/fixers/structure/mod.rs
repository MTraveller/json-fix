pub mod fixer;
pub mod subfixes;

use crate::fixers::structure::fixer::StructureFixer;
use crate::types::fix_scope::{FixScope, ScopeCategory};
use crate::types::fixer_context::FixContext;

/// Entry point for structure-related fixes.
/// Called by the orchestrator when structure issues are detected.
pub fn fix_structure<'a>(ctx: &mut FixContext) -> String {
    let scope = FixScope::new(&ctx.input, &[ScopeCategory::Structure], None);
    let mut fixer = StructureFixer { ctx, scope };
    fixer.apply_all()
}

crate::structure_fixer!(StructureConcatenatedFixer, structure_concatenated);
crate::structure_fixer!(StructureOrphanedBraceFixer, structure_orphaned_brace);
