//!
//! ## Logic Blocks
//!
//! In AvPony, you can use logic blocks, like other templating languages.
//!
//! Blocks use the `{#KEYWORD ...}` syntax, and often have helpers
//! called 'leaves' that denote sub-sections.
//!
//!
//! ### If blocks
//! ```avpony
//! {#if external_condition}
//!   Condition is true.
//! {/if}
//! ```
//!
//! Also features `{:else if}`, `{:else}` leaves.
//!
//! ### For-in blocks
//! ```avpony
//! {#for <ident> in <external_expr> (by <external_expr>)?} <!-- Optional `by` key -->
//!   <-- Per item -->
//! {:else}
//!   <-- No items! -->
//! {/for}
//! ```
//!
//! Also features `{:else}` for empty iterators.
//!
//! ### Await blocks
//! ```avpony
//! {#await promise_or_future then result}
//!   Your action was {result.text}.
//! {/await}
//! ```
//!
//! Also features `{:then}`, and `{:catch}` to divide loading/finished, and error states.
//!
//!
//! ### Key blocks
//! ```avpony
//! {#key <external_expr>}
//!   <!-- When the external expression changes. -->
//! {/key}
//! ```
//!
//! ### Match blocks (Proposal)
//! For TypeScript, potentially look at using
//! [TC39 Pattern Matching Proposal](https://tc39.es/proposal-pattern-matching/#sec-pattern-matching).
//!
//! ```avpony
//! {#match expr}
//!   {:when 1}
//!     You have a new friend.
//!   {:default f}
//!     You have {f} friends.
//! {/match}
//! ```
//!
//! Leaves: `{:when <refutable pat>}`, `{:default <non-refutable pat>}`
//!

pub mod await_block;
pub mod for_block;
pub mod if_block;
pub mod key_block;

use avpony_macros::Spanned;
use await_block::AwaitBlock;
use chumsky::{primitive::choice, Parser};
use for_block::ForBlock;
use if_block::IfBlock;
use key_block::KeyBlock;

use crate::{syntax::external::External, utils::PonyParser};

use super::Node;

pub const KEYWORDS: &[&str] = &["if", "match", "await"];

#[derive(Debug, Clone, Spanned, PartialEq)]
pub enum LogicBlock<Ext: External> {
    If(IfBlock<Ext>),
    For(ForBlock<Ext>),
    Await(AwaitBlock<Ext>),
    Key(KeyBlock<Ext>),
}

impl<Ext: External + 'static> LogicBlock<Ext> {
    pub fn parse_with<'src>(
        node: impl PonyParser<'src, Node<Ext>> + Clone,
    ) -> impl PonyParser<'src, Self> + Clone {
        choice((
            ForBlock::parse_with(node.clone()).map(Self::For),
            IfBlock::parse_with(node.clone()).map(Self::If),
            AwaitBlock::parse_with(node.clone()).map(Self::Await),
            KeyBlock::parse_with(node.clone()).map(Self::Key),
        ))
    }
}
