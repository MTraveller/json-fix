// src/fixers/comma/fixer.rs

use crate::fixers::comma::subfixes::SubCommaFixer;
use crate::types::fix_outcome::FixOutcome;
use crate::types::fix_scope::{FixScope, ScopeCategory};
use crate::types::fixer_context::FixContext;

/// CommaFixer acts as a scoped fixer executor for comma-related syntax issues.
/// It holds context and applies only fixes within its defined scope.
pub struct CommaFixer<'ctx> {
    pub ctx: &'ctx mut FixContext,
    pub scope: FixScope,
}

impl<'ctx> CommaFixer<'ctx> {
    /// Applies all relevant subfixes for comma structures.
    /// Returns a merged FixOutcome representing the total applied changes.
    pub fn apply_all(&mut self) -> FixOutcome {
        let mut outcome = FixOutcome::default();

        outcome.merge(SubCommaFixer::fix_double_commas(self.ctx, &mut self.scope));
        outcome.merge(SubCommaFixer::fix_misaligned_key_value(
            self.ctx,
            &mut self.scope,
        ));
        outcome.merge(SubCommaFixer::fix_orphaned_values(
            self.ctx,
            &mut self.scope,
        ));
        outcome.merge(SubCommaFixer::fix_stray_commas(self.ctx, &mut self.scope));
        outcome.merge(SubCommaFixer::fix_chained_values(self.ctx, &mut self.scope));
        outcome.merge(SubCommaFixer::fix_missing_commas_between_pairs(
            self.ctx,
            &mut self.scope,
        ));
        outcome.merge(SubCommaFixer::fix_trailing_commas(
            self.ctx,
            &mut self.scope,
        ));

        outcome
    }

    /// Initializes a CommaFixer and applies it directly to the given context.
    pub fn apply(ctx: &mut FixContext) -> FixOutcome {
        let scope = FixScope::new(&ctx.input, &[ScopeCategory::Comma], None);
        let mut fixer = CommaFixer { ctx, scope };
        fixer.apply_all()
    }
}
