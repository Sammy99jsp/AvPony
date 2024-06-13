//!
//! # Boolean literals
//!
//! bool_lit = `true` | `false`
//!

use avpony_macros::Spanned;
use chumsky::{primitive::choice, Parser};

use crate::utils::ParseableCloned;

use super::keyword;

#[derive(Debug, Clone, Spanned, PartialEq)]
pub enum BooleanLit {
    False(keyword::False),
    True(keyword::True),
}

impl ParseableCloned for BooleanLit {
    fn parser<'src>() -> impl crate::utils::PonyParser<'src, Self> + Clone {
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
        utils::{Parseable, SourceFile},
    };

    #[test]
    fn parse_bool() {
        let (source, _) = SourceFile::test_file("true");
        assert!(matches!(
            BooleanLit::parser().parse(source.stream()).into_result(),
            Ok(BooleanLit::True(_))
        ));

        let (source, _) = SourceFile::test_file("false");
        assert!(matches!(
            BooleanLit::parser().parse(source.stream()).into_result(),
            Ok(BooleanLit::False(_))
        ));

        let (source, _) = SourceFile::test_file("fafsdbhjjlse");
        assert!(BooleanLit::parser().parse(source.stream()).has_errors());
    }
}
