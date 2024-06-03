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

use crate::utils::{PonyParser, Span};

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct Application {
    span: Span,
    pub function: Box<super::Expr>,
    pub argument: Box<super::Expr>,
}

impl Application {
    pub fn parse_with(expr: impl PonyParser<super::Expr> + Clone) -> impl PonyParser<Self> + Clone {
        expr.clone()
            .map(Box::new)
            .then_ignore(just("[").not().rewind())
            .padded_by(text::whitespace())
            .then(expr.map(Box::new))
            .map_with_span(|(function, argument), span| Self {
                span,
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
        syntax::{application::Application, Expr},
        utils::{stream::SourceFile, Parseable},
    };

    #[test]
    fn tuples() {
        let (source, _) = SourceFile::test_file(r#"9.0px"#);
        let res = Expr::parser().parse(source.stream());
        assert!(matches!(
            res,
            Ok(Expr::Application(Application {
                function: box Expr::Literal(Literal::Number(NumberLit::Float(value))),
                argument: box Expr::Identifier(unit),
                ..
            }))
                if value.value == 9.0 && &unit == "px"
        ));
    }
}
