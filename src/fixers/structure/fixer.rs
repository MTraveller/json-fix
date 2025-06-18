use crate::fixers::structure::subfixes::SubStructureFixer;
use crate::types::fixer_context::FixContext;

pub struct StructureFixer<'ctx> {
    pub ctx: &'ctx mut FixContext,
}

impl<'ctx> StructureFixer<'ctx> {
    pub fn apply_all(&mut self) -> String {
        SubStructureFixer::fix_concatenated_json(self.ctx);
        SubStructureFixer::fix_orphaned_braces(self.ctx);

        self.ctx.input.to_string()
    }

    pub fn apply(ctx: &mut FixContext) {
        let mut fixer = StructureFixer { ctx };
        fixer.apply_all();
    }
}
