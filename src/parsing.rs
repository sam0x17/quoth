//! Parsing utilities for Quoth, including [`ParseStream`], [`Parsable`], etc..

use core::{
    fmt::{Debug, Display},
    hash::Hash,
};
use regex::Regex;
use std::{cmp::min, ops::Deref, rc::Rc, str::FromStr};

use self::parsable::Exact;

use super::*;

/// Represents an error that occurred during parsing.
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
    /// Creates a new [`Error`] with the given [`Span`] and message.
    pub fn new(span: Span, message: impl ToString) -> Error {
        Error(Diagnostic::new(
            DiagnosticLevel::Error,
            span,
            message,
            Option::<String>::None,
            Vec::new(),
        ))
    }

    /// Creates a new [`Error`] expecting a certain value at the given [`Span`].
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

/// Represents the result of a parsing operation.
pub type Result<T> = core::result::Result<T, Error>;

/// Represents a stream of text that can be parsed.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ParseStream {
    source: Rc<Source>,
    /// The current parsing position in the source text as an offset from the beginning of the
    /// source. Advancing this position will consume characters from the source.
    pub position: usize,
}

impl ParseStream {
    /// Returns the source text that this [`ParseStream`] is parsing.
    pub fn source(&self) -> &Rc<Source> {
        &self.source
    }

    /// Returns the current [`Span`] of the [`ParseStream`]. This [`Span`] represents the
    /// current character being parsed.
    pub fn current_span(&self) -> Span {
        Span::new(
            self.source.clone(),
            self.position..(min(self.source().len(), self.position + 1)),
        )
    }

    /// Returns the remaining [`Span`] of the [`ParseStream`]. This [`Span`] represents the remaining
    ///
    pub fn remaining_span(&self) -> Span {
        Span::new(self.source.clone(), self.position..self.source.len())
    }

    /// Attempts to parse a value of type `T` from the [`ParseStream`].
    pub fn parse<T: Parsable>(&mut self) -> Result<T> {
        T::parse(self)
    }

    /// Attempts to parse a specific value of type `T` from the [`ParseStream`].
    pub fn parse_value<T: Parsable>(&mut self, value: T) -> Result<T> {
        T::parse_value(value, self)
    }

    /// note: panics upon invalid regex syntax
    pub fn parse_regex(&mut self, reg: impl Pattern) -> Result<Exact> {
        let reg = reg.to_regex();
        match reg.find(self.remaining()) {
            Some(m) => {
                if m.start() > 0 {
                    return Err(Error::new(
                        self.current_span(),
                        format!("expected match for `{reg}`"),
                    ));
                }
                let start_position = self.position;
                self.position += m.len();
                Ok(Exact::new(Span::new(
                    self.source.clone(),
                    start_position..self.position,
                )))
            }
            None => Err(Error::new(
                self.current_span(),
                format!("expected match for `{reg}`"),
            )),
        }
    }

    /// Peeks at the [`ParseStream`] to see if it can parse the specified regex pattern as the
    /// next value in the [`Source`].
    ///
    /// note: panics upon invalid regex syntax
    ///
    /// Analogue of [`ParseStream::parse_regex`].
    pub fn peek_regex(&self, reg: Regex) -> bool {
        self.fork().parse_regex(reg).is_ok()
    }

    /// Attempts to parse the specified string from the [`ParseStream`].
    ///
    /// Analogue of [`ParseStream::peek_str`].
    pub fn parse_str(&mut self, value: impl ToString) -> Result<Exact> {
        self.parse_value(Exact::from(value))
    }

    /// Attempts to parse the specified string from the [`ParseStream`] case-insensitively.
    ///
    /// Analogue of [`ParseStream::peek_istr`].
    pub fn parse_istr(&mut self, value: impl ToString) -> Result<Exact> {
        let text = value.to_string().to_lowercase();
        let remaining_lower = self.remaining().to_lowercase();
        if remaining_lower.starts_with(&text) {
            return Ok(Exact::new(self.consume(text.len())?));
        }
        let prefix = common_prefix(&text, &remaining_lower);
        let expected = &text[prefix.len()..];
        let span = Span::new(
            self.source.clone(),
            (self.position + prefix.len())..(self.position + text.len()),
        );
        self.position += prefix.len();
        Err(Error::expected(span, expected))
    }

    /// Peeks at the [`ParseStream`] to see if it can parse the specified string as the next value.
    ///
    /// Analogue of [`ParseStream::parse_str`].
    pub fn peek_str(&self, s: impl AsRef<str>) -> bool {
        self.remaining().starts_with(s.as_ref())
    }

    /// Peeks at the [`ParseStream`] to see if it can parse the specified string case-insensitively.
    ///
    /// Analogue of [`ParseStream::parse_istr`].
    pub fn peek_istr(&self, s: impl ToString) -> bool {
        self.remaining()
            .to_lowercase()
            .starts_with(&s.to_string().to_lowercase())
    }

    /// Attempts to parse any value of the specified values from the [`ParseStream`].
    ///
    /// Analogue of [`ParseStream::peek_any_value_of`].
    pub fn parse_any_value_of<T: Parsable, const N: usize>(&mut self, values: [T; N]) -> Result<T> {
        for i in 0..N {
            if self.peek_value(values[i].clone()) {
                return self.parse_value(values[i].clone());
            }
        }
        Err(Error::new(
            self.current_span(),
            format!(
                "expected one of {}",
                values
                    .into_iter()
                    .map(|v| format!("`{}`", v.span().source_text()))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
        ))
    }

    /// Attempts to parse any string of the specified values from the [`ParseStream`].
    ///
    /// Analogue of [`ParseStream::peek_any_str_of`].
    pub fn parse_any_str_of<const N: usize>(
        &mut self,
        values: [impl ToString; N],
    ) -> Result<(Exact, usize)> {
        for (i, s) in values.iter().enumerate() {
            let s = s.to_string();
            if self.peek_str(&s) {
                return Ok((self.parse_str(s)?, i));
            }
        }
        Err(Error::new(
            self.current_span(),
            format!(
                "expected one of {}",
                values
                    .into_iter()
                    .map(|s| format!("`{}`", s.to_string()))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
        ))
    }

    /// Attempts to parse any specified strings from the [`ParseStream`] case-insensitively.
    ///
    /// Analogue of [`ParseStream::peek_any_istr_of`].
    pub fn parse_any_istr_of<const N: usize>(
        &mut self,
        values: [impl ToString; N],
    ) -> Result<(Exact, usize)> {
        for (i, s) in values.iter().enumerate() {
            let s = s.to_string();
            if self.peek_istr(&s) {
                return Ok((self.parse_istr(s)?, i));
            }
        }
        Err(Error::new(
            self.current_span(),
            format!(
                "expected one of {}",
                values
                    .into_iter()
                    .map(|s| format!("`{}`", s.to_string()))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
        ))
    }

    /// Peeks at the [`ParseStream`] to see if it can parse any of the specified values.
    ///
    /// Analogue of [`ParseStream::parse_any_value_of`].
    pub fn peek_any_value_of<T: Parsable, const N: usize>(&self, values: [T; N]) -> bool {
        self.fork().parse_any_value_of(values).is_ok()
    }

    /// Analogue of [`ParseStream::parse_any_str_of`].
    pub fn peek_any_str_of<const N: usize>(&self, values: [impl ToString; N]) -> bool {
        self.fork().parse_any_str_of(values).is_ok()
    }

    /// Analogue of [`ParseStream::parse_any_istr_of`].
    pub fn peek_any_istr_of<const N: usize>(&self, values: [impl ToString; N]) -> bool {
        self.fork().parse_any_istr_of(values).is_ok()
    }

    /// Returns the remaining text in the [`ParseStream`] that has not been parsed.
    ///
    /// The first character of the remaining text is the next character to be parsed.
    pub fn remaining(&self) -> &str {
        &self.source[self.position..]
    }

    /// Cheaply clones the [`ParseStream`] creating a new one at the same position of the
    /// original that can be used to parse independently without consuming characters from the
    /// original.
    ///
    /// This merely involves cloning a single [`Rc`] and a [`usize`], which is why it is so cheap.
    pub fn fork(&self) -> Self {
        self.clone()
    }

    /// Consumes the specified number of characters from the [`ParseStream`] and returns the
    /// consumed characters as a [`Span`].
    ///
    /// Returns an error if the [`ParseStream`] has less remaining characters than `num_chars`.
    pub fn consume(&mut self, num_chars: usize) -> Result<Span> {
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

    /// Consumes the remaining text in the [`ParseStream`] and returns it as a [`Span`].
    pub fn consume_remaining(&mut self) -> Span {
        let span = self.remaining_span();
        self.position = self.source.len();
        span
    }

    /// Tries to return the next character in the [`ParseStream`] without consuming it.
    ///
    /// Returns an error if the [`ParseStream`] is at the end of its input.
    pub fn next_char(&self) -> Result<char> {
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

    /// Parses the next character in the [`ParseStream`] and advances the position by one.
    ///
    /// Returns an error if the [`ParseStream`] is at the end of its input.
    pub fn parse_char(&mut self) -> Result<char> {
        let c = self.next_char()?;
        self.position += 1;
        Ok(c)
    }

    /// If the next character of the [`ParseStream`] is a digit (0-9), returns the digit as a
    /// `u8` _without_ consuming it, otherwise returns an error.
    pub fn next_digit(&self) -> Result<u8> {
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

    /// Tries to parse the next character in the [`ParseStream`] as a digit (0-9) and advances
    /// the position by one if successful.
    pub fn parse_digit(&mut self) -> Result<u8> {
        let digit = self.next_digit()?;
        self.position += 1;
        Ok(digit)
    }

    /// If the next character of the [`ParseStream`] is an alphabetic character (A-Z|a-z), the
    /// position is not advanced and the character is returned, otherwise an error is returned.
    pub fn next_alpha(&self) -> Result<char> {
        let c = self.next_char()?;
        if !c.is_ascii_alphabetic() {
            return Err(Error::new(
                self.current_span(),
                "expected alphabetic (A-Z|a-z)",
            ));
        }
        Ok(c)
    }

    /// Tries to parse the next character in the [`ParseStream`] as an alphabetic character.
    pub fn parse_alpha(&mut self) -> Result<char> {
        let c = self.next_alpha()?;
        self.position += 1;
        Ok(c)
    }

    /// Returns a boolean indicating whether the [`ParseStream`] can parse the specified
    /// [`Parsable`] type at its current position.
    pub fn peek<T: Peekable>(&self) -> bool {
        T::peek(self)
    }

    /// Returns a boolean indicating whether the [`ParseStream`] can parse a specific
    /// [`Peekable`] value at its current position.
    pub fn peek_value<T: Peekable>(&self, value: T) -> bool {
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

/// Attempts to parse the specified string into a value of type `T`.
pub fn parse<T: Parsable>(stream: impl Into<ParseStream>) -> Result<T> {
    T::parse(&mut stream.into())
}

/// Utility function to find the common prefix between two [`str`]s.
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

/// Types that can be parsed using Quoth must implement this trait.
///
/// Note that to satisfy the requirements of [`Parsable`], implementers should implement
/// [`Parsable`] and [`Spanned`] on the type directly, and derive [`ParsableExt`] on the type
/// to get suitable, required impls for [`FromStr`] and [`Display`].
///
/// It is undefined behavior to manually implement [`FromStr`] and [`Display`] on a
/// [`Parsable`] such that they do not correspond with [`Parsable::parse`] and
/// [`Parsable::unparse`] respectively.
///
/// Types that have more than one possible string representation or can be zero-sized (such as
/// [`parsable::Optional`] and [`parsable::Exact`], should implement
/// [`Parsable::parse_value`] manually. Otherwise the default
///
/// # Example
///
/// ```
/// use quoth::*;
///
/// #[derive(Clone, Debug, PartialEq, Eq, Hash, ParsableExt)]
/// pub struct Where(Span);
///
/// impl Parsable for Where {
///    fn parse(stream: &mut ParseStream) -> Result<Self> {
///         Ok(Where(stream.parse_istr("where")?.span()))
///    }
/// }
///
/// impl Spanned for Where {
///     fn span(&self) -> Span {
///        self.0.clone()
///     }
/// }
///
/// let mut stream = ParseStream::from("where are you");
/// let parsed = stream.parse::<Where>().unwrap();
/// assert_eq!(parsed.span().source_text(), "where");
/// assert_eq!(parsed.to_string(), "where");
/// assert_eq!(stream.remaining(), " are you");
/// ```
pub trait Parsable:
    Clone + Debug + PartialEq + Eq + Hash + Display + Spanned + FromStr + Peekable
{
    /// Attempts to parse the specified string into a value of type `T`.
    fn parse(stream: &mut ParseStream) -> Result<Self>;

    /// Attempts to parse a specific value of type `T` from the [`ParseStream`].
    fn parse_value(value: Self, stream: &mut ParseStream) -> Result<Self> {
        let s = value.span();
        let text = s.source_text();
        if stream.remaining().starts_with(text) {
            return stream.parse();
        }
        let prefix = common_prefix(text, stream.remaining());
        let expected = &text[prefix.len()..];
        let span = Span::new(
            stream.source.clone(),
            (stream.position + prefix.len())..(stream.position + text.len()),
        );
        stream.position += prefix.len();
        Err(Error::expected(span, expected))
    }

    /// The reverse of [`Parsable::parse`], this function should return the string
    /// representation of the value.
    ///
    /// This function is used to implement [`Display`] for the type when you derive [`ParsableExt`].
    fn unparse(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span().source_text())
    }
}

impl<T: Parsable> Peekable for T {
    fn peek(stream: &ParseStream) -> bool {
        stream.fork().parse::<Self>().is_ok()
    }

    fn peek_value(value: Self, stream: &ParseStream) -> bool {
        stream.fork().parse_value(value).is_ok()
    }
}

/// Indicates that a type can be used to peek at a [`ParseStream`].
pub trait Peekable {
    /// Returns a boolean indicating whether the [`ParseStream`] can parse the specified type
    /// at the current parsing position.
    fn peek(stream: &ParseStream) -> bool;

    /// Returns a boolean indicating whether the [`ParseStream`] can parse the specified value
    /// at the current parsing position.
    fn peek_value(value: Self, stream: &ParseStream) -> bool;
}

impl Peekable for &str {
    fn peek(_: &ParseStream) -> bool {
        true
    }

    fn peek_value(value: Self, stream: &ParseStream) -> bool {
        stream.remaining().starts_with(value)
    }
}

impl Peekable for String {
    fn peek(_: &ParseStream) -> bool {
        true
    }

    fn peek_value(value: Self, stream: &ParseStream) -> bool {
        stream.remaining().starts_with(&value)
    }
}

impl Peekable for &String {
    fn peek(_: &ParseStream) -> bool {
        true
    }

    fn peek_value(value: Self, stream: &ParseStream) -> bool {
        stream.remaining().starts_with(value)
    }
}

/// Generic over types that can be used to create a Regex
pub trait Pattern: Sized {
    /// Tries to derive a [`Regex`] from the underlying value, panicking if the underlying
    /// value is not valid regex syntax.
    fn to_regex(self) -> Regex {
        self.try_to_regex().unwrap()
    }

    /// Tries to derive a [`Regex`] from the underlying value, returning a [`regex::Error`] if
    /// the value is not a valid [`Regex`].
    fn try_to_regex(self) -> core::result::Result<Regex, regex::Error>;
}

impl Pattern for Regex {
    fn try_to_regex(self) -> core::result::Result<Regex, regex::Error> {
        Ok(self)
    }
}

impl Pattern for &str {
    fn try_to_regex(self) -> core::result::Result<Regex, regex::Error> {
        Regex::new(self)
    }
}

impl Pattern for String {
    fn try_to_regex(self) -> core::result::Result<Regex, regex::Error> {
        Regex::new(&self)
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
    assert_eq!(
        stream.parse_value(Exact::from("hey ")).unwrap().to_string(),
        "hey "
    );
}

#[test]
fn test_parse_any_value_of() {
    use parsable::*;

    let mut stream = ParseStream::from("this 99.2 is really cool");
    assert!(stream.peek_value(Exact::from("this")));
    assert!(stream.peek_value("this"));
    let parsed = stream
        .parse_any_value_of([
            Exact::from("yo"),
            Exact::from("this"),
            Exact::from("this 99.2"),
        ])
        .unwrap();
    assert_eq!(parsed.to_string(), "this");
    assert!(!stream.peek_value(Exact::from(" 998")));
    assert!(stream.peek_any_str_of([" 998", " 99.2"]));
    assert!(stream.peek_any_istr_of([" 99.2 z", " 99.2 IS"]));
    assert!(stream.parse_any_istr_of([" asdf", " 99.2 iS"]).unwrap().1 == 1);
}

#[test]
fn test_str_peeking_and_parsing() {
    let mut stream = ParseStream::from("here ARe 222.44 some cool things");
    assert!(stream.peek_str("here"));
    assert!(stream.peek_istr("HeRe"));
    assert!(!stream.peek_str("HeRe"));
    let parsed = stream.parse_istr("HERe ").unwrap();
    assert_eq!(parsed.to_string(), "here ");
    assert!(!stream.peek_str("are"));
    assert!(stream.peek_istr("arE"));
    let parsed = stream.parse_str("ARe ").unwrap();
    assert_eq!(parsed.span().source_text(), "ARe ");
}

#[test]
fn test_regex_parsing() {
    let mut stream = ParseStream::from("$33.29");
    let parsed = stream
        .parse_regex(r"(?i)\$?-?\d{1,3}(?:,\d{3})*(?:\.\d{1,2})?")
        .unwrap();
    assert_eq!(parsed.span().source_text(), "$33.29");
    let mut stream = ParseStream::from("$33.29");
    let parsed = stream
        .parse_regex(r"^(?i)\$?-?\d{1,3}(?:,\d{3})*(?:\.\d{1,2})?$")
        .unwrap();
    assert_eq!(parsed.span().source_text(), "$33.29");
    let mut stream = ParseStream::from("asdf33.29");
    let parsed = stream
        .parse_regex(r"^(?i)\$?-?\d{1,3}(?:,\d{3})*(?:\.\d{1,2})?$")
        .unwrap_err();
    assert!(parsed.to_string().contains("expected match for"));
    let mut stream = ParseStream::from("hey what $33.29");
    let parsed = stream
        .parse_regex(r"(?i)\$?-?\d{1,3}(?:,\d{3})*(?:\.\d{1,2})?")
        .unwrap_err();
    assert!(parsed.to_string().contains("expected match for"));
}
