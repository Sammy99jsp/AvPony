//!
//! ## Await Blocks
//!
//! ```avpony
//! {#await <external_expr>}
//!     <!-- Pending Promise/Future -->
//! {/await}
//! ```
//!
//! ### Alternate Forms
//! * `{#await <external_expr> then <ident>}`
//! * `{#await <external_expr> catch <ident>}`
//!
//! ### Leaves
//! * `{:then <ident>}`
//! * `{:catch <ident>}`
//!

use std::iter::once;

use avpony_macros::Spanned;
use chumsky::{
    primitive::{choice, just},
    IterParser, Parser,
};

use crate::{
    lexical,
    ponyx::Node,
    syntax::external::External,
    utils::{
        placeholder::{Maybe, MaybeParser},
        ParseableCloned, PonyParser, Span,
    },
};

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct AwaitBlock<Ext: External> {
    span: Span,
    pub expr: Maybe<Ext::Expression>,
    pub branches: Vec<(Branch, Vec<Node<Ext>>)>,
}

impl<Ext: External + 'static> AwaitBlock<Ext> {
    pub fn parse_with<'src>(
        node: impl PonyParser<'src, Node<Ext>> + Clone,
    ) -> impl PonyParser<'src, Self> + Clone {
        let nodes = node.clone().repeated().collect::<Vec<_>>();

        let base = just("#await ").ignore_then(Ext::expression().padded());

        let pending = nodes.clone().map(|v| (Branch::Pending, v));

        let success_or_failure = just(":")
            .ignore_then(Branch::parser())
            .delimited_by(just("{"), just("}"))
            .padded()
            .then(nodes.clone());

        let with_leaves = base
            .clone()
            .delimited_by(just("{"), just("}"))
            .padded()
            .then(pending)
            .then(success_or_failure.repeated().collect::<Vec<_>>())
            .map_with(|((expr, pending), other_branches), ctx| Self {
                span: ctx.span(),
                expr,
                branches: once(pending).chain(other_branches).collect::<Vec<_>>(),
            });

        let inline = base
            .then(Branch::parser())
            .delimited_by(just("{"), just("}"))
            .padded()
            .then(nodes)
            .map_with(|((expr, branch), children), ctx| Self {
                span: ctx.span(),
                expr,
                branches: Vec::from([(branch, children)]),
            });

        choice((with_leaves, inline)).then_ignore(
            just("/await")
                .padded()
                .delimited_by(just("{"), just("}"))
                .padded(),
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Branch {
    Pending,
    Success(Maybe<lexical::Identifier>),
    Failure(Maybe<lexical::Identifier>),
}

impl Branch {
    fn parser<'src>() -> impl PonyParser<'src, Branch> + Clone {
        choice((
            just("then ")
                .ignore_then(lexical::Identifier::parser().maybe())
                .map(Self::Success),
            just("catch ")
                .ignore_then(lexical::Identifier::parser().maybe())
                .map(Self::Failure),
        ))
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

    use chumsky::Parser;

    use crate::{
        ponyx::{
            blocks::{
                await_block::{AwaitBlock, Branch},
                LogicBlock,
            },
            Node,
        },
        syntax::external::typescript::TypeScript,
        utils::{placeholder::Maybe, Parseable, SourceFile},
    };

    #[test]
    fn await_block() {
        let (source, _) = SourceFile::test_file(
            r#"{#await (1).toString()}
    Loading...
{:then res}
    {res}
{:catch err}
    <span color="red">Error:</span> {err.message}
{/await}
"#,
        );
        let res = Node::<TypeScript>::parser().parse(source.stream());
        assert!(res.has_output() && !res.has_errors());
        assert_matches!(
            res.output(),
            Some(Node::Block(LogicBlock::Await(AwaitBlock { branches, .. })))
            if matches!(branches.as_slice(), [
                (Branch::Pending, _),
                (Branch::Success(Maybe::Present(ident1)), _),
                (Branch::Failure(Maybe::Present(ident2)), _),
            ] if ident1 == "res" && ident2 == "err")
        );

        let (source, _) = SourceFile::test_file(
            r#"{#await api.posts.latest() then posts}
    {#for post in posts by post.id}
        <Post id={post.id} content={post.content} />
    {/for}
{/await}
"#,
        );
        let res = Node::<TypeScript>::parser().parse(source.stream());
        assert!(res.has_output() && !res.has_errors());

        assert_matches!(
            res.output(),
            Some(Node::Block(LogicBlock::Await(AwaitBlock { branches, .. })))
            if matches!(branches.as_slice(), [
                (Branch::Success(Maybe::Present(ident1)), children),
            ]
                if ident1 == "posts"
                && matches!(children.as_slice(), [
                    Node::Block(LogicBlock::For(_)),
                ])
            )
        );
    }
}
