//! Home of [`Span`] and related types and traits.

use std::{fmt::Display, ops::Range, path::Path, rc::Rc};

use super::*;

/// Represents a specific range of text within a [`Source`].
///
/// Internally [`Span`] is extremely lightweight and is essentially just a reference to a
/// [`Source`] and a range of bytes within that source, so it can be cheaply cloned and passed
/// around without issue. The underlying [`Source`] mechanism is stored within an [`Rc`] so
/// that it can be shared between multiple [`Span`]s without needing to be cloned. This cheap
/// sharing, combined with the lack of any sort of tokenization in Quoth allows us to provide
/// direct access to the original, unmodified source text for any given [`Span`].
///
/// Spans can be created directly using [`Span::new`], or by using the [`Spanned`] trait to
/// access the underlying [`Span`] of a type.
///
/// ```
/// use quoth::*;
/// use std::rc::Rc;
///
/// let span = Span::new(Rc::new(Source::from_str("Hello, world!")), 0..5);
/// ```
///
/// Spans can be joined together using the [`Span::join`] method, which will return a new
/// [`Span`] that encompasses both of the original spans. This can be useful for combining
/// spans that were generated from different parts of the same source.
///
/// ```
/// use quoth::*;
/// use std::rc::Rc;
///
/// let source = Rc::new(Source::from_str("Hello, world!"));
/// let span1 = Span::new(source.clone(), 0..5);
/// let span2 = Span::new(source.clone(), 7..12);
/// let encompassing_span = span1.join(&span2).unwrap();
/// assert_eq!(encompassing_span.source_text(), "Hello, world");
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Span {
    source: Rc<Source>,
    byte_range: Range<usize>,
}

/// Indicates that two [`Span`]s could not be joined because they do not come from the same [`Source`].
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SpanJoinError;

impl Display for SpanJoinError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "the specified spans do not come from the same source")
    }
}

impl Span {
    /// Returns a blank [`Span`] with no source and a zero-length range.
    ///
    /// Note that blank [`Span`]s are special in that they can be joined with a [`Span`] from
    /// any [`Source`] without error, and will always return the other [`Span`] as the result.
    pub fn blank() -> Span {
        Span::new(Rc::new(Source::from_str("")), 0..0)
    }

    /// Creates a new [`Span`] from a [`Source`] and a byte range.
    pub fn new(source: Rc<Source>, byte_range: Range<usize>) -> Self {
        let mut byte_range = byte_range;
        if source.len() > 0 && byte_range.end > source.len() {
            byte_range.end = source.len();
        }
        Span { source, byte_range }
    }

    /// Returns the [`Source`] that this [`Span`] is associated with.
    pub fn source(&self) -> &Source {
        &self.source
    }

    /// Returns the text of the [`Source`] that this [`Span`] is associated with.
    pub fn source_text(&self) -> IndexedSlice<'_> {
        self.source.slice(self.byte_range.clone())
    }

    /// Returns the path of the [`Source`] that this [`Span`] is associated with, if it has one.
    pub fn source_path(&self) -> Option<&Path> {
        self.source.source_path()
    }

    /// Returns the byte range of this [`Span`], representing the start and end of the span
    /// within the [`Source`].
    ///
    /// Note that because of UTF-8, the start and end of the range may not correspond with the
    /// start and end of a character in the source text.
    pub fn byte_range(&self) -> &Range<usize> {
        &self.byte_range
    }

    /// Returns the line and column of the start of this [`Span`] within the [`Source`].
    pub fn start(&self) -> LineCol {
        let mut line = 0;
        let mut col = 0;
        for c in self.source.slice(0..self.byte_range.start).chars() {
            if *c == '\n' {
                col = 0;
                line += 1;
            } else {
                col += 1;
            }
        }
        LineCol { line, col }
    }

    /// Returns the line and column of the end of this [`Span`] within the [`Source`].
    pub fn end(&self) -> LineCol {
        let LineCol { mut line, mut col } = self.start();
        for c in self
            .source
            .slice(self.byte_range.start..self.byte_range.end)
            .chars()
        {
            if *c == '\n' {
                col = 0;
                line += 1;
            } else {
                col += 1;
            }
        }
        LineCol { line, col }
    }

    /// Returns an iterator over the lines of the [`Source`] that this [`Span`] is associated with,
    pub fn source_lines(&self) -> impl Iterator<Item = (IndexedSlice<'_>, Range<usize>)> + '_ {
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
                let len = line.len();
                if start_line == end_line && end_line == i {
                    Some((line, start_col..end_col))
                } else if i == start_line {
                    Some((line, start_col..len))
                } else if i > start_line && i < end_line {
                    Some((line, 0..len))
                } else if i == end_line {
                    Some((line, 0..end_col))
                } else {
                    None
                }
            })
    }

    /// Joins this [`Span`] with another [`Span`], returning a new [`Span`] that encompasses both.
    ///
    /// If the two spans do not come from the same [`Source`], this method will return an error
    /// unless one or more of the spans is [`Span::blank()`].
    pub fn join(&self, other: &Span) -> core::result::Result<Span, SpanJoinError> {
        if self.source.is_empty() {
            return Ok(other.clone());
        }
        if other.source.is_empty() {
            return Ok(self.clone());
        }
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

    /// Returns whether this [`Span`] is blank, i.e. has a zero-length range.
    pub fn is_blank(&self) -> bool {
        self.byte_range.start == self.byte_range.end
    }
}

/// Represents a line and column within a [`Source`].
///
/// Note that both the line and column are zero-indexed, so the first line and column are both 0.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
pub struct LineCol {
    /// The line number, starting from 0.
    pub line: usize,
    /// The column number, starting from 0.
    pub col: usize,
}

/// A trait for types that have a [`Span`].
pub trait Spanned {
    /// Returns the underlying [`Span`] of self.
    ///
    /// If the type has multiple [`Span`]s, this method should return the primary [`Span`],
    /// i.e. by joining all of the [`Span`]s together, rather than storing a permanent primary
    /// [`Span`] on the type directly.
    fn span(&self) -> Span;
}

impl Spanned for Span {
    fn span(&self) -> Span {
        self.clone()
    }
}

/// A trait for types that have multiple [`Span`]s.
pub trait MultiSpan {
    /// Converts self into a vector of [`Span`]s.
    fn into_spans(self) -> Vec<Span>;
}

impl MultiSpan for Vec<Span> {
    fn into_spans(self) -> Vec<Span> {
        self
    }
}
