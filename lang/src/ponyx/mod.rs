//!
//! ## PonyX Syntax
//!

use avpony_macros::Spanned;
use chumsky::{primitive::choice, recursive::recursive, Parser};
use entity::Entity;
use tag::Tag;
use text::Text;

#[cfg(test)]
use crate::syntax::external::TestLang;

use crate::{
    syntax::external::{External, ExternalExpr},
    utils::{ParseableCloned, PonyParser},
};

pub mod entity;
pub mod tag;
pub mod text;

#[derive(Debug, Clone, Spanned, PartialEq)]
pub enum Node<Ext: External> {
    Text(Text),
    Entity(Entity),
    Tag(Tag<Ext>),
    Mustache(ExternalExpr<Ext>),
}

impl<Ext: External + 'static> ParseableCloned for Node<Ext> {
    fn parser<'src>() -> impl PonyParser<'src, Self> + Clone {
        recursive(|node| {
            choice((
                Text::parser().map(Self::Text),
                Entity::parser().map(Self::Entity),
                Tag::parser_with(node.clone()).map(Self::Tag),
                ExternalExpr::parser().map(Self::Mustache),
            ))
        })
    }
}

#[cfg(test)]
pub type TNode = Node<TestLang>;
