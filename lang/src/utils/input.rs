use std::{
    fs::{self},
    path::Path,
    sync::Arc,
};

use chumsky::input::{Input, WithContext};

use super::span::Span;

pub type PonyInput<'src> = WithContext<Span, &'src str>;

pub struct SourceFile {
    path: Arc<str>,
    contents: String,
}

impl SourceFile {
    pub fn from_file(path: impl AsRef<Path>) -> std::io::Result<Self> {
        let path = path.as_ref();
        let contents = fs::read_to_string(path)?;

        Ok(Self {
            path: path.to_string_lossy().into(),
            contents,
        })
    }

    pub fn stream(&self) -> WithContext<Span, &str> {
        (&self.contents).with_context(self.path.clone())
    }

    #[cfg(test)]
    pub fn test_file(contents: impl ToString) -> (Self, ariadne::Source) {
        let contents = contents.to_string();
        (
            Self {
                path: "TEST".into(),
                contents: contents.clone(),
            },
            ariadne::Source::from(contents),
        )
    }
}
