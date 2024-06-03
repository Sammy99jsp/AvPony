//!
//! ## Tuples
//! ### Grammar
//! ```text
//! tuple_expr := `(` (<expr>,)* `)`
//! ```

use avpony_macros::Spanned;
use chumsky::{
    primitive::just,
    text::{self, TextParser},
    Parser,
};

use crate::{
    lexical::punctuation,
    utils::{ParseableExt, PonyParser, Span},
};

use super::utils::Punctuated;

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct Tuple {
    span: Span,
    pub items: Vec<super::Expr>,
}

impl Tuple {
    pub fn parse_with(
        expr: impl PonyParser<super::Expr> + Clone,
    ) -> impl PonyParser<Self> + Clone {
        expr.clone()
            .padded_by(text::whitespace())
            .then_ignore(punctuation::Comma::parser().padded())
            .then(Punctuated::<_, punctuation::Comma>::optional_trailing_with(
                expr,
            ))
            .delimited_by(just("("), just(")"))
            .map_with_span(|(first, after), span| Self {
                span,
                items: { std::iter::once(first).chain(after.inner).collect() },
            })
    }
}

#[cfg(test)]
mod tests {
    use chumsky::Parser;

    use crate::{
        lexical::{number::NumberLit, Literal},
        syntax::{tuple::Tuple, Expr},
        utils::{stream::SourceFile, Parseable},
    };

    #[test]
    fn tuples() {
        let (source, _) = SourceFile::test_file(r#"(1,)"#);
        let res = Expr::parser().parse(source.stream());
        assert!(matches!(res, Ok(Expr::Tuple(Tuple {
            items,
            ..
        })) if matches!(items.as_slice(), [Expr::Literal(Literal::Number(NumberLit::Integer(_)))])));

        let (source, _) = SourceFile::test_file(r#"(1, 2, (1, 2), "abace\x1F")"#);
        let res = Expr::parser().parse(source.stream());
        assert!(matches!(res, Ok(Expr::Tuple(Tuple {
            items,
            ..
        })) if matches!(items.as_slice(), [
            Expr::Literal(Literal::Number(NumberLit::Integer(_))),
            Expr::Literal(Literal::Number(NumberLit::Integer(_))),
            Expr::Tuple(_),
            Expr::Literal(Literal::String(_)),
        ])));
    }
}
