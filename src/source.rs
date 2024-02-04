use std::{
    ops::Deref,
    path::{Path, PathBuf},
};

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

impl TryFrom<&Path> for Source {
    type Error = std::io::Error;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        std::fs::read_to_string(path).map(|text| Source {
            text,
            path: Some(path.to_path_buf()),
        })
    }
}
