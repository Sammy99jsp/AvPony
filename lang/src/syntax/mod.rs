//!
//! ## Pony Language Syntax
//!
//! The syntax of the main language, concerning expressions.
//!
//! ### Expressions
//!super::Field::KeyValue
//! Pony supports the following as expressions:
//! * __Singular Expressions__
//!   * *Literals*: number (Int, Float), string (`"`-only), and boolean (`true`, `false`) literals.
//!   * *Identifiers*: <ident>
//!   * *Array-like*: `[ (<expr>,)* ]` (with optional trailing); Empty: `[]`
//!   * *Maps*: `( (.<ident key>: <expr value>),* )` (with optional training); Empty: `()` -- // TODO: REVIEW
//!   * *Parenthesised*: `(` <expr> `)`
//! 
//! * __Compound Expressions__
//!     * *Member access*: `<expr reciever>.<ident member>`
//!     * *Indexing*: `<expr>[<expr index>]`
//!     * *Function calls*: `<expr callee>((<expr args>),*)` (with optional training)
//!
//!     // TODO: Review being more liberal with operators, possible Haskel-style `()` declaration.
//!     * *Unary Operations* (no optional whitespace between terms):
//!         * *Prefix Unary Operations*: `<operator> <expr>`
//!         * *Postfix Unary Operations*: `<expr> <operator>`
//!     * *Infix Binary Operations* (with optional whitespace between terms):
//!         * `a <operator> b`
//!

pub mod array;
pub mod map;
pub mod parenthesized;
pub mod utils;

use avpony_macros::Spanned;
use chumsky::{primitive::choice, recursive::recursive, Parser};

use crate::{
    lexical,
    utils::{errors::Error, Parseable},
};

#[derive(Debug, Clone, Spanned, PartialEq)]
pub enum Expr {
    Literal(lexical::Literal),
    Identifier(lexical::Identifier),
    Array(array::Array),
    Map(map::Map),
    Parenthesised(parenthesized::Parenthesized),
}

impl Parseable for Expr {
    fn parser() -> impl chumsky::Parser<char, Self, Error = Error> {
        recursive(|expr| {
            choice((
                lexical::Literal::parser().map(Self::Literal),
                array::Array::parse_with(expr.clone()).map(Self::Array),
                lexical::Identifier::parser().map(Self::Identifier),
                map::Map::parse_with(expr.clone()).map(Self::Map),
                parenthesized::Parenthesized::parse_with(expr.clone()).map(Self::Parenthesised),
            ))
        })
    }
}
