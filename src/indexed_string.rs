use core::fmt::Display;
use core::ops::{Bound, Index, RangeBounds};

pub trait IndexedStr:
    Display + PartialEq<IndexedString> + for<'a> PartialEq<IndexedSlice<'a>> + Index<usize>
{
    fn as_str(&self) -> &str;
    fn len(&self) -> usize;
    fn byte_len(&self) -> usize;
    fn char_at(&self, index: usize) -> Option<char>;
    fn slice<R: RangeBounds<usize>>(&self, range: R) -> IndexedSlice;
    fn chars(&self) -> &Vec<char>;
    fn to_indexed_string(&self) -> IndexedString;
}

#[derive(Clone, Debug, Eq, Hash)]
pub struct IndexedString {
    chars: Vec<char>,
    offsets: Vec<usize>,
    string: String,
}

impl IndexedStr for IndexedString {
    fn as_str(&self) -> &str {
        &self.string
    }

    fn char_at(&self, index: usize) -> Option<char> {
        self.chars.get(index).copied()
    }

    fn chars(&self) -> &Vec<char> {
        &self.chars
    }

    fn len(&self) -> usize {
        self.chars.len()
    }

    fn byte_len(&self) -> usize {
        self.string.len()
    }

    fn slice<R: RangeBounds<usize>>(&self, range: R) -> IndexedSlice {
        let start = match range.start_bound() {
            Bound::Included(&start) => start,
            Bound::Excluded(&start) => start + 1,
            Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            Bound::Included(&end) => end + 1,
            Bound::Excluded(&end) => end,
            Bound::Unbounded => self.chars.len(),
        };
        let start = if start > self.chars.len() {
            self.chars.len()
        } else {
            start
        };
        let end = if end > self.chars.len() {
            self.chars.len()
        } else {
            end
        };

        IndexedSlice {
            source: self,
            start,
            end,
        }
    }

    fn to_indexed_string(&self) -> IndexedString {
        self.clone()
    }
}

impl IndexedString {
    pub fn from_str(s: &str) -> Self {
        let chars: Vec<char> = s.chars().collect();
        let offsets: Vec<usize> = s.char_indices().map(|(i, _)| i).collect();
        IndexedString {
            chars,
            offsets,
            string: s.to_string(),
        }
    }

    pub fn from_chars(chars: impl Iterator<Item = char>) -> Self {
        let chars: Vec<char> = chars.collect();
        let offsets: Vec<usize> = chars.iter().enumerate().map(|(i, _)| i).collect();
        let string: String = chars.iter().collect();
        IndexedString {
            chars,
            offsets,
            string,
        }
    }
}

impl AsRef<str> for IndexedString {
    fn as_ref(&self) -> &str {
        &self.string
    }
}

impl Index<usize> for IndexedString {
    type Output = char;

    fn index(&self, index: usize) -> &Self::Output {
        &self.chars[index]
    }
}

impl Display for IndexedString {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.string)
    }
}

impl<S: AsRef<str>> PartialEq<S> for IndexedString {
    fn eq(&self, other: &S) -> bool {
        self.string == other.as_ref()
    }
}

#[derive(Eq, Debug, Clone)]
pub struct IndexedSlice<'a> {
    source: &'a IndexedString,
    start: usize,
    end: usize,
}

impl<'a> IndexedSlice<'a> {
    pub fn chars(&self) -> impl Iterator<Item = char> + '_ {
        self.source.chars[self.start..self.end].iter().copied()
    }

    pub fn to_indexed_string(&self) -> IndexedString {
        IndexedString::from_chars(self.chars())
    }
}

impl<'a> IndexedStr for IndexedSlice<'a> {
    fn as_str(&self) -> &str {
        if self.start >= self.source.offsets.len()
            || self.end > self.source.offsets.len()
            || self.start > self.end
        {
            return "";
        }

        let start_byte = self.source.offsets[self.start];
        let end_byte = if self.end == self.source.offsets.len() {
            self.source.string.len()
        } else {
            self.source.offsets[self.end]
        };

        &self.source.string[start_byte..end_byte]
    }

    fn len(&self) -> usize {
        self.end - self.start
    }

    fn byte_len(&self) -> usize {
        self.source.offsets[self.end] - self.source.offsets[self.start]
    }

    fn char_at(&self, index: usize) -> Option<char> {
        self.source.char_at(index - self.start)
    }

    fn slice<R: RangeBounds<usize>>(&self, range: R) -> IndexedSlice {
        let start = match range.start_bound() {
            Bound::Included(&start) => start,
            Bound::Excluded(&start) => start + 1,
            Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            Bound::Included(&end) => end + 1,
            Bound::Excluded(&end) => end,
            Bound::Unbounded => self.len(),
        };
        let start = if start > self.len() {
            self.len()
        } else {
            start
        };
        let end = if end > self.len() { self.len() } else { end };

        IndexedSlice {
            source: self.source,
            start: self.start + start,
            end: self.start + end,
        }
    }

    fn chars(&self) -> &Vec<char> {
        &self.source.chars
    }

    fn to_indexed_string(&self) -> IndexedString {
        self.to_indexed_string()
    }
}

impl<'a, S: AsRef<str>> PartialEq<S> for IndexedSlice<'a> {
    fn eq(&self, other: &S) -> bool {
        self.as_str() == other.as_ref()
    }
}

impl<'a> AsRef<str> for IndexedSlice<'a> {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<'a> From<&'a IndexedString> for IndexedSlice<'a> {
    fn from(s: &'a IndexedString) -> Self {
        IndexedSlice {
            source: s,
            start: 0,
            end: s.chars.len(),
        }
    }
}

impl From<String> for IndexedString {
    fn from(s: String) -> Self {
        IndexedString::from_str(&s)
    }
}

impl From<&str> for IndexedString {
    fn from(s: &str) -> Self {
        IndexedString::from_str(s)
    }
}

impl From<&String> for IndexedString {
    fn from(s: &String) -> Self {
        IndexedString::from_str(s)
    }
}

impl<'a> Display for IndexedSlice<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Index<usize> for IndexedSlice<'_> {
    type Output = char;

    fn index(&self, index: usize) -> &Self::Output {
        &self.source.chars[self.start + index]
    }
}

#[test]
fn test_indexed_string_equality() {
    let indexed_string = IndexedString::from_str("h₳ello");
    assert_eq!(indexed_string, "h₳ello");
    assert_eq!(indexed_string.as_str(), "h₳ello");
    assert_eq!(indexed_string.to_string(), "h₳ello");
}

#[test]
fn test_from_chars() {
    let indexed_string = IndexedString::from_chars("h₳ello".chars());
    assert_eq!(indexed_string, "h₳ello");
    assert_eq!(indexed_string.as_str(), "h₳ello");
    assert_eq!(indexed_string.to_string(), "h₳ello");
}

#[test]
fn test_indexing() {
    let indexed_string = IndexedString::from_str("h₳ello");
    assert_eq!(indexed_string[0], 'h');
    assert_eq!(indexed_string.slice(1..4).as_str(), "₳el");
    assert_eq!(indexed_string.slice(4..), "lo");
}

#[test]
fn test_empty_string() {
    let indexed_string: IndexedString = (&String::from("")).into();
    assert_eq!(indexed_string.as_str(), "");
    assert!(indexed_string.char_at(0).is_none());
}

#[test]
fn test_single_character() {
    let indexed_string: IndexedString = String::from("a").into();
    assert_eq!(indexed_string[0], 'a');
    assert_eq!(indexed_string.as_str(), "a");
    assert_eq!(indexed_string.len(), 1);
}

#[test]
fn test_multibyte_characters() {
    let indexed_string: IndexedString = "😊🌍".into();
    assert_eq!(indexed_string[0], '😊');
    assert_eq!(indexed_string[1], '🌍');
    assert_eq!(indexed_string.slice(0..1), "😊");
    assert_eq!(indexed_string.len(), 2);
}

#[test]
fn test_out_of_bounds_indexing() {
    let indexed_string = IndexedString::from_str("test");
    assert!(indexed_string.char_at(10).is_none());
}

#[test]
fn test_reverse_range() {
    let indexed_string = IndexedString::from_str("hello");
    assert_eq!(indexed_string.slice(3..1), "");
}

#[test]
fn test_full_range() {
    let indexed_string = IndexedString::from_str("hello");
    assert_eq!(indexed_string.slice(0..5), "hello");
}

#[test]
fn test_adjacent_ranges() {
    let indexed_string = IndexedString::from_str("hello world");
    assert_eq!(indexed_string.slice(0..5), "hello");
    assert_eq!(indexed_string.slice(5..6), " ");
    assert_eq!(indexed_string.slice(6..11), "world");
}

#[test]
fn test_non_ascii_ranges() {
    let indexed_string = IndexedString::from_str("🎉🌍🚀");
    assert_eq!(indexed_string.slice(0..1), "🎉");
    assert_eq!(indexed_string.slice(1..3).as_str(), "🌍🚀");
}

#[test]
fn test_edge_case_ranges() {
    let indexed_string = IndexedString::from_str("abc");
    assert_eq!(indexed_string.slice(0..0), "");
    assert_eq!(indexed_string.slice(0..1), "a");
    assert_eq!(indexed_string.slice(2..3), "c");
    assert_eq!(indexed_string.slice(3..3), "");
}
