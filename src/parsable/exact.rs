use std::rc::Rc;

use super::*;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Exact(pub Span);

impl Exact {
    pub fn new(span: impl Into<Span>) -> Self {
        Exact(span.into())
    }

    pub fn from(source: impl Into<Source>) -> Self {
        let source = Rc::new(source.into());
        let len = source.len();
        Exact(Span::new(source, 0..len))
    }
}

impl Spanned for Exact {
    fn span(&self) -> Span {
        self.0.clone()
    }
}

make_parsable!(Exact);

impl Parsable for Exact {
    fn parse(stream: &mut ParseStream) -> ParseResult<Self> {
        Ok(Exact(Span::new(
            stream.source().clone(),
            stream.position..stream.position,
        )))
    }

    fn parse_value(value: Self, stream: &mut ParseStream) -> ParseResult<Self> {
        let s = value.0;
        let text = s.source_text();
        if stream.remaining().starts_with(text) {
            let start_position = stream.position;
            stream.position += text.len();
            return Ok(Exact(Span::new(
                stream.source().clone(),
                start_position..stream.position,
            )));
        }
        let prefix = common_prefix(text, stream.remaining());
        stream.consume(prefix.len())?;
        let missing_span = stream.current_span();
        let missing = &text[prefix.len()..];
        Err(Error::expected(missing_span, missing))
    }

    fn set_span(&mut self, span: impl Into<Span>) {
        self.0 = span.into();
    }
}

#[test]
fn test_parse_exact() {
    let mut stream = ParseStream::from("hey this is a cool string");
    assert_eq!(stream.parse::<Exact>().unwrap().0.source_text(), "");
    assert_eq!(
        stream
            .parse_value(Exact::from("hey this"))
            .unwrap()
            .span()
            .source_text(),
        "hey this"
    );
    assert_eq!(stream.position, 8);
    assert!(stream
        .parse_value(Exact::from(" is not cool"))
        .unwrap_err()
        .to_string()
        .contains("expected `not cool`"));
    let mut stream = ParseStream::from("");
    let parsed = stream.parse_value(Exact::from("hey")).unwrap_err();
    assert!(parsed.to_string().contains("expected `hey`"));
}
