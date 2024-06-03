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
    utils::{ParseableExt, PonyParser, Span},
};

#[derive(Debug, Clone, Spanned, PartialEq)]
pub enum UnaryOperator {
    Symbols(Symbolic),
}

impl ParseableExt for UnaryOperator {
    fn parser() -> impl PonyParser<Self> + Clone {
        Symbolic::parser().map(Self::Symbols)
    }
}

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct Symbolic {
    span: Span,
    pub value: String,
}

impl ParseableExt for Symbolic {
    fn parser() -> impl PonyParser<Self> + Clone {
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

impl ParseableExt for BinaryOperator {
    fn parser() -> impl PonyParser<Self> + Clone {
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

impl ParseableExt for NamedBinary {
    fn parser() -> impl PonyParser<Self> + Clone {
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
