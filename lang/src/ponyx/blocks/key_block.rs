//!
//! ## Key Blocks
//! ```avpony
//! {#key <external_expr>}
//!     <!-- Re-renders if the expression changes value. -->
//! {/await}
//! ```
//!

use avpony_macros::Spanned;
use chumsky::{primitive::just, IterParser, Parser};

use crate::{
    ponyx::Node,
    syntax::external::External,
    utils::{placeholder::Maybe, PonyParser, Span},
};

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct KeyBlock<Ext: External> {
    span: Span,
    expr: Maybe<Ext::Expression>,
    children: Vec<Node<Ext>>,
}

impl<Ext: External + 'static> KeyBlock<Ext> {
    pub fn parse_with<'src>(
        node: impl PonyParser<'src, Node<Ext>> + Clone,
    ) -> impl PonyParser<'src, Self> + Clone {
        just("#key ")
            .ignore_then(Ext::expression().padded())
            .delimited_by(just("{"), just("}"))
            .padded()
            .then(node.repeated().collect::<Vec<_>>())
            .then_ignore(just("/key").delimited_by(just("{"), just("}")).padded())
            .map_with(|(expr, children), ctx| Self {
                span: ctx.span(),
                expr,
                children,
            })
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

    use chumsky::Parser;

    use crate::{
        ponyx::{
            blocks::{key_block::KeyBlock, LogicBlock},
            Node,
        },
        syntax::external::typescript::TypeScript,
        utils::{Parseable, SourceFile},
    };

    #[test]
    fn key_block() {
        let (source, _) = SourceFile::test_file(
            r#"{#key props.rerender == true}
    Re-rendered :)
{/key}"#,
        );

        let res = Node::<TypeScript>::parser().parse(source.stream());
        assert!(res.has_output() && !res.has_errors());
        assert_matches!(
            res.output(),
            Some(Node::Block(LogicBlock::Key(KeyBlock {
                children,
                ..
            })))
            if matches!(children.as_slice(), [
                Node::Text(_)
            ])
        )
    }
}
