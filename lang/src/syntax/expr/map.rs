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
    text::{self},
    Parser,
};

use crate::{
    lexical::{identifier::Identifier, punctuation},
    utils::{ParseableCloned, PonyParser, Span},
};

use super::{external::External, utils::Punctuated};

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct Map<Ext: External> {
    span: Span,
    pub fields: Punctuated<Field<Ext>, punctuation::Comma>,
}

impl<Ext: External> Map<Ext> {
    pub fn parse_with<'src>(
        expr: impl PonyParser<'src, super::Expr<Ext>> + Clone,
    ) -> impl PonyParser<'src, Self> + Clone {
        Punctuated::<_, punctuation::Comma>::optional_trailing_with(Field::parse_with(expr))
            .padded()
            .delimited_by(just("("), just(")"))
            .map_with(|fields, ctx| Self {
                span: ctx.span(),
                fields,
            })
    }
}

#[derive(Debug, Clone, Spanned, PartialEq)]
pub enum Field<Ext: External> {
    Key(FieldKey),
    KeyValue(FieldKeyValue<Ext>),
}

impl<Ext: External> Field<Ext> {
    fn parse_with<'src>(
        expr: impl PonyParser<'src, super::Expr<Ext>> + Clone,
    ) -> impl PonyParser<'src, Self> + Clone {
        just(".")
            .ignore_then(Identifier::parser().then_ignore(text::whitespace()))
            .then(
                just("=")
                    .padded_by(text::whitespace())
                    .ignore_then(expr.map(Box::new).padded_by(text::whitespace()))
                    .or_not(),
            )
            .map_with(|(key, value), ctx| {
                match value {
                    Some(value) => Self::KeyValue(FieldKeyValue {
                        span: ctx.span(),
                        key,
                        value,
                    }),
                    None => Self::Key(FieldKey {
                        span: ctx.span(),
                        ident: key,
                    }),
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
pub struct FieldKeyValue<Ext: External> {
    span: Span,
    key: Identifier,
    value: Box<super::Expr<Ext>>,
}

#[cfg(test)]
mod tests {
    use chumsky::Parser;

    use crate::{
        lexical::Literal,
        syntax::{utils::Punctuated, VExpr as Expr},
        utils::{Parseable, SourceFile},
    };

    #[test]
    fn parse_map() {
        let (source, _) = SourceFile::test_file(r#"(.a, .if_="")"#);
        let res = Expr::parser().parse(source.stream());
        println!("{res:#?}");
        assert!(matches!(res.into_result(),
            Ok(Expr::Map(super::Map { fields: Punctuated { inner, .. }, ..}))
                if matches!(inner.as_slice(), [
                    super::Field::Key(super::FieldKey { ident, ..}),
                    super::Field::KeyValue(super::FieldKeyValue { key, value: box Expr::Literal(Literal::String(_)), ..})
                ]
                    if ident == "a" && key == "if_"
        )));

        let (source, _) = SourceFile::test_file(r#"(.nested=(.a=1, .b=2),)"#);
        let res = Expr::parser().parse(source.stream());
        assert!(matches!(res.into_result(),
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
        assert!(matches!(res.into_result(),
        Ok(Expr::Map(super::Map { fields: Punctuated { inner, .. }, ..}))
            if matches!(inner.as_slice(), []
        )));
    }
}
