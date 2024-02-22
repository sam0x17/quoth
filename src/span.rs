use std::{fmt::Display, ops::Range, path::Path, rc::Rc};

use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Span {
    source: Rc<Source>,
    byte_range: Range<usize>,
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SpanJoinError;

impl Display for SpanJoinError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "the specified spans do not come from the same source")
    }
}

impl Span {
    pub fn new(source: Rc<Source>, byte_range: Range<usize>) -> Self {
        let mut byte_range = byte_range;
        if source.len() > 0 && byte_range.end > source.len() - 1 {
            byte_range.end = source.len() - 1;
        }
        Span { source, byte_range }
    }

    pub fn source(&self) -> &Source {
        &self.source
    }

    pub fn source_text(&self) -> &str {
        &self.source[self.byte_range.clone()]
    }

    pub fn source_path(&self) -> Option<&Path> {
        self.source.source_path()
    }

    pub fn byte_range(&self) -> &Range<usize> {
        &self.byte_range
    }

    pub fn start(&self) -> LineCol {
        let mut line = 0;
        let mut col = 0;
        for c in self.source[0..self.byte_range.start].chars() {
            if c == '\n' {
                col = 0;
                line += 1;
            } else {
                col += 1;
            }
        }
        LineCol { line, col }
    }

    pub fn end(&self) -> LineCol {
        let LineCol { mut line, mut col } = self.start();
        for c in self.source[self.byte_range.start..self.byte_range.end].chars() {
            if c == '\n' {
                col = 0;
                line += 1;
            } else {
                col += 1;
            }
        }
        LineCol { line, col }
    }

    pub fn source_lines(&self) -> impl Iterator<Item = (&str, Range<usize>)> {
        let start_line_col = self.start();
        let end_line_col = self.end();
        let start_col = start_line_col.col;
        let start_line = start_line_col.line;
        let end_line = end_line_col.line;
        let end_col = end_line_col.col;
        self.source
            .lines()
            .enumerate()
            .filter_map(move |(i, line)| {
                if start_line == end_line && end_line == i {
                    Some((line, start_col..end_col))
                } else if i == start_line {
                    Some((line, start_col..line.len()))
                } else if i > start_line && i < end_line {
                    Some((line, 0..line.len()))
                } else if i == end_line {
                    Some((line, 0..end_col))
                } else {
                    None
                }
            })
    }

    pub fn join(&self, other: &Span) -> Result<Span, SpanJoinError> {
        if self.source != other.source {
            return Err(SpanJoinError);
        }
        let start = self.byte_range.start.min(other.byte_range.start);
        let end = self.byte_range.end.max(other.byte_range.end);
        Ok(Span {
            source: self.source.clone(),
            byte_range: start..end,
        })
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
pub struct LineCol {
    pub line: usize,
    pub col: usize,
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
