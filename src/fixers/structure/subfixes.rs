use crate::types::fix_step::FixStep;

pub struct SubStructureFixer;

impl SubStructureFixer {
    /// Attempts to fix concatenated root JSON objects (e.g., `}{` → `},{`)
    pub fn fix_concatenated_json(input: &str, steps: &mut Vec<FixStep>) -> String {
        let re = regex::Regex::new(r"\}\s*\{").unwrap();
        let new_input = re.replace_all(input, "},{").to_string();

        if new_input != input {
            steps.push(FixStep::StructureConcatenatedSplit);
        }

        new_input
    }

    /// Attempts to resolve orphaned braces by balancing `{` and `}`
    pub fn fix_orphaned_braces(input: &str, steps: &mut Vec<FixStep>) -> String {
        let mut new_input = input.to_string();
        let open_count = new_input.matches('{').count();
        let close_count = new_input.matches('}').count();

        if open_count > close_count {
            new_input.push_str(&"}".repeat(open_count - close_count));
            steps.push(FixStep::StructureOrphanedBraceResolved);
        } else if close_count > open_count {
            new_input = "{".repeat(close_count - open_count) + &new_input;
            steps.push(FixStep::StructureOrphanedBraceResolved);
        }

        new_input
    }
}
