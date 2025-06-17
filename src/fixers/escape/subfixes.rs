// src/fixers/escape/subfixes.rs

use crate::types::fix_step::FixStep;
use regex::Regex;

pub struct SubEscapeFixer;

impl SubEscapeFixer {
    /// Removes invalid escape sequences like `\q`, `\z`, etc.
    pub fn fix_invalid_escapes(input: &str, steps: &mut Vec<FixStep>) -> String {
        let re = Regex::new(r#"\\[^btnfru"\\]"#).unwrap(); // Matches \ followed by any char that's not a valid escape
        let new_output = re.replace_all(input, "").to_string();
        if new_output != input {
            steps.push(FixStep::EscapeInvalidRemoved);
        }
        new_output
    }

    /// Replaces broken or incomplete Unicode escapes like `\u12` or `\uXYZ` with `\uFFFD` (replacement char).
    pub fn fix_broken_unicode(input: &str, steps: &mut Vec<FixStep>) -> String {
        let re = Regex::new(r#"\\u[0-9a-fA-F]{0,3}(?![0-9a-fA-F])"#).unwrap();
        let new_output = re.replace_all(input, "\\uFFFD").to_string();
        if new_output != input {
            steps.push(FixStep::EscapeUnicodeRepaired);
        }
        new_output
    }
}
