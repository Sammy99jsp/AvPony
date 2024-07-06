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
use chumsky::{primitive::choice, recursive::recursive, Parser};

use crate::{
    syntax::external::External,
    utils::{PonyParser, Span},
};

use super::Expr;

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct Application<Ext: External> {
    pub span: Span,
    pub function: Box<super::Expr<Ext>>,
    pub argument: Box<super::Expr<Ext>>,
}

impl<Ext: External> Application<Ext> {
    pub fn with<'src>(
        singleton: impl PonyParser<'src, super::Expr<Ext>> + Clone + 'src,
    ) -> impl PonyParser<'src, Box<Expr<Ext>>> + Clone
    where
        Ext: 'static,
    {
        recursive(|app| {
            choice((
                singleton
                    .clone()
                    .padded()
                    .then(app)
                    .map_with(|(function, argument), ctx| {
                        Box::new(Expr::Application(Application {
                            span: ctx.span(),
                            function: Box::new(function),
                            argument,
                        }))
                    }),
                singleton.clone().map(Box::new),
            ))
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
