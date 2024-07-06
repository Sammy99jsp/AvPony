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
    utils::{placeholder::MaybeParser, ParseableCloned, PonyParser, Span},
};

use super::{external::External, operation::BinaryOperation};

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

impl<Ext: External> BinaryOperation<Ext> {
    pub fn with<'src>(
        application: impl PonyParser<'src, Box<super::Expr<Ext>>> + Clone,
        expr: impl PonyParser<'src, super::Expr<Ext>> + Clone,
    ) -> impl PonyParser<'src, super::Expr<Ext>> + Clone {
        choice((
            application
                .clone()
                .then(BinaryOperator::parser().padded().then(expr.maybe()))
                .map_with(|(left, (operator, right)), ctx| {
                    super::Expr::BinaryOp(BinaryOperation {
                        span: ctx.span(),
                        operator,
                        operands: (left, Box::new(right)),
                    })
                }),
            application.map(|a| *a),
        ))
    }
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
        syntax::operator::{Symbolic, UnaryOperator},
        utils::{Parseable, SourceFile},
    };

    #[test]
    fn symbols() {
        let (source, _) = SourceFile::test_file(r#"->"#);
        let res = UnaryOperator::parser().parse(source.stream());
        assert!(matches!(
            res.into_result(),
            Ok(UnaryOperator::Symbols(Symbolic {
                value,
                ..
            }))
                if value == "->"
        ))
    }
}
