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
pub mod operation;
pub mod operator;
pub mod parenthesized;
pub mod tuple;
pub mod utils;

use avpony_macros::Spanned;
use chumsky::{primitive::choice, recursive::recursive, Parser};

use crate::{
    lexical,
    utils::{
        errors::{tag::SoloExprOnly, Error},
        ParseableExt, PonyParser,
    },
};

#[derive(Debug, Clone, Spanned, PartialEq)]
pub enum Expr {
    Literal(lexical::Literal),
    Identifier(lexical::Identifier),
    Operator(operator::UnaryOperator),
    Array(array::Array),
    Map(map::Map),
    Tuple(tuple::Tuple),
    Parenthesised(parenthesized::Parenthesized),

    BinaryOp(operation::BinaryOperation),
    MemberAccess(member::MemberAccess),
    Indexing(index::Indexing),
    Application(application::Application),
}

impl ParseableExt for Expr {
    fn parser() -> impl PonyParser<Self> + Clone {
        recursive(|expr| {
            let solo = recursive(|_| {
                choice((
                    lexical::Literal::parser().map(Self::Literal),
                    operator::UnaryOperator::parser().map(Self::Operator),
                    array::Array::parse_with(expr.clone()).map(Self::Array),
                    lexical::Identifier::parser().map(Self::Identifier),
                    map::Map::parse_with(expr.clone()).map(Self::Map),
                    tuple::Tuple::parse_with(expr.clone()).map(Self::Tuple),
                    parenthesized::Parenthesized::parse_with(expr.clone()).map(Self::Parenthesised),
                ))
            })
            .boxed();

            choice((
                operation::BinaryOperation::parse_with(solo.clone()).map(Self::BinaryOp),
                application::Application::parse_with(solo.clone()).map(Self::Application),
                index::Indexing::parse_with(solo.clone()).map(Self::Indexing),
                member::MemberAccess::parse_with(solo.clone()).map(Self::MemberAccess),
                solo.clone(),
            ))
        })
    }
}

#[derive(Debug, Clone, Spanned, PartialEq)]
pub enum SoloExpr {
    Literal(lexical::Literal),
    Identifier(lexical::Identifier),
    Operator(operator::UnaryOperator),
    Array(array::Array),
    Map(map::Map),
    Tuple(tuple::Tuple),
    Parenthesised(parenthesized::Parenthesized),
}

impl ParseableExt for SoloExpr {
    fn parser() -> impl PonyParser<Self> + Clone {
        recursive(|expr| {
            choice((
                lexical::Literal::parser().map(Expr::Literal),
                operator::UnaryOperator::parser().map(Expr::Operator),
                array::Array::parse_with(expr.clone()).map(Expr::Array),
                lexical::Identifier::parser().map(Expr::Identifier),
                map::Map::parse_with(expr.clone()).map(Expr::Map),
                tuple::Tuple::parse_with(expr.clone()).map(Expr::Tuple),
                parenthesized::Parenthesized::parse_with(expr.clone()).map(Expr::Parenthesised),
            ))
        })
        .try_map(|val, span| match val {
            Expr::Literal(t) => Ok(Self::Literal(t)),
            Expr::Identifier(t) => Ok(Self::Identifier(t)),
            Expr::Operator(t) => Ok(Self::Operator(t)),
            Expr::Array(t) => Ok(Self::Array(t)),
            Expr::Map(t) => Ok(Self::Map(t)),
            Expr::Tuple(t) => Ok(Self::Tuple(t)),
            Expr::Parenthesised(t) => Ok(Self::Parenthesised(t)),
            _ => Err(Error::SoloExprOnly(SoloExprOnly::new(span))),
        })
    }
}
