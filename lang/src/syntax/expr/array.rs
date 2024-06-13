//!
//! ## Arrays
//!
//! ### Syntax
//! arr := `[` (<expr> `,`?(end))* `]`
//!

use avpony_macros::Spanned;
use chumsky::{primitive::just, text, Parser};

use crate::{
    lexical::punctuation,
    utils::{PonyParser, Span},
};

use super::{external::External, utils::Punctuated};

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct Array<Ext: External> {
    span: Span,
    pub contents: Punctuated<super::Expr<Ext>, punctuation::Comma>,
}

impl<Ext: External> Array<Ext> {
    pub fn parse_with<'src>(
        inner: impl PonyParser<'src, super::Expr<Ext>> + Clone,
    ) -> impl PonyParser<'src, Self> + Clone {
        Punctuated::optional_trailing_with(inner)
            .delimited_by(
                just("[").then_ignore(text::whitespace()),
                text::whitespace().ignore_then(just("]")),
            )
            .map_with(|contents, ctx| Self {
                span: ctx.span(),
                contents,
            })
    }
}

#[cfg(test)]
mod tests {
    use chumsky::Parser;

    use crate::{
        lexical::{number::NumberLit, Literal},
        syntax::{array::Array, VExpr as Expr},
        utils::{Parseable, SourceFile},
    };

    #[test]
    fn array_parsing() {
        let (source, _) = SourceFile::test_file("[1,3,3]");
        let res = Expr::parser().parse(source.stream());
        assert!(matches!(
            res.into_result(),
            Ok(Expr::Array(Array { contents, ..})) if
                contents.iter().count() == 3
                && contents.iter().any(|item| matches!(item, Expr::Literal(Literal::Number(NumberLit::Integer(_)))))
        ));

        let (source, _) = SourceFile::test_file(
            r#"["apples", "bananas", "coconts", "dragonfruit", "elderberry"]"#,
        );
        let res = Expr::parser().parse(source.stream());

        assert!(matches!(
            res.into_result(),
            Ok(Expr::Array(Array { contents, ..})) if
                contents.iter().count() == 5
                && contents.iter().any(|item| matches!(item, Expr::Literal(Literal::String(_))))
        ));

        let (source, _) = SourceFile::test_file(
            r#"[ ["A", "B", "C",], 
                    [1, 2, 3], 
                    ["D", "E", "F"],
                    [4,5,6,],
                ]"#,
        );

        let res = Expr::parser().parse(source.stream());
        assert!(matches!(
            res.into_result(),
            Ok(Expr::Array(Array { contents, ..})) if
                contents.iter().count() == 4
                && contents.iter().any(|item| matches!(item, Expr::Array(_)))
        ));
    }
}
