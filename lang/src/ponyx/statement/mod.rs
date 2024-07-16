//!
//! ## At-Statements
//!
//! Allows for variable declarations, debug (print) statements.
//!

pub mod debug_statement;

use avpony_macros::Spanned;
use chumsky::{
    primitive::{choice, just},
    Parser,
};
use debug_statement::DebugStatement;

use crate::{
    syntax::external::External,
    utils::{ParseableCloned, PonyParser},
};

#[derive(Debug, Clone, Spanned, PartialEq)]
pub enum AtStatement<Ext: External> {
    Let(Ext::LetDeclaration),
    Const(Ext::ConstDeclaration),
    Debug(DebugStatement<Ext>),
}

impl<Ext: External> ParseableCloned for AtStatement<Ext> {
    fn parser<'src>() -> impl PonyParser<'src, Self> + Clone {
        just("@")
            .ignore_then(choice((
                Ext::let_declaration().map(Self::Let),
                Ext::const_declaration().map(Self::Const),
                DebugStatement::parser().map(Self::Debug),
            )))
            .delimited_by(just("{"), just("}"))
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

    use chumsky::Parser;

    use crate::{
        ponyx::{statement::AtStatement, Node},
        syntax::external::typescript::TypeScript,
        utils::{Parseable, SourceFile},
    };

    #[test]
    fn test_statements() {
        let (source, _) =
            SourceFile::test_file("{@let square_numbers = [1, 4, 9, 16, 25, 36, 49, 64]}");
        let res = Node::<TypeScript>::parser().parse(source.stream());
        assert!(res.has_output() && !res.has_errors());
        assert_matches!(res.output(), Some(Node::Statement(AtStatement::Let(_))));

        let (source, _) = SourceFile::test_file("{@const { a } = {a: 1}}");
        let res = Node::<TypeScript>::parser().parse(source.stream());
        assert!(res.has_output() && !res.has_errors());
        assert_matches!(res.output(), Some(Node::Statement(AtStatement::Const(_))));

        let (source, _) = SourceFile::test_file("{@debug deeply.nested.object}");
        let res = Node::<TypeScript>::parser().parse(source.stream());
        assert!(res.has_output() && !res.has_errors());
        assert_matches!(res.output(), Some(Node::Statement(AtStatement::Debug(_))));
    }
}
