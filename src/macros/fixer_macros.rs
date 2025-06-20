// src/macros/fixer_macros.rs

#[macro_export]
macro_rules! array_fixer {
    ($name:ident, $method:ident) => {
        pub struct $name;

        impl Default for $name {
            fn default() -> Self {
                Self
            }
        }

        impl crate::fixer::Fixer for $name {
            fn apply_all(
                &mut self,
                ctx: &mut crate::types::fixer_context::FixContext,
            ) -> crate::types::fix_outcome::FixOutcome {
                crate::fixers::array::subfixes::SubArrayFixer::$method(ctx)
            }
        }
    };
}
