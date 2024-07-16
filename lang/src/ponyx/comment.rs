//!
//! ## HTML-like Comments
//! ```avpony
//! <!-- THIS IS A COMMENT -->
//! ```
//!

use avpony_macros::Spanned;
use chumsky::{
    primitive::{any, just},
    IterParser, Parser,
};

use crate::utils::{ParseableCloned, PonyParser, Span};

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct Comment {
    span: Span,
    content: String,
}

impl ParseableCloned for Comment {
    fn parser<'src>() -> impl PonyParser<'src, Self> + Clone {
        just("<!--")
            .ignore_then(
                any()
                    .and_is(just("-->").not())
                    .repeated()
                    .collect::<String>(),
            )
            .then_ignore(just("-->"))
            .map_with(|content, ctx| Self {
                span: ctx.span(),
                content,
            })
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

    use chumsky::Parser;

    use crate::{
        ponyx::{comment::Comment, Node},
        syntax::external::typescript::TypeScript,
        utils::{Parseable, SourceFile},
    };

    #[test]
    fn text() {
        let (source, _) = SourceFile::test_file("<!-- ANY COMMENTS? -->");
        let res = Node::<TypeScript>::parser().parse(source.stream());

        assert_matches!(res.output(), Some(Node::Comment(Comment { content, .. })) if content == " ANY COMMENTS? ");
    }
}
