//!
//! Errors whilst parsing a number.
//!

use std::num::ParseIntError;

use ariadne::{Color, ColorGenerator, Fmt, Label};
use avpony_macros::Spanned;

use crate::{lexical::number, utils::Span};

use super::ErrorI;

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct InvalidInt {
    span: Span,
    internal: ParseIntError,
    extra: Option<String>,
}

impl InvalidInt {
    pub fn from_internal(span: Span, internal: ParseIntError) -> Self {
        Self {
            span,
            internal,
            extra: None,
        }
    }
}

impl From<InvalidInt> for super::Error {
    fn from(value: InvalidInt) -> Self {
        Self::InvalidInt(value)
    }
}

impl ErrorI for InvalidInt {
    fn with_label(mut self, label: String) -> Self {
        self.extra.replace(label);
        self
    }

    fn to_report(self) -> ariadne::Report<'static, Span> {
        let (code, message) = match self.internal.kind() {
            std::num::IntErrorKind::PosOverflow => {
                ("S100", "Integer positive overflow: Integer too large.")
            }
            std::num::IntErrorKind::NegOverflow => {
                ("S101", "Integer negative overflow: Integer too small.")
            }
            _ => unimplemented!(), // The rest of these IntErrorKind-s are already covered by parsing.
        };

        let mut colors = ColorGenerator::new();
        let int_literal = Color::Yellow;
        let mut builder = self
            .span
            .clone()
            .build_report(ariadne::ReportKind::Error)
            .with_code(code)
            .with_message(message)
            .with_label(
                Label::new(self.span.clone())
                    .with_color(colors.next())
                    .with_message(
                        "AvPony does not currently support integer literals of this value.",
                    ),
            )
            .with_note(format!(
                "Integer literals are limited to between {} and {}.",
                number::IntType::MIN.fg(int_literal),
                number::IntType::MAX.fg(int_literal)
            ))
            .with_help(
                "Don't use integer literals, or file a bug report to increase the integer size.",
            );

        if let Some(extra) = self.extra {
            builder.add_label(
                Label::new(self.span)
                    .with_color(colors.next())
                    .with_message(extra),
            )
        }

        builder.finish()
    }
}

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct MultipleNumericDividers {
    span: Span,
    extra: Option<String>,
}

impl MultipleNumericDividers {
    pub(crate) fn new(span: Span) -> Self {
        Self { span, extra: None }
    }
}

impl From<MultipleNumericDividers> for super::Error {
    fn from(value: MultipleNumericDividers) -> Self {
        Self::MultipleNumericDividers(value)
    }
}

impl ErrorI for MultipleNumericDividers {
    fn with_label(mut self, label: <super::Error as chumsky::Error<char>>::Label) -> Self {
        self.extra.replace(label);
        self
    }

    fn to_report(self) -> ariadne::Report<'static, crate::utils::Span> {
        let mut colors = ColorGenerator::new();
        self.span
            .clone()
            .build_report(ariadne::ReportKind::Error)
            .with_code("S110")
            .with_message("Multiple numeric separators in a row.")
            .with_label(
                Label::new(self.span)
                    .with_color(colors.next())
                    .with_message("Excess underscores."),
            )
            .with_help("Remove the excess underscores.")
            .finish()
    }
}

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct DivdersBadlyPlaced {
    span: Span,
    extra: Option<String>,
}

impl DivdersBadlyPlaced {
    pub(crate) fn new(span: Span) -> Self {
        Self { span, extra: None }
    }
}

impl From<DivdersBadlyPlaced> for super::Error {
    fn from(value: DivdersBadlyPlaced) -> Self {
        Self::DivdersBadlyPlaced(value)
    }
}

impl ErrorI for DivdersBadlyPlaced {
    fn with_label(mut self, label: <super::Error as chumsky::Error<char>>::Label) -> Self {
        self.extra.replace(label);
        self
    }

    fn to_report(self) -> ariadne::Report<'static, crate::utils::Span> {
        let mut colors = ColorGenerator::new();
        self.span
            .clone()
            .build_report(ariadne::ReportKind::Error)
            .with_code("S111")
            .with_message("Improperly placed numeric seperators.")
            .with_label(
                Label::new(self.span)
                    .with_color(colors.next())
                    .with_message("You can only place underscores between digits."),
            )
            .with_help("Remove the excess underscores.")
            .finish()
    }
}
