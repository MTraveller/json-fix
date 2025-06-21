use crate::fixers::structure::subfixes::SubStructureFixer;
use crate::types::fix_outcome::FixOutcome;
use crate::types::fix_scope::{FixScope, ScopeCategory};
use crate::types::fixer_context::FixContext;

/// StructureFixer acts as a scoped fixer executor for structure-related syntax issues.
/// It holds context and applies only fixes within its defined scope.
pub struct StructureFixer<'ctx> {
    pub ctx: &'ctx mut FixContext,
    pub scope: FixScope,
}

impl<'ctx> StructureFixer<'ctx> {
    /// Applies all relevant subfixes for structural issues.
    /// Returns a merged FixOutcome representing the total applied changes.
    pub fn apply_all(&mut self) -> FixOutcome {
        let mut outcome = FixOutcome::default();

        outcome.merge(SubStructureFixer::fix_concatenated_json(
            self.ctx,
            &mut self.scope,
        ));
        outcome.merge(SubStructureFixer::fix_orphaned_braces(
            self.ctx,
            &mut self.scope,
        ));

        outcome
    }

    /// Initializes a StructureFixer and applies it directly to the given context.
    pub fn apply(ctx: &mut FixContext) -> FixOutcome {
        let scope = FixScope::new(&ctx.input, &[ScopeCategory::Structure], None);
        let mut fixer = StructureFixer { ctx, scope };
        fixer.apply_all()
    }
}
