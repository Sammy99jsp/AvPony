use std::{ops::Range, sync::Arc};

///
/// Where something is in source code.
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
pub struct Span {
    path: Arc<str>,

    ///
    /// In this case, [Range<usize>] refers to the byte
    /// offset in a source code file.
    ///
    range: Range<usize>,
}

impl chumsky::span::Span for Span {
    type Context = Arc<str>;

    type Offset = usize;

    fn new(context: Self::Context, range: Range<Self::Offset>) -> Self {
        Self {
            path: context,
            range,
        }
    }

    fn context(&self) -> Self::Context {
        self.path.clone()
    }

    fn start(&self) -> Self::Offset {
        self.range.start
    }

    fn end(&self) -> Self::Offset {
        self.range.end
    }
}

impl ariadne::Span for Span {
    type SourceId = Arc<str>;

    fn source(&self) -> &Self::SourceId {
        &self.path
    }

    fn start(&self) -> usize {
        self.range.start
    }

    fn end(&self) -> usize {
        self.range.end
    }
}

impl Span {
    ///
    /// Make a nice-looking [ariadne] error at the location
    /// of this span.
    ///
    pub(super) fn build_report(
        self,
        kind: ariadne::ReportKind,
    ) -> ariadne::ReportBuilder<'_, Self> {
        ariadne::Report::build(kind, self.path.as_ref(), self.range.start)
    }

    pub fn relative_range(&self, range: Range<usize>) -> Self {
        Self {
            path: self.path.clone(),
            range: (self.range.start + range.start)..(self.range.start + range.end),
        }
    }
}

///
/// The span of the entirety of
/// this syntax node.
///
pub trait Spanned {
    fn span(&self) -> self::Span;
}

impl Spanned for Span {
    fn span(&self) -> self::Span {
        self.clone()
    }
}
