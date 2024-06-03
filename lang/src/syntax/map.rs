//!
//! ## Maps
//!
//! ### Syntax
//!
//! ```text
//! field_key_value := `.` <ident key> `=` <expr value>
//! field_key := `.` <ident key>
//! field := field_key_value | field_key
//!
//! map := `(` (field),* `)`
//! ```
//!

use avpony_macros::Spanned;
use chumsky::{
    primitive::just,
    text::{self, TextParser},
    Parser,
};

use crate::{
    lexical::{identifier::Identifier, punctuation},
    utils::{ParseableExt, PonyParser, Span},
};

use super::utils::Punctuated;

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct Map {
    span: Span,
    pub fields: Punctuated<Field, punctuation::Comma>,
}

impl Map {
    pub fn parse_with(expr: impl PonyParser<super::Expr> + Clone) -> impl PonyParser<Self> + Clone {
        Punctuated::<_, punctuation::Comma>::optional_trailing_with(Field::parse_with(expr))
            .padded()
            .delimited_by(just("("), just(")"))
            .map_with_span(|fields, span| Self { span, fields })
    }
}

#[derive(Debug, Clone, Spanned, PartialEq)]
pub enum Field {
    Key(FieldKey),
    KeyValue(FieldKeyValue),
}

impl Field {
    fn parse_with(expr: impl PonyParser<super::Expr> + Clone) -> impl PonyParser<Self> + Clone {
        just(".")
            .ignore_then(Identifier::parser().then_ignore(text::whitespace()))
            .then(
                just("=")
                    .padded_by(text::whitespace())
                    .ignore_then(expr.map(Box::new).padded_by(text::whitespace()))
                    .or_not(),
            )
            .map_with_span(|(key, value), span| {
                match value {
                    Some(value) => Self::KeyValue(FieldKeyValue { span, key, value }),
                    None => Self::Key(FieldKey { span, ident: key }),
                }
                .clone()
            })
    }
}

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct FieldKey {
    span: Span,
    ident: Identifier,
}

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct FieldKeyValue {
    span: Span,
    key: Identifier,
    value: Box<super::Expr>,
}

#[cfg(test)]
mod tests {
    use chumsky::Parser;

    use crate::{
        lexical::Literal,
        syntax::{utils::Punctuated, Expr},
        utils::{stream::SourceFile, Parseable},
    };

    #[test]
    fn parse_map() {
        let (source, _) = SourceFile::test_file(r#"(.a, .if_="")"#);
        let res = Expr::parser().parse(source.stream());
        assert!(matches!(res,
            Ok(Expr::Map(super::Map { fields: Punctuated { inner, .. }, ..}))
                if matches!(inner.as_slice(), [
                    super::Field::Key(super::FieldKey { ident, ..}),
                    super::Field::KeyValue(super::FieldKeyValue { key, value: box Expr::Literal(Literal::String(_)), ..})
                ]
                    if ident == "a" && key == "if_"
        )));

        let (source, _) = SourceFile::test_file(r#"(.nested=(.a=1, .b=2),)"#);
        let res = Expr::parser().parse(source.stream());
        assert!(matches!(res,
        Ok(Expr::Map(super::Map { fields: Punctuated { inner, .. }, ..}))
            if matches!(inner.as_slice(), [
                super::Field::KeyValue(super::FieldKeyValue {
                    key,
                    value: box Expr::Map(super::Map {
                        fields: Punctuated { inner, ..},
                        ..
                    }),
                    ..
                })
            ]
                if key == "nested" &&
                matches!(inner.as_slice(), [_, _])
        )));

        let (source, _) = SourceFile::test_file(r#"()"#);
        let res = Expr::parser().parse(source.stream());
        assert!(matches!(res,
        Ok(Expr::Map(super::Map { fields: Punctuated { inner, .. }, ..}))
            if matches!(inner.as_slice(), []
        )));
    }
}
