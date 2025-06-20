use json_fix::diagnostics::analyze_all_diagnostics;
use json_fix::fixers::escape::fixer::EscapeFixer;
use json_fix::types::{emotion_phase::EmotionPhase, fixer_context::FixContext};

#[test]
fn test_fix_invalid_escape_sequences() {
    let input = r#"{ "text": "Hello\\x World" }"#;
    let diagnostics = analyze_all_diagnostics(input);
    let mut ctx = FixContext::new(input, diagnostics.clone(), EmotionPhase::Ready);
    EscapeFixer::apply(&mut ctx);
    assert_eq!(ctx.input, r#"{ "text": "Hello World" }"#); // Adjust expected result based on fixer logic
}

#[test]
fn test_fix_broken_unicode_escapes() {
    let input = r#"{ "emoji": "\u12" }"#;
    let diagnostics = analyze_all_diagnostics(input);
    let mut ctx = FixContext::new(input, diagnostics.clone(), EmotionPhase::Ready);
    EscapeFixer::apply(&mut ctx);
    assert_eq!(ctx.input, r#"{ "emoji": "\uFFFD" }"#);
}
