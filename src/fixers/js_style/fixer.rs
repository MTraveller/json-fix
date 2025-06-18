use crate::fixers::js_style::subfixes::SubJsStyleFixer;
use crate::types::fixer_context::FixContext;

pub struct JsStyleFixer<'ctx> {
    pub ctx: &'ctx mut FixContext,
}

impl<'ctx> JsStyleFixer<'ctx> {
    pub fn apply_all(&mut self) -> String {
        SubJsStyleFixer::fix_undefined(self.ctx);
        SubJsStyleFixer::fix_nan(self.ctx);
        SubJsStyleFixer::remove_js_comments(self.ctx);

        self.ctx.input.to_string()
    }

    pub fn apply(ctx: &mut FixContext) {
        let mut fixer = JsStyleFixer { ctx };
        fixer.apply_all();
    }
}
