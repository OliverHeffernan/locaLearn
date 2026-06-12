use std::{fs, path::PathBuf};

use crate::{
    generation::{GenerationRequest, GenerationResponse, StudyContext},
    providers::GenerationProvider,
    study_set::StudySetLayout,
    Result, StudyError,
};

/// Stable name for a generated study artifact type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactName(String);

impl ArtifactName {
    /// Creates an artifact name.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Returns the artifact name as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Defines one kind of generated study material.
pub trait StudyArtifact {
    /// Returns the stable artifact name used in configuration.
    fn name(&self) -> ArtifactName;

    /// Builds the prompt for this artifact.
    fn prompt(&self, context: &StudyContext) -> String;

    /// Returns the destination path for generated content.
    fn output_path(&self, layout: &StudySetLayout) -> PathBuf;

    /// Generates and persists this artifact.
    fn generate(
        &self,
        layout: &StudySetLayout,
        context: &StudyContext,
        provider: &dyn GenerationProvider,
    ) -> Result<()> {
        let request = GenerationRequest::new(self.prompt(context), context.clone());
        provider
            .generate(request, false)
            .and_then(|response| self.write_response(layout, response))
    }

    /// Writes provider output to this artifact's destination.
    fn write_response(&self, layout: &StudySetLayout, response: GenerationResponse) -> Result<()> {
        let output_path = self.output_path(layout);
        output_path.parent().map(fs::create_dir_all).transpose()?;
        fs::write(output_path, response.content()).map_err(Into::into)
    }
}

/// Registry that resolves configured artifact names into artifact implementations.
pub struct DefaultArtifactRegistry {
    artifacts: Vec<Box<dyn StudyArtifact>>,
}

impl Default for DefaultArtifactRegistry {
    fn default() -> Self {
        Self {
            artifacts: vec![
                Box::new(FlashcardsArtifact),
                Box::new(MultipleChoiceArtifact),
                Box::new(FillBlanksArtifact),
                Box::new(PracticeTestArtifact),
            ],
        }
    }
}

impl DefaultArtifactRegistry {
    /// Finds artifacts by their configured names.
    pub fn resolve<'a>(
        &'a self,
        names: impl Iterator<Item = &'a str>,
    ) -> Result<Vec<&'a dyn StudyArtifact>> {
        names
            .map(|name| {
                self.artifacts
                    .iter()
                    .map(Box::as_ref)
                    .find(|artifact| artifact.name().as_str() == name)
                    .map(|artifact| artifact as &dyn StudyArtifact)
                    .ok_or_else(|| StudyError::ArtifactNotFound(name.to_owned()))
            })
            .collect()
    }
}

/// Generates flashcards.
pub struct FlashcardsArtifact;

impl StudyArtifact for FlashcardsArtifact {
    fn name(&self) -> ArtifactName {
        ArtifactName::new("flashcards")
    }

    fn prompt(&self, _context: &StudyContext) -> String {
        "Create concise flashcards from the resources. Use Markdown with one card per section, including front and back fields.".to_owned()
    }

    fn output_path(&self, layout: &StudySetLayout) -> PathBuf {
        layout
            .generated_dir()
            .join("flashcards")
            .join("flashcards.md")
    }
}

/// Generates multiple-choice questions.
pub struct MultipleChoiceArtifact;

impl StudyArtifact for MultipleChoiceArtifact {
    fn name(&self) -> ArtifactName {
        ArtifactName::new("multiple_choice")
    }

    fn prompt(&self, _context: &StudyContext) -> String {
        "Create multiple-choice questions from the resources. Include four options, the correct answer, and a brief explanation.".to_owned()
    }

    fn output_path(&self, layout: &StudySetLayout) -> PathBuf {
        layout
            .generated_dir()
            .join("multiple_choice")
            .join("questions.md")
    }
}

/// Generates fill-in-the-blank questions.
pub struct FillBlanksArtifact;

impl StudyArtifact for FillBlanksArtifact {
    fn name(&self) -> ArtifactName {
        ArtifactName::new("fill_blanks")
    }

    fn prompt(&self, _context: &StudyContext) -> String {
        "Create fill-in-the-blank exercises from the resources. Include an answer key after the exercises.".to_owned()
    }

    fn output_path(&self, layout: &StudySetLayout) -> PathBuf {
        layout
            .generated_dir()
            .join("fill_blanks")
            .join("questions.md")
    }
}

/// Generates a practice test.
pub struct PracticeTestArtifact;

impl StudyArtifact for PracticeTestArtifact {
    fn name(&self) -> ArtifactName {
        ArtifactName::new("practice_test")
    }

    fn prompt(&self, _context: &StudyContext) -> String {
        "Create a balanced practice test from the resources. Include short-answer, multiple-choice, and applied questions with a marking guide.".to_owned()
    }

    fn output_path(&self, layout: &StudySetLayout) -> PathBuf {
        layout
            .generated_dir()
            .join("practice_tests")
            .join("practice_test.md")
    }
}
