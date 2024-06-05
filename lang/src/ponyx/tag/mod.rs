//!
//! ## PonyX Tags
//!
//! ```text
//! tag_name            := <ident> (`.` <ident>)*
//!
//! self_closing_tag    := `<` <tag_name> (<ws>)+ (<attribute> (<ws>)+)* `/>`
//!
//! open_tag            := `<` <tag_name name> (<attribute> ` `)* `>`
//! close_tag           := `</`<tag_name name> `>`
//! enlosing_tag        := open_tag (<node>)* close_tag
//!
//! tag                 := enclosing_tag | self_closing_tag
//! ```
//!

pub mod attribute;
pub mod name;

use attribute::Attribute;
use avpony_macros::Spanned;
use chumsky::{
    primitive::{choice, just},
    text, Parser,
};
use name::TagName;

use crate::utils::{
    errors::{tag::UnclosedTag, Error},
    ParseableExt, PonyParser, Span,
};

#[derive(Debug, Clone, Spanned, PartialEq)]
pub enum Tag {
    SelfClosing(SelfClosingTag),
    Enclosing(EnclosingTag),
}

impl Tag {
    pub fn parser_with(
        node: impl PonyParser<super::Node> + Clone,
    ) -> impl PonyParser<Self> + Clone {
        just("<")
            .ignore_then(TagName::parser())
            .then(
                text::whitespace()
                    .at_least(1)
                    .ignore_then(Attribute::parser())
                    .repeated(),
            )
            .then_ignore(text::whitespace())
            .then(choice((
                just("/>").ignored().map_with_span(|_, _| None),
                just(">")
                    .ignore_then(node.repeated())
                    .then(just("</").ignore_then(TagName::parser()))
                    .map_with_span(|(children, name), _| Some((children, name))),
            )))
            .try_map(|((name, attributes), enclosing), span| {
                if let Some((children, name_end)) = enclosing {
                    if name != name_end {
                        return Err(Error::UnclosedTag(UnclosedTag::new(span, name, name_end)));
                    }

                    return Ok(Self::Enclosing(EnclosingTag {
                        span,
                        name,
                        attributes,
                        children,
                    }));
                }

                Ok(Self::SelfClosing(SelfClosingTag {
                    span,
                    name,
                    attributes,
                }))
            })
    }
}

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct SelfClosingTag {
    span: Span,
    pub name: TagName,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct EnclosingTag {
    span: Span,
    pub name: TagName,
    pub attributes: Vec<Attribute>,
    pub children: Vec<super::Node>,
}

#[cfg(test)]
mod tests {
    use chumsky::Parser;

    use crate::{
        lexical::Literal,
        ponyx::{
            tag::{
                attribute::{Attribute, AttributeAssignment, AttributeKey, Directive},
                EnclosingTag, SelfClosingTag, Tag,
            },
            text::Text,
            Node,
        },
        syntax::SoloExpr,
        utils::{stream::SourceFile, Parseable},
    };

    #[test]
    fn test_tag() {
        let (source, _) = SourceFile::test_file(r#"<Image a11y:alt="Picture of a rat." active />"#);
        let res = Node::parser().parse(source.stream());
        assert!(matches!(
            res,
            Ok(Node::Tag(Tag::SelfClosing(SelfClosingTag {
                name,
                attributes,
                ..
            })))
                if name == *"Image"
                && matches!(attributes.as_slice(),
                    [
                        Attribute::KeyValue(AttributeAssignment {
                            key: AttributeKey::Directive(Directive { base, director, ..}),
                            value: SoloExpr::Literal(Literal::String(value)),
                            ..
                        }),
                        Attribute::Key(AttributeKey::Named(key2))
                    ]
                        if base == "a11y" && director == "alt"
                        && key2 == "active"
                        && value == "Picture of a rat."
                )
        ));
        let (source, _) = SourceFile::test_file("<Box>\n  Hey there!\n</Box>");
        let res = Node::parser().parse(source.stream());
        assert!(matches!(
            res,
            Ok(Node::Tag(Tag::Enclosing(EnclosingTag {
                name,
                children,
                ..
            })))
                if name == *"Box"
                && matches!(
                    children.as_slice(),
                    [Node::Text(Text { text, ..})]
                        if text.trim() == "Hey there!"
                )
        ))
    }
}
