//!
//! ## Tokens
//!

use avpony_macros::Spanned;
use chumsky::{primitive::choice, Parser};

use crate::utils::{errors::Error, Parseable};

use self::{boolean::BooleanLit, number::NumberLit, string::StringLit};

pub mod identifier;
pub mod keyword;
pub mod number;
pub mod string;
pub mod boolean;
pub mod punctuation;

#[derive(Debug, Clone, Spanned, PartialEq)]
pub enum Literal {
    Number(NumberLit),
    String(StringLit),
    Boolean(BooleanLit),
}

impl Parseable for Literal {
    fn parser() -> impl chumsky::Parser<char, Self, Error = Error> {
        choice((
            NumberLit::parser().map(Self::Number),
            StringLit::parser().map(Self::String),
            BooleanLit::parser().map(Self::Boolean),
        ))
    }
}

