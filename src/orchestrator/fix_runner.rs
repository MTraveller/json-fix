// src/orchestrator/fix_runner.rs

use crate::orchestrator::{diag_commander::DiagCommander, fix_commander::FixCommander};
use crate::types::diagnostic_core::FixDiagnostic;
use crate::types::{
    emotion_phase::EmotionPhase, fix_report::FixReport, fix_scope::FixScope,
    fixer_context::FixContext,
};

/// Entry point for orchestrating the full fix flow using dynamic scope-based logic.
pub fn orchestrate_fix_flow(input: &str) -> FixReport {
    let diagnostics: FixDiagnostic = DiagCommander::run_all(&input); // 🔍 1. Analyze
    let emotion_phase = EmotionPhase::Ready;
    let scope_categories = DiagCommander::map(&diagnostics); // 🎯 2. Map categories
    let scope = FixScope::new(input, &scope_categories); // 🛡 3. Define allowed scope
    let mut ctx = FixContext::new(input, diagnostics.clone(), emotion_phase);

    let outcome = FixCommander::run_fixers(&mut ctx, &scope); // 🧠 4. Run scope-matched fixers

    FixReport {
        original: input.to_string(),
        fixed: ctx.input,
        diagnostics,
        steps: ctx.steps.clone(),
        fixer_outcomes: vec![outcome],
    }
}
