//!
//! ## Indexing
//! ### Grammar
//! ```text
//! index_expr := <expr receiver> `[` <expr index> `]`
//! ```
//!

use avpony_macros::Spanned;
use chumsky::{
    primitive::just,
    text::{self, TextParser},
    Parser,
};

use crate::utils::{errors::Error, Span};

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct Indexing {
    span: Span,
    receiver: Box<super::Expr>,
    index: Box<super::Expr>,
}

impl Indexing {
    pub fn parse_with(
        expr: impl chumsky::Parser<char, super::Expr, Error = Error> + Clone,
    ) -> impl chumsky::Parser<char, Self, Error = Error> {
        expr.clone()
            .then(expr.padded().delimited_by(just("["), just("]")))
            .map_with_span(|(receiver, index), span| Self {
                span,
                receiver: Box::new(receiver),
                index: Box::new(index),
            })
    }
}

#[cfg(test)]
mod tests {
    use chumsky::Parser;

    use crate::{
        lexical::Literal,
        syntax::{index::Indexing, Expr},
        utils::{stream::SourceFile, Parseable},
    };

    #[test]
    fn member_access() {
        let (source, _) = SourceFile::test_file(r#"dict["key"]"#);
        let res = Expr::parser().parse(source.stream());
        assert!(matches!(
            res,
            Ok(Expr::Indexing(Indexing {
                receiver: box Expr::Identifier(ident),
                index: box Expr::Literal(Literal::String(key)),
                ..
            })) if (&ident == "dict") && (&key == "key")
        ))
    }
}
