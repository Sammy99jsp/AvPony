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

use crate::utils::{errors::Error, Parseable, Span};

///
/// A sequence of a certain token type,
/// delimited by certain token.
/// 
#[derive(Debug, Clone, Spanned)]
pub struct Punctuated<Token, Punct> {
    span: Span,
    inner: Vec<Token>,
    __marker: PhantomData<Punct>,
}

impl<Token: Parseable, Punct: Parseable> Punctuated<Token, Punct> {
    pub fn iter(&self) -> impl Iterator<Item = &Token> + '_ {
        self.inner.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Token> + '_ {
        self.inner.iter_mut()
    }

    pub fn with_trailing() -> impl Parser<char, Self, Error = Error> {
        Token::parser()
            .padded()
            .then_ignore(just(",").padded())
            .repeated()
            .map_with_span(|inner, span| Self {
                span,
                inner,
                __marker: PhantomData,
            })
    }

    pub fn with_optional_trailing() -> impl Parser<char, Self, Error = Error> {
        Token::parser()
            .then_ignore(text::whitespace())
            .then_ignore(just(",").padded())
            .repeated()
            .then(Token::parser().or_not())
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
mod tests {
    use chumsky::Parser;

    use crate::{
        lexical::{number::NumberLit, punctuation},
        syntax::utils::Punctuated,
        utils::stream::SourceFile,
    };

    #[test]
    fn with_trailing() {
        let (source, _) = SourceFile::test_file("1.1, 1.3, 2.2 ,");
        let res =
            Punctuated::<NumberLit, punctuation::Comma>::with_trailing().parse(source.stream());
        assert!(matches!(
            res,
            Ok(Punctuated { inner, .. }) if inner.len() == 3
        ));
    }

    #[test]
    fn with_optional_trailing() {
        let (source, _) = SourceFile::test_file("1.1, 2.2 , 2.2");
        let res = Punctuated::<NumberLit, punctuation::Comma>::with_optional_trailing()
            .parse(source.stream());
        assert!(matches!(
            res,
            Ok(Punctuated { inner, .. }) if inner.len() == 3
        ));

        let (source, _) = SourceFile::test_file("1.1");
        let res = Punctuated::<NumberLit, punctuation::Comma>::with_optional_trailing()
            .parse(source.stream());
        assert!(matches!(
            res,
            Ok(Punctuated { inner, .. }) if inner.len() == 1
        ));

        let (source, _) = SourceFile::test_file("1.1,");
        let res = Punctuated::<NumberLit, punctuation::Comma>::with_optional_trailing()
            .parse(source.stream());
        assert!(matches!(
            res,
            Ok(Punctuated { inner, .. }) if inner.len() == 1
        ));
    }
}
