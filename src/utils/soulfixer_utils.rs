use crate::types::{fix_step::FixStep, fixer_context::FixContext};
use regex::Regex;

/// Applies a regex fix with whisper-aware logging and emotional sensitivity.
pub fn apply_fix(
    ctx: &mut FixContext,
    regex: &Regex,
    replacement: &str,
    step: FixStep,
    reason: &str,
) -> String {
    let result = regex.replace_all(&ctx.input, replacement).to_string();

    if result != ctx.input {
        ctx.steps.push(step);
        ctx.whisper(&format!("🔧 Applied {:?}: {}", step, reason));
    } else {
        ctx.whisper(&format!("↪️ Skipped {:?} — no match found.", step));
    }

    result
}
