//!
//! ## Debug Statements
//!
//! ```avpony
//! {#for item in basket by item.id}
//!     {@debug item} <!-- Item is logged to the console. -->
//! {/for}
//! ```
//!

use avpony_macros::Spanned;
use chumsky::{primitive::just, Parser};

use crate::{
    syntax::external::External,
    utils::{placeholder::Maybe, ParseableCloned, PonyParser, Span},
};

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct DebugStatement<Ext: External> {
    span: Span,
    expr: Maybe<Ext::Expression>,
}

impl<Ext: External> ParseableCloned for DebugStatement<Ext> {
    fn parser<'src>() -> impl PonyParser<'src, Self> + Clone {
        just("debug ")
            .ignore_then(Ext::expression().padded())
            .map_with(|expr, ctx| Self {
                span: ctx.span(),
                expr,
            })
    }
}
