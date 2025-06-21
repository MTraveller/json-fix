use json_fix::diagnostics::analyze_all_diagnostics;
use json_fix::fixers::key::fixer::KeysFixer;
use json_fix::types::{emotion_phase::EmotionPhase, fixer_context::FixContext};

#[test]
fn test_fix_unquoted_keys() {
    let input = "{name: \"John\", age: 30}";
    let diagnostics = analyze_all_diagnostics(input);
    let mut ctx = FixContext::new(input, diagnostics.clone(), EmotionPhase::Ready);
    KeysFixer::apply(&mut ctx);
    assert_eq!(ctx.input, "{\"name\": \"John\", \"age\": 30}");
}

#[test]
fn test_fix_key_traps() {
    let input = r#"{ emotion: , "hopeful" }"#;
    let diagnostics = analyze_all_diagnostics(input);
    let mut ctx = FixContext::new(input, diagnostics.clone(), EmotionPhase::Ready);
    KeysFixer::apply(&mut ctx);
    assert_eq!(ctx.input, r#"{ "null": , "hopeful" }"#);
}
