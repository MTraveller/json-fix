// src/fixers/misc/fixer.rs

use crate::fixers::misc::subfixes::SubMiscFixer;
use crate::types::fixer_context::FixContext;

pub struct MiscFixer<'ctx> {
    pub ctx: &'ctx mut FixContext,
}

impl<'ctx> MiscFixer<'ctx> {
    pub fn apply_all(&mut self) -> String {
        self.ctx.input = SubMiscFixer::fix_null_slots(self.ctx);
        self.ctx.input = SubMiscFixer::fix_fallback_artifacts(self.ctx);

        self.ctx.input.to_string()
    }

    pub fn apply(ctx: &mut FixContext) {
        let mut fixer = MiscFixer { ctx };
        fixer.apply_all();
    }
}
