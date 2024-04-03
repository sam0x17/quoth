use super::*;

use crate as quoth;

#[derive(Clone, Debug, PartialEq, Eq, Hash, ParsableExt, Spanned)]
pub struct Everything(Span);

impl Parsable for Everything {
    fn parse(stream: &mut ParseStream) -> Result<Self> {
        let span = Span::new(
            stream.source().clone(),
            stream.position..(stream.source().len()),
        );
        stream.position = stream.source().len();
        Ok(Everything(span))
    }

    fn parse_value(value: Self, stream: &mut ParseStream) -> Result<Self> {
        let s = value.span();
        let text = s.source_text();
        if stream.remaining() == text {
            return Ok(Everything(stream.consume(text.len())?));
        }
        let prefix = common_prefix(&text, stream.remaining());
        stream.consume(prefix.len())?;
        let missing_span = stream.current_span();
        let missing = text.slice(prefix.len()..);
        if missing.len() > 0 {
            return Err(Error::expected(missing_span, missing));
        }
        Err(Error::new(missing_span, "expected end of input"))
    }
}

#[test]
fn test_parse_everything() {
    use std::rc::Rc;
    let mut stream = ParseStream::from("this is a triumph");
    stream.parse::<Everything>().unwrap();
    stream.parse::<Nothing>().unwrap();
    stream.parse::<Everything>().unwrap();
    let mut stream = ParseStream::from("this is a triumph");
    stream.position = 4;
    assert_eq!(
        stream.parse::<Everything>().unwrap().span().source_text(),
        " is a triumph"
    );
    let mut stream = ParseStream::from("this is a triumph");
    let parsed = stream.fork().parse::<Everything>().unwrap();
    stream.parse_value(parsed.clone()).unwrap();
    let mut stream = ParseStream::from("this is a triumph");
    stream.position = 1;
    let e = stream.parse_value(parsed).unwrap_err();
    assert!(e.message().contains("expected"));
    let mut stream = ParseStream::from("this is a triumph");
    let parsed = Everything(Span::new(
        Rc::new(Source::from_str("this is b")),
        0.."this is b".len(),
    ));
    let e = stream.parse_value(parsed).unwrap_err();
    assert!(e.message().contains("expected `b`"));
    let mut stream = ParseStream::from("this is a triumph");
    let parsed = Everything(Span::new(
        Rc::new(Source::from_str("this is a")),
        0.."this is a".len(),
    ));
    let e = stream.parse_value(parsed).unwrap_err();
    assert!(e.message().contains("expected end of input"));
}
