//!
//! # String literals
//!
//! ### Grammar
//! Here, we are using Rust's [string literals](https://doc.rust-lang.org/reference/tokens.html#string-literals)
//! as they are easier to parse than ECMAScript's.
//!
//! ### Examples
//! ```ignore
//! "apples";
//! "\x61pples";    // ASCII Escape
//! "\u{97}apples"; // Unicode Escape
//! "c_str\0";      // Null Character Escape
//! "I am a tall\   
//! Guy!";          // Line continuation (no newline in resultant string)
//! "\"I\'m Still Standing\" by Elton John" // Escaped quotes
//! ```
//!

use avpony_macros::Spanned;
use chumsky::{
    primitive::{choice, filter, just, one_of},
    Parser,
};

// TODO: There's definitely a better approach out there,
// but at the same time, I also don't want to allocate an additional String,
// so it's this for now.
pub fn to_hex_value(ch: char) -> u32 {
    match ch {
        '0' => 0x0,
        '1' => 0x1,
        '2' => 0x2,
        '3' => 0x3,
        '4' => 0x4,
        '5' => 0x5,
        '6' => 0x6,
        '7' => 0x7,
        '8' => 0x8,
        '9' => 0x9,
        'A' => 0xA,
        'B' => 0xB,
        'C' => 0xC,
        'D' => 0xD,
        'E' => 0xE,
        'F' => 0xF,
        'a' => 0xA,
        'b' => 0xB,
        'c' => 0xC,
        'd' => 0xD,
        'e' => 0xE,
        'f' => 0xF,
        _ => unimplemented!("Must be a hex [0-9a-fA-F] character here!"),
    }
}

use crate::utils::{
    errors::{
        string::{InvalidAsciiCode, InvalidEscapeSequence, InvalidUnicodeCodePoint},
        Error,
    },
    Parseable, Span,
};

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct StringLit {
    pub span: Span,
    pub value: String,
}
impl Parseable for StringLit {
    fn parser() -> impl Parser<char, Self, Error = Error> {
        StringPart::parser()
            .repeated()
            .delimited_by(just("\""), just("\""))
            .map_with_span(|parts, span| {
                let mut value = String::new();
                parts.iter().for_each(|part| part.write_value(&mut value));
                Self { span, value }
            })
    }
}

impl PartialEq<str> for StringLit {
    fn eq(&self, other: &str) -> bool {
        self.value == other
    }
}

#[derive(Debug, Clone, Spanned, PartialEq)]
enum StringPart {
    Verbatim(Verbatim),
    QuoteEscape(QuoteEscape),
    AsciiEscape(AsciiEscape),
    UnicodeEscape(UnicodeEscape),
    StringContinue(StringContinue),
}

impl StringPart {
    fn write_value(&self, buf: &mut String) {
        match self {
            StringPart::Verbatim(v) => buf.push_str(&v.value),
            StringPart::QuoteEscape(esc) => buf.push(esc.value),
            StringPart::AsciiEscape(esc) => buf.push(esc.value),
            StringPart::UnicodeEscape(esc) => buf.push(esc.value),
            StringPart::StringContinue(_) => (),
        }
    }
}

impl Parseable for StringPart {
    fn parser() -> impl chumsky::Parser<char, Self, Error = Error> {
        choice((
            Verbatim::parser().map(Self::Verbatim),
            QuoteEscape::parser().map(Self::QuoteEscape),
            AsciiEscape::parser().map(Self::AsciiEscape),
            UnicodeEscape::parser().map(Self::UnicodeEscape),
            StringContinue::parser().map(Self::StringContinue),
            just("\\").ignored().try_map(|_, span| {
                Err(Error::InvalidEscapeSequence(InvalidEscapeSequence::new(
                    span,
                )))
            }),
        ))
    }
}

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct Verbatim {
    span: Span,
    value: String,
}

impl Parseable for Verbatim {
    fn parser() -> impl chumsky::Parser<char, Self, Error = Error> {
        one_of("\"\\\r")
            .not()
            .repeated()
            .at_least(1)
            .map_with_span(|value, span| {
                let value = value.into_iter().collect::<String>();
                Self { span, value }
            })
    }
}

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct QuoteEscape {
    span: Span,
    value: char,
}

impl Parseable for QuoteEscape {
    fn parser() -> impl chumsky::Parser<char, Self, Error = Error> {
        just("\\")
            .ignore_then(choice((just("\'"), just("\""))))
            .map_with_span(|value, span| {
                // Unwrap ok -- there's exactly one charcter!
                let value = value.chars().next().unwrap();

                Self { span, value }
            })
    }
}

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct AsciiEscape {
    span: Span,
    value: char,
}

impl Parseable for AsciiEscape {
    fn parser() -> impl chumsky::Parser<char, Self, Error = Error> {
        let hex_digits = filter(|ch: &char| ch.is_ascii_hexdigit()).repeated();

        just("\\").ignore_then(choice((
            just("x")
                .ignore_then(hex_digits.at_least(1).at_most(2))
                .map(|hex| {
                    hex.into_iter()
                        .rev()
                        .map(to_hex_value)
                        .enumerate()
                        .map(|(nibble, val)| val << (4 * nibble))
                        .sum()
                })
                .try_map(|value: u32, span| {
                    // If code is higher than original (non-extended) 7-bit ASCII codes,
                    // it's not a valid ASCII escape -- I know, shocker(!)
                    if value >= 0x80 {
                        return Err(Error::InvalidAsciiCode(InvalidAsciiCode::new(span, value)));
                    }

                    Ok(AsciiEscape {
                        span,
                        value: char::from_u32(value).unwrap(),
                    })
                }),
            just("n").map_with_span(|_, span| AsciiEscape { span, value: '\n' }),
            just("r").map_with_span(|_, span| AsciiEscape { span, value: '\r' }),
            just("t").map_with_span(|_, span| AsciiEscape { span, value: '\t' }),
            just("\\").map_with_span(|_, span| AsciiEscape { span, value: '\\' }),
            just("0").map_with_span(|_, span| AsciiEscape { span, value: '\0' }),
        )))
    }
}

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct UnicodeEscape {
    span: Span,
    value: char,
}

impl Parseable for UnicodeEscape {
    fn parser() -> impl chumsky::Parser<char, Self, Error = Error> {
        let hex_digits = filter(|ch: &char| ch.is_ascii_hexdigit()).repeated();
        just("\\u")
            .ignore_then(
                hex_digits
                    .at_least(1)
                    .at_most(6)
                    .delimited_by(just("{"), just("}")),
            )
            .map(|hex| {
                hex.into_iter()
                    .rev()
                    .map(to_hex_value)
                    .enumerate()
                    .map(|(nibble, val)| val << (4 * nibble))
                    .sum()
            })
            .try_map(|code_point: u32, span: Span| {
                char::from_u32(code_point)
                    .map(|value| Self {
                        span: span.clone(),
                        value,
                    })
                    .ok_or(InvalidUnicodeCodePoint::new(span, code_point))
                    .map_err(Into::into)
            })
    }
}

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct StringContinue {
    span: Span,
}

impl Parseable for StringContinue {
    fn parser() -> impl Parser<char, Self, Error = Error> {
        just("\\\n").map_with_span(|_, span| Self { span })
    }
}

#[cfg(test)]
mod tests {
    use chumsky::Parser;

    use crate::{
        lexical::string::StringLit,
        utils::{self, stream::SourceFile, Parseable},
    };

    #[test]
    fn str_lit() {
        let (source, _) = SourceFile::test_file("\"122\"");
        assert_eq!(
            StringLit::parser().parse(source.stream()).unwrap().value,
            "122"
        );

        let (source, _) = SourceFile::test_file("\"ѡ0=Ĺ㇖Ҧ⪛󫽛ڶsL:͡홴㰪ܬQi瘤W񡆇򎔼򧏸$uЏ󉅋򁻢򅈞񆞯U�\"");
        assert_eq!(
            StringLit::parser().parse(source.stream()).unwrap().value,
            "ѡ0=Ĺ㇖Ҧ⪛󫽛ڶsL:͡홴㰪ܬQi瘤W񡆇򎔼򧏸$uЏ󉅋򁻢򅈞񆞯U�"
        );

        // TODO: Insert more rigourous, weird utf-8 tests here.
    }

    #[test]
    fn undelimited() {
        let (source, _) = SourceFile::test_file("\"");
        assert!(matches!(
            StringLit::parser()
                .parse(source.stream())
                .unwrap_err()
                .first(),
            Some(utils::errors::Error::UnexpectedToken(_))
        ));

        let (source, _) = SourceFile::test_file("\"aaaa");
        assert!(matches!(
            StringLit::parser()
                .parse(source.stream())
                .unwrap_err()
                .first(),
            Some(utils::errors::Error::UnexpectedToken(_))
        ));
    }

    #[test]
    fn escape_sequences() {
        let (source, _) = SourceFile::test_file("\"Long train \\\ncomming!\"");
        assert_eq!(
            StringLit::parser().parse(source.stream()).unwrap().value,
            "Long train comming!",
        );

        let (source, _) = SourceFile::test_file("\"\\n\\r\\t\\\\\\0\"");
        assert_eq!(
            StringLit::parser().parse(source.stream()).unwrap().value,
            "\n\r\t\\\0",
        );

        let (source, _) = SourceFile::test_file("\"\\'\\\"\"");
        assert_eq!(
            StringLit::parser().parse(source.stream()).unwrap().value,
            "\'\"",
        );

        let (source, _) = SourceFile::test_file("\"\\x7F\\x65\"");
        assert_eq!(
            StringLit::parser().parse(source.stream()).unwrap().value,
            "\x7F\x65",
        );

        let (source, _) = SourceFile::test_file("\"\\xFF\"");
        assert!(matches!(
            StringLit::parser()
                .parse(source.stream())
                .unwrap_err()
                .first(),
            Some(utils::errors::Error::InvalidAsciiCode(_)),
        ));

        let (source, _) = SourceFile::test_file("\"\\u{FF}\"");
        assert_eq!(
            StringLit::parser().parse(source.stream()).unwrap().value,
            "\u{FF}",
        );

        let (source, _) = SourceFile::test_file("\"\\u{D800}\"");
        assert!(matches!(
            StringLit::parser()
                .parse(source.stream())
                .unwrap_err()
                .first(),
            Some(utils::errors::Error::InvalidUnicodeCodePoint(_)),
        ));
    }
}
