//! 
//! ## Pony Expressions
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

pub use super::external;

use super::external::{External, ExternalExpr};
use avpony_macros::Spanned;
use chumsky::{primitive::choice, recursive::recursive, Parser};

use crate::{
    lexical,
    utils::{error::tag::SoloExprOnly, Error, ParseableCloned, PonyParser},
};

#[derive(Debug, Clone, Spanned, PartialEq)]
pub enum Expr<Ext: External> {
    Literal(lexical::Literal),
    Identifier(lexical::Identifier),
    Operator(operator::UnaryOperator),
    Array(array::Array<Ext>),
    Map(map::Map<Ext>),
    Tuple(tuple::Tuple<Ext>),
    Parenthesised(parenthesized::Parenthesized<Ext>),
    External(ExternalExpr<Ext>),

    BinaryOp(operation::BinaryOperation<Ext>),
    MemberAccess(member::MemberAccess<Ext>),
    Indexing(index::Indexing<Ext>),
    Application(application::Application<Ext>),
}

impl<Ext: External + 'static> ParseableCloned for Expr<Ext> {
    fn parser<'src>() -> impl PonyParser<'src, Self> + Clone {
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
                    ExternalExpr::parser().map(Self::External),
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
pub enum SoloExpr<Ext: External> {
    Literal(lexical::Literal),
    Identifier(lexical::Identifier),
    Operator(operator::UnaryOperator),
    Array(array::Array<Ext>),
    Map(map::Map<Ext>),
    Tuple(tuple::Tuple<Ext>),
    Parenthesised(parenthesized::Parenthesized<Ext>),
    External(ExternalExpr<Ext>),
}

impl<Ext: External + 'static> ParseableCloned for SoloExpr<Ext> {
    fn parser<'src>() -> impl PonyParser<'src, Self> + Clone {
        recursive(|expr| {
            choice((
                lexical::Literal::parser().map(Expr::Literal),
                operator::UnaryOperator::parser().map(Expr::Operator),
                array::Array::parse_with(expr.clone()).map(Expr::Array),
                lexical::Identifier::parser().map(Expr::Identifier),
                map::Map::parse_with(expr.clone()).map(Expr::Map),
                tuple::Tuple::parse_with(expr.clone()).map(Expr::Tuple),
                parenthesized::Parenthesized::parse_with(expr.clone()).map(Expr::Parenthesised),
                ExternalExpr::parser().map(Expr::External),
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
            Expr::External(t) => Ok(Self::External(t)),
            _ => Err(Error::SoloExprOnly(SoloExprOnly::new(span))),
        })
    }
}

#[cfg(test)]
pub type VExpr = Expr<super::external::TestLang>;
