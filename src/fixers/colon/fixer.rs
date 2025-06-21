use crate::fixers::colon::subfixes::SubColonFixer;
use crate::types::fix_outcome::FixOutcome;
use crate::types::fix_scope::{FixScope, ScopeCategory};
use crate::types::fixer_context::FixContext;

/// ColonFixer acts as a scoped fixer executor for colon-related syntax issues.
/// It holds context and applies only fixes within its defined scope.
pub struct ColonFixer<'ctx> {
    pub ctx: &'ctx mut FixContext,
    pub scope: FixScope,
}

impl<'ctx> ColonFixer<'ctx> {
    /// Applies all relevant subfixes for colon structures.
    /// Returns a merged FixOutcome representing the total applied changes.
    pub fn apply_all(&mut self) -> FixOutcome {
        let mut outcome = FixOutcome::default();

        outcome.merge(SubColonFixer::fix_missing_colons(self.ctx, &mut self.scope));
        outcome.merge(SubColonFixer::fix_colon_misuse(self.ctx, &mut self.scope));

        outcome
    }

    /// Initializes a ColonFixer and applies it directly to the given context.
    pub fn apply(ctx: &mut FixContext) -> FixOutcome {
        let scope = FixScope::new(&ctx.input, &[ScopeCategory::Colon], None);
        let mut fixer = ColonFixer { ctx, scope };
        fixer.apply_all()
    }
}
