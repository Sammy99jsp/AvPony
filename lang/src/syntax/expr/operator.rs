//!
//! ## Operators
//! ### Grammar
//!
//!

use avpony_macros::Spanned;
use chumsky::{
    primitive::{choice, just, one_of},
    IterParser, Parser,
};

use crate::{
    lexical::{punctuation, Identifier},
    utils::{ParseableCloned, PonyParser, Span},
};

#[derive(Debug, Clone, Spanned, PartialEq)]
pub enum UnaryOperator {
    Symbols(Symbolic),
}

impl ParseableCloned for UnaryOperator {
    fn parser<'src>() -> impl PonyParser<'src, Self> + Clone {
        Symbolic::parser().map(Self::Symbols)
    }
}

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct Symbolic {
    span: Span,
    pub value: String,
}

impl ParseableCloned for Symbolic {
    fn parser<'src>() -> impl PonyParser<'src, Self> + Clone {
        one_of(punctuation::OPERATOR)
            .repeated()
            .at_least(1)
            .collect()
            .map_with(|symbol: Vec<_>, ctx| Self {
                span: ctx.span(),
                value: symbol.into_iter().collect(),
            })
    }
}

#[derive(Debug, Clone, Spanned, PartialEq)]
pub enum BinaryOperator {
    Symbols(Symbolic),
    Named(NamedBinary),
}

impl ParseableCloned for BinaryOperator {
    fn parser<'src>() -> impl PonyParser<'src, Self> + Clone {
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

impl ParseableCloned for NamedBinary {
    fn parser<'src>() -> impl PonyParser<'src, Self> + Clone {
        Identifier::parser()
            .delimited_by(just("`"), just("`"))
            .map_with(|ident, ctx| Self {
                span: ctx.span(),
                ident,
            })
    }
}

#[cfg(test)]
mod tests {
    use chumsky::Parser;

    use crate::{
        syntax::{
            operator::{Symbolic, UnaryOperator},
            VExpr as Expr,
        },
        utils::{Parseable, SourceFile},
    };

    #[test]
    fn symbols() {
        let (source, _) = SourceFile::test_file(r#"->"#);
        let res = Expr::parser().parse(source.stream());
        assert!(matches!(
            res.into_result(),
            Ok(Expr::Operator(UnaryOperator::Symbols(Symbolic {
                value,
                ..
            })))
                if value == "->"
        ))
    }
}
