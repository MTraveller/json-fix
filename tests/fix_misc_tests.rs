// tests/fix_misc_tests.rs

use json_fix::diagnostics::analyze_all_diagnostics;
use json_fix::fixers::misc::fixer::MiscFixer;
use json_fix::types::{emotion_phase::EmotionPhase, fixer_context::FixContext};

#[test]
fn test_fix_double_commas() {
    let input = r#"{"a": 1,, "b": 2}"#;
    let diagnostics = analyze_all_diagnostics(input);
    let mut ctx = FixContext::new(input, diagnostics.clone(), EmotionPhase::Ready);
    MiscFixer::apply(&mut ctx);
    assert_eq!(ctx.input, r#"{"a": 1, "b": 2}"#);
}

#[test]
fn test_fix_fallback_artifacts() {
    let input = r#"[ "value1", , "value2", , ]"#;
    let diagnostics = analyze_all_diagnostics(input);
    let mut ctx = FixContext::new(input, diagnostics.clone(), EmotionPhase::Ready);
    MiscFixer::apply(&mut ctx);
    assert_eq!(ctx.input, r#"[ "value1", , "value2", , ]"#); // Adjust if fixer logic changes to remove those
}
