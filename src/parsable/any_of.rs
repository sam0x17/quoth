use super::*;

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
struct AnyOf(Vec<String>, Span);

make_parsable!(AnyOf);

impl Spanned for AnyOf {
    fn span(&self) -> Span {
        self.1.clone()
    }
}

impl Parsable for AnyOf {
    fn parse(stream: &mut ParseStream) -> ParseResult<Self> {
        todo!()
    }

    fn set_span(&mut self, span: impl Into<Span>) {
        self.1 = span.into();
    }
}
