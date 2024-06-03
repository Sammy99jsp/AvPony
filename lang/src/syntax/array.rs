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

use super::utils::Punctuated;

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct Array {
    span: Span,
    contents: Punctuated<super::Expr, punctuation::Comma>,
}

impl Array {
    pub fn parse_with(
        inner: impl PonyParser<super::Expr> + Clone,
    ) -> impl PonyParser<Self> + Clone {
        Punctuated::optional_trailing_with(inner)
            .delimited_by(
                just("[").then_ignore(text::whitespace()),
                text::whitespace().ignore_then(just("]")),
            )
            .map_with_span(|contents, span| Self { span, contents })
    }
}

#[cfg(test)]
mod tests {
    use chumsky::Parser;

    use crate::{
        lexical::{number::NumberLit, Literal},
        syntax::{array::Array, Expr},
        utils::{stream::SourceFile, Parseable},
    };

    #[test]
    fn array_parsing() {
        let (source, _) = SourceFile::test_file("[1,3,3]");
        let res = Expr::parser().parse(source.stream());

        assert!(matches!(
            res,
            Ok(Expr::Array(Array { contents, ..})) if
                contents.iter().count() == 3
                && contents.iter().any(|item| matches!(item, Expr::Literal(Literal::Number(NumberLit::Integer(_)))))
        ));

        let (source, _) = SourceFile::test_file(
            r#"["apples", "bananas", "coconts", "dragonfruit", "elderberry"]"#,
        );
        let res = Expr::parser().parse(source.stream());

        assert!(matches!(
            res,
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
            res,
            Ok(Expr::Array(Array { contents, ..})) if
                contents.iter().count() == 4
                && contents.iter().any(|item| matches!(item, Expr::Array(_)))
        ));
    }
}
