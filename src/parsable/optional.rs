use std::{fmt::Display, str::FromStr};

use super::*;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[must_use]
pub enum Optional<T: Parsable> {
    Some(T),
    None,
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
        todo!()
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
        todo!()
    }

    fn set_span(&mut self, span: impl Into<Span>) {
        todo!()
    }
}
