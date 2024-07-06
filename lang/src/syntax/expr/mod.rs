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
use utils::Accessor;

use crate::{
    lexical,
    utils::{
        error::tag::SoloExprOnly,
        placeholder::{HasPlaceholder, Marker},
        Error, ParseableCloned, PonyParser,
    },
};

#[derive(Debug, Clone, Spanned, PartialEq)]
pub enum Expr<Ext: External> {
    Literal(lexical::Literal),
    Identifier(lexical::Identifier),
    UnaryOp(operation::UnarayOperation<Ext>),
    Array(array::Array<Ext>),
    Map(map::Map<Ext>),
    Tuple(tuple::Tuple<Ext>),
    Parenthesised(parenthesized::Parenthesized<Ext>),
    External(ExternalExpr<Ext>),

    MemberAccess(member::MemberAccess<Ext>),
    Indexing(index::Indexing<Ext>),
    BinaryOp(operation::BinaryOperation<Ext>),
    Application(application::Application<Ext>),
}

fn solo_expr<'src, Ext: External + 'src>(
    expr: impl PonyParser<'src, Expr<Ext>> + Clone + 'src,
) -> impl PonyParser<'src, Expr<Ext>> + Clone {
    choice((
        lexical::Literal::parser().map(Expr::Literal),
        operation::UnarayOperation::parse_with(expr.clone()).map(Expr::UnaryOp),
        lexical::Identifier::parser().map(Expr::Identifier),
        ExternalExpr::parser().map(Expr::External),
        array::Array::parse_with(expr.clone()).map(Expr::Array),
        tuple::Tuple::parse_with(expr.clone()).map(Expr::Tuple),
        map::Map::parse_with(expr.clone()).map(Expr::Map),
        parenthesized::Parenthesized::parse_with(expr.clone()).map(Expr::Parenthesised),
    ))
    .boxed()
}

impl<Ext: External + 'static> ParseableCloned for Expr<Ext> {
    fn parser<'src>() -> impl PonyParser<'src, Self> + Clone {
        recursive(|expr| {
            let solo = solo_expr(expr.clone());
            let singleton = Accessor::with(solo, expr.clone());
            let application = application::Application::with(singleton);

            operation::BinaryOperation::with(application, expr)
        })
    }
}

impl<Ext: External> HasPlaceholder for Expr<Ext> {
    type Marker = ExprMarker;
}

pub struct ExprMarker;

impl Marker for ExprMarker {
    const ID: u8 = 16;

    const NAME: &'static str = "EXPRESSION";

    fn new() -> Self {
        Self
    }
}
#[derive(Debug, Clone, Spanned, PartialEq)]
pub enum SoloExpr<Ext: External> {
    Literal(lexical::Literal),
    Identifier(lexical::Identifier),
    Array(array::Array<Ext>),
    Map(map::Map<Ext>),
    Tuple(tuple::Tuple<Ext>),
    Parenthesised(parenthesized::Parenthesized<Ext>),
    External(ExternalExpr<Ext>),
    UnaryOp(operation::UnarayOperation<Ext>),
}

impl<Ext: External + 'static> SoloExpr<Ext> {
    fn parser<'src>() -> impl PonyParser<'src, SoloExpr<Ext>> + Clone {
        solo_expr(Expr::parser()).try_map(|expr, span| match expr {
            Expr::Literal(t) => Ok(Self::Literal(t)),
            Expr::Identifier(t) => Ok(Self::Identifier(t)),
            Expr::Array(t) => Ok(Self::Array(t)),
            Expr::Map(t) => Ok(Self::Map(t)),
            Expr::Tuple(t) => Ok(Self::Tuple(t)),
            Expr::Parenthesised(t) => Ok(Self::Parenthesised(t)),
            Expr::External(t) => Ok(Self::External(t)),
            Expr::UnaryOp(t) => Ok(Self::UnaryOp(t)),
            _ => Err(Error::SoloExprOnly(SoloExprOnly::new(span))),
        })
    }
}

impl<Ext: External + 'static> ParseableCloned for SoloExpr<Ext> {
    fn parser<'src>() -> impl PonyParser<'src, Self> + Clone {
        SoloExpr::parser()
    }
}

impl<Ext: External> HasPlaceholder for SoloExpr<Ext> {
    type Marker = SoloExprMarker;
}

pub struct SoloExprMarker;

impl Marker for SoloExprMarker {
    const ID: u8 = 17;

    const NAME: &'static str = "SOLO_EXPRESSION";

    fn new() -> Self {
        Self
    }
}

#[cfg(test)]
pub type VExpr = Expr<super::external::TestLang>;
#[cfg(test)]
pub type VSoloExpr = SoloExpr<super::external::TestLang>;
