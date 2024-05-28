//!
//! # Boolean literals
//!
//! bool_lit = `true` | `false`
//!

use avpony_macros::Spanned;
use chumsky::{primitive::choice, Parser};

use crate::utils::{errors::Error, Parseable};

use super::keyword;

#[derive(Debug, Clone, Spanned, PartialEq)]
pub enum BooleanLit {
    False(keyword::False),
    True(keyword::True),
}

impl Parseable for BooleanLit {
    fn parser() -> impl chumsky::Parser<char, Self, Error = Error> {
        choice((
            keyword::False::parser().map(Self::False),
            keyword::True::parser().map(Self::True),
        ))
    }
}

#[cfg(test)]
mod tests {
    use chumsky::Parser;

    use crate::{
        lexical::boolean::BooleanLit,
        utils::{stream::SourceFile, Parseable},
    };

    #[test]
    fn parse_bool() {
        let (source, _) = SourceFile::test_file("true");
        assert!(matches!(
            BooleanLit::parser().parse(source.stream()),
            Ok(BooleanLit::True(_))
        ));

        let (source, _) = SourceFile::test_file("false");
        assert!(matches!(
            BooleanLit::parser().parse(source.stream()),
            Ok(BooleanLit::False(_))
        ));

        let (source, _) = SourceFile::test_file("fafsdbhjjlse");
        assert!(BooleanLit::parser().parse(source.stream()).is_err());
    }
}
