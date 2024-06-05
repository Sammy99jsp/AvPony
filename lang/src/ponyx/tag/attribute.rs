//!
//! ## PonyX Attributes
//!
//! These take inspiration from JSX' attributes,
//! and Svelte's directives.
//!
//! In this case, we have two main forms of attributes:
//! * Named attributes: `checked`
//! * Directives: `on:click`
//!
//! In PonyX, directives are used for two primary purposes:
//! * A method of attaching event handlers, or binds:
//!     ```text
//!     <Input.TextBox
//!         in:fly=(.left=10px)
//!         bind:value={my_variable}
//!     />
//!     ```
//! * Or as a namespace for specialized attributes, such as accessibility:
//!     ```text
//!     <Image
//!         src=(p"very_funny_image.jpg")
//!         a11y:alt="A very funny image of a frog dancing."
//!     />
//!     ```
//!

use avpony_macros::Spanned;
use chumsky::{primitive::just, Parser};

use crate::{
    lexical,
    syntax::SoloExpr,
    utils::{ParseableExt, PonyParser, Span},
};

#[derive(Debug, Clone, Spanned, PartialEq)]
pub enum Attribute {
    Key(AttributeKey),
    KeyValue(AttributeAssignment),
}

impl ParseableExt for Attribute {
    fn parser() -> impl PonyParser<Self> + Clone {
        AttributeKey::parser()
            .then(just("=").ignore_then(SoloExpr::parser()).or_not())
            .map_with_span(|(key, value), span| match value {
                None => Self::Key(key),
                Some(value) => Self::KeyValue(AttributeAssignment { span, key, value }),
            })
    }
}

#[derive(Debug, Clone, Spanned, PartialEq)]
pub enum AttributeKey {
    Named(NamedAttribute),
    Directive(Directive),
}

impl ParseableExt for AttributeKey {
    fn parser() -> impl PonyParser<Self> + Clone {
        lexical::Identifier::parser()
            .then(
                just(":")
                    .ignore_then(lexical::Identifier::parser())
                    .or_not(),
            )
            .map_with_span(|(base, director), span| match director {
                None => Self::Named(base),
                Some(director) => Self::Directive(Directive {
                    span,
                    base,
                    director,
                }),
            })
    }
}

pub type NamedAttribute = lexical::Identifier;

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct Directive {
    span: Span,
    pub base: lexical::Identifier,
    pub director: lexical::Identifier,
}

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct AttributeAssignment {
    span: Span,
    pub key: AttributeKey,
    pub value: SoloExpr,
}

#[cfg(test)]
mod tests {
    use chumsky::Parser;

    use crate::{
        lexical::{
            number::{IntegerLit, NumberLit},
            Literal,
        },
        ponyx::tag::attribute::{Attribute, AttributeAssignment, AttributeKey, Directive},
        syntax::SoloExpr,
        utils::{stream::SourceFile, Parseable},
    };

    #[test]
    fn test() {
        let (source, _) = SourceFile::test_file("active");
        let res = Attribute::parser().parse(source.stream());
        assert!(matches!(
            res,
            Ok(Attribute::Key(AttributeKey::Named(ident))) if ident == *"active"
        ));

        let (source, _) = SourceFile::test_file("on:click");
        assert!(matches!(
            Attribute::parser().parse(source.stream()),
            Ok(Attribute::Key(AttributeKey::Directive(Directive { base, director, ..})))
                if base == *"on" && director == *"click"
        ));

        let (source, _) = SourceFile::test_file("value=2");
        let res = Attribute::parser().parse(source.stream());
        assert!(matches!(
            res,
            Ok(Attribute::KeyValue(AttributeAssignment {
                key: AttributeKey::Named(ident),
                value: SoloExpr::Literal(Literal::Number(
                    NumberLit::Integer(IntegerLit { value, ..})
                )),
                ..
            }))
                if ident == *"value" && value == 2
        ));

        let (source, _) = SourceFile::test_file(r#"a11y:alt="Alt text""#);
        let res = Attribute::parser().parse(source.stream());
        assert!(matches!(
            res,
            Ok(Attribute::KeyValue(AttributeAssignment {
                key: AttributeKey::Directive(Directive {base, director, .. }),
                value: SoloExpr::Literal(Literal::String(s)),
                ..
            }))
                if base == *"a11y"
                && director == *"alt"
                && s == *"Alt text"
        ));
    }
}
