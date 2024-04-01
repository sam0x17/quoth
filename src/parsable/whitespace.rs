use super::*;

use crate as quoth;

#[derive(Clone, PartialEq, Eq, Debug, Hash, ParsableExt, Spanned)]
pub struct Whitespace(Span);

impl Parsable for Whitespace {
    fn parse(stream: &mut ParseStream) -> Result<Self> {
        let start_position = stream.position;
        while let Ok(c) = stream.next_char() {
            if !c.is_whitespace() {
                break;
            }
            stream.consume(1)?;
        }
        if start_position == stream.position {
            return Err(Error::new(stream.current_span(), "expected whitespace"));
        }
        Ok(Whitespace(Span::new(
            stream.source().clone(),
            start_position..stream.position,
        )))
    }
}

#[test]
fn test_parse_whitespace() {
    let mut stream = ParseStream::from("this is some stuff");
    let parsed = stream.parse::<Whitespace>().unwrap_err();
    assert!(parsed.to_string().contains("expected whitespace"));
    let mut stream = ParseStream::from("\t\t  \n hey");
    let parsed = stream.parse::<Whitespace>().unwrap();
    assert_eq!(parsed.span().source_text(), "\t\t  \n ");
}
