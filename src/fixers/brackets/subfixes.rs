// src/fixers/brackets/subfixes.rs

use crate::types::fix_step::FixStep;
use regex::Regex;

pub struct SubBracketFixer;

impl SubBracketFixer {
    /// Fixes unbalanced brackets by removing obvious extra closing braces.
    pub fn fix_extra_closing_brace(input: &str, steps: &mut Vec<FixStep>) -> String {
        let re = Regex::new(r"\}\s*\}").unwrap();
        if re.is_match(input) {
            // no unwrap_or here if using regex crate
            let new_str = re.replace_all(input, "}").to_string();
            steps.push(FixStep::BracketsExtraRemoved);
            new_str
        } else {
            input.to_string()
        }
    }

    /// Adds a closing brace if input appears to be missing one at the end.
    pub fn fix_missing_closing_brace(input: &str, steps: &mut Vec<FixStep>) -> String {
        let open_count = input.matches('{').count();
        let close_count = input.matches('}').count();
        if open_count > close_count {
            let new_output = format!("{}{}", input, "}".repeat(open_count - close_count));
            steps.push(FixStep::BracketsMissingAdded);
            new_output
        } else {
            input.to_string()
        }
    }

    /// Attempts to balance braces if counts are mismatched in general.
    pub fn fix_unbalanced(input: &str, steps: &mut Vec<FixStep>) -> String {
        let open_count = input.matches('{').count();
        let close_count = input.matches('}').count();
        if open_count < close_count {
            let new_output = input.trim_end_matches('}').to_string();
            steps.push(FixStep::BracketsBalanced);
            new_output
        } else if open_count > close_count {
            let new_output = format!("{}{}", input, "}".repeat(open_count - close_count));
            steps.push(FixStep::BracketsBalanced);
            new_output
        } else {
            input.to_string()
        }
    }
}
