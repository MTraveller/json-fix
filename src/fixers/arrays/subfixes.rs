// fixers/arrays/subfixes.rs

use crate::types::fix_step::FixStep;

pub struct SubArrayFixer;

impl SubArrayFixer {
    /// Fixes trailing commas in arrays, e.g., [1, 2, 3, ]
    pub fn fix_trailing_commas(input: &str, steps: &mut Vec<FixStep>) -> String {
        let re = regex::Regex::new(r#",\s*(\])"#).unwrap();
        let new_input = re.replace_all(input, "$1").to_string();

        if new_input != input {
            steps.push(FixStep::ArraysTrailingCommaFixed);
        }

        new_input
    }

    /// Fixes general array structural issues (e.g., missing commas or malformed brackets)
    pub fn fix_array_structure(input: &str, steps: &mut Vec<FixStep>) -> String {
        let re = regex::Regex::new(r#""\s*("[^"]*")"#).unwrap();
        let new_input = re.replace_all(input, r#", $1"#).to_string();

        if new_input != input {
            steps.push(FixStep::ArraysStructureCorrected);
        }

        new_input
    }
}
