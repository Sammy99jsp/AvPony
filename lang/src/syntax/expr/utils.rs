//!
//! Utilities for syntax.
//!

use std::marker::PhantomData;

use avpony_macros::Spanned;
use chumsky::{primitive::just, text, IterParser, Parser};

use crate::utils::{ParseableCloned, PonyParser, Span};

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
