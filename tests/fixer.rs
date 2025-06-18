use json_fix::diagnostics::analyze_all_diagnostics;
use json_fix::fixers::comma::fixer::CommaFixer;
use json_fix::types::emotion_phase::EmotionPhase;
use json_fix::types::fixer_context::FixContext;

#[test]
fn test_fix_double_commas() {
    let input = r#"[1,, 2]"#;
    let diagnostics = analyze_all_diagnostics(input);
    let emotion_phase = EmotionPhase::Ready;
    let mut ctx = FixContext::new(input, diagnostics.clone(), emotion_phase);

    CommaFixer::apply(&mut ctx);

    assert_eq!(ctx.input, "[1, 2]");
}

#[test]
fn test_fix_trailing_comma_in_array() {
    let input = r#"[1, 2, ]"#;
    let diagnostics = analyze_all_diagnostics(input);
    let emotion_phase = EmotionPhase::Ready;
    let mut ctx = FixContext::new(input, diagnostics.clone(), emotion_phase);

    CommaFixer::apply(&mut ctx);

    assert_eq!(ctx.input, "[1, 2]");
}

#[test]
fn test_fix_missing_commas_between_pairs() {
    let input = r#"{"a": 1 "b": 2}"#;
    let diagnostics = analyze_all_diagnostics(input);
    let emotion_phase = EmotionPhase::Ready;
    let mut ctx = FixContext::new(input, diagnostics.clone(), emotion_phase);

    CommaFixer::apply(&mut ctx);

    assert_eq!(ctx.input, r#"{"a": 1, "b": 2}"#);
}

#[test]
fn test_fix_stray_comma_after_opening() {
    let input = r#"[,"a", "b"]"#;
    let diagnostics = analyze_all_diagnostics(input);
    let emotion_phase = EmotionPhase::Ready;
    let mut ctx = FixContext::new(input, diagnostics.clone(), emotion_phase);

    CommaFixer::apply(&mut ctx);

    assert_eq!(ctx.input, r#"["a", "b"]"#);
}
