// src/fixers/quotes/subfixes.rs

use crate::types::fix_step::FixStep;
use regex::Regex;

pub struct SubQuotesFixer;

impl SubQuotesFixer {
    pub fn fix_single_quotes(input: &str, steps: &mut Vec<FixStep>) -> String {
        let re = Regex::new(r#"'([^']*)'"#).unwrap();
        let new_output = re.replace_all(input, r#""$1""#).to_string();
        if new_output != input {
            steps.push(FixStep::QuotesSingleConverted);
        }
        new_output
    }

    pub fn fix_curly_quotes(input: &str, steps: &mut Vec<FixStep>) -> String {
        let mut new_output = input.to_string();

        let curly_pairs = vec![("“", "\""), ("”", "\""), ("‘", "'"), ("’", "'")];

        let mut changed = false;
        for (curly, ascii) in curly_pairs {
            if new_output.contains(curly) {
                new_output = new_output.replace(curly, ascii);
                changed = true;
            }
        }

        if changed {
            steps.push(FixStep::QuotesCurlyNormalized);
        }

        new_output
    }
}
