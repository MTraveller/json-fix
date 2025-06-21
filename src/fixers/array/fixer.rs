// src/fixers/arrays/fixer.rs

use crate::fixers::array::subfixes::SubArrayFixer;
use crate::types::fix_outcome::FixOutcome;
use crate::types::fix_scope::{FixScope, ScopeCategory};
use crate::types::fixer_context::FixContext;

/// ArrayFixer acts as a scoped fixer executor for array-related syntax issues.
/// It holds context and applies only fixes within its defined scope.
pub struct ArrayFixer<'ctx> {
    pub ctx: &'ctx mut FixContext,
    pub scope: FixScope,
}

impl<'ctx> ArrayFixer<'ctx> {
    /// Applies all relevant subfixes for array structures.
    /// Returns a merged FixOutcome representing the total applied changes.
    pub fn apply_all(&mut self) -> FixOutcome {
        let mut outcome = FixOutcome::default();

        outcome.merge(SubArrayFixer::fix_trailing_commas(
            self.ctx,
            &mut self.scope,
        ));
        outcome.merge(SubArrayFixer::fix_array_structure(
            self.ctx,
            &mut self.scope,
        ));
        outcome.merge(SubArrayFixer::fix_leading_commas(self.ctx, &mut self.scope));

        outcome
    }

    /// Initializes an ArrayFixer and applies it directly to the given context.
    pub fn apply(ctx: &mut FixContext) -> FixOutcome {
        let scope = FixScope::new(&ctx.input, &[ScopeCategory::Array], None);
        let mut fixer = ArrayFixer { ctx, scope };
        fixer.apply_all()
    }
}
