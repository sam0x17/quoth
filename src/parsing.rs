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
    source: Rc<Source>,
    position: usize,
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
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Nothing(Span);

impl Spanned for Nothing {
    fn span(&self) -> Span {
        self.0.clone()
    }
}

impl Display for Nothing {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl FromStr for Nothing {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse(s)
    }
}

impl Parsable for Nothing {
    fn parse(stream: &mut ParseStream) -> ParseResult<Self> {
        if stream.position < stream.source.len() {
            return Err(Error::new(
                stream.current_span(),
                format!(
                    "expected nothing, found `{}`",
                    stream.current_span().source_text()
                )
                .as_str(),
            ));
        }
        Ok(Nothing(stream.current_span()))
    }
}

#[test]
fn test_parse_nothing() {
    let mut stream = ParseStream::from("");
    stream.parse::<Nothing>().unwrap();
    let mut stream = ParseStream::from("this won't work");
    assert!(stream.parse::<Nothing>().is_err());
}
