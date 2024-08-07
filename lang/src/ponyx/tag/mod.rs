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
    text, IterParser, Parser,
};
use name::TagName;

use crate::{
    syntax::external::External,
    utils::{error::tag::UnclosedTag, ParseableCloned, PonyParser, Span},
};

#[derive(Debug, Clone, Spanned, PartialEq)]
pub enum Tag<Ext: External> {
    SelfClosing(SelfClosingTag<Ext>),
    Enclosing(EnclosingTag<Ext>),
}

impl<Ext: External + 'static> Tag<Ext> {
    pub fn parser_with<'src>(
        node: impl PonyParser<'src, super::Node<Ext>> + Clone,
    ) -> impl PonyParser<'src, Self> + Clone {
        just("<")
            .ignore_then(TagName::parser())
            .then(
                text::whitespace().ignore_then(
                    Attribute::parser()
                        .separated_by(text::whitespace().at_least(1))
                        .collect(),
                ),
            )
            // BUG: <A key=/> should be recoverable, but isn't:
            //      tries to parse `/` as an expression.
            .then_ignore(text::whitespace())
            .then(choice((
                just("/>").ignored().map(|_| None),
                just(">")
                    .ignore_then(node.repeated().collect())
                    .then(just("</").ignore_then(TagName::parser()))
                    .then_ignore(just(">"))
                    .map(|(children, name)| Some((children, name))),
            )))
            .try_map(|((name, attributes), enclosing), span| {
                if let Some((children, name_end)) = enclosing {
                    if name != name_end {
                        return Err(UnclosedTag::new(span, name, name_end).into());
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
pub struct SelfClosingTag<Ext: External> {
    span: Span,
    pub name: TagName,
    pub attributes: Vec<Attribute<Ext>>,
}

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct EnclosingTag<Ext: External> {
    span: Span,
    pub name: TagName,
    pub attributes: Vec<Attribute<Ext>>,
    pub children: Vec<super::Node<Ext>>,
}

#[cfg(test)]
mod tests {
    use chumsky::Parser;

    use crate::{
        lexical::{number::NumberLit, Literal},
        ponyx::{
            tag::{
                attribute::{Attribute, AttributeAssignment, AttributeKey, Directive},
                EnclosingTag, SelfClosingTag, Tag,
            },
            text::Text,
            TNode as Node,
        },
        syntax::{application::Application, parenthesized::Parenthesized, Expr, SoloExpr},
        utils::{placeholder::Maybe, Parseable, SourceFile},
    };

    #[test]
    fn test_tag() {
        let (source, _) = SourceFile::test_file(r#"<Image a11y:alt="Picture of a rat." active />"#);
        let res = Node::parser().parse(source.stream());
        assert!(matches!(
            res.into_result(),
            Ok(Node::Tag(Tag::SelfClosing(SelfClosingTag {
                name,
                attributes,
                ..
            })))
                if name == *"Image"
                && matches!(attributes.as_slice(),
                    [
                        Attribute::KeyValue(AttributeAssignment {
                            key: AttributeKey::Directive(Directive { base, director: Maybe::Present(director), ..}),
                            value: Maybe::Present(SoloExpr::Literal(Literal::String(value))),
                            ..
                        }),
                        Attribute::Key(AttributeKey::Named(key2))
                    ]
                        if base == "a11y" && director == "alt"
                        && key2 == "active"
                        && value == "Picture of a rat."
                )
        ));
        let (source, _) = SourceFile::test_file(
            r#"<Box border=(5pt) >
            Hey there!
        </Box>"#,
        );
        let res = Node::parser().parse(source.stream());

        assert!(matches!(
            res.into_result(),
            Ok(Node::Tag(Tag::Enclosing(EnclosingTag {
                name,
                children,
                attributes,
                ..
            })))
                if name == *"Box"
                && matches!(
                    children.as_slice(),
                    [Node::Text(Text { text, ..})]
                        if text.trim() == "Hey there!"
                )
                && matches!(
                    attributes.as_slice(),
                    [Attribute::KeyValue(AttributeAssignment {
                        key: AttributeKey::Named(attr),
                        value: Maybe::Present(SoloExpr::Parenthesised(Parenthesized {
                            inner: box Expr::Application(Application {
                                function: box Expr::Literal(Literal::Number(NumberLit::Integer(int))),
                                argument: box Expr::Identifier(unit),
                                ..
                            }),
                            ..
                        })),
                        ..
                    })]
                        if attr == "border" && int.value == 5 && unit == "pt"
                )
        ))
    }
}
