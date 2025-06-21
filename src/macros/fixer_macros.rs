// 💡 AUTO-GENERATED FILE — DO NOT EDIT
// ⚙️  To regenerate: cargo run --bin fixer_macro_codegen

#![allow(clippy::all)]

#[macro_export]
macro_rules! misc_fixer {
	($name:ident, $method:ident) => {
		pub struct $name;

		impl Default for $name {
			fn default() -> Self {
				Self
			}
		}

		impl crate::fixers::Fixer for $name {
			fn apply_all(
				&mut self,
				ctx: &mut crate::types::fixer_context::FixContext,
			) -> crate::types::fix_outcome::FixOutcome {
				crate::types::fix_outcome::FixOutcome::from(
					crate::fixers::misc::subfixes::SubMiscFixer::$method(ctx),
				)
			}
		}
	};
}

#[macro_export]
macro_rules! escape_fixer {
	($name:ident, $method:ident) => {
		pub struct $name;

		impl Default for $name {
			fn default() -> Self {
				Self
			}
		}

		impl crate::fixers::Fixer for $name {
			fn apply_all(
				&mut self,
				ctx: &mut crate::types::fixer_context::FixContext,
			) -> crate::types::fix_outcome::FixOutcome {
				crate::types::fix_outcome::FixOutcome::from(
					crate::fixers::escape::subfixes::SubEscapeFixer::$method(ctx),
				)
			}
		}
	};
}

#[macro_export]
macro_rules! js_style_fixer {
	($name:ident, $method:ident) => {
		pub struct $name;

		impl Default for $name {
			fn default() -> Self {
				Self
			}
		}

		impl crate::fixers::Fixer for $name {
			fn apply_all(
				&mut self,
				ctx: &mut crate::types::fixer_context::FixContext,
			) -> crate::types::fix_outcome::FixOutcome {
				crate::types::fix_outcome::FixOutcome::from(
					crate::fixers::js_style::subfixes::SubJsStyleFixer::$method(ctx),
				)
			}
		}
	};
}

#[macro_export]
macro_rules! array_fixer {
	($name:ident, $method:ident) => {
		pub struct $name;

		impl Default for $name {
			fn default() -> Self {
				Self
			}
		}

		impl crate::fixers::Fixer for $name {
			fn apply_all(
				&mut self,
				ctx: &mut crate::types::fixer_context::FixContext,
			) -> crate::types::fix_outcome::FixOutcome {
				crate::types::fix_outcome::FixOutcome::from(
					crate::fixers::array::subfixes::SubArrayFixer::$method(ctx),
				)
			}
		}
	};
}

#[macro_export]
macro_rules! quote_fixer {
	($name:ident, $method:ident) => {
		pub struct $name;

		impl Default for $name {
			fn default() -> Self {
				Self
			}
		}

		impl crate::fixers::Fixer for $name {
			fn apply_all(
				&mut self,
				ctx: &mut crate::types::fixer_context::FixContext,
			) -> crate::types::fix_outcome::FixOutcome {
				crate::types::fix_outcome::FixOutcome::from(
					crate::fixers::quote::subfixes::SubQuoteFixer::$method(ctx),
				)
			}
		}
	};
}

#[macro_export]
macro_rules! markdown_fixer {
	($name:ident, $method:ident) => {
		pub struct $name;

		impl Default for $name {
			fn default() -> Self {
				Self
			}
		}

		impl crate::fixers::Fixer for $name {
			fn apply_all(
				&mut self,
				ctx: &mut crate::types::fixer_context::FixContext,
			) -> crate::types::fix_outcome::FixOutcome {
				crate::types::fix_outcome::FixOutcome::from(
					crate::fixers::markdown::subfixes::SubMarkdownFixer::$method(ctx),
				)
			}
		}
	};
}

#[macro_export]
macro_rules! colon_fixer {
	($name:ident, $method:ident) => {
		pub struct $name;

		impl Default for $name {
			fn default() -> Self {
				Self
			}
		}

		impl crate::fixers::Fixer for $name {
			fn apply_all(
				&mut self,
				ctx: &mut crate::types::fixer_context::FixContext,
			) -> crate::types::fix_outcome::FixOutcome {
				crate::types::fix_outcome::FixOutcome::from(
					crate::fixers::colon::subfixes::SubColonFixer::$method(ctx),
				)
			}
		}
	};
}

#[macro_export]
macro_rules! comma_fixer {
	($name:ident, $method:ident) => {
		pub struct $name;

		impl Default for $name {
			fn default() -> Self {
				Self
			}
		}

		impl crate::fixers::Fixer for $name {
			fn apply_all(
				&mut self,
				ctx: &mut crate::types::fixer_context::FixContext,
			) -> crate::types::fix_outcome::FixOutcome {
				crate::types::fix_outcome::FixOutcome::from(
					crate::fixers::comma::subfixes::SubCommaFixer::$method(ctx),
				)
			}
		}
	};
}

#[macro_export]
macro_rules! key_fixer {
	($name:ident, $method:ident) => {
		pub struct $name;

		impl Default for $name {
			fn default() -> Self {
				Self
			}
		}

		impl crate::fixers::Fixer for $name {
			fn apply_all(
				&mut self,
				ctx: &mut crate::types::fixer_context::FixContext,
			) -> crate::types::fix_outcome::FixOutcome {
				crate::types::fix_outcome::FixOutcome::from(
					crate::fixers::key::subfixes::SubKeyFixer::$method(ctx),
				)
			}
		}
	};
}

#[macro_export]
macro_rules! bracket_fixer {
	($name:ident, $method:ident) => {
		pub struct $name;

		impl Default for $name {
			fn default() -> Self {
				Self
			}
		}

		impl crate::fixers::Fixer for $name {
			fn apply_all(
				&mut self,
				ctx: &mut crate::types::fixer_context::FixContext,
			) -> crate::types::fix_outcome::FixOutcome {
				crate::types::fix_outcome::FixOutcome::from(
					crate::fixers::bracket::subfixes::SubBracketFixer::$method(ctx),
				)
			}
		}
	};
}

#[macro_export]
macro_rules! structure_fixer {
	($name:ident, $method:ident) => {
		pub struct $name;

		impl Default for $name {
			fn default() -> Self {
				Self
			}
		}

		impl crate::fixers::Fixer for $name {
			fn apply_all(
				&mut self,
				ctx: &mut crate::types::fixer_context::FixContext,
			) -> crate::types::fix_outcome::FixOutcome {
				crate::types::fix_outcome::FixOutcome::from(
					crate::fixers::structure::subfixes::SubStructureFixer::$method(ctx),
				)
			}
		}
	};
}

