use super::*;

use crate as quoth;

#[derive(Clone, Debug, PartialEq, Eq, Hash, ParsableExt, Spanned)]
pub struct Nothing(Span);

impl Parsable for Nothing {
    fn parse(stream: &mut ParseStream) -> Result<Self> {
        if stream.position < stream.source().len() {
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

    fn parse_value(_value: Self, stream: &mut ParseStream) -> Result<Self> {
        stream.parse()
    }

    fn unparse(&self, _: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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

#[test]
fn test_parse_value_nothing() {
    use crate::Rc;
    let mut stream = ParseStream::from("");
    stream
        .parse_value(Nothing(Span::new(Rc::new(Source::from_str("")), 0..0)))
        .unwrap();
    let mut stream = ParseStream::from("this won't work");
    assert!(
        stream
            .parse_value(Nothing(Span::new(Rc::new(Source::from_str("")), 0..0)))
            .is_err()
    );
}
