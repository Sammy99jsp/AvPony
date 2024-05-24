//!
//! ## Tokens
//!

use avpony_macros::Spanned;

use self::{number::NumberLit, string::StringLit};

pub mod identifier;
pub mod keyword;
pub mod number;
pub mod string;

#[derive(Debug, Clone, Spanned, PartialEq)]
pub enum Literal {
    Number(NumberLit),
    String(StringLit),
}
