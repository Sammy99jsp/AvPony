//!
//! ## Function Call
//!
//! ```grammar
//! call_expr := <expr function> `(` (<expr arguments>,)* `)`
//! ```
//!

use avpony_macros::Spanned;
use chumsky::{primitive::just, Parser};

use crate::{
    lexical::punctuation,
    utils::{errors::Error, Span},
};

use super::utils::Punctuated;

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct Call {
    span: Span,
    function: Box<super::Expr>,
    arguments: Punctuated<super::Expr, punctuation::Dot>,
}

impl Call {
    pub fn parse_with(
        expr: impl chumsky::Parser<char, super::Expr, Error = Error> + Clone,
    ) -> impl chumsky::Parser<char, Self, Error = Error> {
        expr.clone()
            .map(Box::new)
            .then(
                Punctuated::optional_trailing_with(expr.clone()).delimited_by(just("("), just(")")),
            )
            .map_with_span(|(function, arguments), span| Self {
                span,
                function,
                arguments,
            })
            .map_err(|err| {
                eprintln!("{err:?}");
                err
            })
    }
}

#[cfg(test)]
mod tests {
    use chumsky::Parser;

    use crate::{
        lexical::{number::NumberLit, Literal},
        syntax::{call::Call, utils::Punctuated, Expr},
        utils::{stream::SourceFile, Parseable},
    };

    #[test]
    fn call() {
        let (source, _) = SourceFile::test_file(r#"func("https://avdanos.org")"#);
        let res = Expr::parser().parse(source.stream());
        assert!(matches!(res,
            Ok(Expr::Call(Call {
                function: box Expr::Identifier(function),
                arguments: Punctuated { inner, ..},
                ..
            }))
                if &function == "func"
                && matches!(
                    inner.as_slice(),
                    [Expr::Literal(Literal::String(s))]
                        if s == "https://avdanos.org"
                )
        ));

        let (source, _) = SourceFile::test_file(r#"add(1.2, -10)"#);
        let res = Expr::parser().parse(source.stream());
        assert!(matches!(res,
            Ok(Expr::Call(Call {
                function: box Expr::Identifier(function),
                arguments: Punctuated { inner, ..},
                ..
            }))
                if &function == "add"
                && matches!(
                    inner.as_slice(),
                    [Expr::Literal(Literal::Number(NumberLit::Float(f))),
                    Expr::Literal(Literal::Number(NumberLit::Integer(i)))]
                        if f.value == 1.2 && i.value == -10
                )
        ));
    }
}
