// src/fixers/brackets/fixer.rs

use crate::fixers::brackets::subfixes::SubBracketFixer;
use crate::types::fixer_context::FixContext;

pub struct BracketFixer<'ctx> {
    pub ctx: &'ctx mut FixContext,
}

impl<'ctx> BracketFixer<'ctx> {
    pub fn apply_all(&mut self) -> String {
        SubBracketFixer::fix_extra_closing_brace(self.ctx);
        SubBracketFixer::fix_missing_closing_brace(self.ctx);

        self.ctx.input.to_string()
    }

    pub fn apply(ctx: &mut FixContext) {
        let mut fixer = BracketFixer { ctx };
        fixer.apply_all();
    }
}
