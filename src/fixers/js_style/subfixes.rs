// src/fixers/js_style/subfixes.rs

use crate::types::fix_scope::{FixScope, ScopeCategory};
use crate::types::{emotion_phase::EmotionPhase, fix_step::FixStep, fixer_context::FixContext};
use crate::utils::soulfixer_utils::apply_fix;

pub struct SubJsStyleFixer;

impl SubJsStyleFixer {
    /// Converts `undefined` → `null`
    pub fn fix_undefined(ctx: &mut FixContext, scope: &mut FixScope) -> String {
        if ctx.emotion_phase == EmotionPhase::Frozen {
            ctx.whisper("🥶 EmotionPhase is Frozen. Skipping fix_undefined.");
            return ctx.input.to_string();
        }

        if !scope.allows(ScopeCategory::JsStyle) {
            ctx.whisper("FixScope excludes JsStyle: skipping fix_undefined.");
            return ctx.input.to_string();
        }

        apply_fix(
            ctx,
            "RE_UNDEFINED",
            "null",
            FixStep::JsUndefinedReplaced,
            "Replaced `undefined` with `null`",
        )
    }

    /// Converts `NaN` or `Infinity` → `null`
    pub fn fix_nan(ctx: &mut FixContext, scope: &mut FixScope) -> String {
        if ctx.emotion_phase == EmotionPhase::Frozen {
            ctx.whisper("🥶 EmotionPhase is Frozen. Skipping fix_nan.");
            return ctx.input.to_string();
        }

        if !scope.allows(ScopeCategory::JsStyle) {
            ctx.whisper("FixScope excludes JsStyle: skipping fix_nan.");
            return ctx.input.to_string();
        }

        apply_fix(
            ctx,
            "RE_NAN_INFINITY",
            "null",
            FixStep::JsNaNReplaced,
            "Replaced `NaN` and `Infinity` with `null`",
        )
    }

    /// Removes JS-style comments
    pub fn remove_js_comments(ctx: &mut FixContext, scope: &mut FixScope) -> String {
        if ctx.emotion_phase == EmotionPhase::Frozen {
            ctx.whisper("🥶 EmotionPhase is Frozen. Skipping remove_js_comments.");
            return ctx.input.to_string();
        }

        if !scope.allows(ScopeCategory::JsStyle) {
            ctx.whisper("FixScope excludes JsStyle: skipping remove_js_comments.");
            return ctx.input.to_string();
        }

        apply_fix(
            ctx,
            "RE_JS_COMMENTS",
            "",
            FixStep::JsCommentsRemoved,
            "Removed JavaScript-style comments",
        )
    }
}
