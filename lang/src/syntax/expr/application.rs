//!
//! ## Application
//!
//! Haskell-style function calling.
//!
//! ### Grammar
//! ```text
//! appl_expr := <expr func> <expr arg>
//! ```
//!

use avpony_macros::Spanned;
use chumsky::{primitive::just, text, Parser};

use crate::{
    syntax::external::External,
    utils::{PonyParser, Span},
};

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct Application<Ext: External> {
    span: Span,
    pub function: Box<super::Expr<Ext>>,
    pub argument: Box<super::Expr<Ext>>,
}

impl<Ext: External> Application<Ext> {
    pub fn parse_with<'src>(
        expr: impl PonyParser<'src, super::Expr<Ext>> + Clone,
    ) -> impl PonyParser<'src, Self> + Clone {
        expr.clone()
            .map(Box::new)
            .then_ignore(just("[").not().rewind())
            .padded_by(text::whitespace())
            .then(expr.map(Box::new))
            .map_with(|(function, argument), ctx| Self {
                span: ctx.span(),
                function,
                argument,
            })
    }
}

#[cfg(test)]
mod tests {
    use chumsky::Parser;

    use crate::{
        lexical::{number::NumberLit, Literal},
        syntax::{application::Application, VExpr as Expr},
        utils::{Parseable, SourceFile},
    };

    #[test]
    fn tuples() {
        let (source, _) = SourceFile::test_file(r#"9.0px"#);
        let res = Expr::parser().parse(source.stream());
        assert!(matches!(
            res.into_result(),
            Ok(Expr::Application(Application {
                function: box Expr::Literal(Literal::Number(NumberLit::Float(value))),
                argument: box Expr::Identifier(unit),
                ..
            }))
                if value.value == 9.0 && &unit == "px"
        ));
    }
}
