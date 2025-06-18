pub mod fixer;
pub mod subfixes;

use crate::fixers::js_style::fixer::JsStyleFixer;
use crate::types::fixer_context::FixContext;

/// Entry point for JS-style fixes.
/// Called by the orchestrator when JS-style issues are detected.
pub fn fix_js_style<'a>(ctx: &mut FixContext) -> String {
    let mut fixer = JsStyleFixer { ctx };
    fixer.apply_all()
}
