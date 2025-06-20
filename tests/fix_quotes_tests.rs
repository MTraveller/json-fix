use json_fix::diagnostics::analyze_all_diagnostics;
use json_fix::fixers::quote::fixer::QuoteFixer;
use json_fix::types::{emotion_phase::EmotionPhase, fixer_context::FixContext};

#[test]
fn test_fix_unquoted_keys() {
    let input = r#"{name: "Alice", age: 30}"#;
    let diagnostics = analyze_all_diagnostics(input);
    let mut ctx = FixContext::new(input, diagnostics.clone(), EmotionPhase::Ready);
    QuoteFixer::apply(&mut ctx);
    assert_eq!(ctx.input, r#"{"name": "Alice", "age": 30}"#);
}

#[test]
fn test_fix_single_quotes_to_double() {
    let input = r#"{'name': 'Bob'}"#;
    let diagnostics = analyze_all_diagnostics(input);
    let mut ctx = FixContext::new(input, diagnostics.clone(), EmotionPhase::Ready);
    QuoteFixer::apply(&mut ctx);
    assert_eq!(ctx.input, r#"{"name": "Bob"}"#);
}
