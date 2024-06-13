use ariadne::{Report, ReportKind};
use avpony_macros::ErrorType;
use swc_common::errors::{EmitterWriter, Handler};

use crate::utils::{ErrorI, Span};

#[ErrorType(crate::utils::Error)]
pub struct TSError {
    span: Span,
    code: String,
    kind: ReportKind<'static>,
}

impl ErrorI for TSError {
    fn to_report(self) -> Report<'static, Span> {
        todo!()
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
        println!("{diag:?}");

        let (kind, code) = diag
            .get_code()
            .map(|id| match id {
                swc_common::errors::DiagnosticId::Error(err) => (ReportKind::Error, err.clone()),
                swc_common::errors::DiagnosticId::Lint(lint) => (ReportKind::Warning, lint.clone()),
            })
            .unwrap_or_else(|| (ReportKind::Error, "TS????".to_string()));

        let msgs = diag.message.iter().map(|a| a.0.clone()).collect::<String>();
        println!("{msgs}");

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
