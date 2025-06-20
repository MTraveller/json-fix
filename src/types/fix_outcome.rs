// src/types/fix_outcome.rs

#[derive(Debug, Clone)]
pub struct FixOutcome {
    pub fixer_name: String,
    pub changes_applied: usize,
    pub description: String,
}

impl Default for FixOutcome {
    fn default() -> Self {
        Self {
            fixer_name: String::new(),
            changes_applied: 0,
            description: String::new(),
        }
    }
}

impl FixOutcome {
    pub fn merge(&mut self, other: FixOutcome) {
        self.changes_applied += other.changes_applied;
        if self.description.is_empty() {
            self.description = other.description;
        } else if !other.description.is_empty() {
            self.description = format!("{}\n{}", self.description, other.description);
        }
    }
}
