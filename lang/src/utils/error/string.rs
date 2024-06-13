//!
//! Errors whilst parsing a string.
//!

use ariadne::{ColorGenerator, Label};
use avpony_macros::ErrorType;

use crate::utils::Span;

use super::ErrorI;

#[ErrorType(crate::utils::Error)]
pub struct InvalidUnicodeCodePoint {
    span: Span,
    erroneous: u32,
}

impl InvalidUnicodeCodePoint {
    pub(crate) fn new(span: Span, erroneous: u32) -> Self {
        Self { span, erroneous }
    }
}

impl ErrorI for InvalidUnicodeCodePoint {
    fn to_report(self) -> ariadne::Report<'static, crate::utils::Span> {
        let mut colors = ColorGenerator::new();

        self.span
            .clone()
            .build_report(ariadne::ReportKind::Error)
            .with_code("S200")
            .with_message("Invalid unicode character escape.")
            .with_label(
                Label::new(self.span)
                    .with_color(colors.next())
                    .with_message(format!(
                        "{:x} is not a valid Unicode character point.",
                        self.erroneous
                    )),
            )
            .finish()
    }
}

#[ErrorType(crate::utils::Error)]
pub struct InvalidAsciiCode {
    span: Span,
    erroneous: u32,
}

impl InvalidAsciiCode {
    pub(crate) fn new(span: Span, erroneous: u32) -> Self {
        Self { span, erroneous }
    }
}

impl ErrorI for InvalidAsciiCode {
    fn to_report(self) -> ariadne::Report<'static, crate::utils::Span> {
        let mut colors = ColorGenerator::new();

        self.span
            .clone()
            .build_report(ariadne::ReportKind::Error)
            .with_code("S201")
            .with_message("Invalid ASCII character escape.")
            .with_label(
                Label::new(self.span)
                    .with_color(colors.next())
                    .with_message(format!(
                        "{:x} is not a valid ASCII code (0x00 to 0x7F, inclusive).",
                        self.erroneous
                    )),
            )
            .finish()
    }
}

#[ErrorType(crate::utils::Error)]
pub struct InvalidEscapeSequence {
    span: Span,
}

impl InvalidEscapeSequence {
    pub fn new(span: Span) -> Self {
        Self { span }
    }
}

impl ErrorI for InvalidEscapeSequence {
    fn to_report(self) -> ariadne::Report<'static, crate::utils::Span> {
        let mut colors = ColorGenerator::new();

        self.span
            .clone()
            .build_report(ariadne::ReportKind::Error)
            .with_code("S102")
            .with_message("Invalid escape sequence.")
            .with_label(
                Label::new(self.span)
                    .with_color(colors.next())
                    .with_message("Starting here."),
            )
            .with_help("Try removing this `\\`.")
            .finish()
    }
}
