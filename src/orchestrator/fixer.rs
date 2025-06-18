// src/orchestrator/fixer.rs

use crate::diagnostics::{analyze_all_diagnostics, FixDiagnostics};
use crate::fixers::{
    arrays::fixer::ArrayFixer, brackets::fixer::BracketFixer, comma::fixer::CommaFixer,
    escape::fixer::EscapeFixer, keys::fixer::KeysFixer, markdown::fixer::MarkdownFixer,
    misc::fixer::MiscFixer, quotes::fixer::QuoteFixer,
};
use crate::types::emotion_phase::EmotionPhase;
use crate::types::{fix_report::FixReport, fixer_context::FixContext};

pub struct FixOrchestrator;

impl FixOrchestrator {
    pub fn apply_all(input: &str) -> FixReport {
        let diagnostics: FixDiagnostics = analyze_all_diagnostics(input);
        let emotion_phase = EmotionPhase::Ready;
        let mut ctx = FixContext::new(input, diagnostics.clone(), emotion_phase);

        {
            let ctx_mut = &mut ctx;
            Self::apply_array_fixes(ctx_mut, &diagnostics);
            Self::apply_bracket_fixes(ctx_mut, &diagnostics);
            Self::apply_comma_fixes(ctx_mut, &diagnostics);
            Self::apply_escape_fixes(ctx_mut, &diagnostics);
            Self::apply_key_fixes(ctx_mut, &diagnostics);
            Self::apply_markdown_fixes(ctx_mut, &diagnostics);
            Self::apply_quote_fixes(ctx_mut, &diagnostics);
            Self::apply_misc_fixes(ctx_mut, &diagnostics);
        }

        FixReport {
            original: input.to_string(),
            fixed: ctx.input,
            diagnostics,
            steps: ctx.steps.clone(),
        }
    }

    fn apply_array_fixes<'a>(ctx: &mut FixContext, diagnostics: &FixDiagnostics) {
        if diagnostics.has_array_issues {
            ArrayFixer::apply(ctx);
        }
    }

    fn apply_bracket_fixes<'a>(ctx: &mut FixContext, diagnostics: &FixDiagnostics) {
        if diagnostics.has_bracket_issues {
            BracketFixer::apply(ctx);
        }
    }

    fn apply_comma_fixes<'a>(ctx: &mut FixContext, diagnostics: &FixDiagnostics) {
        if diagnostics.has_missing_commas {
            CommaFixer::apply(ctx);
        }
    }

    fn apply_escape_fixes<'a>(ctx: &mut FixContext, diagnostics: &FixDiagnostics) {
        if diagnostics.has_escape_issues {
            EscapeFixer::apply(ctx);
        }
    }

    fn apply_key_fixes<'a>(ctx: &mut FixContext, diagnostics: &FixDiagnostics) {
        if diagnostics.has_key_errors {
            KeysFixer::apply(ctx);
        }
    }

    fn apply_markdown_fixes<'a>(ctx: &mut FixContext, diagnostics: &FixDiagnostics) {
        if diagnostics.has_markdown_wrappers {
            MarkdownFixer::apply(ctx);
        }
    }

    fn apply_quote_fixes<'a>(ctx: &mut FixContext, diagnostics: &FixDiagnostics) {
        if diagnostics.has_quote_issues {
            QuoteFixer::apply(ctx);
        }
    }

    fn apply_misc_fixes<'a>(ctx: &mut FixContext, diagnostics: &FixDiagnostics) {
        if diagnostics.has_misc_issues {
            MiscFixer::apply(ctx);
        }
    }
}
