//!
//! ## AvPony File
//!
//! See [File].
//!
//!

use avpony_macros::Spanned;
use chumsky::{primitive::just, Parser};

use crate::{
    ponyx,
    utils::{Parseable, PonyParser, Span},
};

use super::external::External;

///
/// The root node of all syntax trees.
///
/// All AvPony files contain:
/// * A "module", written in an external language;
/// * A "fence" `---`, to seperate the sections;
/// * PonyX nodes, for UI.
///
#[derive(Debug, Clone, PartialEq, Spanned)]
pub struct File<Ext: External> {
    span: Span,
    module: Ext::Module,
    pony: ponyx::Node<Ext>,
}

impl<E: External + 'static> Parseable for File<E> {
    fn parser<'src>() -> impl PonyParser<'src, Self> {
        E::module()
            .then_ignore(just("---").padded())
            .then(ponyx::Node::parser())
            .map_with(|(module, pony), ctx| File {
                span: ctx.span(),
                module,
                pony,
            })
    }
}
