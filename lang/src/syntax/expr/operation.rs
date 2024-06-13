//!
//! ## Binary Operation
//! ### Grammar
//! ```text
//! bin_expr := <expr> <bin_oper> <expr>
//! ```
//!

use avpony_macros::Spanned;
use chumsky::{text, Parser};

use crate::utils::{ParseableCloned, PonyParser, Span};

use super::{external::External, operator::BinaryOperator};

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct BinaryOperation<Ext: External> {
    span: Span,
    pub operator: BinaryOperator,
    pub operands: [Box<super::Expr<Ext>>; 2],
}

impl<Ext: External> BinaryOperation<Ext> {
    pub fn parse_with<'src>(
        expr: impl PonyParser<'src, super::Expr<Ext>> + Clone,
    ) -> impl PonyParser<'src, Self> + Clone {
        expr.clone()
            .padded_by(text::whitespace())
            .map(Box::new)
            .then(BinaryOperator::parser())
            .then(expr.padded_by(text::whitespace()).map(Box::new))
            .map_with(|((first, operator), second), ctx| Self {
                span: ctx.span(),
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
        syntax::{operation::BinaryOperation, VExpr as Expr},
        utils::{Parseable, SourceFile},
    };

    #[test]
    fn symbols() {
        let (source, _) = SourceFile::test_file(r#"2 -> 4"#);
        let res = Expr::parser().parse(source.stream()).into_result();
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
        let res = Expr::parser().parse(source.stream()).into_result();
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
