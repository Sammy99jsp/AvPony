//!
//! ## PonyX Tag Name
//!
//! The intention is for this to be language dependent:
//! * *TypeScript*: `Path.To.Component`
//! * *Rust*: `path::to::Component`
//!

use std::fmt::Display;

use avpony_macros::Spanned;
use chumsky::{primitive::just, Parser};

use crate::{
    lexical,
    utils::{ParseableExt, PonyParser, Span},
};

#[derive(Debug, Clone, Spanned)]
pub struct TagName {
    span: Span,
    path: Vec<lexical::Identifier>,
}

impl Display for TagName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.path
            .iter()
            .map(|a| a.value.as_str())
            .intersperse(".")
            .try_for_each(|a| write!(f, "{a}"))
    }
}

impl PartialEq<TagName> for TagName {
    fn eq(&self, other: &TagName) -> bool {
        self.path
            .iter()
            .zip(other.path.iter())
            .all(|(i1, i2)| i1.same_name_as(i2))
    }
}

impl PartialEq<str> for TagName {
    fn eq(&self, other: &str) -> bool {
        self.path
            .iter()
            .zip(other.split(":"))
            .all(|(ident, p)| ident == p)
    }
}

impl ParseableExt for TagName {
    fn parser() -> impl PonyParser<Self> + Clone {
        lexical::Identifier::parser()
            .then(
                just(".")
                    .ignore_then(lexical::Identifier::parser())
                    .repeated(),
            )
            .map_with_span(|(first, after), span| Self {
                span,
                path: std::iter::once(first).chain(after).collect(),
            })
    }
}
