// src/fixers/escape/fixer.rs

use crate::fixers::escape::subfixes::SubEscapeFixer;
use crate::types::fixer_context::FixContext;

pub struct EscapeFixer<'ctx> {
    pub ctx: &'ctx mut FixContext,
}

impl<'ctx> EscapeFixer<'ctx> {
    pub fn apply_all(&mut self) -> String {
        SubEscapeFixer::fix_invalid_escapes(self.ctx);
        SubEscapeFixer::fix_broken_unicode(self.ctx);

        self.ctx.input.to_string()
    }

    pub fn apply(ctx: &mut FixContext) {
        let mut fixer = EscapeFixer { ctx };
        fixer.apply_all();
    }
}
