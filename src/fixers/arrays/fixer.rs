// src/fixers/arrays/fixer.rs

use crate::fixers::arrays::subfixes::SubArrayFixer;
use crate::types::fixer_context::FixContext;

pub struct ArrayFixer<'ctx> {
    pub ctx: &'ctx mut FixContext,
}

impl<'ctx> ArrayFixer<'ctx> {
    pub fn apply_all(&mut self) -> String {
        SubArrayFixer::fix_trailing_commas(self.ctx);
        SubArrayFixer::fix_array_structure(self.ctx);
        self.ctx.input.to_string()
    }

    pub fn apply(ctx: &mut FixContext) {
        let mut fixer = ArrayFixer { ctx };
        fixer.apply_all();
    }
}
