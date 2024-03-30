use std::{fmt::Display, str::FromStr};

use super::*;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[must_use]
pub enum Optional<T: Parsable> {
    Some(T),
    None,
}

impl<T: Parsable> Optional<T> {
    pub fn is_none(&self) -> bool {
        *self == Optional::None
    }

    pub fn is_some(&self) -> bool {
        *self != Optional::None
    }
}

impl<T: Parsable> From<Option<T>> for Optional<T> {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(val) => Optional::Some(val),
            None => Optional::None,
        }
    }
}

impl<T: Parsable> From<Optional<T>> for Option<T> {
    fn from(value: Optional<T>) -> Self {
        match value {
            Optional::Some(val) => Some(val),
            Optional::None => None,
        }
    }
}

impl<T: Parsable> Spanned for Optional<T> {
    fn span(&self) -> Span {
        match self {
            Optional::Some(val) => val.span().clone(),
            Optional::None => Span::blank(),
        }
    }
}

impl<T: Parsable> Display for Optional<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Optional::Some(val) => write!(f, "{val}"),
            Optional::None => Ok(()),
        }
    }
}

impl<T: Parsable> FromStr for Optional<T> {
    type Err = T::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Ok(val) = s.parse() else {
            return Ok(Optional::None);
        };
        Ok(Optional::Some(val))
    }
}

impl<T: Parsable> Parsable for Optional<T> {
    fn parse(stream: &mut ParseStream) -> ParseResult<Self> {
        if stream.peek::<T>() {
            return Ok(Optional::Some(stream.parse::<T>()?));
        }
        Ok(Optional::None)
    }

    fn parse_value(value: Self, stream: &mut ParseStream) -> ParseResult<Self> {
        match value {
            Optional::Some(val) => stream.parse_value(val).map(Optional::Some),
            Optional::None => Ok(Optional::None),
        }
    }
}

#[test]
fn test_parse_optional() {
    use super::numbers::*;
    let mut stream = ParseStream::from("hey");
    let parsed = stream.parse::<Optional<Everything>>().unwrap();
    assert_eq!(parsed.span().source_text(), "hey");
    assert!(stream
        .parse::<U64>()
        .unwrap_err()
        .to_string()
        .contains("expected digit"));
    let mut stream = ParseStream::from("99 hey");
    let parsed = stream.parse::<Optional<U64>>().unwrap();
    assert_eq!(parsed.span().source_text(), "99");
    assert!(parsed.is_some());
    let parsed = stream.parse::<Optional<U64>>().unwrap();
    assert_eq!(parsed, Optional::None);
    assert!(parsed.is_none());
    let mut stream = ParseStream::from("174 hey");
    let parsed = stream
        .parse_value(Optional::Some(Exact::from("174")))
        .unwrap();
    assert_eq!(parsed.span().source_text(), "174");
    let parsed = stream
        .parse_value(Optional::Some(Exact::from("22")))
        .unwrap_err();
    assert!(parsed.to_string().contains("expected `22`"));
    let parsed = stream.parse::<Optional<U64>>().unwrap();
    assert!(parsed.is_none());
}
