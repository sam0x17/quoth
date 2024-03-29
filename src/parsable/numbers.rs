use std::rc::Rc;

use super::*;

// enables usage of quoth proc macros within quoth
use crate as quoth;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct U64(u64, Span);

impl U64 {
    pub fn value(&self) -> u64 {
        self.0
    }
}

impl Spanned for U64 {
    fn span(&self) -> Span {
        self.1.clone()
    }
}

impl Parsable for U64 {
    fn parse(stream: &mut ParseStream) -> ParseResult<Self> {
        let mut digits = Vec::new();
        let start_position = stream.position;
        while let Ok(_) = stream.next_digit() {
            digits.push(stream.parse_digit()?);
        }
        if digits.is_empty() {
            return Err(Error::new(stream.current_span(), "expected digit"));
        }
        let digits = digits
            .into_iter()
            .map(|d| d.to_string())
            .collect::<String>();
        let parsed: u64 = match digits.parse() {
            Ok(val) => val,
            Err(err) => {
                return Err(Error::new(
                    Span::new(stream.source().clone(), start_position..stream.position),
                    err.to_string(),
                ))
            }
        };
        let span = Span::new(stream.source().clone(), start_position..stream.position);
        Ok(U64(parsed, span))
    }

    fn set_span(&mut self, span: impl Into<Span>) {
        self.1 = span.into();
    }
}

make_parsable!(U64);

impl From<U64> for u64 {
    fn from(value: U64) -> Self {
        value.0
    }
}

impl From<U64> for u128 {
    fn from(value: U64) -> Self {
        value.0.into()
    }
}

impl From<U64> for i128 {
    fn from(value: U64) -> Self {
        value.0.into()
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct U128(u128, Span);

impl U128 {
    pub fn value(&self) -> u128 {
        self.0
    }
}

impl Spanned for U128 {
    fn span(&self) -> Span {
        self.1.clone()
    }
}

impl Parsable for U128 {
    fn parse(stream: &mut ParseStream) -> ParseResult<Self> {
        let mut digits = Vec::new();
        let start_position = stream.position;
        while let Ok(_) = stream.next_digit() {
            digits.push(stream.parse_digit()?);
        }
        if digits.is_empty() {
            return Err(Error::new(stream.current_span(), "expected digit"));
        }
        let digits = digits
            .into_iter()
            .map(|d| d.to_string())
            .collect::<String>();
        let parsed: u128 = match digits.parse() {
            Ok(val) => val,
            Err(err) => {
                return Err(Error::new(
                    Span::new(stream.source().clone(), start_position..stream.position),
                    err.to_string(),
                ))
            }
        };
        let span = Span::new(stream.source().clone(), start_position..stream.position);
        Ok(U128(parsed, span))
    }

    fn set_span(&mut self, span: impl Into<Span>) {
        self.1 = span.into();
    }
}

make_parsable!(U128);

impl From<U128> for u128 {
    fn from(value: U128) -> Self {
        value.0
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct I64(i64, Span);

impl I64 {
    pub fn value(&self) -> i64 {
        self.0
    }
}

impl Spanned for I64 {
    fn span(&self) -> Span {
        self.1.clone()
    }
}

impl Parsable for I64 {
    fn parse(stream: &mut ParseStream) -> ParseResult<Self> {
        let mut digits = Vec::new();
        let start_position = stream.position;
        let mut sign = 1;
        if stream.next_char()? == '-' {
            stream.consume(1)?;
            sign = -1;
        }
        while let Ok(_) = stream.next_digit() {
            digits.push(stream.parse_digit()?);
        }
        if digits.is_empty() {
            return Err(Error::new(stream.current_span(), "expected digit"));
        }
        let digits = digits
            .into_iter()
            .map(|d| d.to_string())
            .collect::<String>();
        let parsed: i64 = match digits.parse() {
            Ok(val) => val,
            Err(err) => {
                return Err(Error::new(
                    Span::new(stream.source().clone(), start_position..stream.position),
                    err.to_string(),
                ))
            }
        };
        let span = Span::new(stream.source().clone(), start_position..stream.position);
        Ok(I64(parsed * sign, span))
    }

    fn set_span(&mut self, span: impl Into<Span>) {
        self.1 = span.into();
    }
}

make_parsable!(I64);

impl From<I64> for i64 {
    fn from(value: I64) -> Self {
        value.0
    }
}

impl From<I64> for i128 {
    fn from(value: I64) -> Self {
        value.0.into()
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct I128(i128, Span);

impl I128 {
    pub fn value(&self) -> i128 {
        self.0
    }
}

impl Spanned for I128 {
    fn span(&self) -> Span {
        self.1.clone()
    }
}

impl Parsable for I128 {
    fn parse(stream: &mut ParseStream) -> ParseResult<Self> {
        let mut digits = Vec::new();
        let start_position = stream.position;
        let mut sign = 1;
        if stream.next_char()? == '-' {
            stream.consume(1)?;
            sign = -1;
        }
        while let Ok(_) = stream.next_digit() {
            digits.push(stream.parse_digit()?);
        }
        if digits.is_empty() {
            return Err(Error::new(stream.current_span(), "expected digit"));
        }
        let digits = digits
            .into_iter()
            .map(|d| d.to_string())
            .collect::<String>();
        let parsed: i128 = match digits.parse() {
            Ok(val) => val,
            Err(err) => {
                return Err(Error::new(
                    Span::new(stream.source().clone(), start_position..stream.position),
                    err.to_string(),
                ))
            }
        };
        let span = Span::new(stream.source().clone(), start_position..stream.position);
        Ok(I128(parsed * sign, span))
    }

    fn set_span(&mut self, span: impl Into<Span>) {
        self.1 = span.into();
    }
}

make_parsable!(I128);

impl From<I128> for i128 {
    fn from(value: I128) -> Self {
        value.0
    }
}

impl From<I64> for I128 {
    fn from(value: I64) -> Self {
        I128(value.0.into(), value.1)
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Decimal(rust_decimal::Decimal, Span);

impl Decimal {
    pub fn new(val: impl Into<rust_decimal::Decimal>, span: impl Into<Span>) -> Self {
        Decimal(val.into(), span.into())
    }

    pub fn value(&self) -> rust_decimal::Decimal {
        self.0
    }
}

impl From<rust_decimal::Decimal> for Decimal {
    fn from(value: rust_decimal::Decimal) -> Self {
        let st = value.to_string();
        let len = st.len();
        let span = Span::new(Rc::new(Source::from_str(st)), 0..len);
        Decimal(value, span.into())
    }
}

impl From<Decimal> for rust_decimal::Decimal {
    fn from(value: Decimal) -> Self {
        value.0
    }
}

impl Spanned for Decimal {
    fn span(&self) -> Span {
        self.1.clone()
    }
}

make_parsable!(Decimal);

impl Parsable for Decimal {
    fn parse(stream: &mut ParseStream) -> ParseResult<Self> {
        let start_position = stream.position;
        if stream.next_char()? == '-' {
            stream.consume(1)?;
        }
        stream.parse_digit()?;
        while let Ok(_) = stream.parse_digit() {}
        stream.parse_value(Exact::from("."))?;
        stream.parse_digit()?;
        while let Ok(_) = stream.parse_digit() {}
        let span = Span::new(stream.source().clone(), start_position..stream.position);
        Ok(Decimal(
            span.source_text()
                .parse()
                .map_err(|e| Error::new(span.clone(), e))?,
            span,
        ))
    }

    fn set_span(&mut self, span: impl Into<Span>) {
        self.1 = span.into();
    }
}

/// A bounded version of [`I64`].
///
/// Bounds are _inclusive_, so [`BoundedI64<3, 7>`] means only 3, 4, 5, 6, and 7 are allowed
/// as values.
#[derive(ParsableExt, Clone, PartialEq, Eq, Hash, Debug)]
pub struct BoundedI64<const MIN: i64, const MAX: i64>(I64);

impl<const MIN: i64, const MAX: i64> BoundedI64<MIN, MAX> {
    pub fn value(&self) -> i64 {
        self.0 .0
    }
}

impl<const MIN: i64, const MAX: i64> Spanned for BoundedI64<MIN, MAX> {
    fn span(&self) -> Span {
        self.0 .1.clone()
    }
}

impl<const MIN: i64, const MAX: i64> Parsable for BoundedI64<MIN, MAX> {
    fn parse(stream: &mut ParseStream) -> ParseResult<Self> {
        let i = stream.parse::<I64>()?;
        if i.0 < MIN {
            return Err(Error::new(
                i.span(),
                format!("must be greater than or equal to {MIN}"),
            ));
        }
        if i.0 > MAX {
            return Err(Error::new(
                i.span(),
                format!("must be less than or equal to {MAX}"),
            ));
        }
        Ok(BoundedI64(i))
    }

    fn set_span(&mut self, span: impl Into<Span>) {
        self.0 .1 = span.into();
    }
}

#[test]
fn test_parse_bounded_int64() {
    let mut stream = ParseStream::from("33");
    let parsed = stream.parse::<BoundedI64<20, 40>>().unwrap();
    assert_eq!(parsed.to_string(), "33");
    let mut stream = ParseStream::from("33");
    let parsed = stream.parse::<BoundedI64<34, 40>>().unwrap_err();
    assert!(parsed
        .to_string()
        .contains("must be greater than or equal to 34"));
    let mut stream = ParseStream::from("41");
    let parsed = stream.parse::<BoundedI64<34, 40>>().unwrap_err();
    assert!(parsed
        .to_string()
        .contains("must be less than or equal to 40"));
}

#[test]
fn test_parse_int128() {
    let mut stream = ParseStream::from("-34833749837489858394735");
    let parsed = stream.parse::<I128>().unwrap();
    assert_eq!(parsed.to_string(), "-34833749837489858394735");
    assert_eq!(parsed.value(), -34833749837489858394735);
    let mut stream = ParseStream::from("-hey");
    let parsed = stream.parse::<I64>().unwrap_err();
    assert!(parsed.to_string().contains("expected digit"));
}

#[test]
fn test_parse_int64() {
    let mut stream = ParseStream::from("-348385735");
    let parsed = stream.parse::<I64>().unwrap();
    assert_eq!(parsed.to_string(), "-348385735");
    assert_eq!(parsed.value(), -348385735);
    let mut stream = ParseStream::from("-hey");
    let parsed = stream.parse::<I64>().unwrap_err();
    assert!(parsed.to_string().contains("expected digit"));
}

#[test]
fn test_parse_decimal() {
    let mut stream = ParseStream::from("55.63");
    let parsed = stream.parse::<Decimal>().unwrap();
    assert_eq!(parsed.to_string(), "55.63");
    assert_eq!(parsed.value().to_string(), "55.63");
    let mut stream = ParseStream::from("hey");
    let parsed = stream.parse::<Decimal>().unwrap_err();
    assert!(parsed.to_string().contains("expected digit"));
    let mut stream = ParseStream::from("44");
    let parsed = stream.parse::<Decimal>().unwrap_err();
    assert!(parsed.to_string().contains("expected `.`"));
    let mut stream = ParseStream::from("-24785.24458");
    let parsed = stream.parse::<Decimal>().unwrap();
    assert_eq!(parsed.to_string(), "-24785.24458");
    assert_eq!(parsed.value().to_string(), "-24785.24458");
}

#[test]
fn test_parse_uint64() {
    let mut stream = ParseStream::from("78358885");
    let parsed = stream.parse::<U64>().unwrap();
    assert_eq!(parsed.0, 78358885);
    assert_eq!("78358885", parsed.span().source_text());
    let mut stream = ParseStream::from("00078358885");
    let parsed = stream.parse::<U64>().unwrap();
    assert_eq!(parsed.0, 78358885);
    assert_eq!("00078358885", parsed.span().source_text());
    let mut stream = ParseStream::from("hey");
    let e = stream.parse::<U64>().unwrap_err();
    assert!(e.message().contains("expected digit"));
    let mut stream = ParseStream::from("99999999999999999999999999999999999999999999999999");
    let e = stream.parse::<U64>().unwrap_err();
    assert!(e.message().contains("number too large"));
    let mut stream = ParseStream::from("00000000000000000000000000000000000000000000000009");
    let parsed = stream.parse::<U64>().unwrap();
    assert_eq!(parsed.0, 9);
    assert_eq!(
        "00000000000000000000000000000000000000000000000009",
        parsed.span().source_text()
    );
    let parsed: U64 = "12345".parse().unwrap();
    assert_eq!(parsed.value(), 12345);
}

#[test]
fn test_parse_uint128() {
    let mut stream = ParseStream::from("7835883984793847893748985");
    let parsed = stream.parse::<U128>().unwrap();
    assert_eq!(parsed.0, 7835883984793847893748985);
    assert_eq!("7835883984793847893748985", parsed.span().source_text());
    let mut stream = ParseStream::from("00078358885");
    let parsed = stream.parse::<U128>().unwrap();
    assert_eq!(parsed.0, 78358885);
    assert_eq!("00078358885", parsed.span().source_text());
    let mut stream = ParseStream::from("hey");
    let e = stream.parse::<U128>().unwrap_err();
    assert!(e.message().contains("expected digit"));
    let mut stream =
        ParseStream::from("99999999999999999999999999999999999999999999999999999999999999999");
    let e = stream.parse::<U128>().unwrap_err();
    assert!(e.message().contains("number too large"));
    let mut stream = ParseStream::from("00000000000000000000000000000000000000000000000009");
    let parsed = stream.parse::<U128>().unwrap();
    assert_eq!(parsed.0, 9);
    assert_eq!(
        "00000000000000000000000000000000000000000000000009",
        parsed.span().source_text()
    );
    let parsed: U128 = "12345".parse().unwrap();
    assert_eq!(parsed.value(), 12345);
}
