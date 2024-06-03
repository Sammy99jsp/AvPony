//!
//! Utilities for syntax.
//!

use std::marker::PhantomData;

use avpony_macros::Spanned;
use chumsky::{
    primitive::just,
    text::{self, TextParser},
    Parser,
};

use crate::utils::{ParseableExt, PonyParser, Span};

///
/// A sequence of a certain token type,
/// delimited by certain token.
///
#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct Punctuated<Token, Punct> {
    pub span: Span,
    pub inner: Vec<Token>,
    __marker: PhantomData<Punct>,
}

impl<Token, Punct: ParseableExt> Punctuated<Token, Punct> {
    pub fn iter(&self) -> impl Iterator<Item = &Token> + '_ {
        self.inner.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Token> + '_ {
        self.inner.iter_mut()
    }

    pub fn trailing_with(inner: impl PonyParser<Token> + Clone) -> impl PonyParser<Self> + Clone {
        inner
            .padded()
            .then_ignore(just(",").padded())
            .repeated()
            .map_with_span(|inner, span| Self {
                span,
                inner,
                __marker: PhantomData,
            })
    }

    pub fn trailing() -> impl PonyParser<Self> + Clone
    where
        Token: ParseableExt,
    {
        Self::trailing_with(Token::parser())
    }

    pub fn optional_trailing_with(
        inner: impl PonyParser<Token> + Clone,
    ) -> impl PonyParser<Self> + Clone {
        inner
            .clone()
            .then_ignore(text::whitespace())
            .then_ignore(just(",").padded())
            .repeated()
            .then(inner.or_not())
            .map_with_span(|(mut inner, after), span| Self {
                span,
                inner: {
                    inner.extend(after);
                    inner
                },
                __marker: PhantomData,
            })
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
        utils::{stream::SourceFile, Parseable},
    };

    #[test]
    fn with_trailing() {
        let (source, _) = SourceFile::test_file("1.1, 1.3, 2.2 ,");
        let res = Punctuated::<NumberLit, punctuation::Comma>::trailing().parse(source.stream());
        assert!(matches!(
            res,
            Ok(Punctuated { inner, .. }) if inner.len() == 3
        ));
    }

    #[test]
    fn with_optional_trailing() {
        let (source, _) = SourceFile::test_file("1.1, 2.2 , 2.2");
        let res = Punctuated::<_, punctuation::Comma>::optional_trailing_with(&NumberLit::parser())
            .parse(source.stream());
        assert!(matches!(
            res,
            Ok(Punctuated { inner, .. }) if inner.len() == 3
        ));

        let (source, _) = SourceFile::test_file("1.1");
        let res = Punctuated::<_, punctuation::Comma>::optional_trailing_with(&NumberLit::parser())
            .parse(source.stream());
        assert!(matches!(
            res,
            Ok(Punctuated { inner, .. }) if inner.len() == 1
        ));

        let (source, _) = SourceFile::test_file("1.1,");
        let res = Punctuated::<_, punctuation::Comma>::optional_trailing_with(&NumberLit::parser())
            .parse(source.stream());
        assert!(matches!(
            res,
            Ok(Punctuated { inner, .. }) if inner.len() == 1
        ));
    }
}
