//!
//! Common utilities for error handling, spans, etc.
//!

pub mod errors;

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
#[derive(Debug, Clone)]
pub struct Span(Arc<str>, Range<usize>);

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

///
/// Retrurns the span of the entirety of
/// this syntax node.
///
pub trait Spanned {
    fn span(&self) -> Span;
}

///
/// Something that can be parsed.
///
pub trait Parseable: Sized + Debug + Clone + Spanned {
    fn parser() -> impl chumsky::Parser<char, Self, Error = errors::Error>;
}
