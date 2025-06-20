// src/fixers/brackets/subfixes.rs

use crate::types::{emotion_phase::EmotionPhase, fix_step::FixStep, fixer_context::FixContext};
use crate::utils::soulfixer_utils::apply_fix;

pub struct SubBracketFixer;

impl SubBracketFixer {
    /// Fixes unbalanced brackets by removing obvious extra closing braces.
    pub fn fix_extra_closing_brace(ctx: &mut FixContext) -> String {
        if ctx.emotion_phase == EmotionPhase::Frozen {
            ctx.whisper("Frozen: skipping fix_extra_closing_brace");
            return ctx.input.to_string();
        }
        apply_fix(
            ctx,
            "RE_DOUBLE_CLOSING_BRACE",
            "}",
            FixStep::BracketsExtraRemoved,
            "Removed extra closing brace",
        )
    }

    /// Adds a closing brace if input appears to be missing one at the end.
    pub fn fix_missing_closing_brace(ctx: &mut FixContext) -> String {
        if ctx.emotion_phase == EmotionPhase::Frozen {
            ctx.whisper("Frozen: skipping fix_missing_closing_brace");
            return ctx.input.to_string();
        }
        let open_count = ctx.input.matches('{').count();
        let close_count = ctx.input.matches('}').count();
        if open_count > close_count {
            let new_output = format!("{}{}", ctx.input, "}".repeat(open_count - close_count));
            ctx.record(FixStep::BracketsMissingAdded);
            ctx.whisper("🔧 Added missing closing brace(s)");
            new_output
        } else {
            ctx.input.to_string()
        }
    }

    /// Attempts to balance braces if counts are mismatched in general.
    pub fn fix_unbalanced(ctx: &mut FixContext) -> String {
        if ctx.emotion_phase == EmotionPhase::Frozen {
            ctx.whisper("Frozen: skipping fix_unbalanced");
            return ctx.input.to_string();
        }
        let open_count = ctx.input.matches('{').count();
        let close_count = ctx.input.matches('}').count();
        if open_count < close_count {
            let new_output = ctx.input.trim_end_matches('}').to_string();
            ctx.record(FixStep::BracketsBalanced);
            ctx.whisper("🔧 Balanced mismatched braces");
            new_output
        } else if open_count > close_count {
            let new_output = format!("{}{}", ctx.input, "}".repeat(open_count - close_count));
            ctx.record(FixStep::BracketsBalanced);
            ctx.whisper("🔧 Balanced mismatched braces");
            new_output
        } else {
            ctx.input.to_string()
        }
    }
}
