use std::{ops::Range, sync::Arc};

#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    path: Arc<str>,
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
    type SourceId = str;

    fn source(&self) -> &Self::SourceId {
        self.path.as_ref()
    }

    fn start(&self) -> usize {
        self.range.start
    }

    fn end(&self) -> usize {
        self.range.end
    }
}

impl Span {
    pub(super) fn build_report(
        self,
        kind: ariadne::ReportKind,
    ) -> ariadne::ReportBuilder<'_, Self> {
        ariadne::Report::build(kind, self.path.as_ref(), self.range.start)
    }
}

pub trait Spanned {
    fn span(&self) -> self::Span;
}

impl Spanned for Span {
    fn span(&self) -> self::Span {
        self.clone()
    }
}
