//!
//! ## Member Access expression.
//!
//! ### Grammar
//! ```text
//! member_access := <expr receiver> `.` <ident member>
//! ```
//!

use avpony_macros::Spanned;
use chumsky::{primitive::just, Parser};

use crate::{
    lexical,
    utils::{
        placeholder::{Maybe, MaybeParser},
        ParseableCloned, PonyParser, Span,
    },
};

use super::{external::External, utils::Accessor};

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct MemberAccess<Ext: External> {
    pub span: Span,
    pub receiver: Box<super::Expr<Ext>>,
    pub member: Maybe<lexical::Identifier>,
}

impl<Ext: External> MemberAccess<Ext> {
    pub(super) fn partial<'src>() -> impl PonyParser<'src, Accessor<Ext>> + Clone {
        just(".")
            .ignore_then(lexical::Identifier::parser().maybe())
            .map_with(|member, ctx| Accessor::Member(member, ctx.span()))
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
        syntax::{
            member::MemberAccess, parenthesized::Parenthesized, VExpr as Expr,
            VSoloExpr as SoloExpr,
        },
        utils::{placeholder::Maybe, Parseable, SourceFile},
    };

    #[test]
    fn member_access() {
        let (source, _) = SourceFile::test_file(r#"(a.)"#);
        let res = SoloExpr::parser().parse(source.stream());
        assert!(matches!(
            res.output(),
            Some(SoloExpr::Parenthesised(Parenthesized {inner: box Expr::MemberAccess(MemberAccess {
                receiver:
                    box Expr::Identifier(base),
                member: Maybe::Placeholder(_),
                ..
            }), ..})) if base == "a"
        ));

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
                member: Maybe::Present(member),
                ..
            })) if &member == "to_string"
        ));

        let (source, _) = SourceFile::test_file(r#"a."#);
        let res = Expr::parser().parse(source.stream());
        assert!(matches!(
            res.output(),
            Some(Expr::MemberAccess(MemberAccess {
                receiver:
                    box Expr::Identifier(base),
                member: Maybe::Placeholder(_),
                ..
            })) if base == "a"
        ));
    }
}
