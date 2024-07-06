//!
//! Represents a placeholder for a syntax node in source code,
//! for recoverable errors.
//!

use std::fmt::Debug;

use avpony_macros::Spanned;
use chumsky::Parser;

use crate::utils::Span;

use super::{error::expected::Expected, PonyParser};

pub trait Marker: Sized {
    const ID: u8;
    const NAME: &'static str;

    fn new() -> Self;
}

pub trait HasPlaceholder {
    type Marker: Marker;
}

#[derive(Clone, PartialEq, Spanned)]
pub struct Placeholder {
    span: Span,
    marker: u8,
    display: &'static str,
}

impl Placeholder {
    fn new<M: Marker>(span: Span) -> Self {
        Self {
            span,
            marker: M::ID,
            display: M::NAME,
        }
    }

    pub fn id(&self) -> u8 {
        self.marker
    }

    pub fn expected(&self) -> &'static str {
        self.display
    }

    pub fn at<M: Marker>(span: Span) -> Self {
        Self {
            span,
            marker: M::ID,
            display: M::NAME,
        }
    }
}

impl Debug for Placeholder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(self.expected()).field(&self.span).finish()
    }
}

//
#[derive(Debug, Clone, PartialEq)]
pub enum Maybe<P: HasPlaceholder> {
    Present(P),
    Placeholder(Placeholder),
}

impl<P: HasPlaceholder> Maybe<P> {
    pub fn map_into<T>(self, map: impl Fn(P) -> T, unwrapper: impl Fn(Placeholder) -> T) -> T {
        match self {
            Maybe::Present(m) => map(m),
            Maybe::Placeholder(ph) => unwrapper(ph),
        }
    }
}

pub trait MaybeParser<'src, P: HasPlaceholder>: PonyParser<'src, P> + Sized + Clone {
    fn maybe(self) -> impl PonyParser<'src, Maybe<P>> + Clone {
        self.or_not().validate(|opt, ctx, emitter| {
            if let Some(opt) = opt {
                return Maybe::Present(opt);
            };

            let span: Span = ctx.span();
            let ph = Placeholder::new::<P::Marker>(span.relative_range(0..1));
            emitter.emit(Expected::new(ph.clone()).into());
            Maybe::Placeholder(ph)
        })
    }
}

impl<'src, P: HasPlaceholder, Par: PonyParser<'src, P> + Clone> MaybeParser<'src, P> for Par {}
