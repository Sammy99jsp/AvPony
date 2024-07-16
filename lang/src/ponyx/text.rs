//!
//! ## Text node.
//!
//! Literally anything except any of `{}<>&`.
//!

use avpony_macros::Spanned;
use chumsky::{
    primitive::{any, one_of},
    IterParser, Parser,
};

use crate::utils::{ParseableCloned, PonyParser, Span};

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct Text {
    span: Span,
    pub text: String,
}

impl ParseableCloned for Text {
    fn parser<'src>() -> impl PonyParser<'src, Self> + Clone {
        any()
            .and_is(one_of("{}<>&").not())
            .repeated()
            .at_least(1)
            .collect()
            .map_with(|text: Vec<_>, ctx| Self {
                span: ctx.span(),
                text: text.into_iter().collect(),
            })
    }
}

#[cfg(test)]
mod tests {
    use chumsky::{primitive::any, Parser};

    use crate::{
        ponyx::text::Text,
        utils::{Parseable, SourceFile},
    };

    #[test]
    fn text() {
        let (source, _) = SourceFile::test_file("    a \n  a <Apple>");
        let res = Text::parser()
            .then_ignore(any().repeated())
            .parse(source.stream());
        assert!(matches!(res.into_result(), Ok(Text { text, .. }) if text == "    a \n  a "));
    }
}
