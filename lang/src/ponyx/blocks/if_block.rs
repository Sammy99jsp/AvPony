//!
//! ## If Blocks
//!
//! ```avpony
//! {#if <external_expr>}
//!     A
//! {:else if <external_expr>}
//!     B
//! {:else}
//!     C
//! {/if}
//! ```
//!
//! ### Leaves
//! * `{:else if <external_expr>} <node*>`
//! * `{:else}`
//!
//!
use std::iter::once;

use avpony_macros::Spanned;
use chumsky::{
    primitive::{choice, just},
    text, IterParser, Parser,
};

use crate::{
    ponyx::Node,
    syntax::external::External,
    utils::{error::blocks::UnreachableBranch, placeholder::Maybe, PonyParser, Span, Spanned},
};

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct IfBranch<Ext: External> {
    span: Span,
    expr: Maybe<Ext::Expression>,
    contents: Vec<Node<Ext>>,
}

impl<Ext: External + 'static> IfBranch<Ext> {
    fn parse_with<'src>(
        node: impl PonyParser<'src, Node<Ext>> + Clone,
    ) -> impl PonyParser<'src, Self> + Clone {
        just("#if ")
            .ignore_then(Ext::expression().padded())
            .delimited_by(just("{"), just("}"))
            .then(node.repeated().collect().padded())
            .map_with(|(expr, contents), ctx| Self {
                span: ctx.span(),
                expr,
                contents,
            })
    }
}

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct ElseIfBranch<Ext: External> {
    span: Span,
    expr: Maybe<Ext::Expression>,
    contents: Vec<Node<Ext>>,
}

impl<Ext: External + 'static> ElseIfBranch<Ext> {
    fn parse_with<'src>(
        node: impl PonyParser<'src, Node<Ext>> + Clone,
    ) -> impl PonyParser<'src, Self> + Clone {
        just(":else if ")
            .ignore_then(Ext::expression())
            .delimited_by(just("{"), just("}"))
            .then(node.repeated().collect().padded())
            .map_with(|(expr, contents), ctx| Self {
                span: ctx.span(),
                expr,
                contents,
            })
    }
}

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct ElseBranch<Ext: External> {
    span: Span,
    contents: Vec<Node<Ext>>,
}

impl<Ext: External + 'static> ElseBranch<Ext> {
    fn parse_with<'src>(
        node: impl PonyParser<'src, Node<Ext>> + Clone,
    ) -> impl PonyParser<'src, Self> + Clone {
        just(":else")
            .ignore_then(text::whitespace())
            .ignored()
            .delimited_by(just("{"), just("}"))
            .ignore_then(node.repeated().collect().padded())
            .map_with(|contents, ctx| Self {
                span: ctx.span(),
                contents,
            })
    }
}

#[derive(Debug, Clone, Spanned, PartialEq)]
pub enum Branch<Ext: External> {
    If(IfBranch<Ext>),
    ElseIf(ElseIfBranch<Ext>),
    Else(ElseBranch<Ext>),
}

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct IfBlock<Ext: External> {
    span: Span,
    branches: Vec<Branch<Ext>>,
}

impl<Ext: External + 'static> IfBlock<Ext> {
    pub fn parse_with<'src>(
        node: impl PonyParser<'src, Node<Ext>> + Clone,
    ) -> impl PonyParser<'src, Self> + Clone {
        IfBranch::parse_with(node.clone())
            .padded()
            .map(Branch::If)
            .then(
                choice((
                    ElseIfBranch::parse_with(node.clone())
                        .padded()
                        .map(Branch::ElseIf),
                    ElseBranch::parse_with(node).padded().map(Branch::Else),
                ))
                .repeated()
                .collect::<Vec<Branch<Ext>>>(),
            )
            .then_ignore(
                just("/if")
                    .padded()
                    .delimited_by(just("{"), just("}"))
                    .padded(),
            )
            .validate(|(start, branches): (_, Vec<Branch<Ext>>), ctx, emitter| {
                match branches.as_slice() {
                    [] => (),
                    [_] => (),
                    branches => {
                        // Do a small optimisation, to not include any branches
                        // after an `{:else}` (if any exist).
                        let branches_before_after_else =
                            branches.split_once(|branch| matches!(branch, Branch::Else(_)));

                        // Give a warning for unreachable code.
                        match branches_before_after_else {
                            Some((before, after)) if !after.is_empty() => {
                                let span = after
                                    .iter()
                                    .map(|branch| branch.span())
                                    .reduce(|acc, b| acc.combine(b).unwrap())
                                    .unwrap();

                                emitter.emit(UnreachableBranch::new(span, ctx.span()).into());

                                // Get all branches before (and including) the `{:else}`.
                                let branches = branches[..=before.len()].to_owned();
                                return (start, branches);
                            }
                            _ => (),
                        }
                    }
                }

                (start, branches)
            })
            .map_with(|(opening, branches), ctx| Self {
                span: ctx.span(),
                branches: once(opening).chain(branches).collect(),
            })
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

    use chumsky::Parser;

    use crate::{
        ponyx::{
            blocks::{
                if_block::{Branch, IfBlock},
                LogicBlock,
            },
            Node,
        },
        syntax::external::typescript::TypeScript,
        utils::{Error, Parseable, SourceFile},
    };

    #[test]
    fn if_block() {
        let (source, _) = SourceFile::test_file(
            r#"{#if 1}
    Hello, world.  &nbsp;
{:else if a == 0}
    Don't stop here.
{:else}
    NOPE.
{/if}"#,
        );
        let res = Node::<TypeScript>::parser().parse(source.stream());

        assert!(res.has_output() && !res.has_errors());

        assert_matches!(
            res.output(),
            Some(Node::Block(LogicBlock::If(IfBlock { branches, .. }))) if matches!(branches.as_slice(), [
                Branch::If(_),
                Branch::ElseIf(_),
                Branch::Else(_),
            ])
        );

        let (source, _) = SourceFile::test_file(
            r#"{#if 1}
    Hello, world.  &nbsp;
{:else}
    NOPE.
{:else if a == 0}
    Don't stop here.
{/if}"#,
        );
        let res = Node::<TypeScript>::parser().parse(source.stream());

        assert!(res.has_output() && res.has_errors());

        assert_matches!(
            res.output(),
            Some(Node::Block(LogicBlock::If(IfBlock { branches, .. }))) if matches!(branches.as_slice(), [
                Branch::If(_),
                Branch::Else(_),
            ])
        );

        assert_matches!(res.errors().next(), Some(Error::UnreachableBranch(_)));
    }
}
