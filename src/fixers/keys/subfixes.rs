// src/fixers/keys/subfixes.rs

use crate::types::fix_step::FixStep;
use regex::Regex;

pub struct SubKeyFixer;

impl SubKeyFixer {
    pub fn fix_unquoted_keys(input: &str, steps: &mut Vec<FixStep>) -> String {
        let re = Regex::new(r#"(?m)(?P<pre>[^"\s{,])(?P<key>\w+)\s*:"#).unwrap();
        let new_output = re.replace_all(input, "${pre}\"${key}\":").to_string();
        if new_output != input {
            steps.push(FixStep::KeysUnquotedFixed);
        }
        new_output
    }

    pub fn fix_key_traps(input: &str, steps: &mut Vec<FixStep>) -> String {
        let re = Regex::new(r#""[^"]*"\s*:\s*(,|\])"#).unwrap();
        let new_output = re.replace_all(input, "\"null\"$1").to_string();
        if new_output != input {
            steps.push(FixStep::KeysTrapResolved);
        }
        new_output
    }
}
