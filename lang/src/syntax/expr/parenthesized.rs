//!
//! ## Parenthesised expression
//!
//! Used in PonyX to allow for more complicated expressions
//! in attribute values.
//!
//! Not to be confused with a [super::map::Map]!
//!
//! ```text
//! paren_expr := `(` <expr> `)`
//! ```
//!

use avpony_macros::Spanned;
use chumsky::{primitive::just, text, Parser};

use crate::utils::{PonyParser, Span};

use super::external::External;

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct Parenthesized<Ext: External> {
    span: Span,
    pub inner: Box<super::Expr<Ext>>,
}

impl<Ext: External> Parenthesized<Ext> {
    pub fn parse_with<'src>(
        expr: impl PonyParser<'src, super::Expr<Ext>> + Clone,
    ) -> impl PonyParser<'src, Self> + Clone {
        expr.padded_by(text::whitespace())
            .delimited_by(just("("), just(")"))
            .map_with(|inner, ctx| Self {
                span: ctx.span(),
                inner: Box::new(inner),
            })
    }
}

#[cfg(test)]
mod tests {
    use chumsky::Parser;

    use crate::{
        lexical::Literal,
        syntax::VExpr as Expr,
        utils::{Parseable, SourceFile},
    };

    #[test]
    fn parened() {
        let (source, _) = SourceFile::test_file(r#"(1)"#);
        let res = Expr::parser().parse(source.stream());
        assert!(matches!(
            res.into_result(),
            Ok(Expr::Parenthesised(super::Parenthesized {
                inner: box Expr::Literal(Literal::Number(_)),
                ..
            }))
        ));

        let (source, _) = SourceFile::test_file(r#"((.a = 1))"#);
        let res = Expr::parser().parse(source.stream());
        assert!(matches!(
            res.into_result(),
            Ok(Expr::Parenthesised(super::Parenthesized {
                inner: box Expr::Map(_),
                ..
            }))
        ));

        let (source, _) = SourceFile::test_file(r#"(((.a = 1)))"#);
        let res = Expr::parser().parse(source.stream());
        assert!(matches!(
            res.into_result(),
            Ok(Expr::Parenthesised(super::Parenthesized {
                inner:
                    box Expr::Parenthesised(super::Parenthesized {
                        inner: box Expr::Map(_),
                        ..
                    }),
                ..
            }))
        ));
    }
}
