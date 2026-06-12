use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{Result, StudyError};

use super::StudySetMetadata;

/// Files and directories that make up a study set.
#[derive(Debug, Clone)]
pub struct StudySetLayout {
    root: PathBuf,
}

impl StudySetLayout {
    /// Creates a layout rooted at the provided path.
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    /// Finds a study set layout from an existing root directory.
    pub fn discover(root: &Path) -> Result<Self> {
        let layout = Self::new(root);
        layout
            .metadata_path()
            .exists()
            .then_some(layout)
            .ok_or_else(|| StudyError::NotAStudySet(root.to_path_buf()))
    }

    /// Returns the root directory.
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Returns the path to metadata.
    pub fn metadata_path(&self) -> PathBuf {
        self.root.join("study.toml")
    }

    /// Returns the path to agent instructions.
    pub fn agent_instructions_path(&self) -> PathBuf {
        self.root.join("AGENTS.md")
    }

    /// Returns the resources directory.
    pub fn resources_dir(&self) -> PathBuf {
        self.root.join("resources")
    }

    /// Returns the generated artifacts directory.
    pub fn generated_dir(&self) -> PathBuf {
        self.root.join("generated")
    }

    /// Returns the generated flashcards Markdown file.
    pub fn flashcards_path(&self) -> PathBuf {
        self.generated_dir()
            .join("flashcards")
            .join("flashcards.md")
    }

    /// Returns the generated multiple-choice Markdown file.
    pub fn multiple_choice_path(&self) -> PathBuf {
        self.generated_dir()
            .join("multiple_choice")
            .join("questions.md")
    }

    /// Returns the generated fill-in-the-blanks Markdown file.
    pub fn fill_blanks_path(&self) -> PathBuf {
        self.generated_dir()
            .join("fill_blanks")
            .join("questions.md")
    }

    /// Returns the generated practice test Markdown file.
    pub fn practice_test_path(&self) -> PathBuf {
        self.generated_dir()
            .join("practice_tests")
            .join("practice_test.md")
    }

    /// Returns every directory created for a new study set.
    pub fn initial_directories(&self) -> Vec<PathBuf> {
        [
            self.resources_dir(),
            self.resources_dir().join("readings"),
            self.resources_dir().join("notes"),
            self.resources_dir().join("slides"),
            self.resources_dir().join("images"),
            self.generated_dir(),
            self.generated_dir().join("flashcards"),
            self.generated_dir().join("multiple_choice"),
            self.generated_dir().join("fill_blanks"),
            self.generated_dir().join("practice_tests"),
            self.root.join("sessions"),
            self.root.join("exports"),
        ]
        .into_iter()
        .collect()
    }

    /// Loads study metadata from disk.
    pub fn read_metadata(&self) -> Result<StudySetMetadata> {
        fs::read_to_string(self.metadata_path())
            .map_err(Into::into)
            .and_then(|content| toml::from_str(&content).map_err(Into::into))
    }
}
