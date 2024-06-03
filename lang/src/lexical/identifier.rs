//!
//! ## Identifiers
//!
//! In parsing, we first treat all identifiers as an [UncheckedIdentifier],
//! then guaranteeing that it is not a pony-reserved keyword before accepting it as a (safe) [Identifier].
//!
//! We use [unicode_ident] for the Unicode standard implementation (see below).
//!
//! ### Grammar
//! We are in line with the [Rust Reference on in Identifiers](https://doc.rust-lang.org/reference/identifiers.html),
//! which follows [Unicode 15.0 Standard Annex #31](https://www.unicode.org/reports/tr31/tr31-37.html).
//!
//! ***Note***: we don't support Rust's raw identifiers.
//!
//! ```text
//! unchecked_identifier := xid_start xid_continue* | _ xid_continue*
//! ```
//!

use avpony_macros::Spanned;
use chumsky::{
    primitive::{choice, filter, just},
    Parser,
};

use crate::utils::{errors::identifier::ReservedIdentifier, ParseableExt, Span};

use super::keyword;

///
/// This identifier could be a keyword, or just an identifier.
///
#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct UncheckedIdentifier {
    span: Span,
    value: String,
}

impl ParseableExt for UncheckedIdentifier {
    fn parser() -> impl crate::utils::PonyParser<Self> + Clone {
        let xid_start = filter::<char, _, _>(|ch: &char| unicode_ident::is_xid_start(*ch));
        let xid_continue = filter::<char, _, _>(|ch: &char| unicode_ident::is_xid_continue(*ch));

        choice((
            xid_start
                .then(xid_continue.repeated())
                .map_with_span(|(start, continued), span| Self {
                    span,
                    value: std::iter::once(start).chain(continued).collect(),
                }),
            just("_")
                .ignore_then(xid_continue.repeated().at_least(1))
                .map_with_span(|continued, span| Self {
                    span,
                    value: std::iter::once('_').chain(continued).collect(),
                }),
        ))
    }
}

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct Identifier {
    span: Span,
    value: String,
}

impl ParseableExt for Identifier {
    fn parser() -> impl crate::utils::PonyParser<Self> + Clone {
        UncheckedIdentifier::parser().try_map(|UncheckedIdentifier { span, value }, _| {
            if keyword::is_keyword(&value) {
                Err(ReservedIdentifier::new(span, value).into())
            } else {
                Ok(Self { span, value })
            }
        })
    }
}

impl PartialEq<str> for Identifier {
    fn eq(&self, other: &str) -> bool {
        self.value == other
    }
}

#[cfg(test)]
mod tests {
    use chumsky::Parser;

    use crate::{
        lexical::identifier::Identifier,
        utils::{stream::SourceFile, Parseable},
    };

    #[test]
    fn safe_identifiers() {
        let (source, _) = SourceFile::test_file("a");
        assert!(
            matches!(Identifier::parser().parse(source.stream()), Ok(Identifier { value, .. }) if value == "a")
        );

        let (source, _) = SourceFile::test_file("_aa");
        assert!(
            matches!(Identifier::parser().parse(source.stream()), Ok(Identifier { value, .. }) if value == "_aa")
        );

        let (source, _) = SourceFile::test_file("__aa");
        assert!(
            matches!(Identifier::parser().parse(source.stream()), Ok(Identifier { value, .. }) if value == "__aa")
        );

        let (source, _) = SourceFile::test_file("𪧦氺");
        assert!(
            matches!(Identifier::parser().parse(source.stream()), Ok(Identifier { value, .. }) if value == "𪧦氺")
        );
    }

    #[test]
    fn invalid_identifiers() {
        let (source, _) = SourceFile::test_file("if");
        assert!(Identifier::parser().parse(source.stream()).is_err(),);

        let (source, _) = SourceFile::test_file("else");
        assert!(Identifier::parser().parse(source.stream()).is_err(),);

        let (source, _) = SourceFile::test_file("_");
        assert!(Identifier::parser().parse(source.stream()).is_err(),);
    }
}
