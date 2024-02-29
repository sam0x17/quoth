use std::fmt::Display;

use super::*;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct U64(u64, Span);

impl Spanned for U64 {
    fn span(&self) -> Span {
        self.1.clone()
    }
}

impl Parsable for U64 {
    fn parse(value: Option<Self>, stream: &mut ParseStream) -> ParseResult<Self> {
        todo!()
    }

    fn unparse(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

make_parsable!(U64);
