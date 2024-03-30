//! home of [`Source`] and related types.

#[cfg(doc)]
use super::*;

use std::{
    ops::Deref,
    path::{Path, PathBuf},
};

/// Represents source text that can be indexed into to define individual [`Span`]s.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Source {
    text: String,
    path: Option<PathBuf>,
}

impl Source {
    /// Returns the underlying text of this [`Source`], with original formatting.
    pub fn source_text(&self) -> &str {
        &self.text
    }

    /// Returns the path of the file that this [`Source`] was read from, if it was read from a file.
    pub fn source_path(&self) -> Option<&Path> {
        self.path.as_ref().map(|path| path.as_path())
    }

    /// Returns the length of the underlying text of this [`Source`].
    pub fn from_str(string: impl Into<String>) -> Self {
        Source {
            text: string.into(),
            path: None,
        }
    }

    /// Reads the contents of a file and returns a [`Source`] with the file's text.
    ///
    /// Since no parsing is done at this stage, only IO or encoding errors will be returned,
    /// regardless of the validity of the syntax in the file.
    pub fn from_file(path: impl AsRef<Path>) -> core::result::Result<Self, std::io::Error> {
        std::fs::read_to_string(path.as_ref()).map(|text| Source {
            text,
            path: Some(path.as_ref().to_path_buf()),
        })
    }

    /// Sets the path of the file that this [`Source`] was read from.
    pub fn set_path(&mut self, path: Option<impl AsRef<Path>>) {
        self.path = path.map(|p| p.as_ref().to_path_buf());
    }
}

impl Deref for Source {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.text
    }
}

impl<S: ToString> From<S> for Source {
    fn from(value: S) -> Self {
        Source {
            text: value.to_string(),
            path: None,
        }
    }
}
