//!
//! ## Error types.
//!

pub mod number;

use avpony_macros::Spanned;

use self::number::{DivdersBadlyPlaced, InvalidInt, MultipleNumericDividers};

#[derive(Debug, Clone, Spanned, PartialEq)]
pub enum Error {
    UnexpectedToken(UnexpectedToken),
    InvalidInt(InvalidInt),
    MultipleNumericDividers(MultipleNumericDividers),
    DivdersBadlyPlaced(DivdersBadlyPlaced),
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
            Self::UnexpectedToken(err) => Self::UnexpectedToken(err.with_label(label)),
            Self::InvalidInt(err) => Self::InvalidInt(err.with_label(label)),
            Self::MultipleNumericDividers(err) => {
                Self::MultipleNumericDividers(err.with_label(label))
            }
            Self::DivdersBadlyPlaced(err) => Self::DivdersBadlyPlaced(err.with_label(label)),
        }
    }

    fn merge(self, _: Self) -> Self {
        // TODO: Review this!
        self
    }
}

pub trait ErrorI: Sized + super::Spanned + Into<Error> + PartialEq {
    fn with_label(self, label: <Error as chumsky::Error<char>>::Label) -> Self;
    fn to_report(self) -> ariadne::Report<'static, super::Span>;
}

impl ErrorI for Error {
    fn with_label(self, label: <Error as chumsky::Error<char>>::Label) -> Self {
        <Self as chumsky::Error<char>>::with_label(self, label)
    }

    fn to_report(self) -> ariadne::Report<'static, super::Span> {
        match self {
            Error::UnexpectedToken(err) => err.to_report(),
            Error::InvalidInt(err) => err.to_report(),
            Error::MultipleNumericDividers(err) => err.to_report(),
            Error::DivdersBadlyPlaced(err) => err.to_report(),
        }
    }
}

#[derive(Debug, Clone, Spanned, PartialEq)]
pub struct UnexpectedToken {
    span: super::Span,
    found: Option<char>,
    expected: Vec<char>,
    extra: Option<String>,
}

impl From<UnexpectedToken> for Error {
    fn from(value: UnexpectedToken) -> Self {
        Self::UnexpectedToken(value)
    }
}

impl ErrorI for UnexpectedToken {
    fn with_label(mut self, label: String) -> Self {
        self.extra = Some(label);
        self
    }

    fn to_report(self) -> ariadne::Report<'static, super::Span> {
        todo!()
    }
}
