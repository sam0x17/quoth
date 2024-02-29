use super::*;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct PInt64(u64, Span);

impl Spanned for PInt64 {
    fn span(&self) -> Span {
        self.1.clone()
    }
}

impl Parsable for PInt64 {
    fn parse(stream: &mut ParseStream) -> ParseResult<Self> {
        todo!()
    }

    fn set_span(&mut self, span: impl Into<Span>) {
        self.1 = span.into();
    }
}

make_parsable!(PInt64);
