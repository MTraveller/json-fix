// src/fixers/misc/fixer.rs

use crate::fixers::misc::subfixes::SubMiscFixer;
use crate::types::fix_outcome::FixOutcome;
use crate::types::fix_scope::{FixScope, ScopeCategory};
use crate::types::fixer_context::FixContext;

pub struct MiscFixer<'ctx> {
    pub ctx: &'ctx mut FixContext,
    pub scope: FixScope,
}

impl<'ctx> MiscFixer<'ctx> {
    pub fn apply_all(&mut self) -> FixOutcome {
        let mut outcome = FixOutcome::default();

        outcome.merge(SubMiscFixer::fix_null_slots(self.ctx, &mut self.scope));
        outcome.merge(SubMiscFixer::fix_fallback_artifacts(
            self.ctx,
            &mut self.scope,
        ));

        outcome
    }

    pub fn apply(ctx: &mut FixContext) -> FixOutcome {
        let scope = FixScope::new(&ctx.input, &[ScopeCategory::Misc], None);
        let mut fixer = MiscFixer { ctx, scope };
        fixer.apply_all()
    }
}
