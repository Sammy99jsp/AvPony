//!
//! Common utilities used throughout parsing.
//! 

pub mod error;
pub mod input;
pub mod span;

use std::fmt::Debug;

use chumsky::extra::Full;
use chumsky::Parser;
pub use error::{Error, ErrorI};
pub use input::{PonyInput, SourceFile};
pub use span::{Span, Spanned};

pub type Extra<'a> = Full<Error, (), ()>;

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
