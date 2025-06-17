use crate::fixers::js_style::subfixes::SubJsStyleFixer;
use crate::types::fix_step::FixStep;

pub struct JsStyleFixer<'a> {
    pub input: &'a str,
    pub steps: &'a mut Vec<FixStep>,
}

impl<'a> JsStyleFixer<'a> {
    pub fn apply_all(&mut self) -> String {
        let mut output = self.input.to_string();

        output = SubJsStyleFixer::fix_undefined(&output, self.steps);
        output = SubJsStyleFixer::fix_nan(&output, self.steps);
        output = SubJsStyleFixer::remove_js_comments(&output, self.steps);

        output
    }
}
