//!
//! ## PonyX Syntax
//!

use avpony_macros::Spanned;
use entity::Entity;
use text::Text;

pub mod entity;
pub mod tags;
pub mod text;

#[derive(Debug, Clone, Spanned, PartialEq)]
pub enum Node {
    Text(Text),
}
