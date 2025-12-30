//! home of [`Source`] and related types.

use super::*;

use core::ops::Deref;
#[cfg(feature = "std")]
use std::path::{Path, PathBuf};

/// Represents source text that can be indexed into to define individual [`Span`]s.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Source {
    text: IndexedString,
    #[cfg(feature = "std")]
    path: Option<PathBuf>,
}

impl Source {
    /// Returns the underlying text of this [`Source`], with original formatting.
    pub fn source_text(&self) -> IndexedSlice<'_> {
        self.text.as_slice()
    }

    /// Creates a new [`Source`] from a string.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(string: impl AsRef<str>) -> Self {
        Source {
            text: IndexedString::from_str(string.as_ref()),
            #[cfg(feature = "std")]
            path: None,
        }
    }

    /// Creates a new [`Source`] from an [`IndexedString`].
    pub fn from_indexed_string(text: IndexedString) -> Self {
        Source {
            text,
            #[cfg(feature = "std")]
            path: None,
        }
    }

    /// Reads the contents of a file and returns a [`Source`] with the file's text.
    ///
    /// Since no parsing is done at this stage, only IO or encoding errors will be returned,
    /// regardless of the validity of the syntax in the file.
    #[cfg(feature = "std")]
    pub fn from_file(path: impl AsRef<Path>) -> core::result::Result<Self, std::io::Error> {
        std::fs::read_to_string(path.as_ref()).map(|text| Source {
            text: IndexedString::from(&text),
            path: Some(path.as_ref().to_path_buf()),
        })
    }

    /// Sets the path of the file that this [`Source`] was read from.
    #[cfg(feature = "std")]
    pub fn set_path(&mut self, path: Option<impl AsRef<Path>>) {
        self.path = path.map(|p| p.as_ref().to_path_buf());
    }

    /// Returns the path of the file that this [`Source`] was read from, if it was read from a file.
    #[cfg(feature = "std")]
    pub fn source_path(&self) -> Option<&Path> {
        self.path.as_deref()
    }
}

impl Deref for Source {
    type Target = IndexedString;

    fn deref(&self) -> &Self::Target {
        &self.text
    }
}

impl<S: ToString> From<S> for Source {
    fn from(value: S) -> Self {
        Source {
            text: IndexedString::from(value.to_string()),
            #[cfg(feature = "std")]
            path: None,
        }
    }
}
