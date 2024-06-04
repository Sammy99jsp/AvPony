//!
//! ## Text node.
//!
//! Literally anything except `{}<>`.
//!

use avpony_macros::Spanned;
use chumsky::{primitive::one_of, Parser};

use crate::utils::{ParseableExt, PonyParser, Span};

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct Text {
    span: Span,
    text: String,
}

impl ParseableExt for Text {
    fn parser() -> impl PonyParser<Self> + Clone {
        one_of("{}<>")
            .not()
            .repeated()
            .at_least(1)
            .map_with_span(|text: Vec<char>, span| Self {
                span,
                text: text.into_iter().collect(),
            })
    }
}

#[cfg(test)]
mod tests {
    use chumsky::Parser;

    use crate::{
        ponyx::text::Text,
        utils::{stream::SourceFile, Parseable},
    };

    #[test]
    fn text() {
        let (source, _) = SourceFile::test_file("    a \n  a <Apple>");
        assert!(
            matches!(Text::parser().parse(source.stream()), Ok(Text { text, .. }) if text == "    a \n  a ")
        );
    }
}
