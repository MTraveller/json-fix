use crate::types::fix_step::FixStep;

pub struct SubJsStyleFixer;

impl SubJsStyleFixer {
    /// Converts `undefined` → `null`
    pub fn fix_undefined(input: &str, steps: &mut Vec<FixStep>) -> String {
        let re = regex::Regex::new(r"\bundefined\b").unwrap();
        let new_input = re.replace_all(input, "null").to_string();

        if new_input != input {
            steps.push(FixStep::JsUndefinedReplaced);
        }

        new_input
    }

    /// Converts `NaN` or `Infinity` → `null`
    pub fn fix_nan(input: &str, steps: &mut Vec<FixStep>) -> String {
        let re = regex::Regex::new(r"\b(NaN|Infinity|-Infinity)\b").unwrap();
        let new_input = re.replace_all(input, "null").to_string();

        if new_input != input {
            steps.push(FixStep::JsNaNReplaced);
        }

        new_input
    }

    /// Removes JS-style comments
    pub fn remove_js_comments(input: &str, steps: &mut Vec<FixStep>) -> String {
        let re = regex::Regex::new(r"(//[^\n]*|/\*[\s\S]*?\*/)").unwrap();
        let new_input = re.replace_all(input, "").to_string();

        if new_input != input {
            steps.push(FixStep::JsCommentsRemoved);
        }

        new_input
    }
}
