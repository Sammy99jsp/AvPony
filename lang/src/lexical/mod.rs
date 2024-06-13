//!
//! ## Tokens
//!

use avpony_macros::Spanned;
use chumsky::{primitive::choice, Parser};

use crate::utils::{ParseableCloned, PonyParser};

use self::{boolean::BooleanLit, number::NumberLit, string::StringLit};

pub mod boolean;
pub mod identifier;
pub mod keyword;
pub mod number;
pub mod punctuation;
pub mod string;

pub use identifier::Identifier;

#[derive(Debug, Clone, Spanned, PartialEq)]
pub enum Literal {
    Number(NumberLit),
    String(StringLit),
    Boolean(BooleanLit),
}

impl ParseableExt for Literal {
    fn parser() -> impl PonyParser<Self> + Clone {
        choice((
            NumberLit::parser().map(Self::Number),
            StringLit::parser().map(Self::String),
            BooleanLit::parser().map(Self::Boolean),
        ))
    }
}
