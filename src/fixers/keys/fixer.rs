// src/fixers/keys/fixer.rs

use crate::fixers::keys::subfixes::SubKeyFixer;
use crate::types::fixer_context::FixContext;

pub struct KeysFixer<'ctx> {
    pub ctx: &'ctx mut FixContext,
}

impl<'ctx> KeysFixer<'ctx> {
    pub fn apply_all(&mut self) -> String {
        self.ctx.input = SubKeyFixer::fix_unquoted_keys(self.ctx);
        self.ctx.input = SubKeyFixer::fix_key_traps(self.ctx);
        self.ctx.input.to_string()
    }

    pub fn apply(ctx: &mut FixContext) {
        let mut fixer = KeysFixer { ctx };
        fixer.apply_all();
    }
}
