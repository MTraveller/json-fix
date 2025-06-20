// src/utils/souldiag_utils.rs

use crate::types::fixer_context::FixContext;
use crate::utils::regex_loader::{FANCY_REGEX_MAP, REGEX_MAP};

/// Applies a diagnosis check using regex_key, logging the result and updating the flag if matched.
pub fn apply_diagnosis(ctx: &mut FixContext, regex_key: &str, flag: &mut bool, reason: &str) {
    if *flag {
        return;
    }

    if let Some(regex) = REGEX_MAP.get(regex_key) {
        if regex.is_match(&ctx.input) {
            *flag = true;
            ctx.whisper(&format!("🧠 Diagnosed: {}", reason));
        } else {
            ctx.whisper(&format!(
                "↪️ Skipped diagnosis — no match for: {}",
                regex_key
            ));
        }
        return;
    }

    if let Some(regex) = FANCY_REGEX_MAP.get(regex_key) {
        if regex.is_match(&ctx.input).unwrap_or(false) {
            *flag = true;
            ctx.whisper(&format!("🧠 Diagnosed (fancy): {}", reason));
        } else {
            ctx.whisper(&format!(
                "↪️ Skipped diagnosis (fancy) — no match for: {}",
                regex_key
            ));
        }
    } else {
        ctx.whisper(&format!("❌ Diagnosis regex not found: {}", regex_key));
    }
}
