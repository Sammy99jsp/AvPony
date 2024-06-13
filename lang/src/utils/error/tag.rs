//!
//! Errors for PonyX tags.
//!

use ariadne::{ColorGenerator, Fmt, Label, ReportKind};
use avpony_macros::ErrorType;

use crate::{
    ponyx::tag::name::TagName,
    utils::{Span, Spanned},
};

#[ErrorType(crate::utils::Error)]
pub struct SoloExprOnly {
    span: Span,
}

impl SoloExprOnly {
    pub fn new(span: Span) -> Self {
        Self { span }
    }
}

impl super::ErrorI for SoloExprOnly {
    fn to_report(self) -> ariadne::Report<'static, crate::utils::Span> {
        let mut colors = ColorGenerator::new();

        self.span
            .clone()
            .build_report(ReportKind::Error)
            .with_code("X100")
            .with_message("Unexpected expression")
            .with_label(
                Label::new(self.span)
                    .with_color(colors.next())
                    .with_message("Expected a solo expression here"),
            )
            .with_note("Solo expressions are literals, arrays, tuples, maps, external, or parenthesized expressions.")
            .finish()
    }
}

#[ErrorType(crate::utils::Error)]
pub struct UnclosedTag {
    span: Span,
    opening: (Span, String),
    closing: (Span, String),
}

impl UnclosedTag {
    pub fn new(span: Span, opening: TagName, closing: TagName) -> Self {
        Self {
            span,
            opening: (opening.span(), opening.to_string()),
            closing: (closing.span(), closing.to_string()),
        }
    }
}

impl super::ErrorI for UnclosedTag {
    fn to_report(self) -> ariadne::Report<'static, crate::utils::Span> {
        let mut colors = ColorGenerator::new();
        let opening = colors.next();
        let closing = colors.next();

        self.span
            .clone()
            .build_report(ReportKind::Error)
            .with_code("X101")
            .with_message("Unclosed tag")
            .with_labels([
                Label::new(self.span)
                    .with_color(colors.next())
                    .with_message("Whilst parsing this tag."),
                Label::new(self.opening.0)
                    .with_color(opening)
                    .with_message(format!(
                        "`<{}>` tag opened here.",
                        (&self.opening.1).fg(opening)
                    )),
                Label::new(self.closing.0)
                    .with_color(closing)
                    .with_message(format!(
                        "Expected `</{}>`, got `</{}>`.",
                        (&self.opening.1).fg(opening),
                        (&self.closing.1).fg(closing)
                    )),
            ])
            .with_help(format!(
                "Rename `</{}>` to `</{}>`",
                self.opening.1.fg(opening),
                self.closing.1.fg(closing)
            ))
            .finish()
    }
}
