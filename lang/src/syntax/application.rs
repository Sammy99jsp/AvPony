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
use chumsky::{text, Parser};

use crate::utils::{errors::Error, Span};

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct Application {
    span: Span,
    function: Box<super::Expr>,
    argument: Box<super::Expr>,
}

impl Application {
    pub fn parse_with(
        expr: impl chumsky::Parser<char, super::Expr, Error = Error> + Clone,
    ) -> impl chumsky::Parser<char, Self, Error = Error> {
        expr.clone()
            .map(Box::new)
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
