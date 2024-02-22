use std::{
    ops::Deref,
    path::{Path, PathBuf},
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Source {
    text: String,
    path: Option<PathBuf>,
}

impl Source {
    pub fn source_text(&self) -> &str {
        &self.text
    }

    pub fn source_path(&self) -> Option<&Path> {
        self.path.as_ref().map(|path| path.as_path())
    }

    pub fn from_string(string: String) -> Self {
        Source {
            text: string,
            path: None,
        }
    }

    pub fn from_str(string: impl AsRef<str>) -> Self {
        Source {
            text: string.as_ref().to_string(),
            path: None,
        }
    }

    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, std::io::Error> {
        std::fs::read_to_string(path.as_ref()).map(|text| Source {
            text,
            path: Some(path.as_ref().to_path_buf()),
        })
    }

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
