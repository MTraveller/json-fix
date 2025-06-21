use crate::fixers::quote::subfixes::SubQuoteFixer;
use crate::types::fix_outcome::FixOutcome;
use crate::types::fix_scope::{FixScope, ScopeCategory};
use crate::types::fixer_context::FixContext;

/// QuoteFixer acts as a scoped fixer executor for quote-related syntax issues.
/// It holds context and applies only fixes within its defined scope.
pub struct QuoteFixer<'ctx> {
    pub ctx: &'ctx mut FixContext,
    pub scope: FixScope,
}

impl<'ctx> QuoteFixer<'ctx> {
    /// Applies all relevant subfixes for quote structures.
    /// Returns a merged FixOutcome representing the total applied changes.
    pub fn apply_all(&mut self) -> FixOutcome {
        let mut outcome = FixOutcome::default();

        outcome.merge(SubQuoteFixer::fix_single_quotes(self.ctx, &mut self.scope));
        outcome.merge(SubQuoteFixer::fix_curly_quotes(self.ctx, &mut self.scope));

        outcome
    }

    /// Initializes a QuoteFixer and applies it directly to the given context.
    pub fn apply(ctx: &mut FixContext) -> FixOutcome {
        let scope = FixScope::new(
            &ctx.input,
            &[ScopeCategory::Quote],
            crate::types::emotion_phase::EmotionPhase::Ready,
        );
        let mut fixer = QuoteFixer { ctx, scope };
        fixer.apply_all()
    }
}
