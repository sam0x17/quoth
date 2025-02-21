use std::rc::Rc;

use super::*;

use crate as quoth;

#[derive(Clone, Debug, Hash, PartialEq, Eq, ParsableExt, Spanned)]
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

impl Parsable for Exact {
    fn parse(stream: &mut ParseStream) -> Result<Self> {
        Ok(Exact(Span::new(
            stream.source().clone(),
            stream.position..stream.position,
        )))
    }

    fn parse_value(value: Self, stream: &mut ParseStream) -> Result<Self> {
        let s = value.0;
        let text = s.source_text();
        if stream.remaining().starts_with(&text) {
            let start_position = stream.position;
            stream.position += text.len();
            return Ok(Exact(Span::new(
                stream.source().clone(),
                start_position..stream.position,
            )));
        }
        let prefix = common_prefix(&text, stream.remaining());
        stream.consume(prefix.len())?;
        let missing_span = stream.current_span();
        let missing = text.slice(prefix.len()..);
        Err(Error::expected(missing_span, missing))
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
    assert!(
        stream
            .parse_value(Exact::from(" is not cool"))
            .unwrap_err()
            .to_string()
            .contains("expected `not cool`")
    );
    let mut stream = ParseStream::from("");
    let parsed = stream.parse_value(Exact::from("hey")).unwrap_err();
    assert!(parsed.to_string().contains("expected `hey`"));
    let mut stream = ParseStream::from("3.14");
    stream.consume(1).unwrap();
    let ex = stream.parse_value(Exact::from(".")).unwrap();
    assert_eq!(ex.to_string(), ".");
}
