//!
//! ## Error types.
//!

pub mod html_ref;
pub mod identifier;
pub mod number;
pub mod string;

use ariadne::{Color, ColorGenerator, Fmt, Label};
use avpony_macros::Spanned;
use html_ref::InvalidEntityName;

use self::{
    identifier::ReservedIdentifier,
    number::{DivdersBadlyPlaced, InvalidInt, MultipleNumericDividers},
    string::{InvalidAsciiCode, InvalidEscapeSequence, InvalidUnicodeCodePoint},
};

// TODO: Move the hassle-some implementations to an auto-generating macro.
#[derive(Debug, Clone, Spanned, PartialEq)]
pub enum Error {
    UnexpectedToken(UnexpectedToken),
    InvalidInt(InvalidInt),
    MultipleNumericDividers(MultipleNumericDividers),
    DivdersBadlyPlaced(DivdersBadlyPlaced),
    InvalidUnicodeCodePoint(InvalidUnicodeCodePoint),
    InvalidAsciiCode(InvalidAsciiCode),
    InvalidEscapeSequence(InvalidEscapeSequence),
    ReservedIdentifier(ReservedIdentifier),
    InvalidEntityName(InvalidEntityName),
}

impl chumsky::Error<char> for self::Error {
    type Span = super::Span;
    type Label = String;

    fn expected_input_found<Iter: IntoIterator<Item = Option<char>>>(
        span: Self::Span,
        expected: Iter,
        found: Option<char>,
    ) -> Self {
        Self::UnexpectedToken(UnexpectedToken {
            span,
            expected: expected.into_iter().flatten().collect(),
            found,
            extra: None,
        })
    }

    fn with_label(self, label: Self::Label) -> Self {
        match self {
            Self::UnexpectedToken(err) => Self::UnexpectedToken(err.with_label(label)),
            Self::InvalidInt(err) => Self::InvalidInt(err.with_label(label)),
            Self::MultipleNumericDividers(err) => {
                Self::MultipleNumericDividers(err.with_label(label))
            }
            Self::DivdersBadlyPlaced(err) => Self::DivdersBadlyPlaced(err.with_label(label)),
            Self::InvalidUnicodeCodePoint(err) => {
                Self::InvalidUnicodeCodePoint(err.with_label(label))
            }
            Self::InvalidAsciiCode(err) => Self::InvalidAsciiCode(err.with_label(label)),
            Self::InvalidEscapeSequence(err) => Self::InvalidEscapeSequence(err.with_label(label)),
            Self::ReservedIdentifier(err) => Self::ReservedIdentifier(err.with_label(label)),
            Self::InvalidEntityName(err) => Self::InvalidEntityName(err),
        }
    }

    fn merge(self, _: Self) -> Self {
        // TODO: Review this!
        self
    }
}

pub trait ErrorI: Sized + super::Spanned + Into<Error> + PartialEq {
    fn with_label(self, label: <Error as chumsky::Error<char>>::Label) -> Self;
    fn to_report(self) -> ariadne::Report<'static, super::Span>;
}

impl ErrorI for Error {
    fn with_label(self, label: <Error as chumsky::Error<char>>::Label) -> Self {
        <Self as chumsky::Error<char>>::with_label(self, label)
    }

    fn to_report(self) -> ariadne::Report<'static, super::Span> {
        match self {
            Self::UnexpectedToken(err) => err.to_report(),
            Self::InvalidInt(err) => err.to_report(),
            Self::MultipleNumericDividers(err) => err.to_report(),
            Self::DivdersBadlyPlaced(err) => err.to_report(),
            Self::InvalidUnicodeCodePoint(err) => err.to_report(),
            Self::InvalidAsciiCode(err) => err.to_report(),
            Self::InvalidEscapeSequence(err) => err.to_report(),
            Self::ReservedIdentifier(err) => err.to_report(),
            Self::InvalidEntityName(err) => err.to_report(),
        }
    }
}

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct UnexpectedToken {
    span: super::Span,
    found: Option<char>,
    expected: Vec<char>,
    extra: Option<String>,
}

impl From<UnexpectedToken> for Error {
    fn from(value: UnexpectedToken) -> Self {
        Self::UnexpectedToken(value)
    }
}

impl ErrorI for UnexpectedToken {
    fn with_label(mut self, label: String) -> Self {
        self.extra = Some(label);
        self
    }

    fn to_report(self) -> ariadne::Report<'static, super::Span> {
        let mut colors = ColorGenerator::new();
        let blue = Color::Blue;

        let error_color = colors.next();
        let mut builder = self
            .span
            .clone()
            .build_report(ariadne::ReportKind::Error)
            .with_code("S000")
            .with_message("Syntax Error: Unexpected token")
            .with_label(
                Label::new(self.span.clone())
                    .with_color(error_color)
                    .with_message(if self.expected.is_empty() {
                        "Here.".to_string()
                    } else {
                        format!(
                            "Expected one of {} here. Got {}.",
                            self.expected
                                .iter()
                                .map(|ch| format!("`{}`", ch.fg(blue)))
                                .intersperse(", ".to_string())
                                .collect::<String>(),
                            self.found
                                .map(|ch| format!("`{}`", ch.fg(error_color)))
                                .unwrap_or(format!("{}", "<EOF>".fg(error_color)))
                        )
                    }),
            );

        if let Some(extra) = self.extra {
            builder = builder.with_label(
                Label::new(self.span)
                    .with_color(colors.next())
                    .with_message(extra),
            );
        }

        builder.finish()
    }
}
