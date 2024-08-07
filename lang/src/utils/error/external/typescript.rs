//!
//! TypeScript Errors
//!

use ariadne::{Label, Report, ReportKind};
use avpony_macros::ErrorType;
use swc_common::errors::{EmitterWriter, Handler};

use crate::utils::{ErrorI, Span};

///
/// A representation of a TypeScript parser (+ linter?) error.
///
#[ErrorType(crate::utils::Error)]
pub struct TSError {
    span: Span,
    code: String,
    kind: ReportKind<'static>,
}

impl ErrorI for TSError {
    fn to_report(self) -> Report<'static, Span> {
        // TODO: Finish this.
        self.span
            .clone()
            .build_report(self.kind)
            .with_code(self.code)
            .with_label(Label::new(self.span).with_message("TS ERROR HERE!"))
            .finish()
    }
}

fn dummy_out() -> impl std::io::Write + Sync {
    std::fs::OpenOptions::new()
        .write(true)
        .open("/dev/null")
        .expect("Not on a supported unix system!")
}

pub trait ConvertTSError {
    fn convert(self, span: Span) -> TSError;
}

impl ConvertTSError for swc_ecma_parser::error::Error {
    fn convert(self, span: Span) -> TSError {
        let emitter = EmitterWriter::new(Box::new(dummy_out()), None, false, true);
        let handler = Handler::with_emitter(true, false, Box::new(emitter));
        let mut diag = self.into_diagnostic(&handler);
        diag.emit();

        let (kind, code) = diag
            .get_code()
            .map(|id| match id {
                swc_common::errors::DiagnosticId::Error(err) => (ReportKind::Error, err.clone()),
                swc_common::errors::DiagnosticId::Lint(lint) => (ReportKind::Warning, lint.clone()),
            })
            .unwrap_or_else(|| (ReportKind::Error, "TS????".to_string()));

        let msgs = diag.message.iter().map(|a| a.0.clone()).collect::<String>();
        println!("{msgs}");
        // TODO: Add extra info here.

        TSError {
            span: diag
                .span
                .primary_span()
                .map(|s| span.convert_ecma(s))
                .unwrap_or(span),
            code,
            kind,
        }
    }
}
