//!
//! Identifier Errors
//!

use ariadne::{ColorGenerator, Fmt, Label};
use avpony_macros::ErrorType;

use crate::utils::Span;

use super::ErrorI;

#[ErrorType(crate::utils::Error)]
pub struct ReservedIdentifier {
    span: Span,
    erroneous: String,
}

impl ReservedIdentifier {
    pub fn new(span: Span, erroneous: String) -> Self {
        Self { span, erroneous }
    }
}

impl ErrorI for ReservedIdentifier {
    fn to_report(self) -> ariadne::Report<'static, crate::utils::Span> {
        let mut colors = ColorGenerator::new();
        let err_color = colors.next();
        self.span
            .clone()
            .build_report(ariadne::ReportKind::Error)
            .with_code("S300")
            .with_message("Use of reserved identifier")
            .with_label(
                Label::new(self.span)
                    .with_color(err_color)
                    .with_message(format!("You cannot use `{}`", self.erroneous.fg(err_color))),
            )
            .finish()
    }
}
