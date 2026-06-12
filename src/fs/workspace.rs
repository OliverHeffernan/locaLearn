use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::Result;

/// Filesystem helper responsible for creating directories and files.
#[derive(Default)]
pub struct WorkspaceWriter;

impl WorkspaceWriter {
    /// Creates all directories in the provided iterator.
    pub fn create_directories<'a>(
        &self,
        directories: impl IntoIterator<Item = &'a PathBuf>,
    ) -> Result<()> {
        directories
            .into_iter()
            .try_for_each(|directory| fs::create_dir_all(directory).map_err(Into::into))
    }

    /// Writes a UTF-8 text file, creating the parent directory when present.
    pub fn write_text(&self, path: &Path, content: &str) -> Result<()> {
        path.parent().map(fs::create_dir_all).transpose()?;
        fs::write(path, content).map_err(Into::into)
    }
}
