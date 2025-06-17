pub mod fixer;
pub mod subfixes;

use crate::fixers::js_style::fixer::JsStyleFixer;
use crate::types::fix_step::FixStep;

/// Entry point for JS-style fixes.
pub fn fix_js_style(input: &str, steps: &mut Vec<FixStep>) -> String {
    let mut fixer = JsStyleFixer { input, steps };
    fixer.apply_all()
}
