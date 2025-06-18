// src/fixers/markdown/subfixes.rs

use crate::types::{emotion_phase::EmotionPhase, fix_step::FixStep, fixer_context::FixContext};
use crate::utils::regex_utils::{RE_MARKDOWN_JSON_BLOCK, RE_MARKDOWN_WRAPPER};
use crate::utils::soulfixer_utils::apply_fix;

pub struct SubMarkdownFixer;

impl SubMarkdownFixer {
    /// Removes common Markdown wrappers like code fences (```json ... ```)
    pub fn remove_markdown_wrappers(ctx: &mut FixContext) -> String {
        if ctx.emotion_phase == EmotionPhase::Frozen {
            ctx.whisper("🥶 EmotionPhase is Frozen. Skipping remove_markdown_wrappers.");
            return ctx.input.to_string();
        }

        apply_fix(
            ctx,
            &RE_MARKDOWN_WRAPPER,
            "$1",
            FixStep::MarkdownWrapperRemoved,
            "Removed Markdown code wrapper",
        )
    }

    /// Attempts to extract JSON blocks embedded in Markdown text
    pub fn extract_json_blocks(ctx: &mut FixContext) -> String {
        if ctx.emotion_phase == EmotionPhase::Frozen {
            ctx.whisper("🥶 EmotionPhase is Frozen. Skipping extract_json_blocks.");
            return ctx.input.to_string();
        }

        apply_fix(
            ctx,
            &RE_MARKDOWN_JSON_BLOCK,
            "$1",
            FixStep::MarkdownJsonExtracted,
            "Extracted JSON block from Markdown",
        )
    }
}
