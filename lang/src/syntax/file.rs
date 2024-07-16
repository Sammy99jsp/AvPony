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
            .then(ponyx::Node::parser().padded())
            .map_with(|(module, pony), ctx| File {
                span: ctx.span(),
                module,
                pony,
            })
    }
}

#[cfg(test)]
mod tests {
    use chumsky::Parser;

    use crate::{
        syntax::external::typescript::TypeScript,
        utils::{ErrorI, Parseable, SourceFile},
    };

    use super::File;

    #[test]
    fn parse_ts_file() {
        let src = r#"
        let i = 0;
        ---
        <Button
            primary
            on:click={() => i++}
        >
            Clicked {i} times!
        </Button>
        "#;

        let (source, cache) = SourceFile::test_file(src);

        let res = File::<TypeScript>::parser().parse(source.stream());

        let _ = res
            .into_errors()
            .into_iter()
            .try_for_each(|err| err.to_report().eprint(cache.clone()));
    }
}
