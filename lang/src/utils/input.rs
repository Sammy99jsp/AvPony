use std::{
    fs::{self},
    path::Path,
    sync::Arc,
};

use chumsky::input::{Input, WithContext};

use super::span::Span;

///
/// Input for any Pony parser.
///
pub type PonyInput<'src> = WithContext<Span, &'src str>;

///
/// The raw source code of a file.
///
pub struct SourceFile {
    path: Arc<str>,
    contents: String,
}

impl SourceFile {
    ///
    /// Read from a file on disk.
    ///
    pub fn read(path: impl AsRef<Path>) -> std::io::Result<Self> {
        let path = path.as_ref();
        let contents = fs::read_to_string(path)?;

        Ok(Self {
            path: path.to_string_lossy().into(),
            contents,
        })
    }

    ///
    /// Make a stream of this file,
    /// to be used by a [chumsky::Parser]
    ///
    pub fn stream(&self) -> WithContext<Span, &str> {
        (&self.contents).with_context(self.path.clone())
    }

    ///
    /// Make a virtual file, for unit testing code.
    ///
    #[cfg(test)]
    pub fn test_file(contents: impl ToString) -> (Self, (Arc<str>, ariadne::Source)) {
        let contents = contents.to_string();
        (
            Self {
                path: "TEST".into(),
                contents: contents.clone(),
            },
            ("TEST".into(), ariadne::Source::from(contents)),
        )
    }
}
