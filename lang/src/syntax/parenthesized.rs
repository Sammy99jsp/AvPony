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

use crate::utils::{errors::Error, Span};

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct Parenthesized {
    pub span: Span,
    pub inner: Box<super::Expr>,
}

impl Parenthesized {
    pub fn parse_with(
        expr: impl chumsky::Parser<char, super::Expr, Error = Error>,
    ) -> impl chumsky::Parser<char, Self, Error = Error> {
        expr.padded_by(text::whitespace())
            .delimited_by(just("("), just(")"))
            .map_with_span(|inner, span| Self {
                span,
                inner: Box::new(inner),
            })
    }
}

#[cfg(test)]
mod tests {
    use chumsky::Parser;

    use crate::{
        lexical::Literal,
        syntax::Expr,
        utils::{stream::SourceFile, Parseable},
    };

    #[test]
    fn parened() {
        let (source, _) = SourceFile::test_file(r#"(1)"#);
        let res = Expr::parser().parse(source.stream());
        assert!(matches!(res,
        Ok(Expr::Parenthesised(super::Parenthesized { inner, ..}))
            if matches!(inner.as_ref(), Expr::Literal(Literal::Number(_))
        )));

        let (source, _) = SourceFile::test_file(r#"((.a = 1))"#);
        let res = Expr::parser().parse(source.stream());
        assert!(matches!(res,
        Ok(Expr::Parenthesised(super::Parenthesized { inner, ..}))
            if matches!(inner.as_ref(), Expr::Map(_)
        )));

        let (source, _) = SourceFile::test_file(r#"(((.a = 1)))"#);
        let res = Expr::parser().parse(source.stream());
        assert!(matches!(res,
        Ok(Expr::Parenthesised(super::Parenthesized { inner, ..}))
            if matches!(inner.as_ref(), Expr::Parenthesised(super::Parenthesized { inner, .. })
                if matches!(inner.as_ref(), Expr::Map(_))
            )
        ));
    }
}
