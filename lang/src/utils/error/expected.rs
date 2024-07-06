//!
//! ## Generic 'Expected' errors
//!

use ariadne::{ColorGenerator, Label, ReportKind};
use avpony_macros::ErrorType;

use crate::utils::{placeholder::Placeholder, Span, Spanned};

#[ErrorType(crate::utils::Error)]
pub struct Expected {
    span: Span,
    placeholder: Placeholder,
}

impl Expected {
    pub fn new(placeholder: Placeholder) -> Self {
        Self {
            span: placeholder.span(),
            placeholder,
        }
    }
}

impl super::ErrorI for Expected {
    fn to_report(self) -> ariadne::Report<'static, crate::utils::Span> {
        let mut colors = ColorGenerator::new();
        self.span
            .clone()
            .build_report(ReportKind::Error)
            .with_code(format!("S{}", 132 + self.placeholder.id()))
            .with_message(format!("Expected {}", self.placeholder.expected()))
            .with_label(
                Label::new(self.span)
                    .with_color(colors.next())
                    .with_message(format!("Expected {} here.", self.placeholder.expected())),
            )
            .finish()
    }
}
