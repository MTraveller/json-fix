// src/fixers/escape/fixer.rs

use crate::fixers::escape::subfixes::SubEscapeFixer;
use crate::types::fix_outcome::FixOutcome;
use crate::types::fix_scope::{FixScope, ScopeCategory};
use crate::types::fixer_context::FixContext;

/// EscapeFixer acts as a scoped fixer executor for escape-related syntax issues.
/// It holds context and applies only fixes within its defined scope.
pub struct EscapeFixer<'ctx> {
    pub ctx: &'ctx mut FixContext,
    pub scope: FixScope,
}

impl<'ctx> EscapeFixer<'ctx> {
    /// Applies all relevant subfixes for escape sequences.
    /// Returns a merged FixOutcome representing the total applied changes.
    pub fn apply_all(&mut self) -> FixOutcome {
        let mut outcome = FixOutcome::default();

        outcome.merge(SubEscapeFixer::fix_invalid_escapes(
            self.ctx,
            &mut self.scope,
        ));
        outcome.merge(SubEscapeFixer::fix_broken_unicode(
            self.ctx,
            &mut self.scope,
        ));

        outcome
    }

    /// Initializes an EscapeFixer and applies it directly to the given context.
    pub fn apply(ctx: &mut FixContext) -> FixOutcome {
        let scope = FixScope::new(&ctx.input, &[ScopeCategory::Escape], None);
        let mut fixer = EscapeFixer { ctx, scope };
        fixer.apply_all()
    }
}
