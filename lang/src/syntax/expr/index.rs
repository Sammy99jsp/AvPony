//!
//! ## Indexing
//! ### Grammar
//! ```text
//! index_expr := <expr receiver> `[` <expr index> `]`
//! ```
//!

use avpony_macros::Spanned;
use chumsky::{primitive::just, Parser};

use crate::utils::{
    placeholder::{Maybe, MaybeParser},
    PonyParser, Span,
};

use super::{external::External, utils::Accessor};

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct Indexing<Ext: External> {
    pub span: Span,
    pub receiver: Box<super::Expr<Ext>>,
    pub index: Box<Maybe<super::Expr<Ext>>>,
}

impl<Ext: External> Indexing<Ext> {
    pub(super) fn partial_with<'src>(
        expr: impl PonyParser<'src, super::Expr<Ext>> + Clone,
    ) -> impl PonyParser<'src, Accessor<Ext>> + Clone {
        expr.maybe()
            .padded()
            .delimited_by(just("["), just("]"))
            .map_with(|idx, ctx| Accessor::Index(Box::new(idx), ctx.span()))
    }
}

#[cfg(test)]
mod tests {
    use chumsky::Parser;

    use crate::{
        lexical::Literal,
        syntax::{index::Indexing, parenthesized::Parenthesized, SoloExpr, VExpr as Expr},
        utils::{placeholder::Maybe, Parseable, SourceFile},
    };

    #[test]
    fn member_access() {
        let (source, _) = SourceFile::test_file(r#"dict["key"]"#);
        let res = Expr::parser().parse(source.stream());
        assert!(matches!(
            res.into_result(),
            Ok(Expr::Indexing(Indexing {
                receiver: box Expr::Identifier(ident),
                index: box Maybe::Present(Expr::Literal(Literal::String(key))),
                ..
            })) if (&ident == "dict") && (&key == "key")
        ));

        let (source, _) = SourceFile::test_file(r#"func ["arg1", "arg2", "arg3"]"#);
        let res = Expr::parser().parse(source.stream());
        assert!(matches!(res.into_result(), Ok(Expr::Application(_))));

        let (source, _) = SourceFile::test_file(r#"dict[]"#);
        let res = Expr::parser().parse(source.stream());
        assert!(
            res.has_errors()
                && matches!(
                    res.into_output(),
                    Some(Expr::Indexing(Indexing {
                        receiver: box Expr::Identifier(ident),
                        index: box Maybe::Placeholder(_),
                        ..
                    })) if (&ident == "dict")
                )
        );

        let (source, _) = SourceFile::test_file(r#"(dict[])"#);
        let res = SoloExpr::parser().parse(source.stream());
        println!("{res:?}");
        assert!(
            res.has_errors()
                && matches!(
                    res.into_output(),
                    Some(SoloExpr::Parenthesised(Parenthesized {inner: box Expr::Indexing(Indexing {
                        receiver: box Expr::Identifier(ident),
                        index: box Maybe::Placeholder(_),
                        ..
                    }),.. })) if (&ident == "dict")
                )
        );
    }
}
