use std::{fmt::Display, ops::Range, path::Path, rc::Rc};

use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Span {
    source: Rc<Source>,
    index: Range<usize>,
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SpanJoinError;

impl Display for SpanJoinError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "the specified spans do not come from the same source")
    }
}

impl Span {
    pub fn new(source: Rc<Source>, index: Range<usize>) -> Self {
        let mut index = index;
        if index.end > source.len() - 1 {
            index.end = source.len() - 1;
        }
        Span { source, index }
    }

    pub fn source(&self) -> &Source {
        &self.source
    }
    pub fn source_text(&self) -> &str {
        &self.source[self.index.clone()]
    }

    pub fn source_path(&self) -> Option<&Path> {
        self.source.source_path()
    }

    pub fn join(&self, other: &Span) -> Result<Span, SpanJoinError> {
        if self.source != other.source {
            return Err(SpanJoinError);
        }
        let start = self.index.start.min(other.index.start);
        let end = self.index.end.max(other.index.end);
        Ok(Span {
            source: self.source.clone(),
            index: start..end,
        })
    }
}

pub trait Spanned {
    fn span(&self) -> Span;
}

impl Spanned for Span {
    fn span(&self) -> Span {
        self.clone()
    }
}

pub trait MultiSpan {
    fn into_spans(self) -> Vec<Span>;
}

impl MultiSpan for Vec<Span> {
    fn into_spans(self) -> Vec<Span> {
        self
    }
}
