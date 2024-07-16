//!
//! ## For Blocks
//!
//! ```avpony
//! {#for <ident> in <external_expr>}
//!     {ident}
//! {/for}
//! ```
//! ### Opening
//! Optional (recommended) key syntax: `{#for <ident> in <external_expr> by <external_expr using ident>}`.
//!
//! ### Leaves
//! * `{:else}` -- Upon empty iterator.
//!
//!

use avpony_macros::Spanned;
use chumsky::{primitive::just, IterParser, Parser};

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
pub struct ForBlock<Ext: External> {
    span: Span,
    pub ident: Maybe<lexical::Identifier>,
    pub iter: Maybe<Ext::Expression>,
    pub key: Option<Maybe<Ext::Expression>>,
    pub children: Vec<Node<Ext>>,
    pub empty_case: Option<Vec<Node<Ext>>>,
}

impl<Ext: External + 'static> ForBlock<Ext> {
    pub fn parse_with<'src>(
        node: impl PonyParser<'src, Node<Ext>> + Clone,
    ) -> impl PonyParser<'src, Self> + Clone {
        let opening = just("#for ")
            .ignore_then(lexical::Identifier::parser().padded().maybe())
            .then_ignore(just("in").padded())
            .then(Ext::expression().padded())
            .then(just("by").ignore_then(Ext::expression().padded()).or_not())
            .delimited_by(just("{"), just("}"))
            .padded()
            .then(node.clone().repeated().collect::<Vec<_>>());

        let opt_empty_case = just(":else")
            .padded()
            .delimited_by(just("{"), just("}"))
            .padded()
            .ignore_then(node.repeated().collect::<Vec<_>>())
            .or_not();

        let closing = just("/for")
            .padded()
            .delimited_by(just("{"), just("}"))
            .padded()
            .ignored();

        opening.then(opt_empty_case).then_ignore(closing).map_with(
            |((((ident, iter), key), children), empty_case), ctx| Self {
                span: ctx.span(),
                ident,
                iter,
                key,
                children,
                empty_case,
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

    use chumsky::Parser;

    use crate::{
        ponyx::{
            blocks::{for_block::ForBlock, LogicBlock},
            Node,
        },
        syntax::external::typescript::TypeScript,
        utils::{placeholder::Maybe, Parseable, SourceFile},
    };

    #[test]
    fn for_block() {
        let (source, _) = SourceFile::test_file(
            r#"{#for item in [1,2,2]}
        {item.id}{/for}"#,
        );
        let res = Node::<TypeScript>::parser().parse(source.stream());

        assert!(!res.has_errors() && res.has_output());

        assert_matches!(
            res.output(),
            Some(Node::Block(LogicBlock::For(ForBlock { ident: Maybe::Present(ident), iter: Maybe::Present(_), key: None, children, empty_case: None, .. })))
                if matches!(children.as_slice(), [
                Node::Mustache(_),
            ]) && ident == "item"
        );

        let (source, _) =
            SourceFile::test_file(r#"{#for dog in kennel by dog.name}{item.id}{/for}"#);
        let res = Node::<TypeScript>::parser().parse(source.stream());

        assert!(!res.has_errors() && res.has_output());

        assert_matches!(
            res.output(),
            Some(Node::Block(LogicBlock::For(ForBlock { ident: Maybe::Present(ident), iter: Maybe::Present(_), key: Some(_), children, empty_case: None, .. })))
                if matches!(children.as_slice(), [
                Node::Mustache(_),
            ]) && ident == "dog"
        );
        let (source, _) =
            SourceFile::test_file(r#"{#for dog in kennel by dog.name}{item.id}{:else} A {/for}"#);
        let res = Node::<TypeScript>::parser().parse(source.stream());

        assert!(!res.has_errors() && res.has_output());

        assert_matches!(
            res.output(),
            Some(Node::Block(LogicBlock::For(ForBlock { ident: Maybe::Present(ident), iter: Maybe::Present(_), key: Some(_), children, empty_case: Some(empty_case), .. })))
                if matches!(children.as_slice(), [
                    Node::Mustache(_),
                ])
                && ident == "dog"
                && matches!(empty_case.as_slice(), [
                    Node::Text(_),
                ])

        );
    }
}
