use crate::types::fix_step::FixStep;

pub struct SubColonFixer;

impl SubColonFixer {
    /// Fixes missing colons between keys and values
    pub fn fix_missing_colons(input: &str, steps: &mut Vec<FixStep>) -> String {
        let re = regex::Regex::new(r#""(\w+)"\s+("[^"]+"|\d+|true|false|null)"#).unwrap();
        let new_input = re.replace_all(input, r#""$1": $2"#).to_string();

        if new_input != input {
            steps.push(FixStep::ColonMissingFixed);
        }

        new_input
    }

    /// Fixes misuse of colons (e.g., double colons)
    pub fn fix_colon_misuse(input: &str, steps: &mut Vec<FixStep>) -> String {
        let re = regex::Regex::new(r"::+").unwrap();
        let new_input = re.replace_all(input, ":").to_string();

        if new_input != input {
            steps.push(FixStep::ColonMisuseFixed);
        }

        new_input
    }
}
