// src/fixers/markdown/fixer.rs

use crate::fixers::markdown::subfixes::SubMarkdownFixer;
use crate::types::fixer_context::FixContext;

pub struct MarkdownFixer<'ctx> {
    pub ctx: &'ctx mut FixContext,
}

impl<'ctx> MarkdownFixer<'ctx> {
    pub fn apply_all(&mut self) -> String {
        self.ctx.input = SubMarkdownFixer::remove_markdown_wrappers(self.ctx);
        self.ctx.input = SubMarkdownFixer::extract_json_blocks(self.ctx);
        self.ctx.input.to_string()
    }

    pub fn apply(ctx: &mut FixContext) {
        let mut fixer = MarkdownFixer { ctx };
        fixer.apply_all();
    }
}
