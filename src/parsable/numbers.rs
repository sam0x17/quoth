use std::{fmt::Display, rc::Rc, str::FromStr};

use super::*;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct UInt64(u64, Span);

impl UInt64 {
    pub fn value(&self) -> u64 {
        self.0
    }
}

impl Spanned for UInt64 {
    fn span(&self) -> Span {
        self.1.clone()
    }
}

impl Parsable for UInt64 {
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
        Ok(UInt64(parsed, span))
    }

    fn set_span(&mut self, span: impl Into<Span>) {
        self.1 = span.into();
    }
}

make_parsable!(UInt64);

impl From<UInt64> for u64 {
    fn from(value: UInt64) -> Self {
        value.0
    }
}

impl From<UInt64> for u128 {
    fn from(value: UInt64) -> Self {
        value.0.into()
    }
}

impl From<UInt64> for i128 {
    fn from(value: UInt64) -> Self {
        value.0.into()
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct UInt128(u128, Span);

impl UInt128 {
    pub fn value(&self) -> u128 {
        self.0
    }
}

impl Spanned for UInt128 {
    fn span(&self) -> Span {
        self.1.clone()
    }
}

impl Parsable for UInt128 {
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
        Ok(UInt128(parsed, span))
    }

    fn set_span(&mut self, span: impl Into<Span>) {
        self.1 = span.into();
    }
}

make_parsable!(UInt128);

impl From<UInt128> for u128 {
    fn from(value: UInt128) -> Self {
        value.0
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Int64(i64, Span);

impl Int64 {
    pub fn value(&self) -> i64 {
        self.0
    }
}

impl Spanned for Int64 {
    fn span(&self) -> Span {
        self.1.clone()
    }
}

impl Parsable for Int64 {
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
        Ok(Int64(parsed * sign, span))
    }

    fn set_span(&mut self, span: impl Into<Span>) {
        self.1 = span.into();
    }
}

make_parsable!(Int64);

impl From<Int64> for i64 {
    fn from(value: Int64) -> Self {
        value.0
    }
}

impl From<Int64> for i128 {
    fn from(value: Int64) -> Self {
        value.0.into()
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Int128(i128, Span);

impl Int128 {
    pub fn value(&self) -> i128 {
        self.0
    }
}

impl Spanned for Int128 {
    fn span(&self) -> Span {
        self.1.clone()
    }
}

impl Parsable for Int128 {
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
        Ok(Int128(parsed * sign, span))
    }

    fn set_span(&mut self, span: impl Into<Span>) {
        self.1 = span.into();
    }
}

make_parsable!(Int128);

impl From<Int128> for i128 {
    fn from(value: Int128) -> Self {
        value.0
    }
}

impl From<Int64> for Int128 {
    fn from(value: Int64) -> Self {
        Int128(value.0.into(), value.1)
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
        let span = Span::new(Rc::new(Source::from_string(st)), 0..len);
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

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct BoundedInt64<const MIN: i64, const MAX: i64>(Int64);

impl<const MIN: i64, const MAX: i64> BoundedInt64<MIN, MAX> {
    pub fn value(&self) -> i64 {
        self.0 .0
    }
}

impl<const MIN: i64, const MAX: i64> Spanned for BoundedInt64<MIN, MAX> {
    fn span(&self) -> Span {
        self.0 .1.clone()
    }
}

impl<const MIN: i64, const MAX: i64> Display for BoundedInt64<MIN, MAX> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<const MIN: i64, const MAX: i64> FromStr for BoundedInt64<MIN, MAX> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(&mut ParseStream::from(s))
    }
}

impl<const MIN: i64, const MAX: i64> Parsable for BoundedInt64<MIN, MAX> {
    fn parse(stream: &mut ParseStream) -> ParseResult<Self> {
        let i = stream.parse::<Int64>()?;
        if i.0 <= MIN {
            return Err(Error::new(
                i.span(),
                format!("must be greater than or equal to {MIN}"),
            ));
        }
        if i.0 >= MAX {
            return Err(Error::new(
                i.span(),
                format!("must be less than or equal to {MAX}"),
            ));
        }
        Ok(BoundedInt64(i))
    }

    fn set_span(&mut self, span: impl Into<Span>) {
        self.0 .1 = span.into();
    }
}

#[test]
fn test_parse_bounded_int64() {
    let mut stream = ParseStream::from("33");
    let parsed = stream.parse::<BoundedInt64<20, 40>>().unwrap();
    assert_eq!(parsed.to_string(), "33");
    let mut stream = ParseStream::from("33");
    let parsed = stream.parse::<BoundedInt64<34, 40>>().unwrap_err();
    assert!(parsed
        .to_string()
        .contains("must be greater than or equal to 34"));
    let mut stream = ParseStream::from("41");
    let parsed = stream.parse::<BoundedInt64<34, 40>>().unwrap_err();
    assert!(parsed
        .to_string()
        .contains("must be less than or equal to 40"));
}

#[test]
fn test_parse_int128() {
    let mut stream = ParseStream::from("-34833749837489858394735");
    let parsed = stream.parse::<Int128>().unwrap();
    assert_eq!(parsed.to_string(), "-34833749837489858394735");
    assert_eq!(parsed.value(), -34833749837489858394735);
    let mut stream = ParseStream::from("-hey");
    let parsed = stream.parse::<Int64>().unwrap_err();
    assert!(parsed.to_string().contains("expected digit"));
}

#[test]
fn test_parse_int64() {
    let mut stream = ParseStream::from("-348385735");
    let parsed = stream.parse::<Int64>().unwrap();
    assert_eq!(parsed.to_string(), "-348385735");
    assert_eq!(parsed.value(), -348385735);
    let mut stream = ParseStream::from("-hey");
    let parsed = stream.parse::<Int64>().unwrap_err();
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
    let parsed = stream.parse::<UInt64>().unwrap();
    assert_eq!(parsed.0, 78358885);
    assert_eq!("78358885", parsed.span().source_text());
    let mut stream = ParseStream::from("00078358885");
    let parsed = stream.parse::<UInt64>().unwrap();
    assert_eq!(parsed.0, 78358885);
    assert_eq!("00078358885", parsed.span().source_text());
    let mut stream = ParseStream::from("hey");
    let e = stream.parse::<UInt64>().unwrap_err();
    assert!(e.message().contains("expected digit"));
    let mut stream = ParseStream::from("99999999999999999999999999999999999999999999999999");
    let e = stream.parse::<UInt64>().unwrap_err();
    assert!(e.message().contains("number too large"));
    let mut stream = ParseStream::from("00000000000000000000000000000000000000000000000009");
    let parsed = stream.parse::<UInt64>().unwrap();
    assert_eq!(parsed.0, 9);
    assert_eq!(
        "00000000000000000000000000000000000000000000000009",
        parsed.span().source_text()
    );
    let parsed: UInt64 = "12345".parse().unwrap();
    assert_eq!(parsed.value(), 12345);
}

#[test]
fn test_parse_uint128() {
    let mut stream = ParseStream::from("7835883984793847893748985");
    let parsed = stream.parse::<UInt128>().unwrap();
    assert_eq!(parsed.0, 7835883984793847893748985);
    assert_eq!("7835883984793847893748985", parsed.span().source_text());
    let mut stream = ParseStream::from("00078358885");
    let parsed = stream.parse::<UInt128>().unwrap();
    assert_eq!(parsed.0, 78358885);
    assert_eq!("00078358885", parsed.span().source_text());
    let mut stream = ParseStream::from("hey");
    let e = stream.parse::<UInt128>().unwrap_err();
    assert!(e.message().contains("expected digit"));
    let mut stream =
        ParseStream::from("99999999999999999999999999999999999999999999999999999999999999999");
    let e = stream.parse::<UInt128>().unwrap_err();
    assert!(e.message().contains("number too large"));
    let mut stream = ParseStream::from("00000000000000000000000000000000000000000000000009");
    let parsed = stream.parse::<UInt128>().unwrap();
    assert_eq!(parsed.0, 9);
    assert_eq!(
        "00000000000000000000000000000000000000000000000009",
        parsed.span().source_text()
    );
    let parsed: UInt128 = "12345".parse().unwrap();
    assert_eq!(parsed.value(), 12345);
}
