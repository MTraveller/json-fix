use crate::generated::regex_registry::REGEX_REGISTRY;
use crate::types::fixer_context::FixContext;

/// Applies a regex fix using a key, with whisper-aware logging and emotional sensitivity.
pub fn apply_fix(ctx: &mut FixContext, regex_key: &str, replacement: &str) -> String {
    if let Some(entry) = REGEX_REGISTRY.get(regex_key) {
        let result = entry.regex.replace_all(&ctx.input, replacement).to_string();

        if result != ctx.input {
            ctx.steps.push(entry.fix_step.clone());
            ctx.whisper(&format!(
                "🔧 Applied {}: {}",
                entry.fix_step.id, entry.reason
            ));
        } else {
            ctx.whisper(&format!(
                "↪️ Skipped {} — no match found.",
                entry.fix_step.id
            ));
        }

        result
    } else {
        ctx.whisper(&format!(
            "❌ Regex key not found in registry: {}",
            regex_key
        ));
        ctx.input.clone()
    }
}
