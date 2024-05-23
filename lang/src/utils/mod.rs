//!
//! Common utilities for error handling, spans, etc.
//!

pub mod errors;
pub mod stream;

use std::{fmt::Debug, ops::Range, sync::Arc};

///
/// Where something is in source code.
///
/// In this case, the [Range<usize>] refers to the byte
/// offset in a source code file.
///
/// ### Example
///
/// In file `/some/file.avpony`
/// ```avpony
/// arr.length
/// ```
///
/// Here, the token `arr` has span (`/some/file,avpony`, 0).
///
#[derive(Debug, Clone, PartialEq)]
pub struct Span(Arc<str>, Range<usize>);

impl Span {
    pub(super) fn new(path: &Arc<str>, range: Range<usize>) -> Self {
        Self(path.clone(), range)
    }
}

impl chumsky::Span for self::Span {
    type Context = Arc<str>;
    type Offset = usize;

    fn new(context: Self::Context, range: Range<Self::Offset>) -> Self {
        Self(context, range)
    }

    fn context(&self) -> Self::Context {
        self.0.clone()
    }

    fn start(&self) -> Self::Offset {
        self.1.start
    }

    fn end(&self) -> Self::Offset {
        self.1.end
    }
}

impl ariadne::Span for Span {
    type SourceId = Arc<str>;

    fn source(&self) -> &Self::SourceId {
        &self.0
    }

    fn start(&self) -> usize {
        self.1.start
    }

    fn end(&self) -> usize {
        self.1.end
    }
}

impl Span {
    pub(super) fn build_report(
        self,
        kind: ariadne::ReportKind<'static>,
    ) -> ariadne::ReportBuilder<'_, Self> {
        ariadne::Report::build(kind, self.0.as_ref(), self.1.start)
    }

    pub fn relative_range(&self, range: Range<usize>) -> Self {
        Self(
            self.0.clone(),
            (self.1.start + range.start)..(self.1.start + range.end),
        )
    }
}

///
/// Retrurns the span of the entirety of
/// this syntax node.
///
pub trait Spanned {
    fn span(&self) -> Span;
}

impl Spanned for Span {
    fn span(&self) -> Span {
        self.clone()
    }
}

///
/// Something that can be parsed.
///
pub trait Parseable: Sized + Debug + Clone + Spanned + PartialEq {
    fn parser() -> impl chumsky::Parser<char, Self, Error = errors::Error>;
}
