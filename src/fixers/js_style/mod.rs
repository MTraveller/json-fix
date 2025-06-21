pub mod fixer;
pub mod subfixes;

use crate::fixers::js_style::fixer::JsStyleFixer;
use crate::types::fix_scope::{FixScope, ScopeCategory};
use crate::types::fixer_context::FixContext;

/// Entry point for JS-style fixes.
/// Called by the orchestrator when JS-style issues are detected.
pub fn fix_js_style<'a>(ctx: &mut FixContext) -> String {
    let scope = FixScope::new(&ctx.input, &[ScopeCategory::JsStyle], None);
    let mut fixer = JsStyleFixer { ctx, scope };
    fixer.apply_all()
}

crate::jsstyle_fixer!(JsUndefinedFixer, js_undefined);
crate::jsstyle_fixer!(JsNaNFixer, js_na_n);
crate::jsstyle_fixer!(JsCommentFixer, js_comment);
