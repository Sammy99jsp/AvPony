//!
//! ## Indexing
//! ### Grammar
//! ```text
//! index_expr := <expr receiver> `[` <expr index> `]`
//! ```
//!

use avpony_macros::Spanned;
use chumsky::{primitive::just, Parser};

use crate::utils::{PonyParser, Span};

use super::external::External;

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct Indexing<Ext: External> {
    span: Span,
    pub receiver: Box<super::Expr<Ext>>,
    pub index: Box<super::Expr<Ext>>,
}

impl<Ext: External> Indexing<Ext> {
    pub fn parse_with<'src>(
        expr: impl PonyParser<'src, super::Expr<Ext>> + Clone,
    ) -> impl PonyParser<'src, Self> + Clone {
        expr.clone()
            .then(expr.padded().delimited_by(just("["), just("]")))
            .map_with(|(receiver, index), ctx| Self {
                span: ctx.span(),
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
        syntax::{index::Indexing, VExpr as Expr},
        utils::{Parseable, SourceFile},
    };

    #[test]
    fn member_access() {
        let (source, _) = SourceFile::test_file(r#"dict["key"]"#);
        let res = Expr::parser().parse(source.stream());
        assert!(matches!(
            res.into_result(),
            Ok(Expr::Indexing(Indexing {
                receiver: box Expr::Identifier(ident),
                index: box Expr::Literal(Literal::String(key)),
                ..
            })) if (&ident == "dict") && (&key == "key")
        ));

        let (source, _) = SourceFile::test_file(r#"func ["arg1", "arg2", "arg3"]"#);
        let res = Expr::parser().parse(source.stream());
        assert!(matches!(res.into_result(), Ok(Expr::Application(_))))
    }
}
