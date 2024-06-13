//!
//! ## TypeScript support.
//! 
use chumsky::{
    input::Marker,
    primitive::{any, custom, just},
    span::Span,
    IterParser, Parser,
};
use swc_common::BytePos;
use swc_ecma_parser::{StringInput, Syntax};

use crate::utils::{self, error::external::typescript::ConvertTSError, Error, PonyParser};

#[derive(Debug, Clone, PartialEq)]
pub struct TypeScript;

impl TypeScript {
    fn parse_str<'src, O: swc_common::Spanned>(
        src: &'src str,
        span: utils::Span,
        func: impl Fn(
            &mut swc_ecma_parser::Parser<swc_ecma_parser::lexer::Lexer<'src>>,
        ) -> swc_ecma_parser::PResult<O>,
    ) -> Result<O, Error> {
        let mut parser = swc_ecma_parser::Parser::new(
            Syntax::Typescript(Default::default()),
            StringInput::new(src, BytePos(span.start() as _), BytePos(span.end() as _)),
            None,
        );

        let res = func(&mut parser);

        match res {
            Ok(ok) => Ok(ok),
            Err(err) => Err(err.convert(span).into()),
        }
    }

    fn inline_parse_for<'src, 'parse, O: swc_common::Spanned, F>(
        func: F,
    ) -> impl PonyParser<'src, O> + Clone
    where
        F: Fn(
                &mut swc_ecma_parser::Parser<swc_ecma_parser::lexer::Lexer<'src>>,
            ) -> swc_ecma_parser::PResult<O>
            + Clone
            + 'src,
    {
        custom(move |stream| {
            let span: utils::Span = stream.span(stream.offset()..stream.offset());
            let input_left: &str = stream.slice_from(stream.offset()..);
            let start = stream.offset().raw();
            let end = start + input_left.len();
            Self::parse_str(
                input_left,
                utils::Span::new(span.context(), start..end),
                func.clone(),
            )
            .map(|o| {
                let new_start = o.span().hi.0 as usize;
                stream.rewind(unsafe { Marker::from_raw(new_start, 0) });
                o
            })
        })
    }
}

impl super::External for TypeScript {
    const ID: &'static str = "ts";

    type Module = swc_ecma_ast::Module;
    type Expression = swc_ecma_ast::Expr;

    fn module<'src>() -> impl PonyParser<'src, Self::Module> {
        any()
            .and_is(just("---").not())
            .repeated()
            .collect::<String>()
            .try_map(|src, span: utils::Span| {
                Self::parse_str(&src, span.clone(), swc_ecma_parser::Parser::parse_module)
            })
    }

    fn expression<'src>() -> impl PonyParser<'src, Self::Expression> + Clone {
        Self::inline_parse_for(|parser| parser.parse_expr().map(|b| *b))
    }
}

impl utils::Span {
    pub fn convert_ecma(&self, ecma: swc_common::Span) -> Self {
        Self::new(
            self.context().clone(),
            ((ecma.lo.0 as usize) + 1)..((ecma.hi.0 as usize) + 1),
        )
    }
}
#[cfg(test)]
mod tests {
    use chumsky::{IterParser, Parser};

    use crate::{syntax::external::External, utils::input::SourceFile};

    use super::TypeScript;

    #[test]
    fn parse_expr() {
        let (file, _) = SourceFile::test_file("(1 + )");
        let res = TypeScript::expression()
            .repeated()
            .collect::<Vec<_>>()
            .parse(file.stream())
            .into_result();
    }

    #[test]
    fn parse_module() {
        let (file, _) = SourceFile::test_file(
            r#"
            // Import standard library.
            import "@avpony/prelude";
            
            import { Button } from "@avpony/ui";
            const NAME: string = "CLICK ME!";
            let clicked: boolean;
            "#,
        );
        let res = TypeScript::module().parse(file.stream()).into_result();
        assert!(res.is_ok())
    }
}
