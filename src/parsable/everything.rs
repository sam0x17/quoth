use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Everything(Span);

make_parsable!(Everything);

impl Spanned for Everything {
    fn span(&self) -> Span {
        self.0.clone()
    }
}

impl Parsable for Everything {
    fn parse(stream: &mut ParseStream) -> ParseResult<Self> {
        let span = Span::new(
            stream.source.clone(),
            stream.position..(stream.source.len()),
        );
        stream.position = stream.source.len();
        Ok(Everything(span))
    }

    fn unparse(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.source_text())
    }
}

#[test]
fn test_parse_everything() {
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
}
