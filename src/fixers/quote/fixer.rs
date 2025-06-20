// src/fixers/quotes/fixer.rs

use crate::fixers::quote::subfixes::SubQuotesFixer;
use crate::types::fixer_context::FixContext;

pub struct QuoteFixer<'ctx> {
    pub ctx: &'ctx mut FixContext,
}

impl<'ctx> QuoteFixer<'ctx> {
    pub fn apply_all(&mut self) -> String {
        self.ctx.input = SubQuotesFixer::fix_single_quotes(self.ctx);
        self.ctx.input = SubQuotesFixer::fix_curly_quotes(self.ctx);

        self.ctx.input.to_string()
    }

    pub fn apply(ctx: &mut FixContext) {
        let mut fixer = QuoteFixer { ctx };
        fixer.apply_all();
    }
}
