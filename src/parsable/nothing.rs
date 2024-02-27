use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Nothing(Span);

make_parsable!(Nothing);

impl Spanned for Nothing {
    fn span(&self) -> Span {
        self.0.clone()
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

    fn unparse(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

#[test]
fn test_parse_nothing() {
    let mut stream = ParseStream::from("");
    stream.parse::<Nothing>().unwrap();
    let mut stream = ParseStream::from("this won't work");
    assert!(stream.parse::<Nothing>().is_err());
}
