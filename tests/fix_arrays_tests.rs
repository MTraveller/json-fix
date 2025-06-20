use json_fix::diagnostics::analyze_all_diagnostics;
use json_fix::fixers::array::fixer::ArrayFixer;
use json_fix::types::{emotion_phase::EmotionPhase, fixer_context::FixContext};

#[test]
fn test_fix_trailing_comma_in_array() {
    let input = "[1, 2, 3, ]";
    let diagnostics = analyze_all_diagnostics(input);
    let mut ctx = FixContext::new(input, diagnostics.clone(), EmotionPhase::Ready);
    ArrayFixer::apply(&mut ctx);
    assert_eq!(ctx.input, "[1, 2, 3]");
}

#[test]
fn test_fix_malformed_nested_arrays() {
    let input = "[[1, 2], [3, 4,]]";
    let diagnostics = analyze_all_diagnostics(input);
    let mut ctx = FixContext::new(input, diagnostics.clone(), EmotionPhase::Ready);
    ArrayFixer::apply(&mut ctx);
    assert_eq!(ctx.input, "[[1, 2], [3, 4]]");
}

#[test]
fn test_fix_empty_array_slots() {
    let input = "[1,,2,,3]";
    let diagnostics = analyze_all_diagnostics(input);
    let mut ctx = FixContext::new(input, diagnostics.clone(), EmotionPhase::Ready);
    ArrayFixer::apply(&mut ctx);
    assert_eq!(ctx.input, "[1,,2,,3]");
}

#[test]
fn test_fix_array_string_misalignment() {
    let input = r#"["item1""item2", "item3"]"#;
    let diagnostics = analyze_all_diagnostics(input);
    let mut ctx = FixContext::new(input, diagnostics.clone(), EmotionPhase::Ready);
    ArrayFixer::apply(&mut ctx);
    assert_eq!(ctx.input, r#"["item1""item2", "item3"]"#);
}

#[test]
fn test_fix_leading_comma_in_array() {
    let input = "[, 1, 2]";
    let diagnostics = analyze_all_diagnostics(input);
    let mut ctx = FixContext::new(input, diagnostics.clone(), EmotionPhase::Ready);
    ArrayFixer::apply(&mut ctx);
    assert_eq!(ctx.input, "[1, 2]");
}
