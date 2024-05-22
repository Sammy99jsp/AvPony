//!
//! ## Error types.
//!

#[derive(Debug, Clone)]
pub struct UnexpectedToken {
    span: super::Span,
    found: Option<char>,
    expected: Vec<char>,
    extra: Option<String>,
}

impl UnexpectedToken {
    fn with_label(mut self, label: String) -> Self {
        self.extra = Some(label);
        self
    }
}

#[derive(Debug, Clone)]
pub enum Error {
    UnexpectedToken(UnexpectedToken),
}

impl chumsky::Error<char> for self::Error {
    type Span = super::Span;
    type Label = String;

    fn expected_input_found<Iter: IntoIterator<Item = Option<char>>>(
        span: Self::Span,
        expected: Iter,
        found: Option<char>,
    ) -> Self {
        Self::UnexpectedToken(UnexpectedToken {
            span,
            expected: expected.into_iter().flatten().collect(),
            found,
            extra: None,
        })
    }

    fn with_label(self, label: Self::Label) -> Self {
        match self {
            Error::UnexpectedToken(err) => Self::UnexpectedToken(err.with_label(label)),
        }
    }

    fn merge(self, _: Self) -> Self {
        // TODO: Review this!
        self
    }
}
