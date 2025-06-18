use crate::fixers::colon::subfixes::SubColonFixer;
use crate::types::fixer_context::FixContext;

pub struct ColonFixer<'ctx> {
    pub ctx: &'ctx mut FixContext,
}

impl<'ctx> ColonFixer<'ctx> {
    pub fn apply(ctx: &mut FixContext) {
        let mut fixer = ColonFixer { ctx };
        fixer.apply_all();
    }

    pub fn apply_all(&mut self) -> String {
        SubColonFixer::fix_missing_colons(self.ctx);
        SubColonFixer::fix_colon_misuse(self.ctx);

        self.ctx.input.to_string()
    }
}
