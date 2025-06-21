// src/types/fix_scope.rs

use crate::types::emotion_phase::EmotionPhase;
use crate::types::{diagnostic_core::FixDiagnostic, fixer_flags::FixerFlags};

/// Represents the high-level category or domain of a fix scope.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum ScopeCategory {
    Array,
    Bracket,
    Colon,
    Comma,
    Escape,
    Key,
    Markdown,
    Misc,
    Quote,
    Structure,
    JsStyle,
}

/// Tracks and controls scoped mutations within a single fixer run,
/// ensuring no overlapping or conflicting fixes happen.
#[derive(Debug)]
pub struct FixScope {
    /// The current input string being fixed.
    pub input: String,

    /// Ranges (start..end) of input already modified/reserved by fix steps.
    touched_ranges: Vec<(usize, usize)>,

    /// Active categories or domains allowed to fix in this scope.
    active_scopes: std::collections::HashSet<ScopeCategory>,

    pub emotion_phase: EmotionPhase,
}

impl FixScope {
    /// Creates a new FixScope for a given input string and initial active scopes.
    pub fn new(input: &str, initial_scopes: &[ScopeCategory], emotion_phase: EmotionPhase) -> Self {
        Self {
            input: input.to_string(),
            touched_ranges: Vec::new(),
            active_scopes: initial_scopes.iter().cloned().collect(),
            emotion_phase,
        }
    }

    /// Checks if a proposed range overlaps with any already touched range.
    pub fn can_apply(&self, start: usize, end: usize) -> bool {
        !self
            .touched_ranges
            .iter()
            .any(|&(s, e)| Self::ranges_overlap(s, e, start, end))
    }

    /// Reserves a range for modification if it doesn't overlap with existing ones.
    /// Returns `true` if the range was successfully reserved; `false` otherwise.
    pub fn reserve_range(&mut self, start: usize, end: usize) -> bool {
        if self.can_apply(start, end) {
            self.touched_ranges.push((start, end));
            true
        } else {
            false
        }
    }

    /// Checks if a given scope category is active for this FixScope.
    pub fn is_active(&self, category: ScopeCategory) -> bool {
        self.active_scopes.contains(&category)
    }

    /// Alias for is_active; used for clarity in fixer modules.
    pub fn allows(&self, category: ScopeCategory) -> bool {
        self.is_active(category)
    }

    /// Activates a new scope category.
    pub fn activate_scope(&mut self, category: ScopeCategory) {
        self.active_scopes.insert(category);
    }

    /// Deactivates a scope category.
    pub fn deactivate_scope(&mut self, category: ScopeCategory) {
        self.active_scopes.remove(&category);
    }

    /// Returns a list of all active scope categories.
    pub fn categories(&self) -> Vec<ScopeCategory> {
        self.active_scopes.iter().cloned().collect()
    }

    /// Utility function to check if two ranges overlap.
    fn ranges_overlap(s1: usize, e1: usize, s2: usize, e2: usize) -> bool {
        !(e1 <= s2 || e2 <= s1)
    }

    pub fn should_apply(&self, diag: &FixDiagnostic, flags: &FixerFlags) -> bool {
        if !diag.enabled {
            return false;
        }
        if diag.tags.contains(&"aggressive".to_string()) && !flags.allow_aggressive {
            return false;
        }
        if !self.is_active(diag.kind.into()) {
            return false;
        }
        if self.emotion_phase == EmotionPhase::Frozen && !diag.tags.contains(&"safe".to_string()) {
            return false;
        }
        true
    }
}
