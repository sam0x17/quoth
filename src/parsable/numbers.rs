use std::fmt::Display;

use super::*;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct PInt64(u64, Span);

impl Spanned for PInt64 {
    fn span(&self) -> Span {
        self.1.clone()
    }
}

impl Parsable for PInt64 {
    fn parse(value: Option<Self>, stream: &mut ParseStream) -> ParseResult<Self> {
        match value {
            Some(value) => {
                let st = value.0.to_string();
                if stream.remaining().starts_with(&st) {
                    return Ok(PInt64(value.0, stream.consume(st.len())?));
                }
                return Err(Error::expected(stream.current_span(), st));
            }
            None => {
                todo!()
            }
        }
    }

    fn unparse(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

make_parsable!(PInt64);
