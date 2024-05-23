//!
//! Utility streams for [chumsky].
//!

use std::{str::CharIndices, sync::Arc};

pub(crate) struct CharStream<'a> {
    path: Arc<str>,
    chars: CharIndices<'a>,
}

impl<'a> Iterator for CharStream<'a> {
    type Item = (char, super::Span);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((start, ch)) = self.chars.next() {
            let end = start + ch.len_utf8();

            return Some((ch, super::Span::new(&self.path, start..end)));
        }

        None
    }
}

pub struct SourceFile<'a> {
    path: Arc<str>,
    source_code: &'a str,
}

impl<'a> SourceFile<'a> {
    pub(crate) fn new(path: impl Into<Arc<str>>, source_code: &'a str) -> Self {
        Self {
            path: path.into(),
            source_code,
        }
    }

    #[cfg(test)]
    pub(crate) fn test_file(contents: &'a str) -> (Self, (Arc<str>, ariadne::Source<String>)) {
        let path = Arc::<str>::from("debug.avpony");
        (
            Self::new(path.clone(), contents),
            (path, ariadne::Source::from(contents.to_string())),
        )
    }

    pub(crate) fn stream<'b>(&'b self) -> chumsky::Stream<'b, char, super::Span, CharStream<'b>>
    where
        'a: 'b,
    {
        let end_of_input = self
            .source_code
            .char_indices()
            .last()
            .map(|(i, c)| i..(i + c.len_utf8()))
            .unwrap_or(0..0);
        let end_of_input = super::Span::new(&self.path, end_of_input);

        chumsky::Stream::from_iter(
            end_of_input,
            CharStream {
                path: self.path.clone(),
                chars: self.source_code.char_indices(),
            },
        )
    }
}
