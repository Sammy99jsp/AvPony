//!
//! ## Tuples
//! ### Grammar
//! ```text
//! tuple_expr := `(` (<expr>,)* `)`
//! ```

use avpony_macros::Spanned;
use chumsky::{
    primitive::just,
    text::{self},
    Parser,
};

use crate::{
    lexical::punctuation,
    utils::{ParseableCloned, PonyParser, Span},
};

use super::{external::External, utils::Punctuated};

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct Tuple<Ext: External> {
    span: Span,
    pub items: Vec<super::Expr<Ext>>,
}

impl<Ext: External> Tuple<Ext> {
    pub fn parse_with<'src>(
        expr: impl PonyParser<'src, super::Expr<Ext>> + Clone,
    ) -> impl PonyParser<'src, Self> + Clone {
        expr.clone()
            .padded_by(text::whitespace())
            .then_ignore(punctuation::Comma::parser().padded())
            .then(Punctuated::<_, punctuation::Comma>::optional_trailing_with(
                expr,
            ))
            .delimited_by(just("("), just(")"))
            .map_with(|(first, after), ctx| Self {
                span: ctx.span(),
                items: { std::iter::once(first).chain(after.inner).collect() },
            })
    }
}

#[cfg(test)]
mod tests {
    use chumsky::Parser;

    use crate::{
        lexical::{number::NumberLit, Literal},
        syntax::{tuple::Tuple, VExpr as Expr},
        utils::{Parseable, SourceFile},
    };

    #[test]
    fn tuples() {
        let (source, _) = SourceFile::test_file(r#"(1,)"#);
        let res = Expr::parser().parse(source.stream());
        assert!(matches!(res.into_result(), Ok(Expr::Tuple(Tuple {
            items,
            ..
        })) if matches!(items.as_slice(), [Expr::Literal(Literal::Number(NumberLit::Integer(_)))])));

        let (source, _) = SourceFile::test_file(r#"(1, 2, (1, 2), "abace\x1F")"#);
        let res = Expr::parser().parse(source.stream());
        assert!(matches!(res.into_result(), Ok(Expr::Tuple(Tuple {
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
