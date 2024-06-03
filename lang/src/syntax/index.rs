//!
//! ## Indexing
//! ### Grammar
//! ```text
//! index_expr := <expr receiver> `[` <expr index> `]`
//! ```
//!

use avpony_macros::Spanned;
use chumsky::{primitive::just, text::TextParser, Parser};

use crate::utils::{PonyParser, Span};

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct Indexing {
    span: Span,
    pub receiver: Box<super::Expr>,
    pub index: Box<super::Expr>,
}

impl Indexing {
    pub fn parse_with(expr: impl PonyParser<super::Expr> + Clone) -> impl PonyParser<Self> + Clone {
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
        ));

        let (source, _) = SourceFile::test_file(r#"func ["arg1", "arg2", "arg3"]"#);
        let res = Expr::parser().parse(source.stream());
        assert!(matches!(res, Ok(Expr::Application(_))))
    }
}
