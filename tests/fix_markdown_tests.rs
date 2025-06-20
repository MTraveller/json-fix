use json_fix::diagnostics::analyze_all_diagnostics;
use json_fix::fixers::markdown::fixer::MarkdownFixer;
use json_fix::types::{emotion_phase::EmotionPhase, fixer_context::FixContext};

#[test]
fn test_fix_markdown_escaped_headers() {
    let input = "#\\ Heading One\\n##\\ Heading Two";
    let diagnostics = analyze_all_diagnostics(input);
    let mut ctx = FixContext::new(input, diagnostics.clone(), EmotionPhase::Ready);
    MarkdownFixer::apply(&mut ctx);
    assert_eq!(ctx.input, "# Heading One\n## Heading Two");
}

#[test]
fn test_fix_markdown_wrapper_removal() {
    let input = "```json\n{\"a\":1}\n```";
    let diagnostics = analyze_all_diagnostics(input);
    let mut ctx = FixContext::new(input, diagnostics.clone(), EmotionPhase::Ready);
    MarkdownFixer::apply(&mut ctx);
    assert_eq!(ctx.input, "{\"a\":1}");
}
