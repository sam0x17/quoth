use core::{
    fmt::{Debug, Display},
    hash::Hash,
};
use std::{cmp::min, ops::Deref, rc::Rc, str::FromStr};

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

    pub fn expected(span: Span, expected: impl Display) -> Error {
        Error(Diagnostic::new(
            DiagnosticLevel::Error,
            span,
            format!("expected `{expected}`"),
            Option::<String>::None,
            Vec::new(),
        ))
    }
}

pub type ParseResult<T> = core::result::Result<T, Error>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ParseStream {
    source: Rc<Source>,
    pub position: usize,
}

impl ParseStream {
    pub fn source(&self) -> &Rc<Source> {
        &self.source
    }

    pub fn current_span(&self) -> Span {
        Span::new(
            self.source.clone(),
            self.position..(min(self.source().len(), self.position + 1)),
        )
    }

    pub fn remaining_span(&self) -> Span {
        Span::new(self.source.clone(), self.position..self.source.len())
    }

    pub fn parse<T: Parsable>(&mut self) -> ParseResult<T> {
        T::parse(self)
    }

    pub fn parse_value<T: Parsable>(&mut self, value: T) -> ParseResult<T> {
        T::parse_value(value, self)
    }

    pub fn remaining(&self) -> &str {
        &self.source[self.position..]
    }

    pub fn fork(&self) -> Self {
        self.clone()
    }

    pub fn consume(&mut self, num_chars: usize) -> ParseResult<Span> {
        if self.remaining().len() < num_chars {
            return Err(Error::new(
                self.remaining_span(),
                format!(
                    "expected at least {num_chars} more characters, found {}",
                    self.remaining().len()
                ),
            ));
        }
        let position = self.position;
        self.position += num_chars;
        Ok(Span::new(self.source.clone(), position..self.position))
    }

    pub fn consume_remaining(&mut self) -> Span {
        let span = self.remaining_span();
        self.position = self.source.len();
        span
    }

    pub fn next_char(&self) -> ParseResult<char> {
        if self.remaining().is_empty() {
            return Err(Error::new(self.current_span(), "unexpected end of input"));
        }
        let c = self
            .current_span()
            .source_text()
            .chars()
            .collect::<Vec<_>>()
            .first()
            .cloned()
            .unwrap();
        Ok(c)
    }

    pub fn parse_char(&mut self) -> ParseResult<char> {
        let c = self.next_char()?;
        self.position += 1;
        Ok(c)
    }

    pub fn next_digit(&self) -> ParseResult<u8> {
        Ok(match self.next_char()? {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            _ => return Err(Error::new(self.current_span(), "expected digit (0-9)")),
        })
    }

    pub fn parse_digit(&mut self) -> ParseResult<u8> {
        let digit = self.next_digit()?;
        self.position += 1;
        Ok(digit)
    }

    pub fn next_alpha(&self) -> ParseResult<char> {
        let c = self.next_char()?;
        if !c.is_ascii_alphabetic() {
            return Err(Error::new(
                self.current_span(),
                "expected alphabetic (A-Z|a-z)",
            ));
        }
        Ok(c)
    }

    pub fn parse_alpha(&mut self) -> ParseResult<char> {
        let c = self.next_alpha()?;
        self.position += 1;
        Ok(c)
    }

    pub fn peek<T: Peekable>(&mut self) -> bool {
        T::peek(self)
    }

    pub fn peek_value<T: Peekable>(&mut self, value: T) -> bool {
        T::peek_value(value, self)
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

pub fn common_prefix(s1: &str, s2: &str) -> String {
    let mut result = String::new();
    for (b1, b2) in s1.bytes().zip(s2.bytes()) {
        if b1 == b2 {
            result.push(b1 as char);
        } else {
            break;
        }
    }
    result
}

pub trait Parsable:
    Clone + Debug + PartialEq + Eq + Hash + Display + Spanned + FromStr + Peekable
{
    fn parse(stream: &mut ParseStream) -> ParseResult<Self>;

    fn parse_value(value: Self, stream: &mut ParseStream) -> ParseResult<Self> {
        let s = value.span();
        let text = s.source_text();
        if stream.remaining().starts_with(text) {
            let mut value = value;
            value.set_span(stream.consume(text.len())?);
            return Ok(value);
        }
        let prefix = common_prefix(text, stream.remaining());
        let expected = &text[prefix.len()..];
        let span = Span::new(
            stream.source.clone(),
            (stream.position + prefix.len())..(stream.position + text.len()),
        );
        Err(Error::expected(span, expected))
    }

    fn set_span(&mut self, span: impl Into<Span>);

    fn unparse(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span().source_text())
    }
}

impl<T: Parsable> Peekable for T {
    fn peek(stream: &mut ParseStream) -> bool {
        stream.fork().parse::<Self>().is_ok()
    }

    fn peek_value(value: Self, stream: &mut ParseStream) -> bool {
        let Ok(parsed) = stream.fork().parse::<Self>() else {
            return false;
        };
        parsed == value
    }
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

pub trait Peekable {
    fn peek(stream: &mut ParseStream) -> bool;
    fn peek_value(value: Self, stream: &mut ParseStream) -> bool;
}

impl Peekable for &str {
    fn peek(_: &mut ParseStream) -> bool {
        true
    }

    fn peek_value(value: Self, stream: &mut ParseStream) -> bool {
        stream.remaining().starts_with(value)
    }
}

impl Peekable for String {
    fn peek(_: &mut ParseStream) -> bool {
        true
    }

    fn peek_value(value: Self, stream: &mut ParseStream) -> bool {
        stream.remaining().starts_with(&value)
    }
}

impl Peekable for &String {
    fn peek(_: &mut ParseStream) -> bool {
        true
    }

    fn peek_value(value: Self, stream: &mut ParseStream) -> bool {
        stream.remaining().starts_with(value)
    }
}

#[test]
fn test_parse_digit() {
    let mut stream = ParseStream::from("0183718947");
    assert_eq!(stream.parse_digit().unwrap(), 0);
    assert_eq!(stream.parse_digit().unwrap(), 1);
    assert_eq!(stream.parse_digit().unwrap(), 8);
    assert_eq!(stream.parse_digit().unwrap(), 3);
    assert_eq!(stream.parse_digit().unwrap(), 7);
    assert_eq!(stream.parse_digit().unwrap(), 1);
    assert_eq!(stream.parse_digit().unwrap(), 8);
    assert_eq!(stream.parse_digit().unwrap(), 9);
    assert_eq!(stream.parse_digit().unwrap(), 4);
    assert_eq!(stream.parse_digit().unwrap(), 7);
    stream.parse_digit().unwrap_err();
    let mut stream = ParseStream::from("hey");
    stream.parse_digit().unwrap_err();
}

#[test]
fn test_peeking() {
    use parsable::*;

    let mut stream = ParseStream::from("hey 48734 is cool");
    assert!(stream.peek::<String>());
    assert!(stream.peek::<&str>());
    assert!(stream.peek::<&String>());
    assert_eq!(stream.peek::<Nothing>(), false);
    assert!(stream.peek::<Everything>());
    // assert_eq!(
    //     stream.parse_value(Exact::from("hey ")).unwrap().to_string(),
    //     "hey "
    // );
}
