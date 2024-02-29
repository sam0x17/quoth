use super::*;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct PInt64(u64, Span);

impl PInt64 {
    pub fn value(&self) -> u64 {
        self.0
    }
}

impl Spanned for PInt64 {
    fn span(&self) -> Span {
        self.1.clone()
    }
}

impl Parsable for PInt64 {
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
        Ok(PInt64(parsed, span))
    }

    fn set_span(&mut self, span: impl Into<Span>) {
        self.1 = span.into();
    }
}

make_parsable!(PInt64);

impl From<PInt64> for u64 {
    fn from(value: PInt64) -> Self {
        value.0
    }
}

impl From<PInt64> for u128 {
    fn from(value: PInt64) -> Self {
        value.0.into()
    }
}

impl From<PInt64> for i128 {
    fn from(value: PInt64) -> Self {
        value.0.into()
    }
}

#[test]
fn test_parse_pint64() {
    let mut stream = ParseStream::from("78358885");
    let parsed = stream.parse::<PInt64>().unwrap();
    assert_eq!(parsed.0, 78358885);
    assert_eq!("78358885", parsed.span().source_text());
    let mut stream = ParseStream::from("00078358885");
    let parsed = stream.parse::<PInt64>().unwrap();
    assert_eq!(parsed.0, 78358885);
    assert_eq!("00078358885", parsed.span().source_text());
    let mut stream = ParseStream::from("hey");
    let e = stream.parse::<PInt64>().unwrap_err();
    assert!(e.message().contains("expected digit"));
    let mut stream = ParseStream::from("99999999999999999999999999999999999999999999999999");
    let e = stream.parse::<PInt64>().unwrap_err();
    assert!(e.message().contains("number too large"));
    let mut stream = ParseStream::from("00000000000000000000000000000000000000000000000009");
    let parsed = stream.parse::<PInt64>().unwrap();
    assert_eq!(parsed.0, 9);
    assert_eq!(
        "00000000000000000000000000000000000000000000000009",
        parsed.span().source_text()
    );
    let parsed: PInt64 = "12345".parse().unwrap();
    assert_eq!(parsed.value(), 12345);
}
