//!
//! ## Error types.
//!

pub mod blocks;
pub mod expected;
pub mod expr;
pub mod external;
pub mod html_ref;
pub mod identifier;
pub mod number;
pub mod string;
pub mod tag;

use ariadne::{Color, ColorGenerator, Fmt, Label, ReportKind};
use avpony_macros::{Errors, Spanned};
use blocks::UnreachableBranch;
use chumsky::util::MaybeRef;
use expected::Expected;
use expr::ExpectedExpr;
use external::typescript::TSError;
use html_ref::*;
use identifier::*;
use number::*;
use string::*;
use tag::*;

use super::{PonyInput, Span};

pub trait ErrorI: Sized + super::Spanned + PartialEq {
    fn to_report(self) -> ariadne::Report<'static, super::Span>;
}

#[Errors]
pub enum Error {
    UnexpectedToken(UnexpectedToken),
    TSError(TSError),
    InvalidInt(InvalidInt),
    MultipleNumericDividers(MultipleNumericDividers),
    DivdersBadlyPlaced(DivdersBadlyPlaced),
    ReservedIdentifier(ReservedIdentifier),
    InvalidUnicodeCodePoint(InvalidUnicodeCodePoint),
    InvalidAsciiCode(InvalidAsciiCode),
    InvalidEscapeSequence(InvalidEscapeSequence),
    SoloExprOnly(SoloExprOnly),
    ExpectedExpr(ExpectedExpr),
    InvalidEntityName(InvalidEntityName),
    UnclosedTag(UnclosedTag),
    Expected(Expected),
    UnreachableBranch(UnreachableBranch),
}

impl<'src> chumsky::error::Error<'src, PonyInput<'src>> for Error {
    fn expected_found<E: IntoIterator<Item = Option<MaybeRef<'src, char>>>>(
        expected: E,
        found: Option<MaybeRef<'src, char>>,
        span: Span,
    ) -> Self {
        Self::UnexpectedToken(UnexpectedToken {
            span,
            expected: expected
                .into_iter()
                .map(|maybe| maybe.map(MaybeRef::into_inner))
                .collect(),
            found: found.map(MaybeRef::into_inner),
        })
    }
}

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct UnexpectedToken {
    span: Span,
    expected: Vec<Option<char>>,
    found: Option<char>,
}

impl From<UnexpectedToken> for Error {
    fn from(val: UnexpectedToken) -> Self {
        Error::UnexpectedToken(val)
    }
}

impl ErrorI for UnexpectedToken {
    fn to_report(self) -> ariadne::Report<'static, super::Span> {
        let mut colors = ColorGenerator::new();

        let color = colors.next();
        let blue = Color::Blue;

        let found = self
            .found
            .map(|c| c.to_string())
            .unwrap_or("<EOF>".to_string())
            .fg(color);

        let expected = self
            .expected
            .into_iter()
            .map(|ch| {
                format!(
                    "`{}`",
                    ch.map(|c| c.to_string())
                        .unwrap_or("<EOF>".to_string())
                        .fg(blue)
                )
            })
            .intersperse(", ".to_string())
            .collect::<String>();

        self.span
            .clone()
            .build_report(ReportKind::Error)
            .with_code("S000")
            .with_message("Unexpected Token")
            .with_label(
                Label::new(self.span)
                    .with_color(color)
                    .with_message(format!("Found `{found}`, expected {expected}.")),
            )
            .finish()
    }
}
