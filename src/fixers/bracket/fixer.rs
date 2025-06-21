// src/fixers/brackets/fixer.rs

use crate::fixers::bracket::subfixes::SubBracketFixer;
use crate::types::fix_outcome::FixOutcome;
use crate::types::fix_scope::{FixScope, ScopeCategory};
use crate::types::fixer_context::FixContext;

/// BracketFixer acts as a scoped fixer executor for bracket-related syntax issues.
/// It holds context and applies only fixes within its defined scope.
pub struct BracketFixer<'ctx> {
    pub ctx: &'ctx mut FixContext,
    pub scope: FixScope,
}

impl<'ctx> BracketFixer<'ctx> {
    /// Applies all relevant subfixes for bracket structures.
    /// Returns a merged FixOutcome representing the total applied changes.
    pub fn apply_all(&mut self) -> FixOutcome {
        let mut outcome = FixOutcome::default();

        outcome.merge(SubBracketFixer::brackets_unbalanced(
            self.ctx,
            &mut self.scope,
        ));
        outcome.merge(SubBracketFixer::brackets_extra(self.ctx, &mut self.scope));
        outcome.merge(SubBracketFixer::brackets_missing(self.ctx, &mut self.scope));

        outcome
    }

    /// Initializes an BracketFixer and applies it directly to the given context.
    pub fn apply(ctx: &mut FixContext) -> FixOutcome {
        let scope = FixScope::new(&ctx.input, &[ScopeCategory::Array], None);
        let mut fixer = BracketFixer { ctx, scope };
        fixer.apply_all()
    }
}
