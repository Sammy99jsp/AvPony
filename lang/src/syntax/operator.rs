//!
//! ## Operators
//! ### Grammar
//!
//!

use avpony_macros::Spanned;
use chumsky::{
    primitive::{choice, just, one_of},
    Parser,
};

use crate::{
    lexical::{punctuation, Identifier},
    utils::{errors::Error, Parseable, Span},
};

#[derive(Debug, Clone, Spanned, PartialEq)]
pub enum UnaryOperator {
    Symbols(Symbolic),
}

impl Parseable for UnaryOperator {
    fn parser() -> impl chumsky::Parser<char, Self, Error = Error> {
        Symbolic::parser().map(Self::Symbols)
    }
}

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct Symbolic {
    span: Span,
    pub value: String,
}

impl Parseable for Symbolic {
    fn parser() -> impl chumsky::Parser<char, Self, Error = Error> {
        one_of(punctuation::OPERATOR)
            .repeated()
            .at_least(1)
            .map_with_span(|symbol, span| Self {
                span,
                value: symbol.into_iter().collect(),
            })
    }
}

#[derive(Debug, Clone, Spanned, PartialEq)]
pub enum BinaryOperator {
    Symbols(Symbolic),
    Named(NamedBinary),
}

impl Parseable for BinaryOperator {
    fn parser() -> impl chumsky::Parser<char, Self, Error = crate::utils::errors::Error> {
        choice((
            Symbolic::parser().map(Self::Symbols),
            NamedBinary::parser().map(Self::Named),
        ))
    }
}

impl<'a> PartialEq<&'a str> for BinaryOperator {
    fn eq(&self, other: &&'a str) -> bool {
        match self {
            BinaryOperator::Symbols(s) => s.value == *other,
            BinaryOperator::Named(n) => n.ident == **other,
        }
    }
}

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct NamedBinary {
    span: Span,
    pub ident: Identifier,
}

impl Parseable for NamedBinary {
    fn parser() -> impl chumsky::Parser<char, Self, Error = Error> {
        Identifier::parser()
            .delimited_by(just("`"), just("`"))
            .map_with_span(|ident, span| Self { span, ident })
    }
}

#[cfg(test)]
mod tests {
    use chumsky::Parser;

    use crate::{
        syntax::{
            operator::{Symbolic, UnaryOperator},
            Expr,
        },
        utils::{stream::SourceFile, Parseable},
    };

    #[test]
    fn symbols() {
        let (source, _) = SourceFile::test_file(r#"->"#);
        let res = Expr::parser().parse(source.stream());
        assert!(matches!(
            res,
            Ok(Expr::Operator(UnaryOperator::Symbols(Symbolic {
                value,
                ..
            })))
                if value == "->"
        ))
    }
}
