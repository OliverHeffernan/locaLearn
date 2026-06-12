use std::path::Path;

use crate::{study_set::StudySetLayout, Result};

/// Locates study set roots from filesystem paths.
#[derive(Default)]
pub struct StudySetLocator;

impl StudySetLocator {
    /// Resolves a path into a study set layout.
    pub fn locate(&self, path: &Path) -> Result<StudySetLayout> {
        StudySetLayout::discover(path)
    }
}
