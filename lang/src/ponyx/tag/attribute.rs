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
    syntax::{external::External, SoloExpr},
    utils::{ParseableCloned, PonyParser, Span},
};

#[derive(Debug, Clone, Spanned, PartialEq)]
pub enum Attribute<Ext: External> {
    Key(AttributeKey),
    KeyValue(AttributeAssignment<Ext>),
}

impl<Ext: External + 'static> ParseableCloned for Attribute<Ext> {
    fn parser<'src>() -> impl PonyParser<'src, Self> + Clone {
        AttributeKey::parser()
            .then(just("=").ignore_then(SoloExpr::parser()).or_not())
            .map_with(|(key, value), ctx| match value {
                None => Self::Key(key),
                Some(value) => Self::KeyValue(AttributeAssignment {
                    span: ctx.span(),
                    key,
                    value,
                }),
            })
    }
}

#[derive(Debug, Clone, Spanned, PartialEq)]
pub enum AttributeKey {
    Named(NamedAttribute),
    Directive(Directive),
}

impl ParseableCloned for AttributeKey {
    fn parser<'src>() -> impl PonyParser<'src, Self> + Clone {
        lexical::Identifier::parser()
            .then(
                just(":")
                    .ignore_then(lexical::Identifier::parser())
                    .or_not(),
            )
            .map_with(|(base, director), ctx| match director {
                None => Self::Named(base),
                Some(director) => Self::Directive(Directive {
                    span: ctx.span(),
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
pub struct AttributeAssignment<Ext: External> {
    span: Span,
    pub key: AttributeKey,
    pub value: SoloExpr<Ext>,
}

#[cfg(test)]
mod tests {
    use chumsky::Parser;

    use crate::{
        lexical::{
            number::{IntegerLit, NumberLit},
            Literal,
        },
        ponyx::tag::attribute::{Attribute as Attr, AttributeAssignment, AttributeKey, Directive},
        syntax::{external::TestLang, SoloExpr},
        utils::{Parseable, SourceFile},
    };

    type Attribute = Attr<TestLang>;

    #[test]
    fn test() {
        let (source, _) = SourceFile::test_file("active");
        let res = Attribute::parser().parse(source.stream());
        assert!(matches!(
            res.into_result(),
            Ok(Attribute::Key(AttributeKey::Named(ident))) if ident == *"active"
        ));

        let (source, _) = SourceFile::test_file("on:click");
        assert!(matches!(
            Attribute::parser().parse(source.stream()).into_result(),
            Ok(Attribute::Key(AttributeKey::Directive(Directive { base, director, ..})))
                if base == *"on" && director == *"click"
        ));

        let (source, _) = SourceFile::test_file("value=2");
        let res = Attribute::parser().parse(source.stream());
        assert!(matches!(
            res.into_result(),
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
            res.into_result(),
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
