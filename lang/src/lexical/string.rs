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
    primitive::{any, choice, just, one_of},
    IterParser, Parser,
};

use crate::utils::{
    error::{
        string::{InvalidAsciiCode, InvalidEscapeSequence, InvalidUnicodeCodePoint},
        Error,
    },
    ParseableCloned, PonyParser, Span,
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

pub fn hex_nibbles_to_u32<IntoIter: IntoIterator<Item = char>>(iter: IntoIter) -> u32
where
    IntoIter::IntoIter: DoubleEndedIterator,
{
    iter.into_iter()
        .rev()
        .map(to_hex_value)
        .enumerate()
        .map(|(nibble, val)| val << (4 * nibble))
        .sum()
}

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct StringLit {
    span: Span,
    pub value: String,
}
impl ParseableCloned for StringLit {
    fn parser<'src>() -> impl PonyParser<'src, Self> + Clone {
        StringPart::parser()
            .repeated()
            .collect::<Vec<_>>()
            .delimited_by(just("\""), just("\""))
            .map_with(|parts, ctx| {
                let mut value = String::new();
                parts.iter().for_each(|part| part.write_value(&mut value));
                Self {
                    span: ctx.span(),
                    value,
                }
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

impl ParseableCloned for StringPart {
    fn parser<'src>() -> impl PonyParser<'src, Self> + Clone {
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

impl ParseableCloned for Verbatim {
    fn parser<'src>() -> impl PonyParser<'src, Self> + Clone {
        any()
            .and_is(one_of("\"\\\r").not())
            .repeated()
            .at_least(1)
            .collect::<String>()
            .map_with(|value, ctx| Self {
                span: ctx.span(),
                value,
            })
    }
}

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct QuoteEscape {
    span: Span,
    value: char,
}

impl ParseableCloned for QuoteEscape {
    fn parser<'src>() -> impl PonyParser<'src, Self> + Clone {
        just("\\")
            .ignore_then(choice((just("\'"), just("\""))))
            .map_with(|value, ctx| {
                // Unwrap ok -- there's exactly one charcter!
                let value = value.chars().next().unwrap();

                Self {
                    span: ctx.span(),
                    value,
                }
            })
    }
}

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct AsciiEscape {
    span: Span,
    value: char,
}

impl ParseableCloned for AsciiEscape {
    fn parser<'src>() -> impl PonyParser<'src, Self> + Clone {
        let hex_digit = any().filter(|ch: &char| ch.is_ascii_hexdigit());

        just("\\").ignore_then(choice((
            just("x")
                .ignore_then(hex_digit.repeated().at_least(1).at_most(2).collect())
                .validate(|raw: Vec<_>, ctx, emit| {
                    let value = hex_nibbles_to_u32(raw);

                    // If code is higher than original (non-extended) 7-bit ASCII codes,
                    // it's not a valid ASCII escape -- I know, shocker(!)
                    if value >= 0x80 {
                        emit.emit(InvalidAsciiCode::new(ctx.span(), value).into());
                        return AsciiEscape {
                            span: ctx.span(),
                            value: char::try_from(value).unwrap(),
                        };
                    }

                    AsciiEscape {
                        span: ctx.span(),
                        value: char::try_from(value).unwrap(),
                    }
                }),
            just("n").map_with(|_, ctx| AsciiEscape {
                span: ctx.span(),
                value: '\n',
            }),
            just("r").map_with(|_, ctx| AsciiEscape {
                span: ctx.span(),
                value: '\r',
            }),
            just("t").map_with(|_, ctx| AsciiEscape {
                span: ctx.span(),
                value: '\t',
            }),
            just("\\").map_with(|_, ctx| AsciiEscape {
                span: ctx.span(),
                value: '\\',
            }),
            just("0").map_with(|_, ctx| AsciiEscape {
                span: ctx.span(),
                value: '\0',
            }),
        )))
    }
}

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct UnicodeEscape {
    span: Span,
    value: char,
}

impl ParseableCloned for UnicodeEscape {
    fn parser<'src>() -> impl PonyParser<'src, Self> + Clone {
        let hex_digits = any().filter(|ch: &char| ch.is_ascii_hexdigit()).repeated();
        just("\\u")
            .ignore_then(
                hex_digits
                    .at_least(1)
                    .at_most(6)
                    .collect::<Vec<_>>()
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
            .validate(|code_point: u32, ctx, emitter| {
                match char::from_u32(code_point)
                    .map(|value| Self {
                        span: ctx.span(),
                        value,
                    })
                    .ok_or(InvalidUnicodeCodePoint::new(ctx.span(), code_point))
                {
                    Ok(ok) => ok,
                    Err(err) => {
                        emitter.emit(err.into());
                        UnicodeEscape {
                            span: ctx.span(),
                            value: '\0',
                        }
                    }
                }
            })
    }
}

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct StringContinue {
    span: Span,
}

impl ParseableCloned for StringContinue {
    fn parser<'src>() -> impl PonyParser<'src, Self> + Clone {
        just("\\\n").map_with(|_, ctx| Self { span: ctx.span() })
    }
}

#[cfg(test)]
mod tests {
    use chumsky::Parser;

    use crate::{
        lexical::string::StringLit,
        utils::{Error, Parseable, SourceFile},
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
                .into_errors()
                .first(),
            Some(Error::UnexpectedToken(_))
        ));

        let (source, _) = SourceFile::test_file("\"aaaa");
        assert!(matches!(
            StringLit::parser()
                .parse(source.stream())
                .into_errors()
                .first(),
            Some(Error::UnexpectedToken(_))
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
        let res = StringLit::parser().parse(source.stream());
        assert_eq!(res.unwrap().value, "\x7F\x65",);

        let (source, _) = SourceFile::test_file("\"\\xFF\"");
        let res = StringLit::parser().parse(source.stream());
        assert!(matches!(
            res.errors().next(),
            Some(Error::InvalidAsciiCode(_)),
        ));

        let (source, _) = SourceFile::test_file("\"\\u{FF}\"");
        assert_eq!(
            StringLit::parser().parse(source.stream()).unwrap().value,
            "\u{FF}",
        );

        let (source, _) = SourceFile::test_file("\"\\u{D800}\"");
        let res = StringLit::parser().parse(source.stream());
        assert!(matches!(
            res.into_errors().first(),
            Some(Error::InvalidUnicodeCodePoint(_)),
        ));
    }
}
