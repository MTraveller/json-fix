mod comma_fixer_tests {
    use json_fix::diagnostics::analyze_all_diagnostics;
    use json_fix::fixers::comma::fixer::CommaFixer;
    use json_fix::types::{emotion_phase::EmotionPhase, fixer_context::FixContext};

    #[test]
    fn test_fix_double_commas() {
        let input = r#"[1,, 2]"#;
        let diagnostics = analyze_all_diagnostics(input);
        let mut ctx = FixContext::new(input, diagnostics.clone(), EmotionPhase::Ready);
        CommaFixer::apply(&mut ctx);
        assert_eq!(ctx.input, "[1, 2]");
    }

    #[test]
    fn test_fix_leading_commas() {
        let input = r#"[ ,1, 2]"#;
        let diagnostics = analyze_all_diagnostics(input);
        let mut ctx = FixContext::new(input, diagnostics.clone(), EmotionPhase::Ready);
        CommaFixer::apply(&mut ctx);
        assert_eq!(ctx.input, "[1, 2]");
    }

    #[test]
    fn test_fix_trailing_commas() {
        let input = r#"[1, 2, ]"#;
        let diagnostics = analyze_all_diagnostics(input);
        let mut ctx = FixContext::new(input, diagnostics.clone(), EmotionPhase::Ready);
        CommaFixer::apply(&mut ctx);
        assert_eq!(ctx.input, "[1, 2]");
    }

    #[test]
    fn test_fix_comma_before_closing_bracket() {
        let input = r#"[1, 2,]"#;
        let diagnostics = analyze_all_diagnostics(input);
        let mut ctx = FixContext::new(input, diagnostics.clone(), EmotionPhase::Ready);
        CommaFixer::apply(&mut ctx);
        assert_eq!(ctx.input, "[1, 2]");
    }
}
