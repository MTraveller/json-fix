use crate::fixers::js_style::subfixes::SubJsStyleFixer;
use crate::types::fix_outcome::FixOutcome;
use crate::types::fix_scope::{FixScope, ScopeCategory};
use crate::types::fixer_context::FixContext;

/// JsStyleFixer applies fixes related to JavaScript-like syntax issues.
pub struct JsStyleFixer<'ctx> {
    pub ctx: &'ctx mut FixContext,
    pub scope: FixScope,
}

impl<'ctx> JsStyleFixer<'ctx> {
    /// Applies all JavaScript-style related fixes and returns a unified FixOutcome.
    pub fn apply_all(&mut self) -> FixOutcome {
        let mut outcome = FixOutcome::default();

        outcome.merge(SubJsStyleFixer::fix_undefined(self.ctx, &mut self.scope));
        outcome.merge(SubJsStyleFixer::fix_nan(self.ctx, &mut self.scope));
        outcome.merge(SubJsStyleFixer::remove_js_comments(
            self.ctx,
            &mut self.scope,
        ));

        outcome
    }

    /// Initializes and applies JsStyleFixer for a given context.
    pub fn apply(ctx: &mut FixContext) -> FixOutcome {
        let scope = FixScope::new(&ctx.input, &[ScopeCategory::JsStyle], None);
        let mut fixer = JsStyleFixer { ctx, scope };
        fixer.apply_all()
    }
}
