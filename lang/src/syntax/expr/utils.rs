//!
//! Utilities for syntax.
//!

use std::{collections::VecDeque, marker::PhantomData};

use avpony_macros::Spanned;
use chumsky::{
    primitive::{choice, just},
    recursive::recursive,
    text, IterParser, Parser,
};

use crate::{
    lexical,
    utils::{placeholder::Maybe, ParseableCloned, PonyParser, Span, Spanned},
};

use super::{
    external::External,
    index::{self, Indexing},
    member::{self, MemberAccess},
    Expr,
};

///
/// A sequence of a certain token type,
/// delimited by certain token.
///
#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct Punctuated<Token, Punct> {
    span: Span,
    pub inner: Vec<Token>,
    __marker: PhantomData<Punct>,
}

impl<Token, Punct: ParseableCloned> Punctuated<Token, Punct> {
    pub fn iter(&self) -> impl Iterator<Item = &Token> + '_ {
        self.inner.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Token> + '_ {
        self.inner.iter_mut()
    }

    pub fn trailing_with<'src>(
        inner: impl PonyParser<'src, Token> + Clone,
    ) -> impl PonyParser<'src, Self> + Clone {
        inner
            .padded()
            .then_ignore(just(",").padded())
            .repeated()
            .collect::<Vec<_>>()
            .map_with(|inner, ctx| Self {
                span: ctx.span(),
                inner,
                __marker: PhantomData,
            })
    }

    pub fn trailing<'src>() -> impl PonyParser<'src, Self> + Clone
    where
        Token: ParseableCloned,
    {
        Self::trailing_with(Token::parser())
    }

    pub fn optional_trailing_with<'src>(
        inner: impl PonyParser<'src, Token> + Clone,
    ) -> impl PonyParser<'src, Self> + Clone {
        inner
            .clone()
            .then_ignore(text::whitespace())
            .separated_by(just(",").padded())
            .allow_trailing()
            .collect()
            .map_with(|inner, ctx| Self {
                span: ctx.span(),
                inner,
                __marker: PhantomData,
            })
    }

    pub fn optional_trailing<'src>() -> impl PonyParser<'src, Self> + Clone
    where
        Token: ParseableCloned,
    {
        Self::optional_trailing_with(Token::parser())
    }
}

impl<Token, Punct> IntoIterator for Punctuated<Token, Punct> {
    type Item = Token;

    type IntoIter = <Vec<Token> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

#[cfg(test)]
impl<Token, Punct, Col> PartialEq<Col> for Punctuated<Token, Punct>
where
    Vec<Token>: PartialEq<Col>,
{
    fn eq(&self, other: &Col) -> bool {
        self.inner.eq(other)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(super) enum Accessor<Ext: External> {
    Member(Maybe<lexical::Identifier>, Span),
    Index(Box<Maybe<super::Expr<Ext>>>, Span),
}

impl<Ext: External> Spanned for Accessor<Ext> {
    fn span(&self) -> crate::utils::span::Span {
        match self {
            Accessor::Member(_, s) => s.clone(),
            Accessor::Index(_, s) => s.clone(),
        }
    }
}

impl<Ext: External> Accessor<Ext> {
    fn eat(self, receiver: Expr<Ext>) -> Expr<Ext> {
        match self {
            Accessor::Member(member, span) => Expr::MemberAccess(MemberAccess {
                span: receiver.span().combine(span).unwrap(),
                receiver: Box::new(receiver),
                member,
            }),
            Accessor::Index(index, span) => Expr::Indexing(Indexing {
                span: receiver.span().combine(span).unwrap(),
                receiver: Box::new(receiver),
                index,
            }),
        }
    }
}

impl<Ext: External> Accessor<Ext> {
    fn parse_with<'src>(
        expr: impl PonyParser<'src, super::Expr<Ext>> + Clone + 'src,
    ) -> impl PonyParser<'src, AccessorQueue<Ext>> + Clone
    where
        Ext: 'src,
    {
        recursive(|accessor| {
            let single = choice((
                member::MemberAccess::partial(),
                index::Indexing::partial_with(expr.clone()),
            ));

            choice((
                single.clone().then(accessor).map(AccessorQueue::add),
                single.map(AccessorQueue::from),
            ))
        })
    }

    pub fn with<'src>(
        solo: impl PonyParser<'src, super::Expr<Ext>> + Clone + 'src,
        expr: impl PonyParser<'src, super::Expr<Ext>> + Clone + 'src,
    ) -> impl PonyParser<'src, super::Expr<Ext>> + Clone
    where
        Ext: 'src,
    {
        choice((
            solo.clone()
                .then(Accessor::parse_with(expr.clone()))
                .map(AccessorQueue::fold),
            solo,
        ))
    }
}

#[derive(Debug, Clone)]
pub(super) struct AccessorQueue<Ext: External>(VecDeque<Accessor<Ext>>);

impl<Ext: External> AccessorQueue<Ext> {
    pub fn add((op, mut queue): (Accessor<Ext>, Self)) -> Self {
        queue.0.push_front(op);
        queue
    }

    pub fn fold((ex, queue): (Expr<Ext>, Self)) -> Expr<Ext> {
        queue.0.into_iter().fold(ex, |ex, complex| complex.eat(ex))
    }
}

impl<Ext: External> From<Accessor<Ext>> for AccessorQueue<Ext> {
    fn from(value: Accessor<Ext>) -> Self {
        Self(VecDeque::from([value]))
    }
}

#[cfg(test)]
mod tests {
    use chumsky::Parser;

    use crate::{
        lexical::{number::NumberLit, punctuation},
        syntax::utils::Punctuated,
        utils::{Parseable, SourceFile},
    };

    #[test]
    fn with_trailing() {
        let (source, _) = SourceFile::test_file("1.1, 1.3, 2.2 ,");
        let res = Punctuated::<NumberLit, punctuation::Comma>::trailing().parse(source.stream());
        assert!(matches!(
            res.into_result(),
            Ok(Punctuated { inner, .. }) if inner.len() == 3
        ));
    }

    #[test]
    fn with_optional_trailing() {
        let (source, _) = SourceFile::test_file("1.1, 2.2 , 2.2");
        let res = Punctuated::<_, punctuation::Comma>::optional_trailing_with(&NumberLit::parser())
            .parse(source.stream());
        assert!(matches!(
            res.into_result(),
            Ok(Punctuated { inner, .. }) if inner.len() == 3
        ));

        let (source, _) = SourceFile::test_file("1.1");
        let res = Punctuated::<_, punctuation::Comma>::optional_trailing_with(&NumberLit::parser())
            .parse(source.stream());
        assert!(matches!(
            res.into_result(),
            Ok(Punctuated { inner, .. }) if inner.len() == 1
        ));

        let (source, _) = SourceFile::test_file("1.1,");
        let res = Punctuated::<_, punctuation::Comma>::optional_trailing_with(&NumberLit::parser())
            .parse(source.stream());
        assert!(matches!(
            res.into_result(),
            Ok(Punctuated { inner, .. }) if inner.len() == 1
        ));
    }
}
