//!
//! Common utilities used throughout parsing.
//!

pub mod error;
pub mod input;
pub mod placeholder;
pub mod span;

use std::fmt::Debug;

use chumsky::extra::Full;
use chumsky::Parser;
pub use error::{Error, ErrorI};
pub use input::{PonyInput, SourceFile};
use placeholder::{HasPlaceholder, Marker, Maybe, Placeholder};
pub use span::{Span, Spanned};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct PonyContext {
    pub is_in_member: bool,
}

pub type Extra<'a> = Full<Error, (), PonyContext>;
pub type ExtraM<'a, 'src> = chumsky::input::MapExtra<
    'a,
    'src,
    chumsky::input::WithContext<Span, &'src str>,
    Full<Error, (), PonyContext>,
>;

pub trait PonyParser<'src, O>: chumsky::Parser<'src, PonyInput<'src>, O, Extra<'src>> {}
impl<'src, O, Parser> PonyParser<'src, O> for Parser where
    Parser: chumsky::Parser<'src, PonyInput<'src>, O, Extra<'src>>
{
}

pub trait Parseable: Sized + Debug + Clone + Spanned + PartialEq {
    fn parser<'src>() -> impl PonyParser<'src, Self>;
}

pub trait ParseableCloned: Sized + Debug + Clone + Spanned + PartialEq {
    fn parser<'src>() -> impl PonyParser<'src, Self> + Clone;
}

impl<Cl: ParseableCloned> Parseable for Cl {
    fn parser<'src>() -> impl PonyParser<'src, Self> {
        Cl::parser()
    }
}

pub trait ParseablePlaceholder<'src>:
    PonyParser<'src, Maybe<Self>> + Sized + HasPlaceholder
{
    fn map_or_placeholder<T, F, G>(self, map: F, placeholder_map: G) -> impl PonyParser<'src, T>
    where
        F: (Fn(Self) -> T) + 'static,
        G: (Fn(Placeholder) -> T) + 'static,
    {
        self.map(move |ph| ph.map_into(&map, &placeholder_map))
    }
}

impl<'src, Par: PonyParser<'src, Maybe<Self>> + HasPlaceholder> ParseablePlaceholder<'src> for Par {}

// Useful TODO implementings

#[derive(Debug, Clone, PartialEq)]
pub struct Empty(Span);

impl Spanned for Empty {
    fn span(&self) -> span::Span {
        unimplemented!("A todo parser!")
    }
}

impl ParseableCloned for Empty {
    fn parser<'src>() -> impl PonyParser<'src, Self> + Clone {
        chumsky::primitive::empty().map_with(|(), span| Empty(span.span()))
    }
}

pub struct EmptyMarker;

impl Marker for EmptyMarker {
    const ID: u8 = 64;

    const NAME: &'static str = "EMPTY";

    fn new() -> Self {
        Self
    }
}

impl HasPlaceholder for Empty {
    type Marker = EmptyMarker;
}
