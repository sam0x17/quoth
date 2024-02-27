use core::{
    fmt::{Debug, Display},
    hash::Hash,
};
use std::{ops::Deref, rc::Rc, str::FromStr};

use super::*;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Error(Diagnostic);

impl Deref for Error {
    type Target = Diagnostic;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error {
    pub fn new(span: Span, message: impl ToString) -> Error {
        Error(Diagnostic::new(
            DiagnosticLevel::Error,
            span,
            message,
            Option::<String>::None,
            Vec::new(),
        ))
    }
}

pub type ParseResult<T> = core::result::Result<T, Error>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ParseStream {
    pub source: Rc<Source>,
    pub position: usize,
}

impl ParseStream {
    pub fn current_span(&self) -> Span {
        Span::new(self.source.clone(), self.position..(self.position + 1))
    }

    pub fn parse<T: Parsable>(&mut self) -> ParseResult<T> {
        T::parse(self)
    }
}

impl<S: Into<Source>> From<S> for ParseStream {
    fn from(value: S) -> Self {
        ParseStream {
            source: Rc::new(value.into()),
            position: 0,
        }
    }
}

pub fn parse<T: Parsable>(stream: impl Into<ParseStream>) -> ParseResult<T> {
    T::parse(&mut stream.into())
}

pub trait Parsable: Clone + Debug + PartialEq + Eq + Hash + Display + Spanned + FromStr {
    fn parse(stream: &mut ParseStream) -> ParseResult<Self>;

    fn unparse(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}

#[macro_export]
macro_rules! make_parsable {
    ($ident:ident) => {
        impl core::fmt::Display for $ident {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                self.unparse(f)
            }
        }

        impl core::str::FromStr for $ident {
            type Err = crate::Error;

            fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
                crate::parse(s)
            }
        }
    };
}
