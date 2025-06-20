use json_fix::diagnostics::analyze_all_diagnostics;
use json_fix::fixers::bracket::fixer::BracketFixer;
use json_fix::types::{emotion_phase::EmotionPhase, fixer_context::FixContext};

#[test]
fn test_fix_extra_closing_braces() {
    let input = r#"{"a": 1}}}"#;
    let diagnostics = analyze_all_diagnostics(input);
    let mut ctx = FixContext::new(input, diagnostics.clone(), EmotionPhase::Ready);
    BracketFixer::apply(&mut ctx);
    assert_eq!(ctx.input, r#"{"a": 1}"#);
}

#[test]
fn test_fix_missing_closing_brace() {
    let input = r#"{"a": 1"#;
    let diagnostics = analyze_all_diagnostics(input);
    let mut ctx = FixContext::new(input, diagnostics.clone(), EmotionPhase::Ready);
    BracketFixer::apply(&mut ctx);
    assert_eq!(ctx.input, r#"{"a": 1}"#);
}
