//!
//! ## External Syntax
//!
//! This module governs parsing external syntax,
//! such as modules, and expressions.
//!
//! External languages supported:
//! * TypeScript;
//!
use std::fmt::Debug;

use avpony_macros::Spanned;
use chumsky::{primitive::just, Parser};

#[cfg(test)]
use crate::utils::Empty;

use crate::utils::{
    placeholder::{HasPlaceholder, Maybe},
    ParseableCloned, PonyParser, Span,
};

pub mod typescript;

///
/// An external language declaration.
///
/// AvPony uses these to know how to parse the
/// modules, and expressions of an external language.
///
pub trait External: PartialEq + Debug + Clone {
    const ID: &'static str;

    type Module: PartialEq + Clone + Debug;
    type Expression: PartialEq + Clone + Debug + HasPlaceholder;

    fn module<'src>() -> impl PonyParser<'src, Self::Module>;
    fn expression<'src>() -> impl PonyParser<'src, Maybe<Self::Expression>> + Clone;
}

///
/// An expression originating from an external language,
/// delimited by braces `{}`.
///
#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct ExternalExpr<Ext: External> {
    span: Span,
    expr: Maybe<Ext::Expression>,
}

impl<Ext: External> ParseableCloned for ExternalExpr<Ext> {
    fn parser<'src>() -> impl PonyParser<'src, Self> + Clone {
        Ext::expression()
            .delimited_by(just("{"), just("}"))
            .map_with(|expr, ctx| Self {
                span: ctx.span(),
                expr,
            })
        // .map_err(|err| unimplemented!("SHOULDNOT BE CALLED: {err:?}"))
    }
}

impl<Ext: External> HasPlaceholder for ExternalExpr<Ext> {
    type Marker = <Ext::Expression as HasPlaceholder>::Marker;
}

///
/// A 'void' external language,
/// useful in testing for when you want to
/// satisfy typechecking, and are not interested
/// in the external features themselves.
///
#[cfg(test)]
#[derive(Debug, Clone, PartialEq)]
pub struct TestLang;

#[cfg(test)]
impl External for TestLang {
    const ID: &'static str = "TESTING LANGUAGE";

    type Module = Empty;

    type Expression = Empty;

    fn module<'src>() -> impl PonyParser<'src, Self::Module> {
        Empty::parser()
    }

    fn expression<'src>() -> impl PonyParser<'src, Maybe<Self::Expression>> + Clone {
        Empty::parser().map(Maybe::Present)
    }
}
