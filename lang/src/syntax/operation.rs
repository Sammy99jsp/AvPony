//!
//! ## Binary Operation
//! ### Grammar
//! ```text
//! bin_expr := <expr> <bin_oper> <expr>
//! ```
//!

use avpony_macros::Spanned;
use chumsky::{text, Parser};

use crate::utils::{errors::Error, Parseable, Span};

use super::operator::BinaryOperator;

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct BinaryOperation {
    span: Span,
    operator: BinaryOperator,
    operands: [Box<super::Expr>; 2],
}

impl BinaryOperation {
    pub fn parse_with(
        expr: impl chumsky::Parser<char, super::Expr, Error = Error> + Clone,
    ) -> impl chumsky::Parser<char, Self, Error = Error> {
        expr.clone()
            .padded_by(text::whitespace())
            .map(Box::new)
            .then(BinaryOperator::parser())
            .then(expr.padded_by(text::whitespace()).map(Box::new))
            .map_with_span(|((first, operator), second), span| Self {
                span,
                operator,
                operands: [first, second],
            })
    }
}

#[cfg(test)]
mod tests {
    use chumsky::Parser;

    use crate::{
        lexical::{
            number::{IntegerLit, NumberLit},
            Literal,
        },
        syntax::{operation::BinaryOperation, Expr},
        utils::{stream::SourceFile, Parseable},
    };

    #[test]
    fn symbols() {
        let (source, _) = SourceFile::test_file(r#"2 -> 4"#);
        let res = Expr::parser().parse(source.stream());
        assert!(matches!(
            res,
            Ok(Expr::BinaryOp(BinaryOperation {
                operator,
                operands:
                    [box Expr::Literal(Literal::Number(NumberLit::Integer(IntegerLit {
                        value: 2, ..
                    }))), box Expr::Literal(Literal::Number(NumberLit::Integer(IntegerLit {
                        value: 4, ..
                    })))],
                ..
            })) if operator == "->"
        ))
    }

    #[test]
    fn named() {
        let (source, _) = SourceFile::test_file(r#"2 `to` 4"#);
        let res = Expr::parser().parse(source.stream());
        assert!(matches!(
            res,
            Ok(Expr::BinaryOp(BinaryOperation {
                operator,
                operands:
                    [box Expr::Literal(Literal::Number(NumberLit::Integer(IntegerLit {
                        value: 2, ..
                    }))), box Expr::Literal(Literal::Number(NumberLit::Integer(IntegerLit {
                        value: 4, ..
                    })))],
                ..
            })) if operator == "to"
        ))
    }
}
