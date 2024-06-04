//!
//! ## HTML Character References
//!
//! HTML-style character references: `&lbrace;`, `&#0000;`.
//!
//! [See the spec for them here.](https://html.spec.whatwg.org/multipage/syntax.html#syntax-charref)
//!

use avpony_macros::Spanned;
use chumsky::{
    primitive::{choice, filter, just, one_of},
    text, Parser,
};

use crate::{
    lexical::string::hex_nibbles_to_u32,
    utils::{
        errors::{html_ref::InvalidEntityName, string::InvalidUnicodeCodePoint, Error},
        ParseableExt, PonyParser, Span,
    },
};

mod entities {
    include!(concat!(env!("OUT_DIR"), "/html_entities.rs"));

    pub fn get_by_name(name: &str) -> Option<&'static str> {
        CODES.binary_search(&name).ok().map(|i| VALUES[i])
    }
}

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct Entity {
    span: Span,
    pub value: String,
}

impl ParseableExt for Entity {
    fn parser() -> impl PonyParser<Self> + Clone {
        let hex_digits = filter(|ch: &char| ch.is_ascii_hexdigit())
            .repeated()
            .at_least(1);

        just("&")
            .ignore_then(choice((
                just("#").ignore_then(choice((
                    one_of("xX")
                        .ignore_then(hex_digits)
                        .try_map(|hex, span: Span| {
                            let u = hex_nibbles_to_u32(hex);
                            match char::try_from(u) {
                                Ok(o) => Ok(Self {
                                    span: span.clone(),
                                    value: o.into(),
                                }),
                                Err(_) => Err(Error::InvalidUnicodeCodePoint(
                                    InvalidUnicodeCodePoint::new(span, u),
                                )),
                            }
                        }),
                    text::digits(10).try_map(|digits: String, span: Span| {
                        let u: u32 = digits.parse().unwrap();
                        match char::try_from(u) {
                            Ok(o) => Ok(Self {
                                span: span.clone(),
                                value: o.into(),
                            }),
                            Err(_) => Err(Error::InvalidUnicodeCodePoint(
                                InvalidUnicodeCodePoint::new(span, u),
                            )),
                        }
                    }),
                ))),
                text::ident().try_map(|name: String, span: Span| {
                    let val = entities::get_by_name(name.as_str()).map(ToString::to_string);

                    match val {
                        Some(value) => Ok(Self {
                            span: span.clone(),
                            value,
                        }),
                        None => Err(Error::InvalidEntityName(InvalidEntityName::new(span, name))),
                    }
                }),
            )))
            .then_ignore(just(";"))
    }
}

#[cfg(test)]
mod tests {
    use chumsky::Parser;

    use crate::utils::{stream::SourceFile, Parseable};

    use super::Entity;

    #[test]
    fn named() {
        let (source, _) = SourceFile::test_file("&nbsp;");
        assert!(Entity::parser().parse(source.stream()).is_ok());

        let (source, _) = SourceFile::test_file("&mdash;");
        assert!(matches!(
            Entity::parser().parse(source.stream()),
            Ok(Entity { value, .. }) if value == "—"
        ));

        let (source, _) = SourceFile::test_file("&#0000;");
        assert!(matches!(
            Entity::parser().parse(source.stream()),
            Ok(Entity { value, .. }) if value == "\x00"
        ));

        let (source, _) = SourceFile::test_file("&#x1f44d;");
        assert!(matches!(
            Entity::parser().parse(source.stream()),
            Ok(Entity { value, .. }) if value == "\u{1f44d}"
        ));
    }
}
