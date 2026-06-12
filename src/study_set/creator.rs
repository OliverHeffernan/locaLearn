use std::path::Path;

use crate::{
    fs::{AgentInstructionsTemplate, StudySetAgentInstructions, WorkspaceWriter},
    study_set::{StudySetLayout, StudySetMetadata},
    Result,
};

/// Creates the directory structure and metadata for study sets.
pub struct StudySetCreator {
    writer: WorkspaceWriter,
    instructions: Box<dyn AgentInstructionsTemplate>,
}

impl Default for StudySetCreator {
    fn default() -> Self {
        Self {
            writer: WorkspaceWriter::default(),
            instructions: Box::<StudySetAgentInstructions>::default(),
        }
    }
}

impl StudySetCreator {
    /// Creates a study set under the provided parent directory.
    pub fn create(&self, parent: &Path, set_name: &str) -> Result<()> {
        let metadata = StudySetMetadata::new(set_name);
        let layout = StudySetLayout::new(parent.join(set_name));
        self.writer
            .create_directories(&layout.initial_directories())?;
        self.writer
            .write_text(&layout.metadata_path(), &toml::to_string_pretty(&metadata)?)?;
        self.writer.write_text(
            &layout.agent_instructions_path(),
            &self.instructions.render(&metadata),
        )
    }
}
