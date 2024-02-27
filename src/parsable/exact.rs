use std::ops::Deref;

use super::*;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Exact(pub String, pub Span);

impl Deref for Exact {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Spanned for Exact {
    fn span(&self) -> Span {
        self.1.clone()
    }
}

make_parsable!(Exact);

impl Parsable for Exact {
    fn parse(stream: &mut ParseStream) -> ParseResult<Self> {
        todo!()
    }

    fn unparse(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
