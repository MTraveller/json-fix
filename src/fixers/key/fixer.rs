// src/fixers/keys/fixer.rs

use crate::fixers::key::subfixes::SubKeyFixer;
use crate::types::fix_outcome::FixOutcome;
use crate::types::fix_scope::{FixScope, ScopeCategory};
use crate::types::fixer_context::FixContext;

/// KeysFixer applies fixes to JSON key-related issues such as unquoted keys or traps.
pub struct KeysFixer<'ctx> {
    pub ctx: &'ctx mut FixContext,
    pub scope: FixScope,
}

impl<'ctx> KeysFixer<'ctx> {
    /// Applies all key-related subfixes and merges their results into a FixOutcome.
    pub fn apply_all(&mut self) -> FixOutcome {
        let mut outcome = FixOutcome::default();

        outcome.merge(SubKeyFixer::fix_unquoted_keys(self.ctx, &mut self.scope));
        outcome.merge(SubKeyFixer::fix_key_traps(self.ctx, &mut self.scope));

        outcome
    }

    /// Entry point to run KeysFixer using a FixContext, automatically scoped to Key issues.
    pub fn apply(ctx: &mut FixContext) -> FixOutcome {
        let scope = FixScope::new(&ctx.input, &[ScopeCategory::Key], None);
        let mut fixer = KeysFixer { ctx, scope };
        fixer.apply_all()
    }
}
