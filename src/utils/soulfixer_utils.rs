use crate::types::{fix_step::FixStep, fixer_context::FixContext};
use crate::utils::regex_loader::{FANCY_REGEX_MAP, REGEX_MAP};

/// Applies a regex fix using a key, with whisper-aware logging and emotional sensitivity.
pub fn apply_fix(
    ctx: &mut FixContext,
    regex_key: &str,
    replacement: &str,
    step: FixStep,
    reason: &str,
) -> String {
    if let Some(regex) = REGEX_MAP.get(regex_key) {
        let result = regex.replace_all(&ctx.input, replacement).to_string();

        if result != ctx.input {
            ctx.steps.push(step);
            ctx.whisper(&format!("🔧 Applied {:?}: {}", step, reason));
        } else {
            ctx.whisper(&format!("↪️ Skipped {:?} — no match found.", step));
        }

        return result;
    }

    if let Some(regex) = FANCY_REGEX_MAP.get(regex_key) {
        match regex.replace_all(&ctx.input, replacement).to_string() {
            result if result != ctx.input => {
                ctx.steps.push(step);
                ctx.whisper(&format!("🔧 Applied {:?} (fancy): {}", step, reason));
                result
            }
            _ => {
                ctx.whisper(&format!("↪️ Skipped {:?} — no match found (fancy).", step));
                ctx.input.clone()
            }
        }
    } else {
        ctx.whisper(&format!("❌ Regex key not found: {}", regex_key));
        ctx.input.clone()
    }
}
