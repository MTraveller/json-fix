// src/fixers/comma/fixes.rs

use crate::types::fix_step::FixStep;
use regex::Regex;

pub struct SubCommaFixer;

impl SubCommaFixer {
    pub fn fix_double_commas(input: &str, steps: &mut Vec<FixStep>) -> String {
        let new_output = input.replace(",,", ",");
        if new_output != input {
            steps.push(FixStep::CommaDoubleRemoved);
        }
        new_output
    }

    pub fn fix_misaligned_key_value(input: &str, steps: &mut Vec<FixStep>) -> String {
        let re = Regex::new(r#"\s*:\s*,\s*\""#).unwrap();
        let new_output = re.replace_all(input, ": null, \"").to_string();
        if new_output != input {
            steps.push(FixStep::CommaMisalignmentFixed);
        }
        new_output
    }

    pub fn fix_orphaned_values(input: &str, steps: &mut Vec<FixStep>) -> String {
        let re = Regex::new(r#"null\s*,\s*\"[^\"]+\""#).unwrap();
        let new_output = re.replace_all(input, "null").to_string();
        if new_output != input {
            steps.push(FixStep::CommaOrphanedValueHandled);
        }
        new_output
    }

    pub fn fix_stray_commas(input: &str, steps: &mut Vec<FixStep>) -> String {
        let re = Regex::new(r#"(\{|\[)\s*,"#).unwrap();
        let new_output = re.replace_all(input, "$1").to_string();
        if new_output != input {
            steps.push(FixStep::CommaStrayRemoved);
        }
        new_output
    }

    pub fn fix_chained_values(input: &str, steps: &mut Vec<FixStep>) -> String {
        let re = Regex::new(r#"\"[^\"]+\"\s*,\s*\"[^\"]+\""#).unwrap();
        let new_output = re.replace_all(input, "\"chained_value\"").to_string();
        if new_output != input {
            steps.push(FixStep::CommaChainedValueFixed);
        }
        new_output
    }

    pub fn fix_missing_commas_between_pairs(input: &str, steps: &mut Vec<FixStep>) -> String {
        let re = Regex::new(r#"("[^"]+"\s*:\s*[^,\{\}\[\]]+)\s+("[^"]+"\s*:)"#).unwrap();

        let new_output = re.replace_all(input, "$1, $2").to_string();

        if new_output != input {
            println!("Fix applied: missing commas between pairs");
            steps.push(FixStep::CommaMissingAdded);
            println!("Steps now: {:?}", steps);
        }
        new_output
    }
}
