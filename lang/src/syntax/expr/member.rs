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
    utils::{ParseableCloned, PonyParser, Span},
};

use super::external::External;

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct MemberAccess<Ext: External> {
    span: Span,
    pub receiver: Box<super::Expr<Ext>>,
    pub member: lexical::Identifier,
}

impl<Ext: External> MemberAccess<Ext> {
    pub fn parse_with<'src>(
        expr: impl PonyParser<'src, super::Expr<Ext>> + Clone,
    ) -> impl PonyParser<'src, Self> + Clone {
        expr.map(Box::new)
            .then_ignore(punctuation::Dot::parser())
            .then(lexical::Identifier::parser())
            .map_with(|(receiver, member), ctx| Self {
                span: ctx.span(),
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
        syntax::{member::MemberAccess, parenthesized::Parenthesized, VExpr as Expr},
        utils::{Parseable, SourceFile},
    };

    #[test]
    fn member_access() {
        let (source, _) = SourceFile::test_file(r#"(1.).to_string"#);
        let res = Expr::parser().parse(source.stream());
        assert!(matches!(
            res.into_result(),
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
