//!
//! ## Pony Language Syntax
//!
//! The syntax of the main language, concerning expressions.
//!
//! ### Expressions
//!
//! Pony supports the following as expressions:
//! * __Singular Expressions__
//!   * *Literals*: number (Int, Float), string (`"`-only), and boolean (`true`, `false`) literals.
//!   * *Identifiers*: <ident>
//!   * *Array-like*: `[ (<expr>,)* ]` (with optional trailing); Empty: `[]`
//!   * *Maps*: `(` (.<ident key>: <expr value>),* `)` (with optional training); Empty: `()`
//!   * *Tuples*: `(` <expr>, `)``
//!   * *Parenthesised*: `(` <expr> `)`
//!
//! * __Compound Expressions__
//!     * *Member access*: `<expr reciever>.<ident member>`
//!     * *Indexing*: `<expr>[<expr index>]`
//!     * *Application*: <expr func> <expr args>
//!
//!     // TODO: Review being more liberal with operators, possible Haskel-style `()` declaration.
//!     * *Infix Binary Operations* (with optional whitespace between terms):
//!         * `a <binary_operator> b`
//!

pub mod application;
pub mod array;
pub mod index;
pub mod map;
pub mod member;
pub mod parenthesized;
pub mod tuple;
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
    Tuple(tuple::Tuple),
    Parenthesised(parenthesized::Parenthesized),

    MemberAccess(member::MemberAccess),
    Indexing(index::Indexing),
    Application(application::Application),
}

impl Parseable for Expr {
    fn parser() -> impl chumsky::Parser<char, Self, Error = Error> {
        recursive(|expr| {
            let solo = recursive(|_| {
                choice((
                    lexical::Literal::parser().map(Self::Literal),
                    array::Array::parse_with(expr.clone()).map(Self::Array),
                    lexical::Identifier::parser().map(Self::Identifier),
                    map::Map::parse_with(expr.clone()).map(Self::Map),
                    tuple::Tuple::parse_with(expr.clone()).map(Self::Tuple),
                    parenthesized::Parenthesized::parse_with(expr.clone()).map(Self::Parenthesised),
                ))
            })
            .boxed();

            choice((
                application::Application::parse_with(solo.clone()).map(Self::Application),
                index::Indexing::parse_with(solo.clone()).map(Self::Indexing),
                member::MemberAccess::parse_with(solo.clone()).map(Self::MemberAccess),
                solo.clone(),
            ))
        })
    }
}
