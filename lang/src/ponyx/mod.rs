//!
//! ## PonyX Syntax
//!

use avpony_macros::Spanned;
use blocks::LogicBlock;
use chumsky::{
    primitive::{choice, just},
    recursive::recursive,
    Parser,
};
use comment::Comment;
use entity::Entity;
use statement::AtStatement;
use tag::Tag;
use text::Text;

#[cfg(test)]
use crate::syntax::external::TestLang;

use crate::{
    syntax::external::{External, ExternalExpr},
    utils::{ParseableCloned, PonyParser},
};

pub mod blocks;
pub mod comment;
pub mod entity;
pub mod statement;
pub mod tag;
pub mod text;

#[derive(Debug, Clone, Spanned, PartialEq)]
pub enum Node<Ext: External> {
    Comment(Comment),
    Text(Text),
    Entity(Entity),
    Mustache(ExternalExpr<Ext>),
    Statement(AtStatement<Ext>),
    Block(LogicBlock<Ext>),
    Tag(Tag<Ext>),
}

impl<Ext: External + 'static> ParseableCloned for Node<Ext> {
    fn parser<'src>() -> impl PonyParser<'src, Self> + Clone {
        recursive(|node| {
            choice((
                Entity::parser().map(Self::Entity),
                Text::parser().map(Self::Text),
                Tag::parser_with(node.clone()).map(Self::Tag),
                AtStatement::parser().map(Self::Statement),
                LogicBlock::parse_with(node.clone()).map(Self::Block),
                Comment::parser().map(Self::Comment),
                choice((just("{/"), just("{:")))
                    .not()
                    .rewind()
                    .ignore_then(ExternalExpr::parser())
                    .map(Self::Mustache),
            ))
        })
    }
}

#[cfg(test)]
pub type TNode = Node<TestLang>;
