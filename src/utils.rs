use anyhow::Result;
use std::path::{Path, PathBuf};

/// Try to convert from a path.
pub trait TryFromPath {
    /// Converts from a path reference.
    fn from_path(path: &Path) -> Result<Self>
    where
        Self: Sized;
}

/// Defines a trait for converting from a path.
pub trait FromPath {
    /// Converts from a path reference.
    fn from_path(path: &Path) -> Self;
}

impl FromPath for String {
    fn from_path(path: &Path) -> String {
        path.to_string_lossy().into_owned()
    }
}

impl FromPath for PathBuf {
    fn from_path(path: &Path) -> PathBuf {
        path.to_path_buf()
    }
}
