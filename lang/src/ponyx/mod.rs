//!
//! ## PonyX Syntax
//!

use avpony_macros::Spanned;
use chumsky::{primitive::choice, recursive::recursive, Parser};
use entity::Entity;
use tag::Tag;
use text::Text;

use crate::utils::{ParseableExt, PonyParser};

pub mod entity;
pub mod tag;
pub mod text;

#[derive(Debug, Clone, Spanned, PartialEq)]
pub enum Node {
    Text(Text),
    Entity(Entity),
    Tag(Tag),
}

impl ParseableExt for Node {
    fn parser() -> impl PonyParser<Self> + Clone {
        recursive(|node| {
            choice((
                Text::parser().map(Self::Text),
                Entity::parser().map(Self::Entity),
                Tag::parser_with(node.clone()).map(Self::Tag),
            ))
        })
    }
}
