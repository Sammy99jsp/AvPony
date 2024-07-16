//!
//! ## Errors for Logic Blocks
//!

use ariadne::{ColorGenerator, Label, ReportKind};
use avpony_macros::ErrorType;

use crate::utils::Span;

use super::ErrorI;

#[ErrorType(crate::utils::Error)]
pub struct UnreachableBranch {
    span: Span,
    if_block_span: Span,
}

impl UnreachableBranch {
    pub fn new(span: Span, if_block_span: Span) -> Self {
        Self {
            span,
            if_block_span,
        }
    }
}

impl ErrorI for UnreachableBranch {
    fn to_report(self) -> ariadne::Report<'static, crate::utils::Span> {
        let mut colors = ColorGenerator::new();
        self.span
            .clone()
            .build_report(ReportKind::Warning)
            .with_code("X100")
            .with_message("Unreachable code")
            .with_label(
                Label::new(self.span)
                    .with_color(colors.next())
                    .with_message("These branches are unreachable."),
            )
            .with_label(
                Label::new(self.if_block_span)
                    .with_color(colors.next())
                    .with_message("Inside this `{:if ...}` block."),
            )
            .finish()
    }
}
