// src/fixers/markdown/subfixes.rs

use crate::types::fix_step::FixStep;
use regex::Regex;

pub struct SubMarkdownFixer;

impl SubMarkdownFixer {
    /// Removes common Markdown wrappers like code fences (```json ... ```)
    pub fn remove_markdown_wrappers(input: &str, steps: &mut Vec<FixStep>) -> String {
        let re = Regex::new(r#"(?s)```(?:json)?\s*(.*?)\s*```"#).unwrap();
        let new_output = re.replace_all(input, "$1").to_string();
        if new_output != input {
            steps.push(FixStep::MarkdownWrapperRemoved);
        }
        new_output
    }

    /// Attempts to extract JSON blocks embedded in Markdown text
    pub fn extract_json_blocks(input: &str, steps: &mut Vec<FixStep>) -> String {
        let re = Regex::new(r#"(?s)(?:^|\n)\s*```(?:json)?\s*(.*?)\s*```(?:\n|$)"#).unwrap();
        let new_output = re.replace_all(input, "$1").to_string();
        if new_output != input {
            steps.push(FixStep::MarkdownJsonExtracted);
        }
        new_output
    }
}
