// src/fixers/misc/subfixes.rs

use crate::types::fix_step::FixStep;
use regex::Regex;

pub struct SubMiscFixer;

impl SubMiscFixer {
    pub fn fix_null_slots(input: &str, steps: &mut Vec<FixStep>) -> String {
        let re = Regex::new(r#"(":\s*)(,|\])"#).unwrap();
        let new_output = re.replace_all(input, "${1}null$2").to_string();
        if new_output != input {
            steps.push(FixStep::MiscNullSlotsFilled);
        }
        new_output
    }

    pub fn fix_fallback_artifacts(input: &str, steps: &mut Vec<FixStep>) -> String {
        let re = Regex::new(r#"\s*,\s*("[^"]*")\s*,\s*"#).unwrap();
        let new_output = re.replace_all(input, ", $1").to_string();
        if new_output != input {
            steps.push(FixStep::MiscFallbackApplied);
        }
        new_output
    }
}
