//!
//! HTML Entity Errors
//!

use ariadne::{ColorGenerator, Fmt, Label, ReportKind};
use avpony_macros::ErrorType;

use crate::utils::Span;

#[ErrorType(crate::utils::Error)]
pub struct InvalidEntityName {
    span: Span,
    code: String,
}

impl InvalidEntityName {
    pub fn new(span: Span, code: String) -> Self {
        Self { span, code }
    }
}

impl super::ErrorI for InvalidEntityName {
    fn to_report(self) -> ariadne::Report<'static, Span> {
        let mut colors = ColorGenerator::new();
        let color = colors.next();
        self.span
            .clone()
            .build_report(ReportKind::Warning)
            .with_code("X000")
            .with_message("Invalid HTML Entity code.")
            .with_label(
                Label::new(self.span)
                    .with_color(color)
                    .with_message(format!(
                        "`{}` is not a valid HTML entity code.",
                        (&self.code).fg(color)
                    )),
            )
            .finish()
    }
}
