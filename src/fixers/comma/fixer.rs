// src/fixers/comma/fixer.rs

use crate::fixers::comma::subfixes::SubCommaFixer;
use crate::types::fixer_context::FixContext;

pub struct CommaFixer<'ctx> {
    pub ctx: &'ctx mut FixContext,
}

impl<'ctx> CommaFixer<'ctx> {
    pub fn apply_all(&mut self) -> String {
        SubCommaFixer::fix_double_commas(self.ctx);
        SubCommaFixer::fix_misaligned_key_value(self.ctx);
        SubCommaFixer::fix_orphaned_values(self.ctx);
        SubCommaFixer::fix_stray_commas(self.ctx);
        SubCommaFixer::fix_chained_values(self.ctx);
        SubCommaFixer::fix_missing_commas_between_pairs(self.ctx);
        SubCommaFixer::fix_trailing_commas(self.ctx);

        self.ctx.input.to_string()
    }

    pub fn apply(ctx: &mut FixContext) {
        let mut fixer = CommaFixer { ctx };
        fixer.apply_all();
    }
}
