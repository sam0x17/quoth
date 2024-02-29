use super::*;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Exact(pub Span);

impl Spanned for Exact {
    fn span(&self) -> Span {
        self.0.clone()
    }
}

make_parsable!(Exact);

impl Parsable for Exact {
    fn parse(stream: &mut ParseStream) -> ParseResult<Self> {
        Ok(Exact(stream.consume_remaining()))
    }

    fn set_span(&mut self, span: impl Into<Span>) {
        self.0 = span.into();
    }
}
