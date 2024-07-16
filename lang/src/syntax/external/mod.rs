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
use chumsky::{
    primitive::{just, one_of},
    text, Parser,
};

#[cfg(test)]
use crate::utils::Empty;

use crate::{
    ponyx::blocks,
    utils::{
        self, placeholder::{HasPlaceholder, Maybe}, ParseableCloned, PonyParser, Span
    },
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

    type LetDeclaration: utils::Spanned + PartialEq + Clone + Debug;
    type ConstDeclaration: utils::Spanned + PartialEq + Clone + Debug;

    fn module<'src>() -> impl PonyParser<'src, Self::Module>;
    fn expression<'src>() -> impl PonyParser<'src, Maybe<Self::Expression>> + Clone;
    fn let_declaration<'src>() -> impl PonyParser<'src, Self::LetDeclaration> + Clone;
    fn const_declaration<'src>() -> impl PonyParser<'src, Self::ConstDeclaration> + Clone;
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
        let keywords = blocks::KEYWORDS
            .iter()
            .copied()
            .map(text::keyword)
            .map(|a| a.boxed())
            .reduce(|a, b| a.or(b).boxed())
            .unwrap();

        Ext::expression()
            .and_is(one_of("#/").then(keywords).not())
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
    type LetDeclaration = Empty;
    type ConstDeclaration = Empty;

    fn module<'src>() -> impl PonyParser<'src, Self::Module> {
        Empty::parser()
    }

    fn expression<'src>() -> impl PonyParser<'src, Maybe<Self::Expression>> + Clone {
        Empty::parser().map(Maybe::Present)
    }

    fn let_declaration<'src>() -> impl PonyParser<'src, Self::LetDeclaration> + Clone {
        Empty::parser()
    }

    fn const_declaration<'src>() -> impl PonyParser<'src, Self::ConstDeclaration> + Clone {
        Empty::parser()
    }
}
