use ariadne::{ColorGenerator, Label, ReportKind};
use avpony_macros::ErrorType;

use crate::utils::Span;

#[ErrorType(crate::utils::Error)]
pub struct ExpectedExpr {
    span: Span,
}

impl ExpectedExpr {
    pub fn new(span: Span) -> Self {
        Self { span }
    }
}

impl super::ErrorI for ExpectedExpr {
    fn to_report(self) -> ariadne::Report<'static, crate::utils::Span> {
        let mut colors = ColorGenerator::new();

        self.span
            .clone()
            .build_report(ReportKind::Error)
            .with_code("S999")
            .with_message("Expected expression")
            .with_label(
                Label::new(self.span)
                    .with_color(colors.next())
                    .with_message("Expected an expression here"),
            )
            .finish()
    }
}
