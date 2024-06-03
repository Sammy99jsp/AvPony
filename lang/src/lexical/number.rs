//!
//! ## Number Literals
//!
//! ### Grammar
//! ```text
//! digits := `0` | `1` | `2` | `3` | `4` | `5` | `6` | `7` | `8` | `9` | `_` (where next != `_`)
//!
//! integer_lit := `-`? digits+
//! float_lit := `-`? integer_lit `.` digits*
//!
//! number_lit := integer_lit | float_lit
//! ```
//!
//! ### Examples
//!
//! ```ts
//! 0;
//! 1;
//! 1.;
//! 0.2;
//! .1; // <- Not allowed -- all floats < 1 must have a leading unit 0.
//! 0.0_1;
//! 0.0__2; // <- Not allowed -- only one numeric separator.
//! ```
//!

use std::sync::OnceLock;

use avpony_macros::Spanned;
use chumsky::{
    primitive::{filter, just},
    Parser,
};
use regex::Regex;

use crate::utils::{
    errors::{
        number::{DivdersBadlyPlaced, InvalidInt, MultipleNumericDividers},
        Error,
    },
    ParseableExt, Span,
};

pub type IntType = i32;
pub type FloatType = f64;

#[derive(Debug, Clone, Spanned, PartialEq)]
pub enum NumberLit {
    Integer(IntegerLit),
    Float(FloatLit),
}

static MULTIPLE_NUMERIC_DIVIDERS: OnceLock<Regex> = OnceLock::new();
static BADLY_PLACED_NUMERIC_DIVIDERS: OnceLock<Regex> = OnceLock::new();

impl ParseableExt for NumberLit {
    fn parser() -> impl crate::utils::PonyParser<Self> + Clone {
        let divider_train = MULTIPLE_NUMERIC_DIVIDERS.get_or_init(|| Regex::new(r"_(_+)").unwrap());
        let dividers_badly_placed = BADLY_PLACED_NUMERIC_DIVIDERS
            .get_or_init(|| Regex::new(r"(^_)|(-_)|(_\.)|(\._)|(_$)").unwrap());

        let digits = filter(|ch: &char| ch.is_ascii_digit() || ch == &'_').repeated();

        just("-")
            .map(|_| '-')
            .or_not()
            .then(digits.at_least(1))
            .then(just(".").map(|_| '.').chain(digits).or_not())
            .try_map(|((minus, int), mantissa), span: Span| {
                let is_float = mantissa.is_some();

                let raw_value = minus
                    .into_iter()
                    .chain(int)
                    .chain(mantissa.into_iter().flatten())
                    .collect::<String>();

                // Check for presence of strings of `_`-s
                if let Some(needle) = divider_train.captures(&raw_value) {
                    let span = span.relative_range(needle.get(1).unwrap().range());
                    return Err(Error::MultipleNumericDividers(
                        MultipleNumericDividers::new(span),
                    ));
                }

                if let Some(needle) = dividers_badly_placed.captures(&raw_value) {
                    let span = span.relative_range(needle.get(0).unwrap().range());
                    return Err(Error::DivdersBadlyPlaced(DivdersBadlyPlaced::new(span)));
                }

                let without_underscores = raw_value.replace("_", "");

                if is_float {
                    Ok(without_underscores
                        .parse()
                        .map(|value| {
                            Self::Float(FloatLit {
                                span: span.clone(),
                                value,
                                raw_value,
                            })
                        })
                        .unwrap())
                    // Unwrap, since that ParseFloatError only covers invalid digits, or empty,
                    // which is already covered in parsing!
                } else {
                    without_underscores
                        .parse()
                        .map(|value| {
                            Self::Integer(IntegerLit {
                                span: span.clone(),
                                value,
                                raw_value,
                            })
                        })
                        .map_err(|internal| InvalidInt::from_internal(span, internal))
                        .map_err(Into::into)
                }
            })
    }
}

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct IntegerLit {
    pub span: Span,
    pub value: IntType,
    raw_value: String,
}

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct FloatLit {
    pub span: Span,
    pub value: FloatType,
    raw_value: String,
}

#[cfg(test)]
mod tests {
    use chumsky::Parser;

    use super::NumberLit;
    use crate::{
        lexical::number::{FloatLit, IntegerLit},
        utils::{stream::SourceFile, Parseable},
    };

    #[test]
    fn parsing() {
        let (source, _) = SourceFile::test_file("122");
        assert!(matches!(
            NumberLit::parser().parse(source.stream()),
            Ok(NumberLit::Integer(IntegerLit { value: 122, .. }))
        ));

        let (source, _) = SourceFile::test_file("-122");
        assert!(matches!(
            NumberLit::parser().parse(source.stream()),
            Ok(NumberLit::Integer(IntegerLit { value: -122, .. }))
        ));

        let (source, _) = SourceFile::test_file("1.22");
        assert!(matches!(
            NumberLit::parser().parse(source.stream()),
            Ok(NumberLit::Float(FloatLit { value: 1.22, .. }))
        ));

        let (source, _) = SourceFile::test_file("-1.22");
        assert!(matches!(
            NumberLit::parser().parse(source.stream()),
            Ok(NumberLit::Float(FloatLit { value: -1.22, .. }))
        ));

        let (source, _) = SourceFile::test_file("1_22");
        assert!(matches!(
            NumberLit::parser().parse(source.stream()),
            Ok(NumberLit::Integer(IntegerLit { value: 122, .. }))
        ));

        let (source, _) = SourceFile::test_file("-1_22");
        assert!(matches!(
            NumberLit::parser().parse(source.stream()),
            Ok(NumberLit::Integer(IntegerLit { value: -122, .. }))
        ));

        let (source, _) = SourceFile::test_file("1.2_2");
        assert!(matches!(
            NumberLit::parser().parse(source.stream()),
            Ok(NumberLit::Float(FloatLit { value: 1.22, .. }))
        ));

        let (source, _) = SourceFile::test_file("-1.2_2");
        assert!(matches!(
            NumberLit::parser().parse(source.stream()),
            Ok(NumberLit::Float(FloatLit { value: -1.22, .. }))
        ));
    }

    #[test]
    fn invalid_parsing() {
        let (source, _) = SourceFile::test_file("_122");
        assert!(NumberLit::parser().parse(source.stream()).is_err());

        let (source, _) = SourceFile::test_file("-_122");
        assert!(NumberLit::parser().parse(source.stream()).is_err());

        let (source, _) = SourceFile::test_file("1_.22");
        assert!(NumberLit::parser().parse(source.stream()).is_err());

        let (source, _) = SourceFile::test_file("-1._22");
        assert!(NumberLit::parser().parse(source.stream()).is_err());

        let (source, _) = SourceFile::test_file("122_");
        assert!(NumberLit::parser().parse(source.stream()).is_err());
    }
}
