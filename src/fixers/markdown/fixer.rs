use crate::fixers::markdown::subfixes::SubMarkdownFixer;
use crate::types::fix_outcome::FixOutcome;
use crate::types::fix_scope::{FixScope, ScopeCategory};
use crate::types::fixer_context::FixContext;

pub struct MarkdownFixer<'ctx> {
    pub ctx: &'ctx mut FixContext,
    pub scope: FixScope,
}

impl<'ctx> MarkdownFixer<'ctx> {
    pub fn apply_all(&mut self) -> FixOutcome {
        let mut outcome = FixOutcome::default();

        outcome.merge(SubMarkdownFixer::remove_markdown_wrappers(
            self.ctx,
            &mut self.scope,
        ));
        outcome.merge(SubMarkdownFixer::extract_json_blocks(
            self.ctx,
            &mut self.scope,
        ));

        outcome
    }

    pub fn apply(ctx: &mut FixContext) -> FixOutcome {
        let scope = FixScope::new(&ctx.input, &[ScopeCategory::Markdown], None);
        let mut fixer = MarkdownFixer { ctx, scope };
        fixer.apply_all()
    }
}
