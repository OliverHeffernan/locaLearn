use crate::study_set::StudySetMetadata;

/// Renders agent instructions for a generated workspace.
pub trait AgentInstructionsTemplate {
    /// Produces Markdown instructions for the provided study set metadata.
    fn render(&self, metadata: &StudySetMetadata) -> String;
}

/// Default local-first instructions for study set directories.
#[derive(Default)]
pub struct StudySetAgentInstructions;

impl AgentInstructionsTemplate for StudySetAgentInstructions {
    fn render(&self, metadata: &StudySetMetadata) -> String {
        format!(
            "# {} Agent Instructions\n\n\
Agents may:\n\n\
- Read files inside `resources/`.\n\
- Generate study artifacts into `generated/`.\n\
- Update `study.toml` when metadata changes.\n\
- Create flashcards, multiple-choice questions, fill-in-the-blanks, and practice tests.\n\n\
Agents must not:\n\n\
- Delete source files in `resources/`.\n\
- Write outside this study set directory.\n\
- Send content to a provider other than the configured provider.\n\
- Replace generated artifacts without preserving the user's source resources.\n",
            metadata.name()
        )
    }
}
