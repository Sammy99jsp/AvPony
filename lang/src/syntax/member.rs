//!
//! ## Member Access expression.
//!
//! ### Grammar
//! ```text
//! member_access := <expr receiver> `.` <ident member>
//! ```
//!

use avpony_macros::Spanned;
use chumsky::Parser;

use crate::{
    lexical::{self, punctuation},
    utils::{errors::Error, Parseable, Span},
};

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct MemberAccess {
    pub span: Span,
    pub receiver: Box<super::Expr>,
    pub member: lexical::Identifier,
}

impl MemberAccess {
    pub fn parse_with(
        expr: impl chumsky::Parser<char, super::Expr, Error = Error>,
    ) -> impl chumsky::Parser<char, Self, Error = Error> {
        expr.map(Box::new)
            .then_ignore(punctuation::Dot::parser())
            .then(lexical::Identifier::parser())
            .map_with_span(|(receiver, member), span| Self {
                span,
                receiver,
                member,
            })
    }
}

#[cfg(test)]
mod tests {
    use chumsky::Parser;

    use crate::{
        lexical::{
            number::{FloatLit, NumberLit},
            Literal,
        },
        syntax::{member::MemberAccess, parenthesized::Parenthesized, Expr},
        utils::{stream::SourceFile, Parseable},
    };

    #[test]
    fn member_access() {
        let (source, _) = SourceFile::test_file(r#"(1.).to_string"#);
        let res = Expr::parser().parse(source.stream());
        assert!(matches!(
            res,
            Ok(Expr::MemberAccess(MemberAccess {
                receiver:
                    box Expr::Parenthesised(Parenthesized {
                        inner:
                            box Expr::Literal(Literal::Number(NumberLit::Float(FloatLit {
                                value: 1.0,
                                ..
                            }))),
                        ..
                    }),
                member,
                ..
            })) if &member == "to_string"
        ));
    }
}
